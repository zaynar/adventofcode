use std::fs;

#[derive(Debug)]
struct Range {
    dest: u64,
    src: u64,
    len: u64,
}

#[derive(Debug)]
struct InputFile {
    seeds: Vec<u64>,
    maps: Vec<Vec<Range>>,
}

peg::parser! {
    grammar input_parser() for str {
        rule number() -> u64
            = n:$(['0'..='9']+) {? n.parse().or(Err("u64")) }

        pub rule list() -> Vec<u64>
            = l:(number() ** " ") { l }

        pub rule seeds() -> Vec<u64>
            = "seeds: " l:list() "\n\n" { l }

        pub rule range() -> Range
            = dest:number() " " src:number() " " len:number() "\n" { Range { dest, src, len } }

        pub rule map() -> Vec<Range>
            = ['a'..='z' | '-']+ " map:\n" rs:range()+ { rs }

        pub rule file() -> InputFile
            = seeds:seeds() maps:(map() ++ "\n") { InputFile { seeds, maps } }
    }
}

pub fn main() {
    let input = input_parser::file(&fs::read_to_string("input").unwrap()).unwrap();

    let mut locs = Vec::new();
    for &seed in &input.seeds {
        let mut val = seed;
        for map in &input.maps {
            for range in map {
                if val >= range.src && val < range.src + range.len {
                    val = val.wrapping_sub(range.src).wrapping_add(range.dest);
                    break;
                }
            }
        }
        println!("{} {}", seed, val);
        locs.push(val);
    }
    println!("Answer: {}", locs.iter().min().unwrap());

    for pair in input.seeds.chunks_exact(2) {
        let start = pair[0];
        let len = pair[1];

        let mut min_loc = u64::MAX;

        for seed in start..start+len {
            let mut val = seed;
            for map in &input.maps {
                for range in map {
                    if val >= range.src && val < range.src + range.len {
                        val = val.wrapping_sub(range.src).wrapping_add(range.dest);
                        break;
                    }
                }
            }
            min_loc = min_loc.min(val);
        }
        println!("{} {} {}", start, len, min_loc);
    }
}
