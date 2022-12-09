
pub(crate) fn day6(){
    let contents = std::fs::read_to_string("input/day6.txt").expect("msg");

    //part1(String::from("aaaadabcdaaaaaaa"));
    part1(contents.clone());
    part2(contents.clone());
}

fn find_first_pkg_start(input : String) -> i32 {
    let mut count : i32 = 1;
    for starts in input.as_bytes().windows(4){
        let mut sorted: Vec<u8> = starts.clone().to_vec();
        sorted.sort();
        if sorted[0] < sorted[1] && sorted[1] < sorted[2] && sorted[2] < sorted[3] {  
            println!("{:?}", starts);  
            return count + 3;
        }
       count += 1;
    }
    return -1;
}

fn find_first_msg_start(input : String) -> i32 {
    let mut count : i32 = 1;
    for starts in input.as_bytes().windows(14){
        let mut s: Vec<u8> = starts.clone().to_vec();
        s.sort(); 
        //im turly, truly sorry
        if s[0] < s[1] && s[1] < s[2] && s[2] < s[3] && s[3] < s[4] && s[4] < s[5] && s[5] < s[6] && s[6] < s[7] && s[7] < s[8] && s[8] < s[9] && s[9] < s[10] && s[10] < s[11] && s[11] < s[12] && s[12] < s[13] {  
            println!("{:?}", starts);  
            return count + 13;
        }
       count += 1;
    }
    return -1;
}

fn part1(input : String) -> i32 {
    let result = find_first_pkg_start(input);
    println!("{:?}", result);
    return result;
}

fn part2(input : String) -> i32 {
    let result = find_first_msg_start(input);
    println!("{:?}", result);
    return result;
}