// Part 1: ?
// Part 2: 4 mins

use std::collections::HashMap;

fn run(title: &str, input: u32) {

    let mut x = 0i32;
    let mut y = 0i32;

    let mut grid = HashMap::new();
    let (mut dx, mut dy) = (1, 0);

    for i in 1.. {
        assert!(grid.insert((x, y), i).is_none());

        if i == input {
            println!("{} part 1: {}", title, x.abs() + y.abs());
            break;
        }

        let (ndx, ndy) = (-dy, dx);
        if grid.contains_key(&(x + ndx, y + ndy)) {
            x += dx;
            y += dy;
        } else {
            x += ndx;
            y += ndy;
            (dx, dy) = (ndx, ndy);
        }
    }
}

fn run2(title: &str, input: u32) {

    let mut x = 0i32;
    let mut y = 0i32;

    let mut grid = HashMap::new();
    let (mut dx, mut dy) = (1, 0);

    for i in 1.. {
        let val = (-1..=1).map(|dy| (-1..=1).map(|dx| grid.get(&(x + dx, y + dy)).unwrap_or(&0)).sum::<u32>()).sum::<u32>();

        let val = if val == 0 { 1 } else { val };
        // println!("{},{} {}", x, y, val);

        assert!(grid.insert((x, y), val).is_none());

        if val > input {
            println!("{} part 2: {}", title, val);
            break;
        }

        let (ndx, ndy) = (-dy, dx);
        if grid.contains_key(&(x + ndx, y + ndy)) {
            x += dx;
            y += dy;
        } else {
            x += ndx;
            y += ndy;
            (dx, dy) = (ndx, ndy);
        }
    }
}

const INPUT_DEMO: &str = "";

fn main() {
    run("demo", 1024);
    run2("demo", 1024);
    run("input", 368078);
    run2("input", 368078);
}
