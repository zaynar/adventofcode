use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

#[derive(Debug)]
struct Number {
    value: u32,
    x: usize,
    y: usize,
    w: usize,
}

fn is_symbol(c: char) -> bool {
    match c {
        '0'..='9' => false,
        '.' => false,
        _ => true
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    let mut numbers: Vec<Number> = Vec::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut num = None;
        for (x, c) in line.chars().enumerate() {
            grid.insert((x, y), c);

            if c.is_ascii_digit() {
                let mut n = num.unwrap_or(Number { value: 0, x, y, w: 0 });
                n.value = n.value * 10 + c.to_digit(10).unwrap();
                n.w += 1;
                num = Some(n);
            } else {
                if let Some(n) = num {
                    numbers.push(n);
                }
                num = None;
            }
        }

        if let Some(n) = num {
            numbers.push(n);
        }
    }

    // println!("{:?}", grid);
    // println!("{:#?}", numbers);

    let mut sum = 0;

    for n in numbers.iter() {
        let mut has_symbol = false;
        for y in n.y.saturating_sub(1)..=(n.y + 1) {
            for x in n.x.saturating_sub(1)..=(n.x + n.w) {
                if let Some(c) = grid.get(&(x, y)) {
                    if is_symbol(*c) {
                        has_symbol = true;
                    }
                }
            }
        }
        if has_symbol {
            sum += n.value;
        }
    }

    println!("{}", sum);

    let mut grid_nums: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for n in numbers.iter() {
        for y in n.y.saturating_sub(1)..=(n.y + 1) {
            for x in n.x.saturating_sub(1)..=(n.x + n.w) {
                grid_nums.entry((x, y)).or_insert_with(|| Vec::new()).push(n.value);
            }
        }
    }

    let mut sum_gears = 0;

    for ((x, y), c) in grid {
        if c == '*' {
            if let Some(nums) = grid_nums.get(&(x, y)) {
                if nums.len() == 2 {
                    sum_gears += nums[0] * nums[1];

                }
            }
        }
    }

    println!("{}", sum_gears);
}
