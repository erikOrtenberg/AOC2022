type CrateStacks = Vec<Vec<String>>;
type Moves = Vec<(i32,i32,i32)>;

pub(crate) fn day5(){
    let contents = std::fs::read_to_string("input/day5.txt")
        .expect("this should work");

    let (crate_stacks, moves) = parse_input(contents);
    part1(crate_stacks.clone(), moves.clone());
    part2(crate_stacks.clone(), moves.clone());
}

fn part1(crate_stacks : CrateStacks, moves : Moves) -> String {
    let mut crate_stacks_mut = crate_stacks.clone();
    for m in moves{
        execute_move(&mut crate_stacks_mut, m, true);
    }
    let mut result : String = String::new();
    for vec in crate_stacks_mut{
        result += vec.last().expect("msg").as_str();   
    }
    println!("{:?}", result);
    return result;
}

fn part2(crate_stacks : CrateStacks, moves : Moves) -> String {
    let mut crate_stacks_mut = crate_stacks.clone();
    for m in moves{
        execute_move(&mut crate_stacks_mut, m, false);
    }
    let mut result : String = String::new();
    for vec in crate_stacks_mut{
        result += vec.last().expect("msg").as_str();   
    }
    println!("{:?}", result);
    return result;
}

fn execute_move(crate_stacks : &mut CrateStacks, (amount, from, to) : (i32, i32, i32), part1 : bool) {
    if part1 {
        for _x in 0..amount{
            execute_one_move_part1(crate_stacks, from - 1, to - 1);
        }
    } else {
        execute_one_move_part2(crate_stacks, amount, from - 1, to - 1);
    }
}

fn execute_one_move_part1(crate_stacks : &mut CrateStacks, from : i32, to : i32) {
    let mut from_value: String = String::new(); 
    if let Some(from_vec) = crate_stacks.get_mut(from as usize){
        from_value = (*from_vec.pop().expect("msg")).to_string();
    }
    if let Some(to_vec) = crate_stacks.get_mut(to as usize){
        to_vec.push(from_value);
    }
} 

fn execute_one_move_part2(crate_stacks : &mut CrateStacks, amount : i32, from : i32, to : i32) {
    let mut tmp : Vec<String> = Vec::new();
    for _i in 0..amount {
        let mut from_value = String::new();
        if let Some(from_vec) = crate_stacks.get_mut(from as usize){
            from_value = (*from_vec.pop().expect("msg")).to_string();
        }
        tmp.push(from_value);
    }
    for _i in 0..amount {
        if let Some(to_vec) = crate_stacks.get_mut(to as usize){
            to_vec.push(tmp.pop().expect("msg"));
        }
    }
}


//Parsing. yes, it's 60+ lines. no, i don't know why this is day5. STOP FUCKING ASKING ABOUT WHY I TRANSPOSE A MATRIX ;;((((

fn parse_input(input : String) -> (CrateStacks, Moves) {
    let (crates, moves) = input.split_once("\r\n\r\n").expect("msg");
    return (parse_crates(crates.to_string()), parse_moves(moves.to_string()));
} 

fn split_at_every(input : &str, n : usize) -> Vec<String>{
    let mut result = vec![];
    let mut cur = input;
    while !cur.is_empty() {
        if cur.len() < n {
            result.push(cur.to_string());
            break;
        }
        let (chunk, rest) = cur.split_at(n);
        result.push(chunk.to_string());
        cur = rest;
    }
    return result;
} 

// omg thank you internet
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone, {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn remove_empty_entries(input : &Vec<String>) -> Vec<String> {
    let mut result = input.clone();
    let mut str : String = result.pop().expect("msg");
    while str == "   " || str == "    "{
        str = result.pop().expect("msk");
    }
    result.push(str);
    return result;
}

fn parse_crates(input : String) -> CrateStacks {
    let crate_lines : Vec<Vec<String>> = input.lines().rev().skip(1)
        .map(|x| split_at_every(x, 4))
        .collect();
    let crate_stacks: CrateStacks = transpose(crate_lines).iter()
        .map(|x| remove_empty_entries(x))
        .collect();
    
    return crate_stacks;
}

fn parse_move(input : String) -> (i32, i32, i32) {
    let tmp : Vec<i32> = input.trim_start().split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    return (tmp[0], tmp[1], tmp[2]);
}

fn parse_moves(input : String) -> Moves {
    let moves : Moves = input
        .lines()
        .map(|x| x.chars()
            .filter(|c| c.is_ascii_digit() || c.is_whitespace())
            .collect())
        .map(parse_move)
        .collect();
    return moves;
}