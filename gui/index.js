import {encode, decode} from "vinegar";

const encodeBtn = document.getElementById("encode");
const decodeBtn = document.getElementById("decode");
const clearBtn = document.getElementById("clear");
const plainText = document.getElementById("plain");
const cipherText = document.getElementById("cipher");
const paramSlider = document.getElementById("param-slider");
const paramValue = document.getElementById("param-value");
const keywordInput = document.getElementById("keyword-input");

paramSlider.oninput = () => {
    paramValue.textContent = paramSlider.value;
};

encodeBtn.onclick = () => {
    const param = parseInt(paramSlider.value);
    const keyword = keywordInput.value;
    cipherText.value = encode(plainText.value, param, keyword);
};

decodeBtn.onclick = () => {
    const param = parseInt(paramSlider.value);
    const keyword = keywordInput.value;
    plainText.value = decode(cipherText.value, param, keyword);
};

clearBtn.onclick = () => {
    plainText.value = "";
    cipherText.value = "";
};
