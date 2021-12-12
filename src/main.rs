use std::env;
use std::fs;
use std::path::Path;

fn day_1(input: String) {
    let vals: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Sum of simple increases.
    let mut increases = 0;
    for i in 1..vals.len() {
        if vals[i] > vals[i - 1] {
            increases += 1;
        }
    }
    println!("{}", increases);

    // Sum of increases of sum of three-element sliding window.
    increases = 0;
    for i in 3..vals.len() {
        if vals[i] > vals[i - 3] {
            increases += 1;
        }
    }
    println!("{}", increases);
}

fn day_2(input: String) {
    let commands: Vec<(&str, i32)> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut pieces = s.split(' ');
            (
                pieces.next().unwrap(),
                pieces.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut position = 0;
    let mut depth = 0;
    for c in &commands {
        match c.0 {
            "forward" => position += c.1,
            "down" => depth += c.1,
            "up" => depth -= c.1,
            _ => panic!("Unrecognized command {}", c.0),
        };
    }
    println!("{}", position * depth);

    position = 0;
    depth = 0;
    let mut aim = 0;
    for c in &commands {
        match c.0 {
            "forward" => {
                position += c.1;
                depth += aim * c.1;
            }
            "down" => aim += c.1,
            "up" => aim -= c.1,
            _ => panic!("Unrecognized command {}", c.0),
        };
    }
    println!("{}", position * depth);
}

fn problem_input(input_num: usize) -> String {
    let inpath = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("inputs")
        .join(format!("{}", input_num));
    return fs::read_to_string(inpath.clone())
        .expect(format!("Unable to read file {}", inpath.display()).as_str());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let funcs = [day_1, day_2];

    if args.len() < 2 || args.len() > 3 {
        println!("usage: {} <input number> [input file path]", args[0]);
        return;
    }
    let input_num = args[1]
        .parse::<usize>()
        .expect(format!("Couldn't parse {} as a problem number.", args[1]).as_str());
    if input_num < funcs.len() {
        println!("Can't find function for input {}", input_num);
        return;
    }
    let input_str = match args.len() {
        2 => problem_input(input_num),
        3 => fs::read_to_string(args[2].clone())
            .expect(format!("Couldn't open input file {}", args[2]).as_str()),
        _ => "".to_string(),
    };

    funcs[input_num - 1](input_str)
}
