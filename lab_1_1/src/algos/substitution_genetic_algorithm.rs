use crate::algos::decrypt_shift_vigenere_with_key;
use lazy_static::lazy_static;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng, RngCore};
use std::cmp::Ordering;
use std::collections::HashMap;

type Key = [u8; 7];

const POPULATION_SIZE: u8 = 10;
const ITERATIONS_AMOUNT: u8 = 100;
// const ALPHABET

lazy_static! {
    static ref PROBABILITIES: HashMap<char, f32> = {
        // https://www.nku.edu/~christensen/1402%20Friedman%20test%202.pdf
        let mut m = HashMap::new();
        m.insert('A', 0.082);
        m.insert('B', 0.015);
        m.insert('C', 0.028);
        m.insert('D', 0.043);
        m.insert('E', 0.127);
        m.insert('F', 0.022);
        m.insert('G', 0.02);
        m.insert('H', 0.061);
        m.insert('I', 0.07);
        m.insert('J', 0.002);
        m.insert('K', 0.008);
        m.insert('L', 0.04);
        m.insert('M', 0.024);
        m.insert('N', 0.067);
        m.insert('O', 0.075);
        m.insert('P', 0.019);
        m.insert('Q', 0.001);
        m.insert('R', 0.06);
        m.insert('S', 0.063);
        m.insert('T', 0.091);
        m.insert('U', 0.028);
        m.insert('V', 0.01);
        m.insert('W', 0.023);
        m.insert('X', 0.001);
        m.insert('Y', 0.02);
        m.insert('Z', 0.001);
        m
    };
}

pub struct SubstitutionGeneticAlgorithm {
    data: Vec<u8>,
    rand: ThreadRng,
}

impl SubstitutionGeneticAlgorithm {
    pub fn init(data: Vec<u8>) -> Self {
        SubstitutionGeneticAlgorithm {
            data,
            rand: thread_rng(),
        }
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

    fn make_children(parent1: &Key, parent2: &Key) -> Vec<Key> {
        let mut children = Vec::with_capacity(8);

        children.push([
            parent1[0], parent1[1], parent2[2], parent2[3], parent1[4], parent1[5], parent2[6],
        ]);
        children.push([
            parent2[0], parent2[1], parent1[2], parent1[3], parent2[4], parent2[5], parent1[6],
        ]);

        children.push([
            parent1[0], parent1[1], parent1[2], parent2[3], parent2[4], parent2[5], parent1[6],
        ]);
        children.push([
            parent2[0], parent2[1], parent2[2], parent1[3], parent1[4], parent1[5], parent2[6],
        ]);

        // children.push([
        //     parent1[0], parent1[1], parent1[2], parent1[3], parent2[4], parent2[5], parent2[6],
        // ]);
        // children.push([
        //     parent2[0], parent2[1], parent2[2], parent2[3], parent1[4], parent1[5], parent1[6],
        // ]);
        //
        // children.push([
        //     parent1[0], parent1[1], parent1[2], parent2[3], parent2[4], parent2[5], parent2[6],
        // ]);
        // children.push([
        //     parent2[0], parent2[1], parent2[2], parent1[3], parent1[4], parent1[5], parent1[6],
        // ]);

        children
    }

    // smaller fitness value is better
    fn fitness(&self, key: &Key) -> f32 {
        let decrypted = decrypt_shift_vigenere_with_key(&self.data, key);
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

        frequencies
            .into_iter()
            .map(|(c, f)| (c, f as f32 / text_len))
            .map(|(c, f)| {
                (PROBABILITIES.get(&c).unwrap_or(&0.0) - f + 1.0).powi(4)
            })
            .fold(0.0, |a, e| a + e)
    }

    pub fn solve(&mut self) -> Key {
        let mut population = self.generate_population();

        for _ in 0..ITERATIONS_AMOUNT/2 {
            let mut new_population = Vec::with_capacity(POPULATION_SIZE as usize * 90 * 4);
            for parent1 in 0..POPULATION_SIZE {
                for parent2 in (parent1 + 1)..POPULATION_SIZE {
                    new_population.extend_from_slice(
                        SubstitutionGeneticAlgorithm::make_children(
                            population.get(parent1 as usize).unwrap(),
                            population.get(parent2 as usize).unwrap(),
                        )
                        .as_slice(),
                    );
                }
            }

            let mut new_population = new_population
                .into_iter()
                .map(|e| (e, self.fitness(&e)))
                .collect::<Vec<_>>();
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

            population = new_population[0..10]
                .iter()
                .map(|e| e.0)
                .collect::<Vec<_>>();
        }

        println!("{:?}", population);
        for i in population.iter() {
            let res = String::from_utf8(decrypt_shift_vigenere_with_key(&self.data, i)).unwrap();
            println!("{}", res);
        }
        population[0]
    }
}
