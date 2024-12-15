// Part 1: 7 mins
// Part 1+2: 11 mins

fn run(title: &str, input: &str, w: i32, h: i32) {
    let data: Vec<(i32, i32, i32, i32)> = input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let p = p[2..].split_once(",").unwrap();
            let v = v[2..].split_once(",").unwrap();
            (p.0.parse().unwrap(), p.1.parse().unwrap(), v.0.parse().unwrap(), v.1.parse().unwrap())
        })
        .collect();

    // println!("{:?}", data);

    let mut quads = vec![0, 0, 0, 0];
    for d in &data {
        let t = 100;

        let x = (d.0 + (d.2 + w) * t) % w;
        let y = (d.1 + (d.3 + h) * t) % h;

        if x < w/2 {
            if y < h/2 {
                quads[0] += 1;
            } else if y > h/2 {
                quads[1] += 1;

            }
        } else if x > w/2 {
            if y < h/2 {
                quads[2] += 1;
            } else if y > h/2 {
                quads[3] += 1;

            }
        }
    }

    println!("{} part 1: {}", title, quads.iter().product::<i32>());
}

fn run2(title: &str, input: &str, w: i32, h: i32) {
    let data: Vec<(i32, i32, i32, i32)> = input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let p = p[2..].split_once(",").unwrap();
            let v = v[2..].split_once(",").unwrap();
            (p.0.parse().unwrap(), p.1.parse().unwrap(), v.0.parse().unwrap(), v.1.parse().unwrap())
        })
        .collect();

    // println!("{:?}", data);

    for t in 0.. {
        let mut grid = Vec::new();
        grid.resize((w*h) as usize, ' ');
        for d in &data {
            let x = (d.0 + (d.2 + w) * t) % w;
            let y = (d.1 + (d.3 + h) * t) % h;
            grid[(x + y*w) as usize] = '#';
        }
        let mut n = 0;
        for y in 1..h-1 {
            for x in 1..w-1 {
                if grid[(x + y*w) as usize] == '#' && grid[(x+1 + (y+1)*w) as usize] == '#' {
                    n += 1;
                }
            }
        }

        if n > 100 {
            println!("======== {} {}", t, n);
            for y in 0..h {
                for x in 0..w {
                    print!("{}", grid[(x + y*w) as usize]);
                }
                println!();
            }
        }
    }
}

const INPUT_DEMO: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

fn main() {
    run("demo", INPUT_DEMO, 11, 7);
    run("input", &std::fs::read_to_string("14/input.txt").unwrap(), 101, 103);
    run2("input", &std::fs::read_to_string("14/input.txt").unwrap(), 101, 103);
}
