use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Result},
};

fn sum_file(path: &str) -> Result<i32> {
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open file \"{}\": {}", path, why),
        Ok(file) => file,
    };

    let sum = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .filter_map(|l| l.parse::<i32>().ok())
        .reduce(|a, item| a + item)
        .unwrap();

    Ok(sum)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("Usage: {} <file>", args[0]);
    }

    let sum = sum_file(&args[1]).unwrap();

    println!("Total sum is {}", sum);
}
