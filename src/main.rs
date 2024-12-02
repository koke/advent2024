mod day1;
mod day2;

enum Mode {
    Test,
    Final,
}

enum Part {
    First,
    Second,
}

fn input_file_path(day: u32, part: Part, mode: Mode) -> String {
    let mut path = String::from("input/");
    path.push_str(&format!("day{}/", day));
    match mode {
        Mode::Test => path.push_str("example"),
        Mode::Final => path.push_str("input"),
    }
    match part {
        Part::First => path.push('1'),
        Part::Second => path.push('2'),
    }
    path.push_str(".txt");
    path
}

fn main() {
    day1::first(input_file_path(1, Part::First, Mode::Test));
    day1::first(input_file_path(1, Part::First, Mode::Final));
    day1::second(input_file_path(1, Part::Second, Mode::Test));
    day1::second(input_file_path(1, Part::Second, Mode::Final));

    day2::first(input_file_path(2, Part::First, Mode::Test));
    day2::first(input_file_path(2, Part::First, Mode::Final));
    // day2::second(input_file_path(2, Part::Second, Mode::Test));
    // day2::second(input_file_path(2, Part::Second, Mode::Final));
}
