use std::cmp;
use std::collections::HashMap;
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

fn day_5(input: String) {
    let vents: Vec<Vec<i32>> = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|p| {
                    p.split(',')
                        .map(|val| val.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .flatten()
                .collect()
        })
        .collect();

    let mut vent_count = HashMap::new();
    for vent in &vents {
        if vent[0] == vent[2] {
            // horizontal vent
            for i in cmp::min(vent[1], vent[3])..cmp::max(vent[1], vent[3]) + 1 {
                *vent_count.entry((vent[0], i)).or_insert(0) += 1;
            }
        } else if vent[1] == vent[3] {
            // vertical vent
            for i in cmp::min(vent[0], vent[2])..cmp::max(vent[0], vent[2]) + 1 {
                *vent_count.entry((i, vent[1])).or_insert(0) += 1;
            }
        }
    }

    let multi_vent = vent_count.iter().filter(|v| *v.1 > 1).count();
    println!("{}", multi_vent);

    for vent in &vents {
        if vent[0] != vent[2] && vent[1] != vent[3] {
            // diagonal vent
            let dx = if vent[0] - vent[2] > 0 { -1 } else { 1 };
            let dy = if vent[1] - vent[3] > 0 { -1 } else { 1 };

            let mut x = vent[0];
            let mut y = vent[1];
            while (x, y) != (vent[2], vent[3]) {
                *vent_count.entry((x, y)).or_insert(0) += 1;
                x += dx;
                y += dy;
            }
            *vent_count.entry((x, y)).or_insert(0) += 1;
        }
    }

    let multi_vent = vent_count.iter().filter(|v| *v.1 > 1).count();
    println!("{}", multi_vent);
}

fn day_6(input: String) {
    let mut fish_counts: Vec<u64> = vec![0; 7];
    input
        .trim()
        .split(',')
        .map(|x| fish_counts[x.parse::<usize>().unwrap()] += 1)
        .for_each(drop);

    let mut t8 = 0;
    let mut t7 = 0;
    println!("{}", usize::BITS);
    for gen in 0..256 {
        let tmp = fish_counts[gen % 7];
        fish_counts[gen % 7] += t7;
        t7 = t8;
        t8 = tmp;
        if gen == 79 {
            println!("{}", t7 + t8 + fish_counts.iter().sum::<u64>());
        }
    }
    println!("{}", t7 + t8 + fish_counts.iter().sum::<u64>());
}

fn day_7(input: String) {
    let mut pos: Vec<i32> = input
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    pos.sort();
    let median = pos[pos.len() / 2];
    let cost: i32 = pos.iter().map(|x| (median - x).abs()).sum();
    println!("{}", cost);

    let mut min_cost = i32::MAX;
    for i in pos[0]..pos[pos.len() - 1] {
        let cost: i32 = pos
            .iter()
            .map(|x| {
                let dist = (i - x).abs();
                dist * (dist + 1) / 2
            })
            .sum();
        min_cost = cmp::min(min_cost, cost);
    }
    println!("{}", min_cost);
}

fn day_8(input: String) {
    let entries: Vec<Vec<Vec<String>>> = input
        .lines()
        .map(|e| {
            e.split(" | ")
                .map(|half| {
                    half.trim()
                        .split(' ')
                        .map(|s| {
                            let mut chs: Vec<char> = s.chars().collect();
                            chs.sort();
                            String::from_iter(chs.iter())
                        })
                        .collect()
                })
                .collect()
        })
        .collect();
    let out_count: usize = entries
        .iter()
        .map(|e| {
            e[1].iter()
                .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
                .count()
        })
        .sum();
    println!("{}", out_count);

    let mut global_sum = 0;
    for entry in &entries {
        let mut digit_map: HashMap<i32, &String> = HashMap::new();
        let mut segment_map: HashMap<&String, i32> = HashMap::new();
        let sets: Vec<(&String, HashSet<char>)> = entry[0]
            .iter()
            .map(|e| (e, e.chars().collect::<HashSet<char>>()))
            .collect();
        let mut matched_sets = HashSet::new();
        // Match 1, 4, 7, 8
        for s in &sets {
            let matched = match s.0.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                _ => 0,
            };
            if matched != 0 {
                digit_map.insert(matched, s.0);
                segment_map.insert(s.0, matched);
                matched_sets.insert(s.0);
            }
        }
        // Match 9, 3, 6
        for s in &sets {
            if s.0.len() == 5 {
                if digit_map[&7]
                    .chars()
                    .collect::<HashSet<char>>()
                    .is_subset(&s.1)
                {
                    digit_map.insert(3, s.0);
                    segment_map.insert(s.0, 3);
                    matched_sets.insert(s.0);
                }
            } else if s.0.len() == 6 {
                if !digit_map[&1]
                    .chars()
                    .collect::<HashSet<char>>()
                    .is_subset(&s.1)
                {
                    digit_map.insert(6, s.0);
                    segment_map.insert(s.0, 6);
                    matched_sets.insert(s.0);
                } else if digit_map[&4]
                    .chars()
                    .collect::<HashSet<char>>()
                    .is_subset(&s.1)
                {
                    digit_map.insert(9, s.0);
                    segment_map.insert(s.0, 9);
                    matched_sets.insert(s.0);
                }
            }
        }
        // Match 5, 0, 2
        for s in &sets {
            if matched_sets.contains(&s.0) {
                continue;
            }
            if s.0.len() == 5 {
                if s.1
                    .is_subset(&digit_map[&9].chars().collect::<HashSet<char>>())
                {
                    digit_map.insert(5, s.0);
                    segment_map.insert(s.0, 5);
                    matched_sets.insert(s.0);
                } else {
                    digit_map.insert(2, s.0);
                    segment_map.insert(s.0, 2);
                    matched_sets.insert(s.0);
                }
            } else {
                digit_map.insert(0, s.0);
                segment_map.insert(s.0, 0);
                matched_sets.insert(s.0);
            }
        }

        let mut digit_sum = 0;
        for e in &entry[1] {
            digit_sum *= 10;
            digit_sum += segment_map[e];
        }
        global_sum += digit_sum;
    }
    println!("{}", global_sum);
}

fn day_9(input: String) {
    let heights: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    fn risk(heights: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
        let h = heights[y][x];
        let high = (x > 0 && heights[y][x - 1] <= h)
            || (x < heights[0].len() - 1 && heights[y][x + 1] <= h)
            || (y > 0 && heights[y - 1][x] <= h)
            || (y < heights.len() - 1 && heights[y + 1][x] <= h);
        if high {
            0
        } else {
            heights[y][x] + 1
        }
    }
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    fn basin(heights: &Vec<Vec<u32>>, visited: &mut HashSet<(i32, i32)>, x: i32, y: i32) -> u32 {
        if visited.contains(&(x, y)) || heights[y as usize][x as usize] == 9 {
            return 0;
        }

        let mut size = 0;
        let mut to_search = vec![(x, y)];
        while !to_search.is_empty() {
            let (px, py) = to_search.pop().unwrap();
            if visited.contains(&(px, py)) || heights[py as usize][px as usize] == 9 {
                continue;
            }
            size += 1;
            visited.insert((px, py));
            for (xx, yy) in [(px + 1, py), (px - 1, py), (px, py + 1), (px, py - 1)] {
                if xx >= 0 && xx < heights[0].len() as i32 && yy >= 0 && yy < heights.len() as i32 {
                    to_search.push((xx, yy));
                }
            }
        }
        size
    }

    let mut risk_sum = 0;
    let mut basins = Vec::new();
    for y in 0..heights.len() {
        for x in 0..heights[0].len() {
            risk_sum += risk(&heights, x, y);
            let b = basin(&heights, &mut visited, x as i32, y as i32);
            if b != 0 {
                basins.push(b);
            }
        }
    }
    println!("{}", risk_sum);
    basins.sort();
    println!("{}", basins.iter().rev().take(3).product::<u32>());
}

fn day_10(input: String) {
    fn line_score(line: &str) -> (i32, u64) {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if stack.is_empty() || stack.pop().unwrap() != '(' {
                        return (3, 0);
                    }
                }
                ']' => {
                    if stack.is_empty() || stack.pop().unwrap() != '[' {
                        return (57, 0);
                    }
                }
                '}' => {
                    if stack.is_empty() || stack.pop().unwrap() != '{' {
                        return (1197, 0);
                    }
                }
                '>' => {
                    if stack.is_empty() || stack.pop().unwrap() != '<' {
                        return (25137, 0);
                    }
                }
                _ => (),
            }
        }
        let mut score = 0;
        for c in stack.iter().rev() {
            let cscore = match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("unexpected character {} on stack", c),
            };
            score = score * 5 + cscore;
        }
        (0, score)
    }
    let corrupt_score = input.lines().map(|l| line_score(l).0).sum::<i32>();
    println!("{}", corrupt_score);
    let mut complete_scores = input
        .lines()
        .map(|l| line_score(l).1)
        .filter(|x| *x != 0)
        .collect::<Vec<u64>>();
    complete_scores.sort();
    println!("{}", complete_scores[complete_scores.len() / 2]);
}

fn day_11(input: String) {
    let mut energy: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut flashes = 0;
    let mut flashes_100 = 0;
    let mut all_flash_gen = 0;
    for gen in 0..1000 {
        let flash_start = flashes;
        let mut to_flash: Vec<(i32, i32)> = Vec::new();
        for x in 0..10 {
            for y in 0..10 {
                energy[y][x] += 1;
                if energy[y][x] == 10 {
                    to_flash.push((x as i32, y as i32));
                }
            }
        }
        while !to_flash.is_empty() {
            let (x, y) = to_flash.pop().unwrap();
            flashes += 1;
            for (xx, yy) in [
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x - 1, y),
                (x + 1, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
            ] {
                if xx == -1 || xx == 10 || yy == -1 || yy == 10 {
                    continue;
                }
                energy[yy as usize][xx as usize] += 1;
                if energy[yy as usize][xx as usize] == 10 {
                    to_flash.push((xx, yy));
                }
            }
        }
        for x in 0..10 {
            for y in 0..10 {
                if energy[y][x] > 9 {
                    energy[y][x] = 0;
                }
            }
        }
        if gen == 99 {
            flashes_100 = flashes;
            if all_flash_gen != 0 {
                break;
            }
        }
        if flashes - flash_start == 100 {
            all_flash_gen = gen;
            if gen > 99 {
                break;
            }
        }
    }
    println!("{}", flashes_100);
    println!("{}", all_flash_gen + 1);
}

fn day_12(input: String) {
    let mut nodes: Vec<&str> = Vec::new();
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let mut pieces = line.split('-');
        let (n1, n2) = (pieces.next().unwrap(), pieces.next().unwrap());
        nodes.push(n1);
        nodes.push(n2);
        graph.entry(n1).or_insert(Vec::new()).push(n2);
        graph.entry(n2).or_insert(Vec::new()).push(n1);
    }

    fn dfs<'a>(
        graph: &'a HashMap<&str, Vec<&str>>,
        path: &mut Vec<&'a str>,
        source: &'a str,
        dest: &'a str,
        allow_revisit: bool,
    ) -> Vec<Vec<&'a str>> {
        if source == dest {
            return vec![vec![dest]];
        }
        path.push(source);
        let mut paths = Vec::new();
        for n in &graph[source] {
            let mut revisit_step = allow_revisit;
            if n.chars().next().unwrap().is_lowercase() && path.contains(n) {
                if !allow_revisit || *n == "start" || *n == "end" {
                    continue;
                } else {
                    revisit_step = false;
                }
            }
            let partial_paths = dfs(graph, path, n, dest, revisit_step);
            for mut p in partial_paths {
                p.push(source);
                paths.push(p);
            }
        }
        path.pop();
        paths
    }
    let mut path = Vec::new();
    let paths = dfs(&graph, &mut path, "start", "end", true);
    println!("{}", paths.len());
}

fn day_13(input: String) {
    let mut points = HashSet::new();
    let mut folds = Vec::new();
    for l in input.lines() {
        if l.is_empty() {
            continue;
        }
        match l.strip_prefix("fold along ") {
            Some(suffix) => {
                let mut xy = suffix.split('=');
                folds.push((
                    xy.next().unwrap(),
                    xy.next().unwrap().parse::<u32>().unwrap(),
                ));
            }
            None => {
                let mut xy = l.split(',');
                points.insert((
                    xy.next().unwrap().parse::<u32>().unwrap(),
                    xy.next().unwrap().parse::<u32>().unwrap(),
                ));
            }
        }
    }

    fn reflect_up(points: &mut HashSet<(u32, u32)>, pivot: u32) {
        let inserts: Vec<(u32, u32)> = points
            .iter()
            .filter(|(_, y)| y > &pivot)
            .map(|(x, y)| (*x, pivot - (y - pivot)))
            .collect();
        points.retain(|(_, y)| y < &pivot);
        points.extend(inserts);
    }
    fn reflect_left(points: &mut HashSet<(u32, u32)>, pivot: u32) {
        let inserts: Vec<(u32, u32)> = points
            .iter()
            .filter(|(x, _)| x > &pivot)
            .map(|(x, y)| (pivot - (x - pivot), *y))
            .collect();
        points.retain(|(x, _)| x < &pivot);
        points.extend(inserts);
    }
    for (i, (dir, pivot)) in folds.iter().enumerate() {
        match *dir {
            "x" => reflect_left(&mut points, *pivot),
            "y" => reflect_up(&mut points, *pivot),
            _ => panic!("unexpected fold"),
        }
        if i == 0 {
            println!("{}", points.len());
        }
    }
    let max_x = *points.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *points.iter().map(|(_, y)| y).max().unwrap();
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
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
    let funcs = [
        day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8, day_9, day_10, day_11, day_12,
        day_13,
    ];

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
