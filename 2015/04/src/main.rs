use md5::{Md5, Digest};

fn main() {

    let mut hasher = Md5::new();
    hasher.update(b"bgvyzdsv");
    // hasher.update(b"abcdef");

    for i in 0.. {
        let mut h2 = hasher.clone();
        h2.update(i.to_string().as_bytes());
        let result = h2.finalize();

        if result[0] == 0 && result[1] == 0 && (result[2] & 0xf0) == 0 {
            println!("part 1: {}", i);
            break;
        }
    }

    for i in 0.. {
        let mut h2 = hasher.clone();
        h2.update(i.to_string().as_bytes());
        let result = h2.finalize();

        if result[0] == 0 && result[1] == 0 && result[2] == 0 {
            println!("part 2: {}", i);
            break;
        }
    }
}