// Part 1: 3 mins
// Part 1+2: 14 mins

fn run(title: &str, input: &str) {
    let fields = vec![
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
        // "cid",
    ];

    let mut part1 = 0;
    let mut part2 = 0;
    for entry in input.split("\n\n") {
        if fields.iter().all(|f| {
            entry.contains(&format!("{}:", f))
        }) {
            part1 += 1;

            let ok = entry.trim().split_ascii_whitespace().all(|f| {
                let (k, v) = f.split_once(":").unwrap();
                let ok = match k {
                    "byr" => {
                        let n: i32 = v.parse().unwrap();
                        v.len() == 4 && n >= 1920 && n <= 2002
                    },
                    "iyr" => {
                        let n: i32 = v.parse().unwrap();
                        v.len() == 4 && n >= 2010 && n <= 2020
                    },
                    "eyr" => {
                        let n: i32 = v.parse().unwrap();
                        v.len() == 4 && n >= 2020 && n <= 2030
                    },
                    "hgt" => {
                        if v.ends_with("cm") {
                            let n: i32 = v.strip_suffix("cm").unwrap().parse().unwrap();
                            n >= 150 && n <= 193
                        } else if v.ends_with("in") {
                            let n: i32 = v.strip_suffix("in").unwrap().parse().unwrap();
                            n >= 59 && n <= 76
                        } else {
                            false
                        }
                    },
                    "hcl" => {
                        v.len() == 7 && v.chars().nth(0).unwrap() == '#' && u32::from_str_radix(&v[1..], 16).is_ok()
                    },
                    "ecl" => {
                        ["amb","blu","brn","gry","grn","hzl","oth"].contains(&v)
                    }
                    "pid" => {
                        v.len() == 9 && u64::from_str_radix(v, 10).is_ok()
                    }
                    "cid" => {
                        true
                    }
                    _ => panic!(),
                };
                // println!("{} {} {}", k, ok, v);
                ok
            });
            if ok {
                part2 += 1;
            }
        }
    }

    println!("{} part 1: {}", title, part1);

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

fn main() {
    run("demo", INPUT_DEMO);

    run("invalid", "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
");

    run("valid", "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
");

    run("input", &std::fs::read_to_string("04/input.txt").unwrap());
}
