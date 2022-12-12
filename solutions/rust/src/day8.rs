use std::{collections::HashMap, result};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub(crate) fn day8() {
    let contents = std::fs::read_to_string("input/day8.txt").expect("");

    let parsed_trees = parse(contents);
    let length : Vec<&(u32, u32)>= parsed_trees.keys().collect();
    println!("{:?}",length.len());
    part1(&parsed_trees);
}

fn is_visible_from(dir : &Direction, location@(x,y) : &(u32, u32), map : &HashMap<(u32, u32), u32>) -> bool {

    if *x == 0 || *x == 98 || *y == 0 || *y == 98 {
        //println!("Edge tree!");
        return true;
    }
    let value_at_location = map.get(location).expect("location should exist");

    let mut start : u32;
    let mut end : u32;
    match dir.clone() {
        Direction::Up => {
            end = x.clone() - 1;
            start = 0;
        }
        Direction::Down => {
            start = x.clone() + 1;
            end = 98;
        }
        Direction::Right => {
            start = y.clone() + 1;
            end = 98;
        }
        Direction::Left => {
            end = y.clone() - 1;
            start = 0;
        }
    }

    //println!("Searching {:?}", dir);
    //println!("starting from {:?}", location);

    match dir {
        Direction::Down | Direction::Up => {
            for i in start..end {
                //println!("{:?}", (i, *y));
                if map.get(&(i,*y)).expect("we hope there exists a key here") > value_at_location {
                    return false;
                }
            }
        }
        Direction::Left | Direction::Right => {
            for i in start..end {
                //println!("{:?}", (*x, i));
                if map.get(&(*x,i)).expect("we hope there exists a key here") > value_at_location {
                    return false;
                }
            }
        }
    }

    return true;
}

fn is_visible(location : &(u32, u32), map : &HashMap<(u32, u32), u32>) -> bool {
    for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
        if is_visible_from(&dir, location, map) {
            return true;
        }
    }
    return false;
}

fn part1(map : &HashMap<(u32, u32), u32>) -> i32 {
    let list_of_visible_trees : Vec<&(u32, u32)> = map.keys()
        .filter(|l| is_visible(l, &map.clone()))
        .collect();
    let result = list_of_visible_trees.len() as i32;
    println!("{:?}", result);
    return result;
}

fn parse(input : String) -> HashMap<(u32, u32), u32> {
    let trees : Vec<Vec<u32>> = input.lines().into_iter()
        .map(|x| x.chars().map(|c| c.to_digit(10).expect("should be a digit")).collect())
        .collect();
    let mut result : HashMap<(u32, u32), u32> = HashMap::new();
    for i in 0..trees.len() {
        for j in 0..trees[0].len() {
            result.insert((i as u32, j as u32), trees[i][j]);
        }
    }
    return result;
}