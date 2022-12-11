
type Range = (i32, i32);

pub(crate) fn day4(){
    let contents = std::fs::read_to_string("input/day4.txt").expect("msg");
    let assignment_pairs: Vec<(Range, Range)> = contents
        .lines()
        .map(parse_line)
        .collect();
    part1(assignment_pairs.clone());
    part2(assignment_pairs.clone());
}

fn parse_line(line : &str) -> (Range, Range) {
    let (fst, snd) = line.split_once(",").expect("msg");
    let fst_split : (&str, &str) = fst.split_once("-").expect("msg");
    let snd_split : (&str, &str) = snd.split_once("-").expect("msg");
    let fst_range : Range = (fst_split.0.parse().unwrap(), fst_split.1.parse().unwrap());
    let snd_range : Range = (snd_split.0.parse().unwrap(), snd_split.1.parse().unwrap());
    return (fst_range, snd_range);
}

fn range_contains_range((r1_fst, r1_snd) : Range, (r2_fst, r2_snd) : Range) -> i32 {
    if r1_fst <= r2_fst && r1_snd >= r2_snd {
        return 1;
    } else if r2_fst <= r1_fst && r2_snd >= r1_snd {
        return 1;
    }
    return 0;
}

fn range_overlaps_range((r1_fst, r1_snd) : Range, (r2_fst, r2_snd) : Range) -> i32 {
    if r1_fst <= r2_snd && r1_snd >= r2_fst{
        return 1;
    } else if r2_fst <= r1_snd && r2_snd >= r1_fst {
        return 1;
    }
    return 0;
}

fn part1(assignment_pairs : Vec<(Range, Range)>) -> i32 {
    let result = assignment_pairs
        .iter()
        .map(|(fst, snd)|range_contains_range(*fst, *snd))
        .sum();
    println!("{}", result);
    return result;
}

fn part2(assignment_pairs : Vec<(Range, Range)>) -> i32 {
    let result = assignment_pairs
        .iter()
        .map(|(fst, snd)| range_overlaps_range(*fst, *snd))
        .sum();
    println!("{}", result);
    return result;
}