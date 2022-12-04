use std::fs;
use core::iter::Zip;
use std::slice::Iter;

pub(crate) fn day3() {
    let contents = std::fs::read_to_string("input/day3").expect("this should work");
    let backpacks : Vec<(&str, &str)> = contents
        .lines()
        .inspect(|x| {dbg!(x);})
        .map(create_backpack)
        .collect();
    //println!("{contents}");
    for backpack in backpacks.clone() {
        //println!("{}, {}", backpack.0, backpack.1)
        assert_eq!(backpack.0.len(), backpack.1.len());
    }
    
    part1(backpacks.clone());

}

fn create_backpack(backpack : &str) -> (&str, &str) {
    return backpack.split_at(backpack.len() / 2);   
}

fn evaluate_backpack(side1:&str, side2 :&str) -> i32 {
    for c1 in side1.chars() {
        for c2 in side2.chars() {
            if c1 == c2 {
                return char_to_value(c1);
            }
        }
    }
    return 0;
}

fn char_to_value(c : char) -> i32 {
    if (c.is_uppercase()) {
        return (c.to_ascii_lowercase().to_digit(36).expect("msg") as i32 - 9 + 26);
    }
    return (c.to_digit(36).expect("msg") as i32 - 9);
}

fn part1(backpacks : Vec<(&str, &str)>) -> i32{
    let result = backpacks
        .iter()
        .map(|(side1, side2)| evaluate_backpack(side1, side2))
        .sum();
    println!("{}", result);
    return result;
}