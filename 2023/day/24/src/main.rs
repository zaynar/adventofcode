use itertools::Itertools;
use num::FromPrimitive;
use num::bigint::BigInt;
use num::rational::{Ratio, BigRational};

#[derive(Debug)]
struct Ray {
    pos: (BigInt, BigInt, BigInt),
    vel: (BigInt, BigInt, BigInt),
}

fn dot(a: (BigInt, BigInt, BigInt), b: (BigInt, BigInt, BigInt)) -> BigInt {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn dotff(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn add(a: (BigInt, BigInt, BigInt), b: (BigInt, BigInt, BigInt)) -> (BigInt, BigInt, BigInt) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn addf(a: (BigInt, BigInt, BigInt), b: (f64, f64, f64)) -> (f64, f64, f64) {
    (a.0 as f64 + b.0, a.1 as f64  + b.1, a.2 as f64  + b.2)
}

fn sub(a: (BigInt, BigInt, BigInt), b: (BigInt, BigInt, BigInt)) -> (BigInt, BigInt, BigInt) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn subff(a: (f64, f64, f64), b: (f64, f64, f64)) -> (f64, f64, f64) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn mul(a: (BigInt, BigInt, BigInt), b: BigInt) -> (BigInt, BigInt, BigInt) {
    (a.0 * b, a.1 * b, a.2 * b)
}

fn mulf(a: (BigInt, BigInt, BigInt), b: f64) -> (f64, f64, f64) {
    (a.0 as f64 * b, a.1 as f64  * b, a.2 as f64  * b)
}


fn main() {
    // let range = ((7, 27), (7, 27), (0, 0));
    // let input = std::fs::read_to_string("input-demo").unwrap().lines().map(|line| {
    //     let (p, v) = line.split_once(" @ ").unwrap();
    //     Ray {
    //         pos: p.split(", ").map(|n| n.trim().parse().unwrap()).collect_tuple().unwrap(),
    //         vel: v.split(", ").map(|n| n.trim().parse().unwrap()).collect_tuple().unwrap(),
    //     }
    // }).collect_vec();

    let range = ((
        BigInt::from_i128(200000000000000i128).unwrap(),
        BigInt::from_i128(400000000000000i128).unwrap()
    ),
    (
        BigInt::from_i128(200000000000000i128).unwrap(),
        BigInt::from_i128(400000000000000i128).unwrap()
    ),
    (
        BigInt::from_i128(0).unwrap(),
        BigInt::from_i128(0).unwrap()
    ));
    let input = std::fs::read_to_string("input").unwrap().lines().map(|line| {
        let (p, v) = line.split_once(" @ ").unwrap();
        Ray {
            pos: p.split(", ").map(|n| BigInt::from_i64(n.trim().parse().unwrap()).unwrap()).collect_tuple().unwrap(),
            vel: v.split(", ").map(|n| n.trim().parse().unwrap()).collect_tuple().unwrap(),
        }
    }).collect_vec();

    // Part 1
    let input = input.iter().map(|r| Ray { pos: (r.pos.0, r.pos.1, 0), vel: (r.vel.0, r.vel.1, 0) }).collect_vec();

    // Rays intersect at  pa + |va| * (pb - pa).|va|

    // Rays intersect when
    // pa+va*t = pb+vb*t
    // t = (pb-pa)/(va-vb)
    // p = pa + va*(pb-pa)/(va-vb)

    // pa.x+va.x*t = pb.x+vb.x*t
    // pa.y+va.y*t = pb.y+vb.y*t

    // println!("{:?}", input);

    let mut sum = 0;
    for (ra, rb) in input.iter().tuple_combinations() {
        println!("{:?} {:?}", ra, rb);

        let p1 = ra.pos;
        let p2 = add(ra.pos, ra.vel);
        let p3 = rb.pos;
        let p4 = add(rb.pos, rb.vel);

        // let dmnop = dot(sub(pm, pn), sub(po, pp));
        let d1343 = dot(sub(p1, p3), sub(p4, p3));
        let d4321 = dot(sub(p4, p3), sub(p2, p1));
        let d1321 = dot(sub(p1, p3), sub(p2, p1));
        let d4343 = dot(sub(p4, p3), sub(p4, p3));
        let d2121 = dot(sub(p2, p1), sub(p2, p1));
        if d2121 * d4343 - d4321 * d4321 == 0 {
            println!("Parallel");
            continue;
        }
        let mua = (d1343 * d4321 - d1321 * d4343) as f64 / (d2121 * d4343 - d4321 * d4321) as f64;
        let mub = (d1343 as f64 + mua * d4321 as f64) / d4343 as f64;

        let ca = addf(ra.pos, mulf(ra.vel, mua));
        let cb = addf(rb.pos, mulf(rb.vel, mub));
        let delta = subff(ca, cb);
        let dist2 = dotff(delta, delta);
        println!("  {} {} {}", mua, mub, dist2);

        if mua >= 0.0 && mub >= 0.0 && dist2 < 1e-6 &&
            range.0.0 as f64 <= ca.0 && ca.0 <= range.0.1 as f64  &&
            range.1.0 as f64  <= ca.1 && ca.1 <= range.1.1 as f64  &&
            range.2.0 as f64  <= ca.2 && ca.2 <= range.2.1 as f64
        {
            println!(" OK");
            sum += 1;
        }

        /*
        // let va2 = ra.vel.0 * ra.vel.0 + ra.vel.1 * ra.vel.1 + ra.vel.2 * ra.vel.2;
        // let vb2 = rb.vel.0 * rb.vel.0 + rb.vel.1 * rb.vel.1 + rb.vel.2 * rb.vel.2;
        // let va = (va2 as f64).sqrt();
        // let vb = (vb2 as f64).sqrt();
        let pa = (ra.pos.0 as f64, ra.pos.1 as f64, ra.pos.2 as f64);
        let pb = (rb.pos.0 as f64, rb.pos.1 as f64, rb.pos.2 as f64);
        let va = (ra.vel.0 as f64, ra.vel.1 as f64, ra.vel.2 as f64);
        let vb = (rb.vel.0 as f64, rb.vel.1 as f64, rb.vel.2 as f64);
        let va2 = (va.0.powi(2) + va.1.powi(2) + va.2.powi(2));

        let tu = (
            (pb.0 - pa.0) * va.0 +
            (pb.1 - pa.1) * va.1 +
            (pb.2 - pa.2) * va.2
        );
        let p = (
            pa.0 + va.0 * tu / va2,
            pa.1 + va.1 * tu / va2,
            pa.2 + va.2 * tu / va2,
        );

        let t = tu / va2.sqrt();
        println!("{}\n {:?}\n {:?}\n", t,
            (pa.0 + va.0 * t, pa.1 + va.1 * t, pa.2 + va.2 * t),
            (pb.0 + vb.0 * t, pb.1 + vb.1 * t, pb.2 + vb.2 * t)
        );

    //     let t = if (va.0 - vb.0).abs() < (va.1 - vb.1).abs() {
    //         (pb.0 - pa.0) / (va.0 - vb.0)
    //     } else {
    //         (pb.1 - pa.1) / (va.1 - vb.1)
    //     };
    //     let t = 2.333;
    //     println!("{}\n {:?}\n {:?}\n", t,
    //     (pa.0 + va.0 * t, pa.1 + va.1 * t, pa.2 + va.2 * t),
    //     (pb.0 + vb.0 * t, pb.1 + vb.1 * t, pb.2 + vb.2 * t)
    // )   ;
    //     let p = (pa.0 + va.0 * t, pa.1 + va.1 * t, pa.2 + va.2 * t);
        /*
        let t = f64::sqrt((pb.0 - pa.0).powi(2) + (pb.1 - pa.1).powi(2) + (pb.2 - pa.2).powi(2));
        println!("{:?} {:?} / {:?} {:?}", pa, va, pb, vb);
        let p = if va2 > vb2 {
            let f = va / (va - vb);
            (
                pa.0 + f * (pb.0 - pa.0),
                pa.1 + f * (pb.1 - pa.1),
                pa.2 + f * (pb.2 - pa.2),
            )
        } else {
            let f = vb / (va - vb);
            (
                pb.0 + f * (pb.0 - pa.0),
                pb.1 + f * (pb.1 - pa.1),
                pb.2 + f * (pb.2 - pa.2),
            )
        };
        */
        println!("{} {:?}", tu, p);
        if tu >= 0.0 &&
            range.0.0 <= p.0 && p.0 <= range.0.1 &&
            range.1.0 <= p.1 && p.1 <= range.1.1 &&
            range.2.0 <= p.2 && p.2 <= range.2.1 {
            sum += 1;
        }
        */
    }

    println!("Answer: {}", sum);
}
