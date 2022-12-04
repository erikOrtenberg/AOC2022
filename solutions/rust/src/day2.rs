use std::fs;
use core::iter::Zip;
use std::slice::Iter;

pub(crate) fn day2() {
    let contents = fs::read_to_string("input/day2.txt")
        .expect("should've been able to read file");
    let mut parsed_input : Vec<&str> = contents.split("\n").collect();
    parsed_input.pop(); 
    let mut opponent : Vec<&str> = Vec::new(); 
    let mut player : Vec<&str> = Vec::new();    
    //parse input into two different vectors
    for x in &parsed_input {
        let mut tmp : Vec<&str> = x.split(" ").collect();
        opponent.push(tmp.pop().expect("bogus"));
        player.push(tmp.pop().expect("bogus"));
    }

    let input = player.iter().zip(opponent.iter()); 
    part1(input.clone());
    part2(input.clone());
}



fn evaluate_round(opponent:&str, player:&str) -> i32{
    let mut points : i32 = 0;

    match(opponent, player) {
        ("A","Y") => points += 6,
        ("B","Z") => points += 6,
        ("C","X") => points += 6,
        ("A","X") => points += 3,
        ("B","Y") => points += 3,
        ("C","Z") => points += 3,
        _ => points = points,
    }

    match player {
        "X" => points += 1,
        "Y" => points += 2,
        "Z" => points += 3,
        _ => points = points,
    } 

    return points;
}

fn generate_play<'a>(opponent:&'a str, player:&'a str) -> &'a str {
    match (opponent, player) {
        (opp, "X") => return get_losing(opp),
        (opp, "Y") => return get_draw(opp),
        (opp, "Z") => return get_winning(opp),
        _ => return "",
    }
}

fn get_winning(opponent:&str) -> &str {
    match opponent {
        "A" => return "Y",
        "B" => return "Z",
        _ => return "X",
    }
}
    
fn get_losing(opponent:&str) -> &str {
    match opponent {
        "A" => return "Z",
        "B" => return "X",
        _ => return "Y",
    }
}

fn get_draw(opponent:&str) -> &str {
    match opponent {
        "A" => return "X",
        "B" => return "Y",
        _ => return "Z",
    }
}

fn part1(input : Zip<Iter<&str>, Iter<&str>>) -> i32 {
    let result : i32 = input
        .map(|(opponent, player)| evaluate_round(opponent, player))
        .sum(); 
    println!("{}", result.to_string());
    return result;
}

fn part2(input : Zip<Iter<&str>, Iter<&str>>) -> i32 {
    let result : i32 = input
        .map(|(opponent, player)| evaluate_round(opponent, generate_play(opponent, player)))
        .sum(); 
    println!("{}", result.to_string());
    return result;
}