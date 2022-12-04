use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let read  = File::open("input.txt");
    if let Err(e) = read {
        println!("ubable open file {}", e);
        return
    }

    let buffered = BufReader::new(read.expect("open file"));

    let mut curr = 0;

    let mut m = 0;

    for line in buffered.lines() {

        let l = line.unwrap();

        if l.is_empty()  {
            if curr > m {
                m = curr
            }
            curr = 0;
        } else {
            curr += l.parse::<i32>().unwrap();
        }
    }

    if curr > m {
        m = curr
    }

    println!("{}", m);

    println!("That s all");

}
