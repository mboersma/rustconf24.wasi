use glob::glob;
use range_set_blaze::prelude::*;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut all_exps = RangeSetBlaze::from_iter([0..=99_999_999]);

    for path in glob("examples/cluster_file.*.tsv")? {
        let exp_nums: RangeSetBlaze<_> = BufReader::new(File::open(path?)?)
            .lines()
            .filter_map(|line| line.ok()?.split_once('\t')?.0.parse::<u32>().ok())
            .collect();
        all_exps = all_exps - exp_nums;
    }
    println!("{all_exps}");
    Ok(())
}
