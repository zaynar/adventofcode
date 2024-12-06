peg::parser! {
    grammar input_parser() for str {
        rule num() -> i32
            = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

        rule ident() -> String
            = s:$(['A'..='Z' | 'a'..='z']+) { s.to_owned() }

        rule line() -> (i32, i32, i32, i32, i32)
            = ident() " can fly " v:num() " km/s for " t:num() " seconds, but then must rest for " r:num() " seconds.\n" { (v, t, r, 0, 0) }

        pub rule file() -> Vec<(i32, i32, i32, i32, i32)>
            = line()+
    }
}

fn run(title: &str, input: &str, steps: i32) {
    let mut data = input_parser::file(input).unwrap();

    println!("{:?}", data);

    for i in 0..steps {
        for (v,t,r,d,pts) in &mut data {
            if i % (*t + *r) < *t {
                *d += *v;
            }
        }

        let max = data.iter().map(|(_, _, _, d, _)| *d).max().unwrap();
        for (v,t,r,d,pts) in &mut data {
            if *d == max {
                *pts += 1
            }
        }
    }

    println!("{:?}", data);

    println!("{} part 1: {:?}", title, data.iter().map(|(_, _, _, d, _)| *d).max());
    println!("{} part 2: {:?}", title, data.iter().map(|(_, _, _, _, pts)| *pts).max());
}

const INPUT_DEMO: &str = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
";

fn main() {
    run("demo", INPUT_DEMO, 1000);
    run("demo", INPUT_DEMO, 2503);
    run("input", &std::fs::read_to_string("14/input.txt").unwrap(), 2503);
}
