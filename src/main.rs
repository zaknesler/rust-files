use core::fmt;
use std::{
    collections::HashMap,
    env,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Result},
};

struct FileStats {
    sum: i32,
    mean: f32,
    median: i32,
    mode: i32,
    min: i32,
    max: i32,
}

impl Display for FileStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Stats:\nsum = {}\nmean = {}\nmedian = {}\nmode = {}\nmin = {}\nmax = {}",
            self.sum, self.mean, self.median, self.mode, self.min, self.max
        )
    }
}

fn collect_stats(nums: &Vec<i32>) -> Result<FileStats> {
    Ok(FileStats {
        sum: nums.iter().copied().sum::<i32>(),
        mean: nums.iter().copied().sum::<i32>() as f32 / nums.len() as f32,
        median: {
            let mut sorted: Vec<i32> = nums.clone();
            sorted.sort();
            let mid = sorted.len() / 2;
            if sorted.len() % 2 == 1 {
                sorted[mid - 1]
            } else {
                ((sorted[mid - 1] + sorted[mid]) as f32 / 2.0) as i32
            }
        },
        mode: nums
            .iter()
            .copied()
            .fold(HashMap::with_capacity(nums.len()), |mut freq, x| {
                *freq.entry(x).or_insert(0) += 1;
                freq
            })
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(v, _)| v)
            .unwrap(),
        min: nums.iter().copied().min().unwrap(),
        max: nums.iter().copied().max().unwrap(),
    })
}

fn file_nums(path: &str) -> Result<Vec<i32>> {
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open file \"{}\": {}", path, why),
        Ok(file) => file,
    };

    let nums: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .filter_map(|l| l.parse::<i32>().ok())
        .collect();

    Ok(nums)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("Usage: {} <file>", args[0]);
    }

    let nums = file_nums(&args[1]).unwrap();
    let stats = collect_stats(&nums).unwrap();

    println!("{}", stats);
}
