
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let read  = File::open("src/day2/input.txt");
    if let Err(e) = read {
        println!("ubable open file {}", e);
        return
    }

    let buffered = BufReader::new(read.expect("open file"));

    let mut sum: u32 = 0 ;

    for line in buffered.lines() {

        let l = line.unwrap();
        let (e,m ) = parse_line(l);
        sum += calculate(e,m);


    }



    println!("{}", sum);

    println!("That s all");

}

enum Enemy {
    A,B,C
}

enum Move {
    X,Y,Z
}

fn parse_line(line : String) -> (Enemy, Move) {
    let ab : Vec<&str> = line.split(" ").collect();
    let a = match ab[0] {
        "A" => Enemy::A,
        "B" => Enemy::B,
        "C" => Enemy::C,
        _ => Enemy::A
    };

    let b = match ab[1] {
        "X" => Move::X,
        "Y" => Move::Y,
        "Z" => Move::Z,
        _ => Move::X
    };

    (a,b)
}

fn calculate (e : Enemy, m :Move) -> u32 {
    let bonus = match m {
        Move::X => 1,
        Move::Y => 2,
        Move::Z => 3,
    };

    let win = match e {
        Enemy::A => {
            match m {
                Move::X => {3}
                Move::Y => {6}
                Move::Z => {0}
            }
        }
        Enemy::B => {
            match m {
                Move::X => {0}
                Move::Y => {3}
                Move::Z => {6}
            }
        }
        Enemy::C => {
            match m {
                Move::X => {6}
                Move::Y => {0}
                Move::Z => {3}
            }
        }
    };

    bonus + win
}