use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'lonely_integer' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn lonely_integer(a: &[i32]) -> i32 {
    let mut duplicated = HashSet::new();
    let mut unique = HashSet::new();

    for e in a {
        if !unique.contains(e) && !duplicated.contains(e) {
            unique.insert(*e);
        } else {
            duplicated.insert(*e);
            unique.remove(e);
        }
    }

    match unique.iter().next() {
        Some(x) => *x,
        None => panic!("It must have at least one unique value"),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let a: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = lonely_integer(&a);

    writeln!(&mut fptr, "{}", result).ok();
}
