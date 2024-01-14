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
  const numbersInput = document.getElementById('numbers-input');
  const numbersAlgo1 = document.getElementById('numbers-algo1');
  const numbersAlgo2 = document.getElementById('numbers-algo2');
  const numbersAlgo3 = document.getElementById('numbers-algo3');
  const numbersAlgo4 = document.getElementById('numbers-algo4');
  const numbersKey = document.getElementById('numbers-input2');
  const numbersBtn = document.getElementById('numbers-btn');
  let numbersOutput = document.getElementById('numbers-output');
  let updateKeyVisibility = () => {
    numbersKey.hidden = !numbersAlgo3.checked;
  };
  numbersAlgo1.addEventListener('click', () => updateKeyVisibility());
  numbersAlgo2.addEventListener('click', () => updateKeyVisibility());
  numbersAlgo3.addEventListener('click', () => updateKeyVisibility());
  numbersAlgo4.addEventListener('click', () => updateKeyVisibility());
  let data = {};
  let update = () => {
    data = {};
    data.input = numbersInput.value;
    if (numbersOutput.tagName != "UL") {
      numbersOutput.outerHTML = '<ul id="numbers-output"></ul>'
      numbersOutput = document.getElementById('numbers-output');
    }
    if (numbersAlgo1.checked) {
      data.algorithm = "numbers";
      let num = numbers(data.input);
      numbersOutput.innerHTML = "";
      if (num) numbersOutput.innerHTML += "<li>" + num + "</li>";
    } else if (numbersAlgo2.checked) {
      data.algorithm = "reverse-numbers";
      numbersOutput.innerHTML = "";
      for (let num of reverse_numbers(data.input)) {
        numbersOutput.innerHTML += "<li>" + num + "</li>";
      }
    } else if (numbersAlgo3.checked) {
      data.algorithm = "aes";
      data.key = numbersKey.value;
      numbersOutput.innerHTML = "";
      let dec = decrypt(data.input, data.key);
      if (dec) {
        numbersOutput.innerHTML = '<li id="dec"></li>';
        document.getElementById('dec').innerText = dec;
        document.getElementById('dec').id = "";
      }
    } else if (numbersAlgo4.checked) {
      data.algorithm = "shift";
      if (numbersOutput.tagName != "OL") {
        numbersOutput.outerHTML = '<ol id="numbers-output" start="0"></ol>'
        numbersOutput = document.getElementById('numbers-output');
      }
      for (let val of shift(data.input)) {
        numbersOutput.innerHTML += '<li><pre id="dec"></pre></li>';
        document.getElementById('dec').innerText = val;
        document.getElementById('dec').id = "";
      }
    }
    window.location.hash = compress(JSON.stringify(data));
  };
  numbersBtn.addEventListener('click', () => update());
  if (window.location.hash) {
    try {
      let data = JSON.parse(decompress(window.location.hash.replace('#', '')));
      if (data.algorithm == "numbers") {
        numbersAlgo1.checked = true;
      } else if (data.algorithm == "reverse-numbers") {
        numbersAlgo2.checked = true;
      } else if (data.algorithm == "aes") {
        numbersAlgo3.checked = true;
        numbersKey.value = data.key;
      } else if (data.algorithm == "shift") {
        numbersAlgo4.checked = true;
      }
      numbersInput.value = data.input;
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
