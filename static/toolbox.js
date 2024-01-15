import init_wasm from "./rust/neuro_arg_wasm.js";
import {
  compress,
  decompress,
  decrypt,
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
    "reverse-numbers": 1, reverse_numbers: 1,
    aes: 2,
    shift: 3,
  };
  const algoElems = [];
  for (let i = 1; document.getElementById('numbers-algo' + i); ++i) {
    algoElems.push(document.getElementById('numbers-algo' + i));
  }
  const inputElem = document.getElementById('numbers-input');
  const keyElem = document.getElementById('numbers-input2');
  const btnElem = document.getElementById('numbers-btn');
  let outElem = document.getElementById('numbers-output');
  let updateKeyVisibility = () => {
    keyElem.hidden = !algoElems[algos.aes].checked;
  };
  for (let elem of algoElems) {
    elem.addEventListener('click', () => updateKeyVisibility());
  }
  let update = () => {
    let data = {};
    data.input = inputElem.value;
    if (outElem.tagName != "UL") {
      outElem.outerHTML = '<ul id="numbers-output"></ul>'
      outElem = document.getElementById('numbers-output');
    }
    if (algoElems[algos.numbers].checked) {
      data.algorithm = "numbers";
      let num = numbers(data.input);
      outElem.innerHTML = "";
      if (num) outElem.innerHTML += "<li>" + num + "</li>";
    } else if (algoElems[algos.reverse_numbers].checked) {
      data.algorithm = "reverse-numbers";
      outElem.innerHTML = "";
      for (let num of reverse_numbers(data.input)) {
        outElem.innerHTML += "<li>" + num + "</li>";
      }
    } else if (algoElems[algos.aes].checked) {
      data.algorithm = "aes";
      data.key = keyElem.value;
      outElem.innerHTML = "";
      let dec = decrypt(data.input, data.key);
      if (dec) {
        outElem.innerHTML = '<li id="dec"></li>';
        document.getElementById('dec').innerText = dec;
        document.getElementById('dec').id = "";
      }
    } else if (algoElems[algos.shift].checked) {
      data.algorithm = "shift";
      if (outElem.tagName != "OL") {
        outElem.outerHTML = '<ol id="numbers-output" start="0"></ol>'
        outElem = document.getElementById('numbers-output');
      }
      for (let val of shift(data.input)) {
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
  updateKeyVisibility();
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
