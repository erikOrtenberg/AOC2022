use std::{collections::HashMap};

#[derive(Debug)]
enum Destination {
    Up,
    Into(String),
    Root
}

#[derive(Debug)]
enum Command {
    Cd(Destination),
    Ls(Vec<Fileentry>)
}

type WorkingDirectory = Vec<String>;

type Filesystem = HashMap<String, Vec<Fileentry>>;

#[derive(Debug, Clone)]
enum Fileentry {
    File(String, i32),
    ChildDir(String)
}


fn construct_working_directory(wd : &WorkingDirectory) -> String{
    let mut copy = wd.to_vec();
    copy.reverse();
    let mut dir = String::from("");
    if copy.len() == 0 {
        return String::from("/");
    }
    while copy.len() > 0 {
        dir += "/";
        dir += copy.pop().expect("msg").as_str();
    }
    return dir;
}

fn execute_command(cmd : &Command, wd : &mut WorkingDirectory, fs : &mut Filesystem){
    //println!("{:?}", cmd);
    match cmd {
        Command::Cd(dst) => match dst {
            Destination::Up => {
                wd.pop();
                return;
            }
            Destination::Into(new_dir) => {
                wd.push(new_dir.to_string());
                return;
            }
            Destination::Root => {
                wd.clear();
                return;
            }
        }
        Command::Ls(entries) => {
            fs.insert(construct_working_directory(wd), entries.to_vec()); 
        }
    }
} 

fn evaluate_entry_size(entry : &Fileentry, fs : & Filesystem, wd : &String) -> i32 {
    let mut local_wd : String = wd.to_string();
    match entry {
        Fileentry::File(_, size) => *size,
        Fileentry::ChildDir(dir) => {
            if local_wd != "/"{
                local_wd += "/";
            }
            local_wd += dir.as_str();
            return evaluate_dir_size(fs, &local_wd);
        } 
    }
}

fn evaluate_dir_size(fs : & Filesystem, wd : & String) -> i32 {
    let entries = fs.get(wd).expect("wd shold exist");
    let result = entries.iter()
        .map(|x| evaluate_entry_size(x, fs, wd))
        .sum();
    return result;
}

pub(crate) fn day7(){
    let conents = std::fs::read_to_string("input/day7.txt").expect("");

    let parsed_commands = parse(conents);

    part1(&parsed_commands);
    part2(&parsed_commands);
}

fn part1(commands : &Vec<Command>) -> i32 {
    let mut fs : Filesystem = HashMap::new();
    let mut wd : WorkingDirectory = Vec::new();
    for c in commands {
        execute_command(c, &mut wd, &mut fs);
    }
    let result : i32 = fs.keys()
        .map(|x| evaluate_dir_size(&fs, x))
        .filter(|x| x < &100000)
        .sum();

    println!("{:?}", result);
    return result;
}

fn part2(commands : &Vec<Command>) -> i32 {
    let mut fs : Filesystem = HashMap::new();
    let mut wd : WorkingDirectory = Vec::new();
    for c in commands {
        execute_command(c, &mut wd, &mut fs);
    }
    let dir_sizes : Vec<i32> = fs.keys()
        .map(|x| evaluate_dir_size(&fs, x))
        .filter(|x| x > &3000000)
        .collect();
    let mut highest : i32 = i32::MIN;
    for dir_size in dir_sizes.clone() {
        //println!("{:?}", dir_size);
        if dir_size > highest {
            highest = dir_size;
        }
    }
    let must_clear : i128 = -40000000 + highest as i128;
    println!("{:?}", highest);
    println!("{:?}", must_clear);
    let mut lowest : i32 = i32::MAX;
    for dir_size in dir_sizes.clone() {
        if dir_size < lowest && dir_size as i128 > must_clear {
            lowest = dir_size;
        }
    }
    println!("{:?}", lowest);
    return lowest;
}

fn parse(input : String) -> Vec<Command> {
    let commands = input.split("$");
    let mut parsed_commands : Vec<Command> = Vec::new();
    for command in commands {
        if command == "" {
            continue;
        }
        //println!("{}", command);
        let mut tmp = command.lines();
        let command = tmp.next().expect("should exist a command");
        if command.contains("cd") {
            match command.replace(" cd ", "").as_str() {
                ".." => parsed_commands.push(Command::Cd(Destination::Up)),
                "/" => parsed_commands.push(Command::Cd(Destination::Root)),
                x => parsed_commands.push(Command::Cd(Destination::Into(x.to_string())))
            }
        } else {
            let mut entries :Vec<Fileentry> = Vec::new();
            for line in tmp{
                if line == "" {
                    continue;
                }
                let (size_or_dir, name) = line.split_once(" ").expect("delimiter should exist");
                if size_or_dir == "dir" {
                    entries.push(Fileentry::ChildDir(name.to_string()));
                } else {
                    entries.push(Fileentry::File(name.to_string(), size_or_dir.parse().unwrap()));
                }
            }
            parsed_commands.push(Command::Ls(entries)); 
        }

    } 
    return parsed_commands;
}