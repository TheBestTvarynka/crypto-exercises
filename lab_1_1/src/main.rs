use std::io::{Read, Write};

fn chars_to_byte(chars: &[char]) -> u8 {
    if chars.len() != 8 {
        panic!("Expected array of chars with len = 8 but got: {:?}", chars);
    }

    chars
        .into_iter()
        .map(|c| *c as u8 - 48)
        .enumerate()
        .fold(0, |byte, (index, bit_value)| {
            byte | (bit_value << (7 - index))
        })
}

fn file_content_as_bytes() {
    let mut data = String::new();
    std::fs::File::open("input.txt")
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();

    data.remove(data.len() - 1);

    let bytes = data
        .chars()
        .collect::<Vec<char>>()
        .chunks(8)
        .map(|chunk| chars_to_byte(chunk))
        .collect::<Vec<_>>();

    std::fs::File::create("input_as_bytes.txt").unwrap().write_all(&bytes).unwrap();
}

fn decode_base64() {
    let mut data = String::new();
    std::fs::File::open("input_as_bytes.txt")
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();

    let bytes = base64::decode(&mut data).unwrap();

    std::fs::File::create("input_decoded_base64.txt").unwrap().write_all(&bytes).unwrap();
}

fn main() {
    decode_base64();
}

#[cfg(test)]
mod test {
    use crate::chars_to_byte;

    #[test]
    fn test_chars_to_byte() {
        let byte = chars_to_byte(&['0', '0', '1', '1', '0', '0', '1', '0']);
        assert_eq!(50, byte);
    }
}
