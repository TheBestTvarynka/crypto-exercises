use std::fs::File;
use std::io::Read;

fn decrypt_string(data: String) -> String {
    let mut data = data.chars().collect::<Vec<char>>();
    data.insert(0, '0');
    data.insert(0, '0');

    let mut result = vec!['0'; data.len()];

    for i in (3..data.len() - 1).step_by(4) {
        result[i] = data[i];
        result[i - 1] = data[i + 1];
        result[i - 2] = data[i - 1];
        result[i - 3] = data[i + 2];
    }

    result.into_iter().collect::<String>().replace('!', " ")
}

fn main() {
    let mut data = "".to_owned();
    File::open("intro.txt")
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();

    println!("Encrypted text:");
    println!("{}", data);

    println!("Decrypted text:");
    println!("{}", decrypt_string(data));
}
