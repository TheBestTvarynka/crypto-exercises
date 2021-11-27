// use crate::algos::constants::{bigrams, QUADRITRIGRAMS, TRIGRAMS, unigrams};
use crate::algos::decrypt_shift_vigenere;
use crate::algos::frequency::{FOUR_GRAMS, THREE_GRAMS};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng, RngCore};
use std::cmp::Ordering;
use std::collections::HashMap;

type Key = [u8; 7];

const POPULATION_SIZE: u32 = 100;
const ITERATIONS_AMOUNT: u32 = 1000;
const BASE_NGRAM_LEN: usize = 3;

type Bigram = (char, char);

pub struct SubstitutionGeneticAlgorithm {
    data: Vec<u8>,
    rand: ThreadRng,
    unigrams: HashMap<String, f32>,
    bigrams: HashMap<String, f32>,
    trigrams: HashMap<String, f32>,
    fourgrams: HashMap<String, f32>,
}

pub fn read_ngrams_from_file(filepath: &str) -> HashMap<String, f32> {
    std::fs::read_to_string(filepath)
        .unwrap()
        .split('\n')
        .map(|e| {
            let parts = e.split(',').collect::<Vec<&str>>();
            // println!("{} {}", parts[0], parts[1].trim());
            (parts[0].to_owned(), parts[1].trim().parse::<f32>().unwrap())
        })
        .collect::<HashMap<String, f32>>()
}

impl SubstitutionGeneticAlgorithm {
    pub fn init(data: Vec<u8>) -> Self {
        let r = SubstitutionGeneticAlgorithm {
            data,
            rand: thread_rng(),
            unigrams: read_ngrams_from_file(
                "/home/qkation/Documents/projects/crypto-exercises/lab_1_1/assets/1_grams.csv",
            ),
            bigrams: read_ngrams_from_file(
                "/home/qkation/Documents/projects/crypto-exercises/lab_1_1/assets/2_grams.csv",
            ),
            trigrams: read_ngrams_from_file(
                "/home/qkation/Documents/projects/crypto-exercises/lab_1_1/assets/3_grams.csv",
            ),
            fourgrams: read_ngrams_from_file(
                "/home/qkation/Documents/projects/crypto-exercises/lab_1_1/assets/4_grams.csv",
            ),
        };
        println!("{}", r.unigrams.values().sum::<f32>());
        println!("{}", r.bigrams.values().sum::<f32>());
        println!("{}", r.trigrams.values().sum::<f32>());
        println!("{}", r.fourgrams.values().sum::<f32>());
        r
    }

    fn generate_rand_key(&mut self) -> Key {
        let mut key = [0, 0, 0, 0, 0, 0, 0];
        for i in 0..7 {
            key[i] = self.rand.gen::<u8>() % 25 + 1;
        }
        key
    }

    fn generate_population(&mut self) -> Vec<Key> {
        let mut population = Vec::with_capacity(POPULATION_SIZE as usize);
        for _ in 0..POPULATION_SIZE {
            population.push(self.generate_rand_key());
        }
        population
    }

    fn make_children(&mut self, parent1: &Key, parent2: &Key) -> Vec<Key> {
        let mut children = vec![[0; 7], [0; 7]];

        let k = (self.rand.gen::<usize>() % 5) + 1;

        for i in 0..=k {
            children[0][i] = parent1[i];
        }

        for i in (k + 1)..7 {
            children[0][i] = parent2[i];
        }

        let k = (self.rand.gen::<usize>() % 5) + 1;

        for i in 0..=k {
            children[1][i] = parent2[i];
        }

        for i in (k + 1)..7 {
            children[1][i] = parent1[i];
        }

        // children.push([
        //     parent1[0], parent1[1], parent1[2], parent2[3], parent2[4], parent2[5], parent1[6],
        // ]);
        // children.push([
        //     parent2[0], parent2[1], parent2[2], parent1[3], parent1[4], parent1[5], parent2[6],
        // ]);
        //
        // children.push([
        //     parent1[0], parent2[1], parent1[2], parent2[3], parent1[4], parent2[5], parent1[6],
        // ]);
        // children.push([
        //     parent2[0], parent1[1], parent2[2], parent1[3], parent2[4], parent1[5], parent2[6],
        // ]);

        children
    }

    // smaller fitness value is better
    /*
    fn fitness(&self, key: &Key) -> f32 {
        let decrypted = decrypt_shift_vigenere(&self.data, key);
        let text = String::from_utf8(decrypted).unwrap();
        let text_len = text.len() as f32;

        let mut frequencies = HashMap::new();
        for c in text.chars() {
            let new_f = match frequencies.get(&c) {
                Some(f) => f + 1,
                None => 1,
            };
            frequencies.insert(c, new_f);
        }

        let mut mark = frequencies
            .into_iter()
            .map(|(c, f)| (c, f as f32 / text_len))
            .map(|(c, f)| {
                // println!("{} {}", PROBABILITIES.get(&c).unwrap(), f);
                (PROBABILITIES.get(&c).unwrap() - f).abs()
            })
            .sum();

        mark
    }

    fn fitness2(&self, key: &Key) -> f32 {
        let decrypted = decrypt_shift_vigenere(&self.data, key);
        let text = String::from_utf8(decrypted).unwrap();
        let bigrams_amount = text.len() - 1;

        let mut bigrams: HashMap<Bigram, u32> = HashMap::new();
        text.as_bytes().windows(2).for_each(|bigram| {
            let bigram: Bigram = (bigram[0] as char, bigram[1] as char);
            let new_f = match bigrams.get(&bigram) {
                Some(f) => f + 1,
                None => 1,
            };
            bigrams.insert(bigram, new_f);
        });

        bigrams
            .into_iter()
            .map(|(k, v)| (k, (v as f32 * 100.0) / bigrams_amount as f32))
            .map(|(k, v)| {
                // let ideal = *BIGRAM_PROBABILITIES.get(&k).unwrap();
                // println!("{} {}", BIGRAM_PROBABILITIES.get(&k).unwrap(), v);
                (*BIGRAM_PROBABILITIES.get(&k).unwrap() - v).abs()
            })
            .sum()
        // 0.0
    }

    fn fitness3(&self, key: &Key) -> f32 {
        let decrypted = decrypt_shift_vigenere(&self.data, key);
        let text = String::from_utf8(decrypted).unwrap();
        let text_len = text.len() as f32;

        let mut frequencies = HashMap::new();
        for c in text.chars() {
            let new_f = match frequencies.get(&c) {
                Some(f) => f + 1,
                None => 1,
            };
            frequencies.insert(c, new_f);
        }

        let mut min = 1.0;
        let sum: f32 = frequencies
            .into_iter()
            .map(|(c, f)| {
                let v = *PROBABILITIES.get(&c).unwrap();
                if v < min {
                    min = v;
                }
                (v * text_len - f as f32).abs()
            })
            .sum();
        let k = 2.0 * (text_len - min * text_len);
        (k - sum) / k
    }
     */

    fn get_n_gram(data: &[u8], from: usize, n: usize) -> Option<&[u8]> {
        if data.len() < from + n {
            return None;
        }
        Some(&data[from..(from + n)])
    }

    fn fitness4(&self, key: &Key) -> f32 {
        let decrypted = decrypt_shift_vigenere(&self.data, key);
        let text = String::from_utf8(decrypted).unwrap();
        let text_len = text.len();

        let mut unigrams = HashMap::new();
        let mut bigrams = HashMap::new();
        let mut trigrams = HashMap::new();
        let mut fourgrams = HashMap::new();
        for i in 0..text_len {
            if let Some(n_gram) = SubstitutionGeneticAlgorithm::get_n_gram(text.as_bytes(), i, 1) {
                let new_f = match unigrams.get(n_gram) {
                    Some(f) => f + 1,
                    None => 1,
                };
                unigrams.insert(n_gram, new_f);
            }
            if let Some(n_gram) = SubstitutionGeneticAlgorithm::get_n_gram(text.as_bytes(), i, 2) {
                let new_f = match bigrams.get(&n_gram) {
                    Some(f) => f + 1,
                    None => 1,
                };
                bigrams.insert(n_gram, new_f);
            }
            if let Some(n_gram) = SubstitutionGeneticAlgorithm::get_n_gram(text.as_bytes(), i, 3) {
                let new_f = match trigrams.get(&n_gram) {
                    Some(f) => f + 1,
                    None => 1,
                };
                trigrams.insert(n_gram, new_f);
            }
            if let Some(n_gram) = SubstitutionGeneticAlgorithm::get_n_gram(text.as_bytes(), i, 4) {
                let new_f = match fourgrams.get(&n_gram) {
                    Some(f) => f + 1,
                    None => 1,
                };
                fourgrams.insert(n_gram, new_f);
            }
        }

        unigrams
            .into_iter()
            .map(|(k, v)| {
                let gram = String::from_utf8(k.to_vec()).unwrap();
                // println!("u {}", gram);
                let t_value = *self.unigrams.get(&gram).unwrap_or(&0.0);
                if t_value != 0.0 {
                    (t_value - (v as f32) / (text_len as f32)).abs()
                } else {
                    0.0
                }
            })
            .sum::<f32>()
            + bigrams
                .into_iter()
                .map(|(k, v)| {
                    let gram = String::from_utf8(k.to_vec()).unwrap();
                    // println!("b {}", gram);
                    let t_value = *self.bigrams.get(&gram).unwrap_or(&0.0);
                    if t_value != 0.0 {
                        (t_value - (v as f32) / (text_len as f32 - 1.0)).abs()
                    } else {
                        0.0
                    }
                })
                .sum::<f32>()
            + trigrams
                .into_iter()
                .map(|(k, v)| {
                    let gram = String::from_utf8(k.to_vec()).unwrap();
                    // println!("t {}", gram);
                    let t_value = *self.trigrams.get(&gram).unwrap_or(&0.0);
                    if t_value != 0.0 {
                        (t_value - (v as f32) / (text_len as f32 - 2.0)).abs()
                    } else {
                        0.0
                    }
                })
                .sum::<f32>()
            + fourgrams
                .into_iter()
                .map(|(k, v)| {
                    let gram = String::from_utf8(k.to_vec()).unwrap();
                    // println!("f {}", gram);
                    let t_value = *self.fourgrams.get(&gram).unwrap_or(&0.0);
                    if t_value != 0.0 {
                        (t_value - (v as f32) / (text_len as f32 - 3.0)).abs()
                    } else {
                        0.0
                    }
                })
                .sum::<f32>()
    }

    fn partial_fitness_5(gram_len: usize, text: &String, t_grams: &HashMap<String, u32>) -> f32 {
        let mut ngrams = HashMap::new();
        for ngram in text.as_bytes().windows(gram_len) {
            let ngram = String::from_utf8(ngram.to_vec()).unwrap();
            let new_f = match ngrams.get(&ngram) {
                Some(f) => f + 1,
                None => 1,
            };
            ngrams.insert(ngram, new_f);
        }

        ngrams
            .into_iter()
            .map(|(ngram, f)| {
                let ft = *t_grams.get(&ngram).unwrap_or(&0);
                if ft == 0 {
                    0.0
                } else {
                    f as f32 * (ft as f32).log2()
                }
            })
            .sum()
    }

    fn fitness5(&self, key: &Key) -> f32 {
        let decrypted = decrypt_shift_vigenere(&self.data, key);
        let text = String::from_utf8(decrypted).unwrap();

        // SubstitutionGeneticAlgorithm::partial_fitness_5(3, &text, &THREE_GRAMS)
        //     +
        SubstitutionGeneticAlgorithm::partial_fitness_5(4, &text, &FOUR_GRAMS)
    }

    fn swap(k: &mut Key, i: usize, j: usize) {
        let tmp = k[i];
        k[i] = k[j];
        k[j] = tmp;
    }

    fn mutation(&mut self, population: &mut Vec<Key>) {
        let population_len = population.len();
        let next = |rand: &mut ThreadRng| rand.gen::<usize>() % population_len;
        let rand_key_index = |rand: &mut ThreadRng| rand.gen::<usize>() % 7;
        let rand_shift = |rand: &mut ThreadRng| rand.gen::<usize>() % 25 + 1;

        for i in 0..10 {
            for _ in 0..6 {
                // let i1 = rand_key_index(&mut self.rand);
                // let mut i2 = rand_key_index(&mut self.rand);
                // while i2 != i1 {
                //     i2 = rand_key_index(&mut self.rand);
                // }
                // SubstitutionGeneticAlgorithm::swap(&mut population[i], i1, i2);
                population[i][rand_key_index(&mut self.rand)] = rand_shift(&mut self.rand) as u8;
            }
        }
        for _ in 0..(POPULATION_SIZE / 4) {
            let m = next(&mut self.rand);
            for _ in 0..4 {
                let rk = rand_key_index(&mut self.rand);
                let rv = rand_shift(&mut self.rand) as u8;
                // println!("m: {}, rk: {}, rv: {}", m, rk, rv);
                population[m][rk] = rv;
                // let i1 = rand_key_index(&mut self.rand);
                // let i2 = rand_key_index(&mut self.rand);
                // SubstitutionGeneticAlgorithm::swap(&mut population[m], i1, i2);
            }
        }
    }

    pub fn solve(&mut self) -> Key {
        let mut population = self.generate_population();

        for i in 0..ITERATIONS_AMOUNT {
            println!("iteration: {}", i);
            let mut new_population = Vec::with_capacity(POPULATION_SIZE as usize * 90 * 2);
            for parent1 in 0..POPULATION_SIZE {
                for parent2 in (parent1 + 1)..POPULATION_SIZE {
                    new_population.extend_from_slice(
                        self.make_children(
                            population.get(parent1 as usize).unwrap(),
                            population.get(parent2 as usize).unwrap(),
                        )
                        .as_slice(),
                    );
                }
            }

            let mut new_population = new_population
                .into_iter()
                .map(|e| (e, self.fitness4(&e)))
                .collect::<Vec<_>>();
            // '>' - min value first
            // '<' - max value first
            new_population.sort_by(|p1, p2| {
                if p1.1 == p2.1 {
                    Ordering::Equal
                } else if p1.1 > p2.1 {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            new_population.dedup();

            println!("{} {:?}", new_population[0].1, &new_population[0].0);

            // let prev_gen = population[0..20].to_vec();
            // population = new_population[0..(POPULATION_SIZE as usize - 20)]
            population = new_population[0..POPULATION_SIZE as usize]
                .iter()
                .map(|(k, f)| *k)
                .collect::<Vec<_>>();
            // population.extend_from_slice(&prev_gen);

            let res =
                String::from_utf8(decrypt_shift_vigenere(&self.data, &population[0])).unwrap();
            println!("{} {:?}", res, &population[0]);

            self.mutation(&mut population);
        }

        println!("{:?}", population);

        population[0]
    }
}
