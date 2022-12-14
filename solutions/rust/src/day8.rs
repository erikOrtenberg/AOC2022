use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub(crate) fn day8() {
    let contents = std::fs::read_to_string("input/day8.txt").expect("");

    let parsed_trees = parse(contents);
    let length: Vec<&(u32, u32)> = parsed_trees.keys().collect();
    part1(&parsed_trees);
    part2(&parsed_trees);

}

fn is_visible_from(
    dir: Direction,
    location @ &(x, y): &(u32, u32),
    map: &HashMap<(u32, u32), u32>,
) -> bool {
    if x == 0 || x == 98 || y == 0 || y == 98 {
        //println!("Edge tree!");
        return true;
    }
    let value_at_location = map[location];

    let (start, end) = match dir {
        Direction::Up => (0, x),
        Direction::Down => (x + 1, 99),
        Direction::Right => (y + 1, 99),
        Direction::Left => (0, y),
    };

    //println!("Searching {:?}", dir);
    //println!("starting from {:?}", location);

    match dir {
        Direction::Down | Direction::Up => {
            for i in start..end {
                let value_at = map[&(i, y)];
                //println!("{:?}", (i, *y));
                if value_at >= value_at_location {
                    return false;
                }
            }
        }
        Direction::Left | Direction::Right => {
            for i in start..end {
                //println!("{:?}", (*x, i));
                if map[&(x, i)] >= value_at_location {
                    return false;
                }
            }
        }
    }

    return true;
}

fn is_visible(location: &(u32, u32), map: &HashMap<(u32, u32), u32>) -> bool {
    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    .any(|d| is_visible_from(d, location, map))
}

fn distance_in_dir(
    dir: Direction,
    location @ &(x, y): &(u32, u32),
    map: &HashMap<(u32, u32), u32>,
) -> u32 {
    let value_at_location = map[location];

    let (start, end) = match dir {
        Direction::Up => (0, x),
        Direction::Down => (x + 1, 99),
        Direction::Right => (y + 1, 99),
        Direction::Left => (0, y),
    };

    match dir {
        Direction::Down | Direction::Up => {
            for i in start..end {
                let value_at = map[&(i, y)];
                if value_at >= value_at_location {
                    return i.abs_diff(x);
                }
            }
        }
        Direction::Left | Direction::Right => {
            for i in start..end {
                if map[&(x, i)] >= value_at_location {
                    return i.abs_diff(y);
                }
            }
        }
    }
    match dir {
        Direction::Up => x,
        Direction::Down => 99 - x,
        Direction::Right => 99 - y,
        Direction::Left => y,
    }
}

fn get_scenic_score(location: &(u32, u32), map: &HashMap<(u32, u32), u32>) -> u32 {
    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ].into_iter()
    .map(|dir| distance_in_dir(dir, location, map))
    .product()
}


fn part1(map: &HashMap<(u32, u32), u32>) -> i32 {
    let list_of_visible_trees: Vec<&(u32, u32)> =
        map.keys().filter(|l| is_visible(l, &map.clone())).collect();
    let result = list_of_visible_trees.len() as i32;
    println!("{:?}", result);
    return result;
}

fn part2(map: &HashMap<(u32, u32), u32>) -> u32 {
    let result = map.keys()
    .map(|x| get_scenic_score(x, map))
    .max()
    .unwrap_or_default();
    println!("{:?}", result);
    result
}

fn parse(input: String) -> HashMap<(u32, u32), u32> {
    let trees: Vec<Vec<u32>> = input
        .lines()
        .into_iter()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .collect()
        })
        .collect();
    let mut result: HashMap<(u32, u32), u32> = HashMap::new();
    for i in 0..trees.len() {
        for j in 0..trees[0].len() {
            result.insert((i as u32, j as u32), trees[i][j]);
        }
    }
    return result;
}
