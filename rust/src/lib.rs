use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
};

use aes::cipher::{BlockDecrypt, KeyInit};
use base64::Engine;
use flate2::Compression;
use itertools::Itertools;
use js_sys::BigInt;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug)]
pub struct Shift(Vec<char>);
impl Shift {
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        Self(s.as_ref().chars().collect())
    }
    pub fn with_key<S: AsRef<str>, K: AsRef<str>>(
        s: S,
        k: K,
        inv: bool,
        ignore_spaces: bool,
    ) -> Self {
        let mut s: Vec<_> = s.as_ref().chars().collect();
        let minmax_val = inv
            .then(|| k.as_ref().chars().minmax().into_option())
            .flatten();
        let mut k1 = k.as_ref().chars();
        let mut k = std::iter::from_fn(|| {
            (if let Some(v) = k1.next() {
                Some(v)
            } else {
                k1 = k.as_ref().chars();
                k1.next()
            })
            .map(|x| {
                minmax_val
                    .and_then(|(min, max)| char::from_u32(max as u32 - x as u32 + min as u32))
                    .unwrap_or(x)
            })
        });
        for x in &mut s {
            if !ignore_spaces || *x != ' ' {
                if let Some(y) = k.next().and_then(|y| char::from_u32(*x as u32 + y as u32)) {
                    *x = y;
                }
            }
        }
        Self(s)
    }
}
impl Iterator for Shift {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let ret: String = self.0.iter().copied().collect();
        let mut good = false;
        for x in &mut self.0 {
            if *x != ' ' {
                good = true;
                *x = char::from_u32(*x as u32 - 1)?;
            }
        }
        good.then_some(ret)
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

fn n3_create_grid() -> [[char; 6]; 6] {
    let mut grid = [[char::MIN; 6]; 6];
    let alphabet = "abcdefghijqlmnopqrstuvwxyz1234567890"
        .chars()
        .collect::<Vec<_>>();
    let mut index = 0;

    for row in 0..6 {
        for col in 0..6 {
            grid[row][col] = alphabet[index];
            index += 1;
        }
    }

    grid
}
fn n3_print_grid(grid: [[char; 6]; 6]) -> String {
    let mut output = String::from("");
    for row in 0..6 {
        output += "\\n";
        for col in 0..6 {
            output = format!("{}{} ", output, grid[row][col].to_string());
        }
    }
    return output
}
fn n3_rotate_grid(grid: &mut [[char; 6]; 6], key_numbers: &[usize]) {
    for i in 0..key_numbers.len() {
        let rotation_amount = key_numbers[i];

        if i < 6 {
            n3_rotate_row(grid, i % 6, rotation_amount);
        } else {
            n3_rotate_column(grid, (i - 6) % 6, rotation_amount);
        }
    }
}

fn n3_rotate_row(grid: &mut [[char; 6]; 6], row: usize, mut amount: usize) {
    amount %= 6;
    let mut temp = [char::MIN; 6];

    for col in 0..6 {
        temp[col] = grid[row][col];
    }

    for col in 0..6 {
        grid[row][col] = temp[(col.wrapping_sub(amount).wrapping_add(6)) % 6]
    }
}

fn n3_rotate_column(grid: &mut [[char; 6]; 6], col: usize, mut amount: usize) {
    amount %= 6;
    let mut temp = [char::MIN; 6];

    for row in 0..6 {
        temp[row] = grid[row][col];
    }

    for row in 0..6 {
        grid[row][col] = temp[(row.wrapping_sub(amount).wrapping_add(6)) % 6];
    }
}

pub fn numbers_III_(src: &str, key: Option<&str>) -> Option<String> {
    let mut grid = n3_create_grid();
    let mut output = String::from("Original Grid:");

    output += &n3_print_grid(grid);

    let mut key_numbers: Vec<&str> = src
        .split(" ")
        .collect();
    for num in key_numbers.iter() {
        let mut numbers: Vec<usize>;
        if num.contains(",") {
            numbers = num.split(',')
                .map(|part| part.parse::<usize>().unwrap())
                .collect();
        } else {
            numbers = num.chars()
                .map(|part| part.to_digit(10).unwrap() as usize)
                .collect(); 
        }
        n3_rotate_grid(&mut grid, &numbers);
        //output += " \\nEncrypted Grid after Rotating\\n";
        //output += &n3_print_grid(grid);
    }
    

    output += " \\n \\nEncrypted Grid after Rotating";
    output += &n3_print_grid(grid);
    
    Some(
        output.to_string()
    )
}

#[wasm_bindgen]
pub fn numbers_III(src: &str, key: Option<String>) -> JsValue {
    match numbers_III_(src, key.as_deref()) {
        Some(s) => JsValue::from_str(&s),
        None => JsValue::NULL,
    }
}

pub fn numbers_(src: &str, key: Option<&str>) -> Option<String> {
    let mut num = format!("2{src}91");
    num = format!("{}6", num.parse::<BigInt>().ok()? * BigInt::from(5));
    num = num
        .chars()
        .rev()
        .map(|x| if x == '2' { '3' } else { x })
        .collect();
    num = format!("17{}24", num.parse::<BigInt>().ok()? * BigInt::from(9));
    let key = key.unwrap_or("abcdef");
    let mut map = ['\0'; 10];
    for (a, b) in src.chars().zip(key.chars()) {
        map[(a as u8 - b'0') as usize] = b;
    }
    Some(
        num.chars()
            .map(|x| match x {
                '0'..='9' => {
                    let k = (x as u8 - b'0') as usize;
                    if map[k] == '\0' {
                        x
                    } else {
                        map[k]
                    }
                }
                x => x,
            })
            .collect(),
    )
}

#[wasm_bindgen]
pub fn numbers(src: &str, key: Option<String>) -> JsValue {
    match numbers_(src, key.as_deref()) {
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
        .filter(|x| matches!(numbers_(x, None), Some(x) if x == src))
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

#[wasm_bindgen]
pub fn shift_key(s: &str, k: &str, inv: bool, ignore_spaces: bool) -> JsValue {
    let ret = js_sys::Array::new();
    for val in Shift::with_key(s, k, inv, ignore_spaces) {
        ret.push(&JsValue::from_str(&val));
    }
    ret.into()
}

pub fn vigenere_(s: &str, k: &str, alphabet: &str, inv: bool) -> String {
    let map: BTreeMap<_, _> = alphabet.chars().enumerate().map(|(a, b)| (b, a)).collect();
    let k: Vec<_> = k.chars().filter_map(|c| map.get(&c)).collect();
    let alphabet: Vec<_> = alphabet.chars().collect();

    let mut k1 = k.iter().copied();
    let mut k = std::iter::from_fn(|| {
        if let Some(v) = k1.next() {
            Some(v)
        } else {
            k1 = k.iter().copied();
            k1.next()
        }
    });
    s.chars()
        .map(|x| {
            map.get(&x)
                .and_then(|x| {
                    k.next().and_then(|k| {
                        alphabet
                            .get(
                                (if inv { x + k } else { x + alphabet.len() - k }) % alphabet.len(),
                            )
                            .copied()
                    })
                })
                .unwrap_or(x)
        })
        .collect()
}

#[wasm_bindgen]
pub fn vigenere(s: &str, k: &str, alphabet: &str, inv: bool) -> JsValue {
    JsValue::from_str(vigenere_(s, k, alphabet, inv).as_str())
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test() {
        let test_data = r#"{"data":"a","algo":"test"}"#;
        assert_eq!(
            decompress_(&compress_(test_data).unwrap()).unwrap(),
            test_data
        );
        assert_eq!(
            vigenere_("abcdefgh", "test", "abcdefghijklmnopqrstuvwxyz", false),
            "hxkklboo"
        );
    }

    #[wasm_bindgen_test]
    fn wasm_test() {
        assert_eq!(
            numbers_("572943", Some("abcdef")),
            Some("1bad0fcabc1ebdce".to_owned())
        );
        assert_eq!(
            numbers_(
                "5729438873698993183185",
                Some("92270bf339b1a31d0498defb0573fc7c")
            ),
            Some("83e1090eeb3b82e0e933802e32803120".to_owned())
        );
        assert!(reverse_numbers_("1bad0fcabc1ebdce").contains(&"572943".to_owned()));
    }
}
