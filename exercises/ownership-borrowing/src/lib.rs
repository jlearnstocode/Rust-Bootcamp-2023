#[allow(unused_variables)]
// Exercise 1
// Make it compile
fn exercise1() {
    // Use as many approaches as you can to make it work
    // 1st
    let x = String::from("hello, world");
    let y = &x;
    let z = &x;

    // 2nd
    let x2 = String::from("hello, world");
    let y2 = &x2;
    let z2 = x2; // in case you have no intent to use x2 after this line

    // 3rd
    let x3 = String::from("hello, world");
    let y3 = x3.clone();
    let z3 = x3; // in case you have no intent to use x2 after this line

    // 4th
    let x4 = String::from("hello, world");
    let y4 = x4.clone();
    let z4 = x4.clone(); // clone() is not top suggestion, prefer ref & instead

    // 5th
    let x5 = String::from("hello, world");
    let y5 = &x5;
    let z5 = x5.clone();
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

    let len_additions = additions.len();

    while len_additions > 0_usize {
        let mut addition: f64 = 0.0;

        // Sumar valores en additions
        for element_index in &additions {
            let index_x = *element_index;
            println!("{:p}", &index_x);
            let addition_aux = values[index_x];
            addition = addition_aux + addition;
        }
    }
}

// Exercise 4
// Make it compile
fn exercise4(value: u32) -> String {
    let str_value = value.to_string(); // Convert u32 to String
    let str_ref: &str = &str_value; // Obtain a reference to the String
    str_ref.to_string() // Return the reference to the String
}

// Exercise 5
// Make it compile
use std::collections::HashMap;
fn exercise5() {
    let mut my_map = HashMap::from([(1, "1.0".to_string()), (2, "2.0".to_string())]);

    let key = 3;

    let res: &String = match my_map.get(&key) {
        Some(child) => child,
        None => {
            let value: String = "3.0".to_string();
            my_map.insert(key, value);
            let value: &String = my_map.get(&key).unwrap();
            value
            // HERE IT FAILS
        }
    };

    println!("{}", res);
}

// Exercise 6
// Make it compile

use std::io;

fn exercise6() {
    let mut prev_key = "".to_owned();

    for line in io::stdin().lines() {
        let s = line.unwrap();
        let binding = s.to_string();
        let data = binding.split("\t").collect::<Vec<_>>();
        if prev_key.len() == 0 {
            prev_key = data[0].to_string();
        }
    }
}

// Exercise 7
// Make it compile
fn exercise7() {
    let mut v: Vec<String> = Vec::new();
    {
        let chars = [b'x', b'y', b'z'];
        let s: &str = std::str::from_utf8(&chars).unwrap();
        v.push(s.to_string());
    }
    println!("{:?}", v);
}

// Exercise 8
// Make it compile
fn exercise8() {
    let mut accounting: Vec<String> = vec!["Alice".to_string(), "Ben".to_string()];

    loop {
        let mut add_input = String::from("");

        io::stdin()
            .read_line(&mut add_input)
            .expect("Failed to read line");

        let add_vec: Vec<&str> = add_input.trim()[..].split_whitespace().collect();

        if add_vec.len() < 1 {
            println!("Incorrect input, try again");
            continue;
        }

        let person_0 = add_vec[0];
        println!("person_0 {}", person_0);

        let person = add_vec[0].to_string();
        accounting.push(person);
    }
}
