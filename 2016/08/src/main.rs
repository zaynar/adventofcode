#[derive(Debug)]
enum Step {
    Rect(usize, usize),
    RR(usize, usize),
    RC(usize, usize),
}

peg::parser! {
    grammar input_parser() for str {
        rule num() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

        rule rect() -> Step
            = "rect " a:num() "x" b:num() "\n" { Step::Rect(a, b) }

        rule rr() -> Step
            = "rotate row y=" a:num() " by " b:num() "\n" { Step::RR(a, b) }

        rule rc() -> Step
            = "rotate column x=" a:num() " by " b:num() "\n" { Step::RC(a, b) }

        rule step() -> Step
            = rect() / rr() / rc()

        pub rule file() -> Vec<Step>
            = step()+
    }
}

fn run(title: &str, input: &str) {

    let data = input_parser::file(input).unwrap();

    // println!("{:?}", data);

    let mut grid = [[false; 50]; 6];

    let w = 50;
    let h = 6;

    for step in &data {
        // println!("{:?}", step);
        match step {
            &Step::Rect(a, b) => {
                for y in 0..b {
                    for x in 0..a {
                        grid[y][x] = true;
                    }
                }
            }
            &Step::RR(y, b) => {
                let mut new = grid.clone();
                for x in 0..w {
                    new[y][x] = grid[y][(x+w-b)%w];
                }
                grid = new;
            }
            &Step::RC(x, b) => {
                let mut new = grid.clone();
                for y in 0..h {
                    new[y][x] = grid[(y+h-b)%h][x];
                }
                grid = new;
            }
        }
    }

    println!("{} part 1: {}", title, grid.iter().map(|row| row.iter().filter(|c| **c).count()).sum::<usize>());

    for y in 0..h {
        for x in 0..w {
            print!("{}", if grid[y][x] { '#' } else { ' ' });
        }
        println!();
    }

}

fn main() {
    run("input", &std::fs::read_to_string("08/input.txt").unwrap());
}
