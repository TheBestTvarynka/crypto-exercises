use lab_1_1::algos::decrypt_shift_vigenere;
use lab_1_1::algos::frequency::THREE_GRAMS;
use lab_1_1::algos::substitution_genetic_algorithm::SubstitutionGeneticAlgorithm;
use std::cmp::Ordering;
use std::collections::HashMap;

fn count(data: &Vec<char>) -> HashMap<char, u32> {
    let mut res = HashMap::new();
    for c in data {
        let new_f = match res.get(c) {
            Some(f) => f + 1,
            None => 1,
        };
        res.insert(*c, new_f);
    }
    res
}

fn normalize(data: &HashMap<char, u32>, len: usize) -> HashMap<char, f32> {
    data.iter()
        .map(|(k, v)| (*k, (*v as f32) / len as f32))
        .collect()
}

fn sum_map(data: HashMap<String, f32>) -> f32 {
    data.values().sum::<f32>()
}

fn find_shift(data: &Vec<(char, f32)>, table: &HashMap<char, f32>) -> u8 {
    let calc_delta = |data: &Vec<(char, f32)>, table: &HashMap<char, f32>| {
        let mut res = 0.0;
        for (c, f) in data {
            res += (*table.get(c).unwrap() - f).abs();
        }
        res
    };

    let shift = |data: &Vec<(char, f32)>, n: u8| {
        data.iter()
            .map(|(c, f)| {
                let c = *c as u8;
                let mut new_c = c - n;
                new_c = if new_c < 65 { 26 + new_c } else { new_c };
                (new_c as char, *f)
            })
            .collect::<Vec<(char, f32)>>()
    };

    let mut min_delta = f32::MAX;
    let mut min_shift = 0;
    for i in 1..26 {
        let new_data = shift(data, i);
        let cur_delta = calc_delta(&new_data, table);
        if cur_delta < min_delta {
            min_delta = cur_delta;
            min_shift = i;
        }
    }
    min_shift
}

fn get_unigrams() -> HashMap<char, f32> {
    lab_1_1::algos::substitution_genetic_algorithm::read_ngrams_from_file("/home/qkation/Documents/projects/crypto-exercises/lab_1_1/assets/1_grams.csv").into_iter().map(|(k, v)| {
        (k.as_bytes()[0] as char, v)
    }).collect()
}

fn analytics(data: &[u8]) -> Vec<u8> {
    let mut parts = vec![Vec::with_capacity(93); 7];
    let data_len = data.len();
    let mut i = 0;
    'main: loop {
        for j in 0..7 {
            if i >= data_len {
                break 'main;
            }
            parts[j].push(data[i] as char);
            i += 1;
        }
    }
    let unigrams = get_unigrams();
    let mut res_key = Vec::with_capacity(7);
    for p in parts {
        let p_f = count(&p);
        let p_p = normalize(&p_f, p.len());
        let mut p_vec: Vec<_> = p_p.into_iter().collect();
        res_key.push(find_shift(&p_vec, &unigrams));
        println!("==================================");
    }
    println!("{:?}", res_key);
    res_key
}

fn main() {
    // for (k, v) in THREE_GRAMS.iter() {
    //     println!("{} -> {}", k, v);
    // }
    // partial_brute_force();
    let data = b"EFFPQLEKVTVPCPYFLMVHQLUEWCNVWFYGHYTCETHQEKLPVMSAKSPVPAPVYWMVHQLUSPQLYWLASLFVWPQLMVHQLUPLRPSQLULQESPBLWPCSVRVWFLHLWFLWPUEWFYOTCMQYSLWOYWYETHQEKLPVMSAKSPVPAPVYWHEPPLUWSGYULEMQTLPPLUGUYOLWDTVSQETHQEKLPVPVSMTLEUPQEPCYAMEWWYTYWDLUULTCYWPQLSEOLSVOHTLUYAPVWLYGDALSSVWDPQLNLCKCLRQEASPVILSLEUMQBQVMQCYAHUYKEKTCASLFPYFLMVHQLUPQLHULIVYASHEUEDUEHQBVTTPQLVWFLRYGMYVWMVFLWMLSPVTTBYUNESESADDLSPVYWCYAMEWPUCPYFVIVFLPQLOLSSEDLVWHEUPSKCPQLWAOKLUYGMQEUEMPLUSVWENLCEWFEHHTCGULXALWMCEWETCSVSPYLEMQYGPQLOMEWCYAGVWFEBECPYASLQVDQLUYUFLUGULXALWMCSPEPVSPVMSBVPQPQVSPCHLYGMVHQLUPQLWLRPOEDVMETBYUFBVTTPENLPYPQLWLRPTEKLWZYCKVPTCSTESQPBYMEHVPETCMEHVPETZMEHVPETKTMEHVPETCMEHVPETT";
    let key = analytics(data);
    // let mut algo = SubstitutionGeneticAlgorithm::init(data.clone());
    // let key = algo.solve();
    let res = String::from_utf8(decrypt_shift_vigenere(data, &key)).unwrap();
    println!("{}", res);
    // use lab_1_1::algos::constants::PROBABILITIES;
    // println!("{}", PROBABILITIES.values().sum::<f32>());
}
