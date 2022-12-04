use std::{fs, result};
use core::iter::Zip;
use std::slice::Iter;

pub(crate) fn day3() {
    let contents = std::fs::read_to_string("input/day3").expect("this should work");
    let backpacks : Vec<(&str, &str)> = contents
        .lines()
        //.inspect(|x| {dbg!(x);})
        .map(create_backpack)
        .collect();
    //println!("{contents}");
    for backpack in backpacks.clone() {
        //println!("{}, {}", backpack.0, backpack.1)
        assert_eq!(backpack.0.len(), backpack.1.len());
    }
    
    part1(backpacks.clone());
    part2(&mut backpacks.clone());
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

fn get_overlap(b1 : String, b2 : String, b3 : String) -> char {
    for c1 in b1.chars(){
        for c2 in b2.chars(){
            for c3 in b3.chars(){
                if c1 == c2 && c2 == c3 {
                    return c1;
                }
            }
        }
    }
    return '1';
}

fn find_badge(backpacks : &mut Vec<(& str, & str)>) -> char {
    let b1: (&str, &str) = backpacks.pop().expect("b1");
    let b2: (&str, &str) = backpacks.pop().expect("b2");
    let b3: (&str, &str) = backpacks.pop().expect("b3");
    return get_overlap((b1.0.to_owned() + b1.1), (b2.0.to_owned() + b2.1), b3.0.to_owned() + b3.1);
}

fn part1(backpacks : Vec<(&str, &str)>) -> i32{
    let result = backpacks
        .iter()
        .map(|(side1, side2)| evaluate_backpack(side1, side2))
        .sum();
    println!("{}", result);
    return result;
}

fn part2(backpacks : &mut Vec<(&str, &str)>) -> i32 {
    let mut chars:Vec<char> = Vec::new();
    while backpacks.len() > 0 {
        chars.push(find_badge(backpacks));
    }
    let result = chars
        .iter()
        .map(|c| char_to_value(*c))
        .sum();
    print!("{}", result);
    return result;
}