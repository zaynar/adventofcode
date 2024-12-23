// Part 1: 9 mins
// Part 1+2: 10 mins

fn test(mut vx: i32, mut vy: i32, tx: (i32, i32), ty: (i32, i32)) -> Option<i32> {
    let mut maxy = 0;

    let mut x = 0;
    let mut y = 0;
    loop {
        x += vx;
        y += vy;
        vx -= vx.signum();
        vy -= 1;

        maxy = maxy.max(y);

        // println!("x={} y={} vx={} vy={}", x, y, vx, vy);

        if tx.0 <= x && x <= tx.1 && ty.0 <= y && y <= ty.1 {
            return Some(maxy);
        } else if x > tx.1 || y < ty.0 {
            return None;
        }
    }
}

fn run(title: &str, tx: (i32, i32), ty: (i32, i32)) {

    let mut part2 = 0;
    let mut maxy = 0;
    for vx in 1..1000 {
        for vy in -1000..1000 {
            if let Some(m) = test(vx, vy, tx, ty) {
                maxy = maxy.max(m);
                part2 += 1;
            }
        }
    }

    println!("{} part 1: {}", title, maxy);

    println!("{} part 2: {}", title, part2);
}

fn main() {
    run("demo", (20, 30), (-10, -5));
    run("input", (201, 230), (-99, -65));
}
