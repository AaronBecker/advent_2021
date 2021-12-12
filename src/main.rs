use std::env;
use std::fs;
use std::path::Path;

fn problem_input(input_num: i32) -> String {
    
    let inpath = Path::new(env!("CARGO_MANIFEST_DIR")).join("inputs").join(format!("{}", input_num));
    return fs::read_to_string(inpath.clone()).expect(format!("Unable to read file {}", inpath.display()).as_str());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <input number>", args[0]);
        return
    }
    let input_num = args[1].parse::<i32>().expect(format!("Couldn't parse {} as a problem number.", args[1]).as_str());
    println!("{}", problem_input(input_num));
}
