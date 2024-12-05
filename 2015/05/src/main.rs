use itertools::Itertools;
use fancy_regex::Regex;

fn run(title: &str, input: &str) {
    let mut count = 0;
    for line in input.lines() {
        let vowels = line.chars().filter(|&c| "aeiou".contains(c)).count();
        let dupe = line.chars().dedup().count() != line.chars().count();
        let bad = ["ab", "cd", "pq", "xy"].iter().any(|p| line.contains(p));
        let nice = vowels >= 3 && dupe && !bad;
        // println!("{} {}", nice, line);
        if nice {
            count += 1;
        }
    }

    println!("{} part 1: {}", title, count);

    let mut count = 0;

    let re0 = Regex::new(r"(..).*\1").unwrap();
    let re1 = Regex::new(r"(.).\1").unwrap();

    for line in input.lines() {
        let nice = re0.is_match(line).unwrap() && re1.is_match(line).unwrap();
        // println!("{} {} {:?} {:?}", nice, line, re0.is_match(line), re1.is_match(line));
        if nice {
            count += 1;
        }
    }

    println!("{} part 2: {}", title, count);
}

const INPUT_DEMO: &str = "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb
qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("05/input.txt").unwrap());
}
