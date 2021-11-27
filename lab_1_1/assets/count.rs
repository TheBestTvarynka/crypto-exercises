use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;

fn count_n_grams(n: usize, n_grams: &mut HashMap<String, u32>, data: &[u8]) {
    data.windows(n).for_each(|n_gram| {
        let n_gram = String::from_utf8(n_gram.to_vec()).unwrap();
        let new_f = match n_grams.get(&n_gram) {
            Some(f) => f + 1,
            None => 1,
        };
        n_grams.insert(n_gram, new_f);
    });
}

fn main() {
    let data = String::from_utf8(
        std::fs::read_to_string("dataset")
            .unwrap()
            .chars()
            .into_iter()
            .filter(|b| {
                ((*b as u8) > 64 && (*b as u8) < 91) || ((*b as u8) > 96 && (*b as u8) < 123)
            })
            .map(|b| b.to_uppercase().next().unwrap() as u8)
            .collect::<Vec<u8>>(),
    )
    .unwrap();

    let data = Arc::new(data);

    println!("{}", &data[0..100]);
    println!("{}", data.len());

    let mut handlers = Vec::new();

    for i in 1..=4 {
        let data_a = data.clone();
        handlers.push(std::thread::spawn(move || {
            println!("start calculation of {}-grams", i);

            let mut grams = HashMap::new();
            count_n_grams(i, &mut grams, (*data_a).as_bytes());

            println!("Finish calculation. Start writing...");

            let n_grams_amount = (data_a.len() + 1 - i) as f32;

            let mut file = std::fs::File::create(format!("{}_grams.csv", i)).unwrap();
            for (k, v) in grams {
                file.write(
                    format!(
                        "{},{}\n",
                        k,
                        (v as f32) / n_grams_amount
                    )
                    .as_bytes(),
                )
                .unwrap();
            }

            println!("{}-grams written!", i);
        }));
    }

    for handler in handlers {
        handler.join().unwrap();
    }
}
