use rand::prelude::SliceRandom;
use std::env;

const HELP_MESSAGE: &str = "this is a program to show you The 100
prisoners solution you can pars to the 
program as an argument the number of 
tests that will be preformed";

fn main() {
    let args: Vec<String> = env::args().collect();

    let test_count = match config_args(args) {
        Ok(v) => v,
        Err(_) => {
            std::process::exit(0);
        }
    };

    println!("{HELP_MESSAGE} \n");

    let mut failures = 0;
    let mut successes = 0;

    for _ in 0..test_count {
        let success = run_for_n_prisoners(100);
        if success {
            successes += 1;
        } else {
            failures += 1;
        }
    }

    println!("tests: {test_count}");
    println!("successes: {successes}");
    println!("failures: {failures}");
    let percent: f64 = 100.0 * successes as f64 / test_count as f64;
    println!("{percent}%");
}

fn config_args(args: Vec<String>) -> Result<i32, ()> {
    if args.len() >= 2 {
        if args[1] == "-h" || args[1] == "--help" {
            println!("{HELP_MESSAGE}");

            return Err(());
        } else {
            return match args[1].parse() {
                Ok(v) => Ok(v),
                Err(_) => {
                    println!("illegal argument write command -h or --help");
                    Err(())
                }
            };
        }
    } else {
        Ok(1)
    }
}

fn run_for_n_prisoners(n: i32) -> bool {
    let boxes = get_boxes(n);

    for i in 0..n {
        let count = search(&boxes, i, i as usize, 0);
        // println!("{i} {count}");
        if count > 50 {
            return false;
        }
    }

    true
}

fn search(boxes: &Vec<i32>, prisoner: i32, last_value: usize, count: i32) -> i32 {
    let value = boxes[last_value];

    // println!("key: {last_value}, value: {value}, count {count}, prisoner {prisoner}");

    if value != prisoner {
        return search(boxes, prisoner, value as usize, count + 1);
    }
    count + 1
}

fn get_boxes(prisoners: i32) -> Vec<i32> {
    let mut boxes: Vec<i32> = (0..prisoners).collect();

    let mut rng = rand::thread_rng();
    boxes.shuffle(&mut rng);

    boxes
}
