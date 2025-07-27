import {encode, decode} from "vinegar";

const encodeBtn = document.getElementById("encode");
const decodeBtn = document.getElementById("decode");
const clearBtn = document.getElementById("clear");
const plainText = document.getElementById("plain");
const cipherText = document.getElementById("cipher");

encodeBtn.onclick = () => {
    cipherText.value = encode(plainText.value);
};

decodeBtn.onclick = () => {
    plainText.value = decode(cipherText.value);
};

clearBtn.onclick = () => {
    plainText.value = "";
    cipherText.value = "";
};