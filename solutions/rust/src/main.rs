mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
fn main () {
    let current_day = 6;
    let days = [
    day2::day2,
    day2::day2,
    day3::day3,
    day4::day4,
    day5::day5,
    day6::day6,
    ];
    days[current_day - 1]();
}