mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
fn main () {
    let current_day = 7;
    let days = [
    day2::day2,
    day2::day2,
    day3::day3,
    day4::day4,
    day5::day5,
    day6::day6,
    day7::day7,
    ];
    days[current_day - 1]();
}