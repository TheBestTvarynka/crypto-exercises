"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.comparePasswords = exports.hashPassword = void 0;
const argon2_1 = require("argon2");
const utils_1 = require("../utils/utils");
const xsalsa20 = require('xsalsa20');
const crypto_1 = require("crypto");
const argonOptions = {
    memoryCost: 4096,
    timeCost: 4096,
    parallelism: 1,
};
const hashPassword = async (password) => {
    const passwordHash = await argon2_1.hash(password, argonOptions);
    const cipherKey = utils_1.checkAndGetEnv('CIPHER_KEY');
    const bufferNonce = crypto_1.randomBytes(32);
    const passwordBuffer = Buffer.from(passwordHash);
    const xor = xsalsa20(bufferNonce, Buffer.from(cipherKey, 'utf-8'));
    let cipher = Buffer.from(xor.update(passwordBuffer)).toString('hex');
    return `${bufferNonce.toString('hex')}$${cipher}`;
};
exports.hashPassword = hashPassword;
const comparePasswords = async (password, hash) => {
    const cipherKey = utils_1.checkAndGetEnv('CIPHER_KEY');
    const [nonce, cipher] = hash.split('$');
    const cipherBuffer = Buffer.from(cipher, 'hex');
    const bufferNonce = Buffer.from(nonce, 'hex');
    const xor = xsalsa20(bufferNonce, Buffer.from(cipherKey, 'utf-8'));
    const passwordHash = Buffer.from(xor.update(cipherBuffer)).toString('utf-8');
    return argon2_1.verify(passwordHash, password, argonOptions);
};
exports.comparePasswords = comparePasswords;
//# sourceMappingURL=utils.js.map