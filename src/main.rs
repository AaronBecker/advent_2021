use std::collections::HashSet;
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

fn day_3(input: String) {
    let binvals: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();
    let width = binvals[0].len();
    let mut position_sums = vec![0; width];
    for val in &binvals {
        for (i, c) in val.chars().enumerate() {
            match c {
                '0' => (),
                '1' => position_sums[i] += 1,
                _ => panic!("Unexpected input {}", val),
            }
        }
    }

    let mut epsilon = 0;
    let mut gamma = 0;
    for i in 0..width {
        epsilon *= 2;
        gamma *= 2;
        if position_sums[i] > binvals.len() / 2 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    println!("{}", epsilon * gamma);

    let mut o2_candidates: Vec<&&str> = binvals.iter().collect();
    let mut o2_value = 0;
    for i in 0..width {
        let digit_sum: usize = o2_candidates
            .iter()
            .map(|c| if c.as_bytes()[i] == '1' as u8 { 1 } else { 0 })
            .sum();
        let common_bit = if digit_sum * 2 >= o2_candidates.len() {
            '1' as u8
        } else {
            '0' as u8
        };
        o2_candidates = o2_candidates
            .into_iter()
            .filter(|c| c.as_bytes()[i] == common_bit)
            .collect();
        if o2_candidates.len() == 1 {
            o2_value = isize::from_str_radix(o2_candidates[0], 2).unwrap();
            break;
        }
    }

    let mut co2_candidates: Vec<&&str> = binvals.iter().collect();
    let mut co2_value = 0;
    for i in 0..width {
        let digit_sum: usize = co2_candidates
            .iter()
            .map(|c| if c.as_bytes()[i] == '1' as u8 { 1 } else { 0 })
            .sum();
        let common_bit = if digit_sum * 2 >= co2_candidates.len() {
            '1' as u8
        } else {
            '0' as u8
        };
        co2_candidates = co2_candidates
            .into_iter()
            .filter(|c| c.as_bytes()[i] != common_bit)
            .collect();
        if co2_candidates.len() == 1 {
            co2_value = isize::from_str_radix(co2_candidates[0], 2).unwrap();
            break;
        }
    }

    println!("{}", o2_value * co2_value);
}

fn day_4(input: String) {
    let mut lines = input.split('\n');
    let draws: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let mut boards: Vec<Vec<i32>> = Vec::new();
    boards.push(Vec::new());
    lines
        .filter(|s| !s.is_empty())
        .map(|s| {
            for val in s.split(' ').filter(|s| !s.is_empty()) {
                let ival = val.parse::<i32>().unwrap();
                boards.last_mut().unwrap().push(ival);
                if boards.last().unwrap().len() == 25 {
                    boards.push(Vec::new());
                }
            }
        })
        .for_each(drop);
    boards.pop(); // remove empty final board

    fn board_wins(board: &Vec<i32>, draws: &HashSet<i32>) -> bool {
        for i in 0..5 {
            if draws.contains(&board[i * 5])
                && draws.contains(&board[i * 5 + 1])
                && draws.contains(&board[i * 5 + 2])
                && draws.contains(&board[i * 5 + 3])
                && draws.contains(&board[i * 5 + 4])
            {
                return true;
            }
            if draws.contains(&board[i])
                && draws.contains(&board[i + 5])
                && draws.contains(&board[i + 10])
                && draws.contains(&board[i + 15])
                && draws.contains(&board[i + 20])
            {
                return true;
            }
        }
        false
    }
    let mut marked: HashSet<i32> = HashSet::new();
    'draw_loop: for draw in &draws {
        marked.insert(*draw);
        for board in &boards {
            if board_wins(&board, &marked) {
                let score: i32 = board.iter().filter(|s| !marked.contains(s)).sum::<i32>() * draw;
                println!("{}", score);
                break 'draw_loop;
            }
        }
    }

    marked.clear();
    let mut last_score = 0;
    for draw in &draws {
        marked.insert(*draw);
        boards.retain(|b| {
            let win = board_wins(&b, &marked);
            if win {
                last_score = b.iter().filter(|s| !marked.contains(s)).sum::<i32>() * draw;
            }
            !win
        })
    }
    println!("{}", last_score);
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
    let funcs = [day_1, day_2, day_3, day_4];

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
