import { hash, verify, Options } from 'argon2';
import { checkAndGetEnv } from '../utils/utils';
import { randomBytes } from 'crypto';
import {QueryParam} from "../db/pgWrapper";
const xsalsa20 = require('xsalsa20');

const argonOptions = {
  memoryCost: 4096,
  timeCost: 4096,
  parallelism: 1,
} as Options & { raw: true };

export const hashPassword = async (password: string): Promise<string> => {
  const passwordHash = await hash(password, argonOptions);

  const cipherKey = checkAndGetEnv('CIPHER_KEY');

  const bufferNonce = randomBytes(32);
  const passwordBuffer = Buffer.from(passwordHash);

  const xor = xsalsa20(bufferNonce, Buffer.from(cipherKey, 'utf-8'));
  let cipher = Buffer.from(xor.update(passwordBuffer)).toString('hex');

  return `${bufferNonce.toString('hex')}$${cipher}`;
};

export const comparePasswords = async (password: string, hash: string): Promise<boolean> => {
  const cipherKey = checkAndGetEnv('CIPHER_KEY');

  const [nonce, cipher] = hash.split('$');
  const cipherBuffer = Buffer.from(cipher, 'hex');
  const bufferNonce = Buffer.from(nonce, 'hex');

  const xor = xsalsa20(bufferNonce, Buffer.from(cipherKey, 'utf-8'));
  const passwordHash = Buffer.from(xor.update(cipherBuffer)).toString('utf-8');

  return verify(passwordHash, password, argonOptions);
};

const encryptElem = (elem: QueryParam): QueryParam => {
  let data: string;
  if (typeof elem === 'string') {
    data = elem;
  } else {
    data = elem + '';
  }
  const cipherKey = checkAndGetEnv('CIPHER_KEY');

  const bufferNonce = randomBytes(32);
  const dataBuffer = Buffer.from(data);

  const xor = xsalsa20(bufferNonce, Buffer.from(cipherKey, 'utf-8'));
  let cipher = Buffer.from(xor.update(dataBuffer)).toString('hex');

  return `${bufferNonce.toString('hex')}$${cipher}`;
};

export const encryptParams = (params: QueryParam[]): QueryParam[] => {
  return params.map(p => encryptElem(p));
};

const decrypt = (data: string, key: string): string => {
  const [nonce, cipher] = data.split('$');
  const cipherBuffer = Buffer.from(cipher, 'hex');
  const bufferNonce = Buffer.from(nonce, 'hex');

  const xor = xsalsa20(bufferNonce, Buffer.from(key, 'utf-8'));
  return Buffer.from(xor.update(cipherBuffer)).toString('utf-8');
};

export const decryptQueryResult = (res: any): any => {
  const encryptedRows: any[] = res.rows;
  const rows: any[] = [];
  const cipherKey = checkAndGetEnv('CIPHER_KEY');
  for (const row of encryptedRows) {
    const newRow: any = {};
    for (const field in row) {
      if (row.hasOwnProperty(field)) {
        newRow[field] = decrypt(row[field], cipherKey);
      }
    }
    rows.push(newRow);
  }
  return { ...res, rows };
};
