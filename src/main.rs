use rand::prelude::SliceRandom;

fn main() {
    let mut failures = 0;
    let mut successes = 0;

    let test_count = 10000;
    for _ in 0..test_count {
        let success = run_for_n_prisoners(100);
        if success {
            successes += 1;
        } else {
            failures += 1;
        }
    }

    println!("successes: {successes}, failures: {failures}");
    let percent: f64 = 100.0 * successes as f64 / test_count as f64;
    println!("{percent}%");
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
