mod day2;
mod day3;
mod day4;
fn main () {
    let current_day = 4;
    let days = [
    day2::day2,
    day2::day2,
    day3::day3,
    day4::day4
    ];
    days[current_day - 1]();
}