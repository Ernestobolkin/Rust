function caesarCipher(str, shift) {
    return str.split('')
        .map(char => {
            if (char.match(/[a-z]/i)) {
                let code = char.charCodeAt(0);
                let shiftOffset = code >= 65 && code <= 90 ? 65 : 97;
                return String.fromCharCode(((code - shiftOffset + shift) % 26) + shiftOffset);
            }
            return char;
        })
        .join('');
}

function encrypt(text, shift) {
    return caesarCipher(text, shift);
}

function decrypt(text, shift) {
    return caesarCipher(text, -shift);
}

const encrypted = encrypt("HelloWorld", 3);
const decrypted = decrypt(encrypted, 3);
console.log(encrypted); // KhoorZruog
console.log(decrypted); // HelloWorld




//if 

function ifCheck() {
    let num = 1

    if (num) {
        console.log("inside if")
    }
}