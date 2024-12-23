// Part 1: 8 mins
// Part 1+2: 29 mins

use itertools::Itertools;

fn run(title: &str, input: &str) {

    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines() {
        let (ps, out) = line.split_once(" | ").unwrap();
        let ps = ps.split_ascii_whitespace().map(|s| s.chars().sorted().collect_vec()).collect_vec();
        let out = out.split_ascii_whitespace().map(|s| s.chars().sorted().collect_vec()).collect_vec();

        let mut map = vec![Vec::<char>::new(); 10];
        for p in &ps {
            if [2, 4, 3, 7].contains(&p.len()) {
                part1 += out.iter().filter(|s| *s == p).count();
            }

            if p.len() == 2 {
                map[1] = p.clone();
            } else if p.len() == 4 {
                map[4] = p.clone();
            } else if p.len() == 3 {
                map[7] = p.clone();
            } else if p.len() == 7 {
                map[8] = p.clone();
            }
        }

        for p in &ps {
            if p.len() == 5 {
                if map[7].iter().all(|n| p.contains(&n)) {
                    map[3] = p.clone();
                } else if map[4].iter().filter(|n| p.contains(&n)).count() == 3 {
                    map[5] = p.clone();
                } else {
                    map[2] = p.clone();
                }
            }
        }

        for p in &ps {
            if p.len() == 6 {
                if map[3].iter().all(|n| p.contains(&n)) {
                    map[9] = p.clone();
                } else if map[1].iter().all(|n| p.contains(&n)) {
                    map[0] = p.clone();
                } else {
                    map[6] = p.clone();
                }
            }
        }

        let mut a = '?';
        let mut b = '?';
        let mut c = '?';
        let mut d = '?';
        let mut e = '?';
        let mut f = '?';
        let mut g = '?';

        for s in &map[7] {
            if !map[1].contains(&s) {
                a = *s;
            }
        }

        for s in &map[8] {
            if !map[0].contains(&s) {
                d = *s;
            }
            if !map[6].contains(&s) {
                c = *s;
            }
            if !map[9].contains(&s) {
                e = *s;
            }
        }

        for s in &map[7] {
            if *s != a && *s != c {
                f = *s;
            }
        }

        for s in &map[8] {
            if !map[2].contains(&s) && *s != f {
                b = *s;
            }
        }

        for s in &map[8] {
            if ![a, b, c, d, e, f].contains(s) {
                g = *s;
            }
        }

        let mut num = 0;

        for o in &out {
            let segs = [a, b, c, d, e, f, g].iter().map(|n| if o.contains(n) { 1 } else { 0 }).collect_vec();
            num = num*10 + match segs.as_slice() {
                [1,1,1,0,1,1,1] => 0,
                [0,0,1,0,0,1,0] => 1,
                [1,0,1,1,1,0,1] => 2,
                [1,0,1,1,0,1,1] => 3,
                [0,1,1,1,0,1,0] => 4,
                [1,1,0,1,0,1,1] => 5,
                [1,1,0,1,1,1,1] => 6,
                [1,0,1,0,0,1,0] => 7,
                [1,1,1,1,1,1,1] => 8,
                [1,1,1,1,0,1,1] => 9,
                _ => panic!(),
            };
        }

        println!("{}", num);
        part2 += num;
        // println!("{:?} {} {} {} {} {} {} {}", map, a, b, c, d, e, f, g);
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
";

const INPUT_DEMO2: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

fn main() {
    // run("demo", INPUT_DEMO);
    run("demo", INPUT_DEMO2);
    run("input", &std::fs::read_to_string("08/input.txt").unwrap());
}
