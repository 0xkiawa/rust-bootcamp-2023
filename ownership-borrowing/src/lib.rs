#![allow(dead_code, unused_variables)]

// Exercise 1
// Make it compile
fn exercise1() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = &x;
    let z = &x;
}

// Exercise 2
// Make it compile
// Don't modify code in exercise2 function!
fn exercise2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}
// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

// Exercise 3
// Make it compile
// Dont care about logic
fn exercise3() {
    let values: Vec<f64> = vec![
        2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
        139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
        4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
    ];

    let values_number = values.len();

    let additions: Vec<usize> = vec![0];

    println!("{:?}", values_number);

    //while additions.len() > 0 {
    while !additions.is_empty() {
        let mut addition: f64 = 0.0;

        // Sumar valores en additions
        for element_index in &additions {
            let addition_aux = values[*element_index];
            addition += addition_aux;
        }
    }
}

// Exercise 4
// Make it compile
fn exercise4(value: u32) -> &'static str {
    // let str_value = value.to_string(); // Convert u32 to String
    // let str_ref: &str = &str_value; // Obtain a reference to the String
    // str_ref // Return the reference to the String
    Box::leak(value.to_string().into_boxed_str())
}

// Exercise 5
// Make it compile
use std::collections::HashMap;
fn exercise5() {
    let mut my_map: HashMap<i32, String> = HashMap::from([(1, "1.0".into()), (2, "2.0".into())]);

    let key = 3;
    let value: String = "3.0".into();

    let res = match my_map.get(&key) {
        Some(child) => child,
        None => {
            my_map.insert(key, value.clone());
            &value
        }
    };

    println!("{}", res);
}

// Exercise 6
// Make it compile

use std::io;

fn exercise6() {
    {
        let mut prev_key: &str = "";

        io::stdin().lines().flatten().for_each(|line| {
            let data: Vec<&str> = line.split('\t').collect();
            // if prev_key.len() == 0 {
            if prev_key.is_empty() {
                prev_key = Box::leak(data[0].to_string().into_boxed_str());
            }
        })
    }

    {
        let mut prev_key: Box<str> = "".into();

        io::stdin().lines().flatten().for_each(|line| {
            let data: Vec<&str> = line.split('\t').collect();
            if prev_key.len() == 0 {
                prev_key = data[0].into();
            }
        })
    }
}

// Exercise 7
// Make it compile
fn exercise7() {
    let mut v: Vec<Box<str>> = Vec::new();
    // let mut v: Vec<String> = Vec::new();

    {
        let chars = [b'x', b'y', b'z'];
        let s: &str = std::str::from_utf8(&chars).unwrap();
        v.push(s.into());
    }
    println!("{:?}", v);
}

// Exercise 8
// Make it compile
fn exercise8() {
    let mut accounting: Vec<Box<str>> = vec!["Alice".into(), "Ben".into()];

    loop {
        let mut add_input = String::from("");

        io::stdin()
            .read_line(&mut add_input)
            .expect("Failed to read line");

        let add_vec: Vec<&str> = add_input.trim()[..].split_whitespace().collect();

        //if add_vec.len() < 1 {
        if add_vec.is_empty() {
            println!("Incorrect input, try again");
            continue;
        }

        let person = add_vec[0];
        accounting.push(person.into());
    }
}
