use md5::{Md5, Digest};

fn run(title: &str, input: &str) {
    let mut hasher = Md5::new();
    hasher.update(input);

    let mut n = 0;
    for i in 0.. {
        let mut h2 = hasher.clone();
        h2.update(i.to_string().as_bytes());
        let result = h2.finalize();

        if result[0] == 0 && result[1] == 0 && (result[2] & 0xf0) == 0 {
            print!("{:x}", result[2] & 0x0f);
            n += 1;
            if n == 8 { break; }
        }
    }

    println!();

    let mut n = 0;
    let mut pwd = Vec::new();
    for i in 0..8 {
        pwd.push("?".to_owned());
    }
    for i in 0.. {
        let mut h2 = hasher.clone();
        h2.update(i.to_string().as_bytes());
        let result = h2.finalize();

        if result[0] == 0 && result[1] == 0 && (result[2] & 0xf0) == 0 {
            let pos = (result[2] & 0x0f) as usize;
            println!("{:?} {} {}", pwd, pos, n);
            if pos < 8 && pwd[pos] == "?" {
                pwd[pos] = format!("{:x}", result[3] >> 4);
                n += 1;
                if n == 8 { break; }
            }
        }
    }

    println!("{} part 2: {}", title, pwd.join(""));
}

fn main() {
    run("demo", "abc");
    run("input", "reyedfim");
}
