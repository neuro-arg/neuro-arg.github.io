use std::{collections::BTreeSet, io::Write};

use aes::cipher::{BlockDecrypt, KeyInit};
use base64::Engine;
use flate2::Compression;
use itertools::Itertools;
use js_sys::BigInt;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug)]
pub struct Shift(String, u32);
impl From<String> for Shift {
    fn from(value: String) -> Self {
        Self(value, 0)
    }
}
impl Shift {
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        Self::from(s.as_ref().to_owned())
    }
}
impl Iterator for Shift {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let mut valid = false;
        let ret = self
            .0
            .chars()
            .filter_map(|x| {
                let val = char::from_u32((x as u32).saturating_sub(self.1).max(b' ' as u32));
                if matches!(val, Some(x) if x != ' ') {
                    valid = true;
                }
                val
            })
            .collect();
        self.1 += 1;
        valid.then_some(ret)
    }
}
impl std::iter::FusedIterator for Shift {}

struct MiniSet([bool; 256]);
impl Default for MiniSet {
    fn default() -> Self {
        Self([false; 256])
    }
}
impl MiniSet {
    fn len(&self) -> usize {
        self.0.iter().copied().filter(|x| *x).count()
    }
    fn contains(&self, x: u8) -> bool {
        self.0[x as usize]
    }
    fn iter(&self) -> impl '_ + Iterator<Item = u8> {
        self.0
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, x)| *x)
            .map(|(i, _)| i as u8)
    }
}
impl FromIterator<char> for MiniSet {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut ret = Self::default();
        for ch in iter {
            if let Ok(val) = u8::try_from(ch) {
                ret.0[val as usize] = true;
            }
        }
        ret
    }
}
struct MiniMap([u8; 256]);
impl Default for MiniMap {
    fn default() -> Self {
        Self([0xFF; 256])
    }
}
impl MiniMap {
    fn get(&self, n: u8) -> Option<u8> {
        Some(self.0[n as usize]).filter(|x| *x != 0xFF)
    }
    fn insert(&mut self, k: u8, v: u8) {
        self.0[k as usize] = v;
    }
}
impl FromIterator<(u8, u8)> for MiniMap {
    fn from_iter<T: IntoIterator<Item = (u8, u8)>>(iter: T) -> Self {
        let mut ret = Self::default();
        for (a, b) in iter {
            ret.0[a as usize] = b;
        }
        ret
    }
}

pub fn numbers_(src: &str) -> Option<String> {
    let mut num = format!("2{src}91");
    num = format!("{}6", num.parse::<BigInt>().ok()? * BigInt::from(5));
    num = num
        .chars()
        .rev()
        .map(|x| if x == '2' { '3' } else { x })
        .collect();
    num = format!("17{}24", num.parse::<BigInt>().ok()? * BigInt::from(9));
    for (b, a) in src
        .chars()
        .take(6)
        .collect::<Vec<_>>()
        .into_iter()
        .enumerate()
        .rev()
    {
        num = num.replace(a, std::str::from_utf8(&[b'a' + b as u8]).unwrap());
    }
    Some(num)
}

#[wasm_bindgen]
pub fn numbers(src: &str) -> JsValue {
    match numbers_(src) {
        Some(s) => JsValue::from_str(&s),
        None => JsValue::NULL,
    }
}

pub fn reverse_numbers_(src: &str) -> Vec<String> {
    let set = MiniSet::from_iter(src.chars());
    // sanity check: up to 10 digits
    if set.len() > 10 {
        return vec![];
    }
    // sanity check: all are valid hex digits
    if set.iter().any(|x| !x.is_ascii_hexdigit()) {
        return vec![];
    }
    // sanity check: the start is 17 or 24 (may be partially replaced with hex)
    let mut map = (0..10).map(|i| (b'0' + i, i)).collect::<MiniMap>();
    let mut s = src.to_owned();
    let Some(first2) = s.get(..2) else {
        return vec![];
    };
    let Some(last2) = s.get(s.len() - 2..) else {
        return vec![];
    };
    if !first2
        .to_owned()
        .bytes()
        .chain(last2.to_owned().bytes())
        .zip([1, 7, 2, 4])
        .all(|(x, y)| {
            if let Some(w) = map.get(x) {
                w == y
            } else {
                s = s.replace(x as char, std::str::from_utf8(&[b'0' + y]).unwrap());
                map.insert(x, y);
                true
            }
        })
    {
        return vec![];
    }
    let set = MiniSet::from_iter(s.chars());
    // missing decimal digits
    let missing_digits: Vec<_> = (0u8..10u8).filter(|x| !set.contains(b'0' + x)).collect();
    // all hex digits
    let hex: Vec<_> = set
        .iter()
        .filter(|x| x.is_ascii_alphabetic())
        .map(|x| x - b'a' + 10)
        .collect();
    if missing_digits.len() < hex.len() {
        return vec![];
    }
    let mut ret = vec![];
    for ms in missing_digits
        .iter()
        .copied()
        .permutations(missing_digits.len())
    {
        let mut s = s.clone();
        for (k, v) in hex.iter().copied().zip(ms.into_iter()) {
            s = s.replace(
                (k - 10 + b'a') as char,
                std::str::from_utf8(&[v + b'0']).unwrap(),
            );
        }
        if let Some(s) = s
            .strip_prefix("17")
            .and_then(|s| s.strip_suffix("24"))
            .and_then(|s| s.parse::<BigInt>().ok())
            .and_then(|num| {
                let nine = BigInt::from(9);
                if num.clone() % nine.clone() == 0 {
                    num.checked_div(&nine).ok()
                } else {
                    None
                }
            })
            .and_then(|num| num.to_string(10).ok())
            .map(|s| format!("{s}"))
            .map(|s| s.chars().rev().collect::<String>())
        {
            let Some(s) = s.strip_suffix('6') else {
                continue;
            };
            // any 3 may or may not come from a 2
            for s in s
                .chars()
                .map(|x| match x {
                    '3' => Some('2').into_iter().chain(Some('3')),
                    '2' => None.into_iter().chain(None),
                    x => Some(x).into_iter().chain(None),
                })
                .multi_cartesian_product()
                .filter_map(|x| {
                    x.into_iter()
                        .collect::<String>()
                        .parse::<BigInt>()
                        .ok()
                        .and_then(|num| {
                            let five = BigInt::from(5);
                            if num.clone() % five.clone() == 0 {
                                num.checked_div(&five).ok()
                            } else {
                                None
                            }
                        })
                        .and_then(|num| num.to_string(10).ok())
                        .map(|s| format!("{s}"))
                })
            {
                if let Some(s) = s.strip_suffix("91").and_then(|s| s.strip_prefix('2')) {
                    ret.push(s.to_owned());
                }
            }
        }
    }
    ret.into_iter()
        .filter(|x| matches!(numbers_(x), Some(x) if x == src))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[wasm_bindgen]
pub fn reverse_numbers(s: &str) -> JsValue {
    let nums = reverse_numbers_(s);
    let arr = js_sys::Array::new_with_length(nums.len() as u32);
    for (i, num) in nums.into_iter().enumerate() {
        arr.set(i as u32, JsValue::from_str(&num));
    }
    arr.into()
}

#[wasm_bindgen]
pub fn rust_init() {
    console_error_panic_hook::set_once();
}

pub fn unpad(data: &mut Vec<u8>) -> bool {
    if let Some(count) = data.last().copied() {
        if count == 0
            || data[data.len() - usize::from(count)..]
                .iter()
                .copied()
                .any(|x| x != count)
        {
            false
        } else {
            data.truncate(data.len() - usize::from(count));
            true
        }
    } else {
        false
    }
}

pub fn decrypt1<C: BlockDecrypt>(crypt: C, data: &mut [u8]) -> bool {
    if data.len() % 16 == 0 {
        for chunk in data.chunks_mut(16) {
            crypt.decrypt_block(chunk.into());
        }
        true
    } else {
        false
    }
}

pub fn decrypt_(data: &str, key: &str) -> Option<String> {
    let data = data
        .bytes()
        .map(|x| match x {
            b'-' => b'/',
            b'_' => b'+',
            x => x,
        })
        .filter(|x| x.is_ascii_alphanumeric() || matches!(x, b'=' | b'/' | b'+'))
        .collect::<Vec<_>>();
    let mut data = base64::engine::general_purpose::STANDARD
        .decode(data)
        .ok()?;
    let key = key.as_bytes();
    if !match key.len() * 8 {
        128 => decrypt1(aes::Aes128::new_from_slice(key).ok()?, &mut data),
        192 => decrypt1(aes::Aes192::new_from_slice(key).ok()?, &mut data),
        256 => decrypt1(aes::Aes256::new_from_slice(key).ok()?, &mut data),
        _ => false,
    } {
        return None;
    }
    unpad(&mut data)
        .then_some(data)
        .and_then(|x| String::from_utf8(x).ok())
}

#[wasm_bindgen]
pub fn decrypt(s: &str, k: &str) -> JsValue {
    match decrypt_(s, k) {
        Some(s) => JsValue::from_str(&s),
        None => JsValue::NULL,
    }
}

pub fn compress_(s: &str) -> Option<String> {
    let mut enc = flate2::write::DeflateEncoder::new(vec![], Compression::best());
    enc.write_all(s.as_bytes()).ok()?;
    Some(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(enc.finish().ok()?))
}
#[wasm_bindgen]
pub fn compress(s: &str) -> JsValue {
    match compress_(s) {
        Some(s) => JsValue::from_str(&s),
        None => JsValue::NULL,
    }
}
pub fn decompress_(s: &str) -> Option<String> {
    let mut dec = flate2::write::DeflateDecoder::new(vec![]);
    dec.write_all(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(s)
            .ok()?,
    )
    .ok()?;
    String::from_utf8(dec.finish().ok()?).ok()
}
#[wasm_bindgen]
pub fn decompress(s: &str) -> JsValue {
    match decompress_(s) {
        Some(s) => JsValue::from_str(&s),
        None => JsValue::NULL,
    }
}

#[wasm_bindgen]
pub fn shift(s: &str) -> JsValue {
    let ret = js_sys::Array::new();
    for val in Shift::new(s) {
        ret.push(&JsValue::from_str(&val));
    }
    ret.into()
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    fn test() {
        let test_data = r#"{"data":"a","algo":"test"}"#;
        assert_eq!(
            decompress_(&compress_(test_data).unwrap()).unwrap(),
            test_data
        );
    }

    #[wasm_bindgen_test]
    fn wasm_test() {
        assert_eq!(numbers_("572943"), Some("1bad0fcabc1ebdce".to_owned()));
        assert!(reverse_numbers_("1bad0fcabc1ebdce").contains(&"572943".to_owned()));
    }
}
