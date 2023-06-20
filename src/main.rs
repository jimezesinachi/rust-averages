use std::{collections::HashMap, io};

fn main() {
    println!("Welcome to this simple statistical averages calculator!");

    loop {
        let mut sum: f64 = 0.0;

        let mut vec: Vec<u32> = Vec::new();

        let mut map: HashMap<String, u32> = HashMap::new();

        let mut input = String::new();

        println!("Please input your set of integers, seperated by single whitespaces:");

        io::stdin()
            .read_line(&mut input)
            .expect("Enter a valid input!");

        println!("Your input is: {input}");

        for char in input.chars() {
            if char.is_ascii_digit() {
                let digit = match char.to_digit(10) {
                    Some(integer) => integer,
                    None => continue,
                };

                sum += digit as f64;

                vec.push(digit);
            } else {
                continue;
            }
        }

        println!("Mean: {}", sum / vec.len() as f64);

        println!("Unsorted vector of your input is: {:?}", vec);

        vec.sort();

        println!("Sorted vector of your inputs is: {:?}", vec);

        if vec.len() % 2 == 0 {
            println!("Length of vector is even: {}", vec.len());

            let a = match vec.get(vec.len() / 2) {
                Some(value) => value,
                None => continue,
            };

            let b = match vec.get((vec.len() / 2) - 1) {
                Some(value) => value,
                None => continue,
            };

            let c = ((a.to_owned() as f64) + (b.to_owned() as f64)) / 2 as f64;

            println!("Median: {:?}", c);
        } else {
            println!("Length of vector is odd: {}", vec.len());

            let a = match vec.get(vec.len() / 2) {
                Some(value) => value,
                None => continue,
            };

            println!("Median: {:?}", a);
        }

        for digit in vec {
            map.entry(String::from(digit.to_string()))
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        println!("{:?}", map);

        let mut mode = String::new();

        let mut value = 0;

        for (k, v) in map.iter() {
            if v > &value {
                value = *v;

                mode = k.to_string();
            } else if v == &value {
                value = *v;

                mode.push_str(", ");

                mode.push_str(k);
            }
        }

        println!("Mode: {mode}");
    }
}
