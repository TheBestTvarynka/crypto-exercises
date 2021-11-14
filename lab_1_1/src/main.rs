use std::collections::HashMap;

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

    frequencies.values().into_iter().map(|frequency| frequency * (frequency - 1)).fold(0.0, |sum, value| {
        sum + value as f32
    }) / (len * (len - 1)) as f32
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

fn main() {
    println!("Start");
}

#[cfg(test)]
mod tests {
    use crate::{hack_ceaser, calculate_probabilities};
    use lab_1_1::utils::{hex_str_to_bytes, decode_base64};

    #[test]
    fn test_hex_str_to_bytes() {
        let bytes = hex_str_to_bytes("7958401743454e1756174552475256435e59501a5c524e176f786517545e475f5245191772195019175e4317445f58425b531743565c521756174443455e595017d5b7ab5f525b5b58174058455b53d5b7aa175659531b17505e41525917435f52175c524e175e4417d5b7ab5c524ed5b7aa1b174f584517435f5217515e454443175b524343524517d5b7ab5fd5b7aa17405e435f17d5b7ab5cd5b7aa1b17435f5259174f584517d5b7ab52d5b7aa17405e435f17d5b7ab52d5b7aa1b17435f525917d5b7ab5bd5b7aa17405e435f17d5b7ab4ed5b7aa1b1756595317435f5259174f58451759524f4317545f564517d5b7ab5bd5b7aa17405e435f17d5b7ab5cd5b7aa175650565e591b17435f525917d5b7ab58d5b7aa17405e435f17d5b7ab52d5b7aa1756595317445817585919176e5842175a564e17424452175659175e5953524f1758511754585e59545e53525954521b177f565a5a5e595017535e4443565954521b177c56445e445c5e17524f565a5e5956435e58591b17444356435e44435e54565b17435244434417584517405f564352415245175a52435f5853174e5842175152525b174058425b5317445f584017435f52175552444317455244425b4319");
        println!("{:?}", bytes);
    }

    #[test]
    fn test_hack_ceaser() {
        hack_ceaser(&[121, 88, 64, 23, 67, 69, 78, 23, 86, 23, 69, 82, 71, 82, 86, 67, 94, 89, 80, 26, 92, 82, 78, 23, 111, 120, 101, 23, 84, 94, 71, 95, 82, 69, 25, 23, 114, 25, 80, 25, 23, 94, 67, 23, 68, 95, 88, 66, 91, 83, 23, 67, 86, 92, 82, 23, 86, 23, 68, 67, 69, 94, 89, 80, 23, 213, 183, 171, 95, 82, 91, 91, 88, 23, 64, 88, 69, 91, 83, 213, 183, 170, 23, 86, 89, 83, 27, 23, 80, 94, 65, 82, 89, 23, 67, 95, 82, 23, 92, 82, 78, 23, 94, 68, 23, 213, 183, 171, 92, 82, 78, 213, 183, 170, 27, 23, 79, 88, 69, 23, 67, 95, 82, 23, 81, 94, 69, 68, 67, 23, 91, 82, 67, 67, 82, 69, 23, 213, 183, 171, 95, 213, 183, 170, 23, 64, 94, 67, 95, 23, 213, 183, 171, 92, 213, 183, 170, 27, 23, 67, 95, 82, 89, 23, 79, 88, 69, 23, 213, 183, 171, 82, 213, 183, 170, 23, 64, 94, 67, 95, 23, 213, 183, 171, 82, 213, 183, 170, 27, 23, 67, 95, 82, 89, 23, 213, 183, 171, 91, 213, 183, 170, 23, 64, 94, 67, 95, 23, 213, 183, 171, 78, 213, 183, 170, 27, 23, 86, 89, 83, 23, 67, 95, 82, 89, 23, 79, 88, 69, 23, 89, 82, 79, 67, 23, 84, 95, 86, 69, 23, 213, 183, 171, 91, 213, 183, 170, 23, 64, 94, 67, 95, 23, 213, 183, 171, 92, 213, 183, 170, 23, 86, 80, 86, 94, 89, 27, 23, 67, 95, 82, 89, 23, 213, 183, 171, 88, 213, 183, 170, 23, 64, 94, 67, 95, 23, 213, 183, 171, 82, 213, 183, 170, 23, 86, 89, 83, 23, 68, 88, 23, 88, 89, 25, 23, 110, 88, 66, 23, 90, 86, 78, 23, 66, 68, 82, 23, 86, 89, 23, 94, 89, 83, 82, 79, 23, 88, 81, 23, 84, 88, 94, 89, 84, 94, 83, 82, 89, 84, 82, 27, 23, 127, 86, 90, 90, 94, 89, 80, 23, 83, 94, 68, 67, 86, 89, 84, 82, 27, 23, 124, 86, 68, 94, 68, 92, 94, 23, 82, 79, 86, 90, 94, 89, 86, 67, 94, 88, 89, 27, 23, 68, 67, 86, 67, 94, 68, 67, 94, 84, 86, 91, 23, 67, 82, 68, 67, 68, 23, 88, 69, 23, 64, 95, 86, 67, 82, 65, 82, 69, 23, 90, 82, 67, 95, 88, 83, 23, 78, 88, 66, 23, 81, 82, 82, 91, 23, 64, 88, 66, 91, 83, 23, 68, 95, 88, 64, 23, 67, 95, 82, 23, 85, 82, 68, 67, 23, 69, 82, 68, 66, 91, 67, 25]);
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
}
