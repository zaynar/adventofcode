use itertools::Itertools;

fn dragon(mut a: Vec<u8>, len: usize) -> Vec<u8> {
    while a.len() < len {

        let mut b = a.clone();
        b.reverse();
        b = b.iter().map(|c| 1 - *c).collect_vec();
        a.push(0);
        a.append(&mut b);
    }
    a[0..len].to_vec()
}

fn csum(a: Vec<u8>) -> Vec<u8> {
    let mut r = Vec::new();

    for cs in a.chunks_exact(2) {
        if cs[0] == cs[1] {
            r.push(1);
        } else {
            r.push(0);
        }
    }

    if r.len() % 2 == 0 {
        csum(r)
    } else {
        r
    }
}


fn main() {
    // println!("{:?}", csum([1,1,0,0,1,0,1,1,0,1,0,0].to_vec()));

    let v = dragon([1,0,1,1,1,0,1,1,1,1,1,0,0,1,1,1,1].to_vec(), 272);
    println!("{:?}", csum(v).iter().map(|c| c.to_string()).collect::<String>());

    let v = dragon([1,0,1,1,1,0,1,1,1,1,1,0,0,1,1,1,1].to_vec(), 35651584);
    println!("{:?}", csum(v).iter().map(|c| c.to_string()).collect::<String>());
}
