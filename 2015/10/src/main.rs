use fancy_regex::{Captures, Regex};

fn run(title: &str, input: &str, reps: usize) {
    let re = Regex::new(r"([0-9])\1*").unwrap();

    let mut s = input.to_owned();
    for i in 0..reps {
        s = re.replace_all(s.as_str(), |m: &Captures| {
            format!("{}{}", m.get(0).unwrap().as_str().len(), m.get(1).unwrap().as_str())
        }).to_string();
        // println!("{} {}", i, s.len());
    }

    println!("{} part N: {}", title, s.len());

}

fn main() {
    run("demo", "1", 10);
    run("input", "3113322113", 40);
    run("input", "3113322113", 50);
}
