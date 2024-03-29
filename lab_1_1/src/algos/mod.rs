use crate::utils::check_if_meaningful_text;
use std::collections::HashMap;

pub mod frequency;
pub mod substitution_genetic_algorithm;

fn hack_ceaser(data: &[u8]) {
    for i in 0..=255 {
        println!("--------------------------------------- {}", i);
        let res = data.iter().map(|c| c ^ i).collect::<Vec<u8>>();
        let s = String::from_utf8(res);
        if s.is_ok() {
            println!("{}", s.unwrap());
        }
    }
}

fn calculate_probability(data: &Vec<u8>, key_length: usize) -> f32 {
    let len = data.len() / key_length;
    let mut text = Vec::with_capacity(len + 1);

    let mut i = 0;
    while i < data.len() {
        text.push(data.get(i).unwrap());
        i += key_length;
    }

    let mut frequencies = HashMap::new();
    for c in text {
        let new_f = match frequencies.get(c) {
            Some(f) => f + 1,
            None => 1,
        };
        frequencies.insert(*c, new_f);
    }

    frequencies
        .values()
        .into_iter()
        .map(|frequency| frequency * (frequency - 1))
        .fold(0.0, |sum, value| sum + value as f32)
        / (len * (len - 1)) as f32
}

fn calculate_probabilities(data: &Vec<u8>) -> Vec<f32> {
    let max_key_length = data.len() / 2;
    let mut probabilities = vec![0.0; max_key_length];

    // min key-length = 2
    for key_length in 2..=max_key_length {
        probabilities[key_length - 2] = calculate_probability(data, key_length);
    }

    probabilities
}

pub fn decrypt_xor_vigenere(data: &[u8], key: &[u8]) -> Vec<u8> {
    let data_len = data.len();
    let mut decoded = vec![0; data_len];

    let mut i = 0;
    'main: loop {
        for k_value in key {
            if i >= data_len {
                break 'main;
            }
            decoded[i] = data[i] ^ k_value;
            i += 1;
        }
    }

    decoded
}

pub fn decrypt_shift_vigenere(data: &[u8], key: &[u8]) -> Vec<u8> {
    let data_len = data.len();
    let mut decoded = vec![0; data_len];

    let mut i = 0;
    'main: loop {
        for k_value in key {
            if i >= data_len {
                break 'main;
            }
            let mut new_c = data[i] - k_value;
            decoded[i] = if new_c < 65 { 26 + new_c } else { new_c };
            i += 1;
        }
    }

    decoded
}

pub fn encrypt_shift_vigenere(data: &[u8], key: &[u8]) -> Vec<u8> {
    let data_len = data.len();
    let mut encoded = vec![0; data_len];

    let mut i = 0;
    'main: loop {
        for k_value in key {
            if i >= data_len {
                break 'main;
            }
            let mut new_c = data[i] + k_value;
            encoded[i] = if new_c < 90 { new_c - 26 } else { new_c };
            i += 1;
        }
    }

    encoded
}

fn hack_vigenere(data: &[u8]) {
    for first in 1..255 {
        for second in 1..255 {
            for third in 1..255 {
                let key = [first, second, third];
                match check_if_meaningful_text(decrypt_xor_vigenere(data, &key)) {
                    Ok(s) => println!("{:?} {}", key, s),
                    Err(_) => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algos::{
        calculate_probabilities, decrypt_shift_vigenere, encrypt_shift_vigenere, hack_ceaser,
        hack_vigenere,
    };
    use crate::utils::{decode_base64, hex_str_to_bytes};

    #[test]
    fn test_simple_decrypt_shift_vigenere_with_key() {
        println!("test simple");

        let cipher = b"BBB";
        let key = [1, 1, 1];

        let decrypted = decrypt_shift_vigenere(cipher, &key);

        assert_eq!(decrypted, vec![65, 65, 65]);

        let cipher = b"BBB";
        let key = [5, 5, 5];

        let decrypted = decrypt_shift_vigenere(cipher, &key);

        assert_eq!(decrypted, vec![87, 87, 87]);

        let cipher = b"AAA";
        let key = [1, 1, 1];

        let decrypted = decrypt_shift_vigenere(cipher, &key);

        assert_eq!(decrypted, vec![90, 90, 90]);
    }

    #[test]
    fn test_decrypt_shift_vigenere_with_key() {
        println!("test");

        let original = b"QWERTYUIOPASDFGHJKLZXCVBNMINSIMPLERTERMSTHEFITNESSOFAPARTICULARINDIVIDUALISMERELYTHESUMOFALLTHELOGARITHMSBASE";
        let key = b"QPOFIJMZFHICDJSK";

        let cipher = encrypt_shift_vigenere(original, key);
        let decrypted = decrypt_shift_vigenere(&cipher, key);

        assert_eq!(original, decrypted.as_slice());
    }

    #[test]
    fn test_hex_str_to_bytes() {
        let bytes = hex_str_to_bytes("7958401743454e1756174552475256435e59501a5c524e176f786517545e475f5245191772195019175e4317445f58425b531743565c521756174443455e595017d5b7ab5f525b5b58174058455b53d5b7aa175659531b17505e41525917435f52175c524e175e4417d5b7ab5c524ed5b7aa1b174f584517435f5217515e454443175b524343524517d5b7ab5fd5b7aa17405e435f17d5b7ab5cd5b7aa1b17435f5259174f584517d5b7ab52d5b7aa17405e435f17d5b7ab52d5b7aa1b17435f525917d5b7ab5bd5b7aa17405e435f17d5b7ab4ed5b7aa1b1756595317435f5259174f58451759524f4317545f564517d5b7ab5bd5b7aa17405e435f17d5b7ab5cd5b7aa175650565e591b17435f525917d5b7ab58d5b7aa17405e435f17d5b7ab52d5b7aa1756595317445817585919176e5842175a564e17424452175659175e5953524f1758511754585e59545e53525954521b177f565a5a5e595017535e4443565954521b177c56445e445c5e17524f565a5e5956435e58591b17444356435e44435e54565b17435244434417584517405f564352415245175a52435f5853174e5842175152525b174058425b5317445f584017435f52175552444317455244425b4319");
        println!("{:?}", bytes);
    }

    #[test]
    fn test_hack_ceaser() {
        hack_ceaser(&[
            121, 88, 64, 23, 67, 69, 78, 23, 86, 23, 69, 82, 71, 82, 86, 67, 94, 89, 80, 26, 92,
            82, 78, 23, 111, 120, 101, 23, 84, 94, 71, 95, 82, 69, 25, 23, 114, 25, 80, 25, 23, 94,
            67, 23, 68, 95, 88, 66, 91, 83, 23, 67, 86, 92, 82, 23, 86, 23, 68, 67, 69, 94, 89, 80,
            23, 213, 183, 171, 95, 82, 91, 91, 88, 23, 64, 88, 69, 91, 83, 213, 183, 170, 23, 86,
            89, 83, 27, 23, 80, 94, 65, 82, 89, 23, 67, 95, 82, 23, 92, 82, 78, 23, 94, 68, 23,
            213, 183, 171, 92, 82, 78, 213, 183, 170, 27, 23, 79, 88, 69, 23, 67, 95, 82, 23, 81,
            94, 69, 68, 67, 23, 91, 82, 67, 67, 82, 69, 23, 213, 183, 171, 95, 213, 183, 170, 23,
            64, 94, 67, 95, 23, 213, 183, 171, 92, 213, 183, 170, 27, 23, 67, 95, 82, 89, 23, 79,
            88, 69, 23, 213, 183, 171, 82, 213, 183, 170, 23, 64, 94, 67, 95, 23, 213, 183, 171,
            82, 213, 183, 170, 27, 23, 67, 95, 82, 89, 23, 213, 183, 171, 91, 213, 183, 170, 23,
            64, 94, 67, 95, 23, 213, 183, 171, 78, 213, 183, 170, 27, 23, 86, 89, 83, 23, 67, 95,
            82, 89, 23, 79, 88, 69, 23, 89, 82, 79, 67, 23, 84, 95, 86, 69, 23, 213, 183, 171, 91,
            213, 183, 170, 23, 64, 94, 67, 95, 23, 213, 183, 171, 92, 213, 183, 170, 23, 86, 80,
            86, 94, 89, 27, 23, 67, 95, 82, 89, 23, 213, 183, 171, 88, 213, 183, 170, 23, 64, 94,
            67, 95, 23, 213, 183, 171, 82, 213, 183, 170, 23, 86, 89, 83, 23, 68, 88, 23, 88, 89,
            25, 23, 110, 88, 66, 23, 90, 86, 78, 23, 66, 68, 82, 23, 86, 89, 23, 94, 89, 83, 82,
            79, 23, 88, 81, 23, 84, 88, 94, 89, 84, 94, 83, 82, 89, 84, 82, 27, 23, 127, 86, 90,
            90, 94, 89, 80, 23, 83, 94, 68, 67, 86, 89, 84, 82, 27, 23, 124, 86, 68, 94, 68, 92,
            94, 23, 82, 79, 86, 90, 94, 89, 86, 67, 94, 88, 89, 27, 23, 68, 67, 86, 67, 94, 68, 67,
            94, 84, 86, 91, 23, 67, 82, 68, 67, 68, 23, 88, 69, 23, 64, 95, 86, 67, 82, 65, 82, 69,
            23, 90, 82, 67, 95, 88, 83, 23, 78, 88, 66, 23, 81, 82, 82, 91, 23, 64, 88, 66, 91, 83,
            23, 68, 95, 88, 64, 23, 67, 95, 82, 23, 85, 82, 68, 67, 23, 69, 82, 68, 66, 91, 67, 25,
        ]);
        // compare output
        // when i == 55 then I got readable text:
        // Now try a repeating-key XOR cipher. E.g. it should take a string “hello world” and, given the key is “key”,
        // xor the first letter “h” with “k”, then xor “e” with “e”, then “l” with “y”, and then xor next char “l” with
        // “k” again, then “o” with “e” and so on. You may use an index of coincidence, Hamming distance, Kasiski
        // examination, statistical tests or whatever method you feel would show the best result.
    }

    #[test]
    fn test_find_key_length() {
        let cipher_text = decode_base64("repeating-key_xor_cipher.txt");
        let probabilities = calculate_probabilities(&cipher_text);
        println!("{:?}", probabilities);
        // I compare resulting values and notice that key_length = 3
        // I'll insert a chart in the report
        println!("key_length: 3");
    }

    #[test]
    fn test_hack_vigenere() {
        let cipher_text = [
            27, 66, 5, 56, 85, 76, 45, 16, 15, 35, 84, 9, 108, 68, 3, 108, 81, 24, 56, 81, 15, 39,
            16, 31, 35, 93, 9, 108, 67, 5, 33, 64, 0, 41, 16, 31, 57, 82, 31, 56, 89, 24, 57, 68,
            5, 35, 94, 76, 47, 89, 28, 36, 85, 30, 98, 16, 56, 35, 16, 30, 41, 84, 25, 47, 85, 76,
            56, 88, 9, 108, 83, 3, 33, 64, 0, 41, 72, 5, 56, 73, 76, 35, 86, 76, 56, 88, 5, 63, 16,
            3, 34, 85, 76, 59, 85, 76, 59, 89, 0, 32, 16, 25, 63, 85, 76, 35, 94, 0, 53, 16, 25,
            60, 64, 9, 62, 83, 13, 63, 85, 76, 32, 85, 24, 56, 85, 30, 63, 28, 76, 63, 95, 76, 56,
            88, 9, 108, 91, 9, 53, 67, 28, 45, 83, 9, 108, 89, 31, 108, 95, 2, 32, 73, 76, 126, 6,
            77, 108, 100, 3, 108, 87, 9, 56, 16, 24, 36, 89, 31, 108, 95, 2, 41, 16, 30, 37, 87, 4,
            56, 16, 13, 57, 68, 3, 33, 81, 24, 37, 83, 13, 32, 92, 21, 108, 73, 3, 57, 16, 27, 37,
            92, 0, 108, 64, 30, 35, 82, 13, 46, 92, 21, 108, 94, 9, 41, 84, 76, 56, 95, 76, 57, 67,
            9, 108, 67, 3, 33, 85, 76, 63, 95, 30, 56, 16, 3, 42, 16, 11, 41, 94, 9, 56, 89, 15,
            108, 81, 0, 43, 95, 30, 37, 68, 4, 33, 16, 68, 59, 88, 5, 47, 88, 76, 59, 95, 30, 39,
            85, 8, 108, 68, 4, 41, 16, 14, 41, 67, 24, 108, 92, 13, 63, 68, 76, 53, 85, 13, 62, 25,
            64, 108, 67, 5, 33, 69, 0, 45, 68, 9, 40, 16, 13, 34, 94, 9, 45, 92, 5, 34, 87, 76, 35,
            66, 76, 43, 66, 13, 40, 89, 9, 34, 68, 76, 40, 85, 31, 47, 85, 2, 56, 30, 76, 31, 85,
            30, 37, 95, 25, 63, 92, 21, 96, 16, 27, 62, 89, 24, 41, 16, 5, 56, 16, 30, 37, 87, 4,
            56, 16, 2, 35, 71, 64, 108, 73, 3, 57, 16, 27, 37, 92, 0, 108, 94, 9, 41, 84, 76, 37,
            68, 76, 56, 95, 76, 40, 85, 15, 37, 64, 4, 41, 66, 76, 56, 88, 9, 108, 94, 9, 52, 68,
            76, 35, 94, 9, 108, 81, 31, 108, 71, 9, 32, 92, 66, 108, 114, 9, 45, 66, 76, 37, 94,
            76, 33, 89, 2, 40, 28, 76, 56, 88, 9, 62, 85, 75, 63, 16, 2, 35, 16, 31, 60, 81, 15,
            41, 67, 66, 108, 58, 4, 56, 68, 28, 63, 10, 67, 99, 84, 3, 47, 67, 66, 43, 95, 3, 43,
            92, 9, 98, 83, 3, 33, 31, 8, 35, 83, 25, 33, 85, 2, 56, 31, 8, 99, 1, 36, 21, 7, 40,
            32, 29, 89, 37, 68, 53, 8, 3, 47, 19, 87, 7, 57, 85, 46, 58, 70, 46, 10, 64, 56, 120,
            115, 9, 47, 119, 60, 37, 98, 95, 124, 114, 31, 13, 98, 0, 24, 64, 61, 99, 85, 8, 37,
            68, 83, 57, 67, 28, 113, 67, 4, 45, 66, 5, 34, 87,
        ];
        hack_vigenere(&cipher_text);
    }

    #[test]
    fn test_find_key_len_for_task_1_3() {
        // let data = "EFFPQLEKVTVPCPYFLMVHQLUEWCNVWFYGHYTCETHQEKLPVMSAKSPVPAPVYWMVHQLUSPQLYWLASLFVWPQLMVHQLUPLRPSQLULQESPBLWPCSVRVWFLHLWFLWPUEWFYOTCMQYSLWOYWYETHQEKLPVMSAKSPVPAPVYWHEPPLUWSGYULEMQTLPPLUGUYOLWDTVSQETHQEKLPVPVSMTLEUPQEPCYAMEWWYTYWDLUULTCYWPQLSEOLSVOHTLUYAPVWLYGDALSSVWDPQLNLCKCLRQEASPVILSLEUMQBQVMQCYAHUYKEKTCASLFPYFLMVHQLUPQLHULIVYASHEUEDUEHQBVTTPQLVWFLRYGMYVWMVFLWMLSPVTTBYUNESESADDLSPVYWCYAMEWPUCPYFVIVFLPQLOLSSEDLVWHEUPSKCPQLWAOKLUYGMQEUEMPLUSVWENLCEWFEHHTCGULXALWMCEWETCSVSPYLEMQYGPQLOMEWCYAGVWFEBECPYASLQVDQLUYUFLUGULXALWMCSPEPVSPVMSBVPQPQVSPCHLYGMVHQLUPQLWLRPOEDVMETBYUFBVTTPENLPYPQLWLRPTEKLWZYCKVPTCSTESQPBYMEHVPETCMEHVPETZMEHVPETKTMEHVPETCMEHVPETT";
        // let probabilities = calculate_probabilities(&data.as_bytes().to_vec());
        // println!("{:?}", probabilities);
        // key length == 7
    }
}
