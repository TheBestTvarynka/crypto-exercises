use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref THREE_GRAMS: HashMap<String, u32> = {
        std::fs::read_to_string("/home/qkation/Documents/projects/crypto-exercises/lab_1_1/assets/tri-ngramFrequency.csv")
            .unwrap()
            .split('\n')
            .into_iter()
            .map(|e| {
                let parts = e.split(',').collect::<Vec<&str>>();
                (parts[0].to_owned(), parts[1].trim().parse::<u32>().unwrap())
            })
            .collect::<HashMap<String, u32>>()
    };
    pub static ref FOUR_GRAMS: HashMap<String, u32> = {
        std::fs::read_to_string("/home/qkation/Documents/projects/crypto-exercises/lab_1_1/assets/four-ngramFrequency.csv")
            .unwrap()
            .split('\n')
            .into_iter()
            .map(|e| {
                let parts = e.split(',').collect::<Vec<&str>>();
                (parts[0].to_owned(), parts[1].trim().parse::<u32>().unwrap())
            })
            .collect::<HashMap<String, u32>>()
    };
}
