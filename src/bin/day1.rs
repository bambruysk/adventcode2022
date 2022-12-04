use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let read  = File::open("../day1/input.txt");
    if let Err(e) = read {
        println!("ubable open file {}", e);
        return
    }

    let buffered = BufReader::new(read.expect("open file"));



    let mut elves :Vec<Vec<i32>> = Vec::new();
    let mut elf :Vec<i32> = Vec::new();
    for line in buffered.lines() {

        let l = line.unwrap();

        if l.is_empty()  {
            elves.push(elf);
            elf = Vec::new();
        } else {
            elf.push( l.parse::<i32>().unwrap());
        }
    }



    let mut sum_elves :Vec<i32> = elves.iter().map(|x| x.iter().sum()).collect();
    sum_elves.sort_by(|x1, x2| x2.cmp(x1) );


    let s: i32 = sum_elves.iter().take(3).sum();

    println!("{}", s);

    println!("That s all");

}
