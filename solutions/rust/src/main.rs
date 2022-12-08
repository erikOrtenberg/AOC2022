mod day2;
mod day3;
mod day4;
mod day5;
fn main () {
    let current_day = 5;
    let days = [
    day2::day2,
    day2::day2,
    day3::day3,
    day4::day4,
    day5::day5,
    ];
    days[current_day - 1]();
}