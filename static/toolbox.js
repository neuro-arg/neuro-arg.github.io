import init_wasm from "./rust/neuro_arg_wasm.js";
import {
  compress,
  decompress,
  decrypt,
  shift_key,
  numbers,
  reverse_numbers,
  rust_init,
  shift,
} from "./rust/neuro_arg_wasm.js";
let loaded1 = false;
let loaded2 = false;
let onLoad = () => {
  // test
  let algos = {
    numbers: 0,
    "reverse-numbers": 1,
    aes: 2,
    shift: 3,
    "keyed-shift": 4,
  };
  const algoElems = [];
  for (let i = 1; document.getElementById('algo' + i); ++i) {
    algoElems.push(document.getElementById('algo' + i));
  }
  const inputElem = document.getElementById('input');
  const checkInvElem = document.getElementById('check-inv');
  const checkIgnoreSpacesElem = document.getElementById('check-ignore-spaces');
  const keyElem = document.getElementById('input2');
  const btnElem = document.getElementById('btn');
  const outElem1 = document.getElementById('output1');
  const outElem2 = document.getElementById('output2');
  let updateVisibility = () => {
    let vis = {};
    for (let algo in algos) {
      for (let elem of document.getElementsByClassName('algo-' + algo)) {
        console.log(elem);
        if (vis[elem.id]) continue;
        elem.hidden = !algoElems[algos[algo]].checked;
        if (!elem.hidden) {
          vis[elem.id] = true;
        }
      }
    }
  };
  for (let elem of algoElems) {
    elem.addEventListener('click', () => updateVisibility());
  }
  let update = () => {
    let data = {};
    data.input = inputElem.value;
    outElem1.hidden = true;
    outElem2.hidden = true;
    outElem1.innerHTML = "";
    outElem2.innerHTML = "";
    const outElem = (algoElems[algos.shift].checked || algoElems[algos["keyed-shift"]].checked) ? outElem2 : outElem1;
    outElem.hidden = false;
    if (algoElems[algos.numbers].checked) {
      data.algorithm = "numbers";
      let num = numbers(data.input);
      if (num) outElem.innerHTML += "<li>" + num + "</li>";
    } else if (algoElems[algos["reverse-numbers"]].checked) {
      data.algorithm = "reverse-numbers";
      for (let num of reverse_numbers(data.input)) {
        outElem.innerHTML += "<li>" + num + "</li>";
      }
    } else if (algoElems[algos.aes].checked) {
      data.algorithm = "aes";
      data.key = keyElem.value;
      let dec = decrypt(data.input, data.key);
      if (dec) {
        outElem.innerHTML = '<li id="dec"></li>';
        document.getElementById('dec').innerText = dec;
        document.getElementById('dec').id = "";
      }
    } else if (algoElems[algos.shift].checked) {
      data.algorithm = "shift";
      for (let val of shift(data.input)) {
        outElem.innerHTML += '<li><pre id="dec"></pre></li>';
        document.getElementById('dec').innerText = val;
        document.getElementById('dec').id = "";
      }
    } else if (algoElems[algos["keyed-shift"]].checked) {
      data.algorithm = "keyed-shift";
      data.key = keyElem.value;
      data.inv = checkInvElem.checked;
      data.ignore_spaces = checkIgnoreSpacesElem.checked;
      for (let val of shift_key(data.input, data.key, data.inv, data.ignore_spaces)) {
        outElem.innerHTML += '<li><pre id="dec"></pre></li>';
        document.getElementById('dec').innerText = val;
        document.getElementById('dec').id = "";
      }
    }
    window.location.hash = compress(JSON.stringify(data));
  };
  btnElem.addEventListener('click', () => update());
  inputElem.addEventListener('keypress', event => {
    if (event.key == "Enter") update();
  });
  keyElem.addEventListener('keypress', event => {
    if (event.key == "Enter") update();
  });
  if (window.location.hash) {
    try {
      let data = JSON.parse(decompress(window.location.hash.replace('#', '')));
      algoElems[algos[data.algorithm]].checked = true;
      if (data.key) keyElem.value = data.key;
      inputElem.value = data.input;
      update();
    } catch { }
  }
  updateVisibility();
};
init_wasm().then(_wasm => {
  rust_init();
  loaded1 = true;
  if (loaded2) onLoad();
});
document.addEventListener('DOMContentLoaded', () => {
  loaded2 = true;
  if (loaded1) onLoad();
});
