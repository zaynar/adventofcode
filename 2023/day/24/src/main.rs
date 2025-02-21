use std::f64::consts::PI;

use itertools::Itertools;
use num::{FromPrimitive, Zero, ToPrimitive};
use num::bigint::BigInt;
use num::rational::{Ratio, BigRational};

extern crate nalgebra as na;
use na::{Vector3, Vector2};

#[derive(Debug)]
struct Ray {
    pos: (BigInt, BigInt, BigInt),
    vel: (BigInt, BigInt, BigInt),
}

#[derive(Debug)]
struct RayVec {
    pos: Vector3<f64>,
    vel: Vector3<f64>,
}

#[derive(Debug)]
struct RayBigVec {
    pos: Vector3<BigRational>,
    vel: Vector3<BigRational>,
}

fn dot(a: (BigInt, BigInt, BigInt), b: (BigInt, BigInt, BigInt)) -> BigInt {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn dotf(a: (BigRational, BigRational, BigRational), b: (BigRational, BigRational, BigRational)) -> BigRational {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn add(a: (BigInt, BigInt, BigInt), b: (BigInt, BigInt, BigInt)) -> (BigInt, BigInt, BigInt) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn addf(a: &(BigInt, BigInt, BigInt), b: &(BigRational, BigRational, BigRational)) -> (BigRational, BigRational, BigRational) {
    (BigRational::from(a.0.clone()) + b.0.clone(), BigRational::from(a.1.clone()) + b.1.clone(), BigRational::from(a.2.clone()) + b.2.clone())
}

fn sub(a: &(BigInt, BigInt, BigInt), b: &(BigInt, BigInt, BigInt)) -> (BigInt, BigInt, BigInt) {
    (a.0.clone() - b.0.clone(), a.1.clone() - b.1.clone(), a.2.clone() - b.2.clone())
}

fn subff(a: (BigRational, BigRational, BigRational), b: (BigRational, BigRational, BigRational)) -> (BigRational, BigRational, BigRational) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn mul(a: (BigInt, BigInt, BigInt), b: BigInt) -> (BigInt, BigInt, BigInt) {
    (a.0 * b.clone(), a.1 * b.clone(), a.2 * b.clone())
}

fn mulf(a: &(BigInt, BigInt, BigInt), b: &BigRational) -> (BigRational, BigRational, BigRational) {
    (BigRational::from(a.0.clone()) * b.clone(), BigRational::from(a.1.clone()) * b.clone(), BigRational::from(a.2.clone()) * b.clone())
}


fn main() {
    // part1();
    part2(false);
}

fn part1() {
    /*
    let range = ((
        BigRational::from_i128(7).unwrap(),
        BigRational::from_i128(27).unwrap(),
    ),
    (
        BigRational::from_i128(7).unwrap(),
        BigRational::from_i128(27).unwrap(),
    ),
    (
        BigRational::zero(),
        BigRational::zero(),
    ));
    let input = std::fs::read_to_string("input-demo").unwrap().lines().map(|line| {
        let (p, v) = line.split_once(" @ ").unwrap();
        Ray {
            pos: p.split(", ").map(|n| n.trim().parse().unwrap()).collect_tuple().unwrap(),
            vel: v.split(", ").map(|n| n.trim().parse().unwrap()).collect_tuple().unwrap(),
        }
    }).collect_vec();
/ */
    let range = ((
        BigRational::from_i128(200000000000000i128).unwrap(),
        BigRational::from_i128(400000000000000i128).unwrap()
    ),
    (
        BigRational::from_i128(200000000000000i128).unwrap(),
        BigRational::from_i128(400000000000000i128).unwrap()
    ),
    (
        BigRational::from_i128(0).unwrap(),
        BigRational::from_i128(0).unwrap()
    ));
    let input = std::fs::read_to_string("input").unwrap().lines().map(|line| {
        let (p, v) = line.split_once(" @ ").unwrap();
        Ray {
            pos: p.split(", ").map(|n| n.trim().parse().unwrap()).collect_tuple().unwrap(),
            vel: v.split(", ").map(|n| n.trim().parse().unwrap()).collect_tuple().unwrap(),
        }
    }).collect_vec();
    // */

    // Part 1
    let input = input.iter().map(|r| Ray { pos: (r.pos.0.clone(), r.pos.1.clone(), BigInt::from(0)), vel: (r.vel.0.clone(), r.vel.1.clone(), BigInt::from(0)) }).collect_vec();

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

        let p1 = ra.pos.clone();
        let p2 = add(ra.pos.clone(), ra.vel.clone());
        let p3 = rb.pos.clone();
        let p4 = add(rb.pos.clone(), rb.vel.clone());

        // let dmnop = dot(sub(pm, pn), sub(po, pp));
        let d1343 = dot(sub(&p1, &p3), sub(&p4, &p3));
        let d4321 = dot(sub(&p4, &p3), sub(&p2, &p1));
        let d1321 = dot(sub(&p1, &p3), sub(&p2, &p1));
        let d4343 = dot(sub(&p4, &p3), sub(&p4, &p3));
        let d2121 = dot(sub(&p2, &p1), sub(&p2, &p1));
        if d2121.clone() * d4343.clone() - d4321.clone() * d4321.clone() == BigInt::from(0) {
            println!("Parallel");
            continue;
        }
        let mua = BigRational::new(d1343.clone() * d4321.clone() - d1321.clone() * d4343.clone(), d2121.clone() * d4343.clone() - d4321.clone() * d4321.clone());
        let mub = (BigRational::from(d1343.clone()) + mua.clone() * d4321.clone()) / d4343.clone();

        let ca = addf(&ra.pos, &mulf(&ra.vel, &mua));
        let cb = addf(&rb.pos, &mulf(&rb.vel, &mub));
        let delta = subff(ca.clone(), cb.clone());
        let dist2 = dotf(delta.clone(), delta.clone());
        println!("  {} {} {}", mua, mub, dist2);

        if mua >= BigRational::zero() && mub >= BigRational::zero() && dist2 == BigRational::zero() &&
            range.0.0 <= ca.0 && ca.0 <= range.0.1 &&
            range.1.0 <= ca.1 && ca.1 <= range.1.1 &&
            range.2.0 <= ca.2 && ca.2 <= range.2.1
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

struct Ray2 {
    pos: Vector2<f64>,
    vel: Vector2<f64>
}

fn test_dir(rays: &Vec<RayVec>, dir: &Vector3<f64>) -> f64 {
    let dir = dir.normalize();
    let up = if dir.x.abs() > dir.y.abs() {
        Vector3::new(0.0, 1.0, 0.0)
    } else {
        Vector3::new(1.0, 0.0, 0.0)
    };
    let ax0 = up.cross(&dir);
    let ax1 = ax0.cross(&dir);
    // let perp = if dir.x.abs() > dir.y.abs() {
    //     Vector3::new(0.0, 1.0, 0.0).cross(&dir)
    // } else {
    //     Vector3::new(1.0, 0.0, 0.0).cross(&dir)
    // };
    // println!("{:?} {:?} {:?}", dir, perp, dir.dot(&perp));

    let projected = rays.iter().map(|ray|
        Ray2 { pos: Vector2::new(
            ray.pos.dot(&ax0),
            ray.pos.dot(&ax1),
        ), vel:
        Vector2::new(
            ray.vel.dot(&ax0),
            ray.vel.dot(&ax1),
        )}
    ).collect_vec();

    let mut points = Vec::new();
    for (p0, p1) in projected.iter().tuple_combinations() {
        let pv = p1.vel.perp(&p0.vel);
        if pv != 0.0 {
            // let c = p0.pos + (p0.vel - p1.vel) * (p1.pos - p0.pos).dot(&(p0.vel - p1.vel));
            let c = p0.pos + p1.vel.perp(&(p1.pos - p0.pos)) / pv * p0.vel;
            points.push(c);
        }
    }

    // println!("{:?}", points);
    let mean = points.iter().sum1::<Vector2<f64>>().unwrap() / points.len() as f64;
    let error = points.iter().map(|p| (p - mean).norm()).sum::<f64>() / points.len() as f64;

    error
}

fn float_loop(start: f64, threshold: f64, step_size: f64) -> impl Iterator<Item = f64> {
    std::iter::successors(Some(start), move |&prev| {
        let next = prev + step_size;
        (next < threshold).then_some(next)
    })
}

fn intersection(a0: &RayBigVec, a1: &RayBigVec) -> Option<
    (Vector2<BigRational>, BigRational, BigRational)
    > {
    let p0 = Vector2::new(a0.pos.x.clone(), a0.pos.y.clone());
    let p1 = Vector2::new(a1.pos.x.clone(), a1.pos.y.clone());
    let v0 = Vector2::new(a0.vel.x.clone(), a0.vel.y.clone());
    let v1 = Vector2::new(a1.vel.x.clone(), a1.vel.y.clone());
    let pv = v1.perp(&v0);
    if pv.is_zero() {
        return None;
    }
    // let c = p0.pos + (p0.vel - p1.vel) * (p1.pos - p0.pos).dot(&(p0.vel - p1.vel));
    let t0 = v1.perp(&(p1.clone() - p0.clone())) / pv.clone();
    let t1 = v0.perp(&(p1.clone() - p0.clone())) / pv.clone();
    let c = p0.clone() + v0.clone() * t0.clone();

    // println!("A: {}", p0.clone() + v0 * t0.clone());
    // println!("B: {}", p1.clone() + v1 * t1.clone());

    Some((c, t0, t1))
}



fn part2(demo: bool) {
    let input = std::fs::read_to_string(if demo { "input-demo" } else { "input" }).unwrap().lines().map(|line| {
        let (p, v) = line.split_once(" @ ").unwrap();
        Ray {
            pos: p.split(", ").map(|n| n.trim().parse().unwrap()).collect_tuple().unwrap(),
            vel: v.split(", ").map(|n| n.trim().parse().unwrap()).collect_tuple().unwrap(),
        }
    }).collect_vec();

    let input_v = input.iter().map(|r| RayVec {
        pos: Vector3::new(r.pos.0.to_f64().unwrap(), r.pos.1.to_f64().unwrap() /*- 300000000000000.0*/, r.pos.2.to_f64().unwrap()),
        vel: Vector3::new(r.vel.0.to_f64().unwrap(), r.vel.1.to_f64().unwrap(), r.vel.2.to_f64().unwrap()),
    }).collect_vec();

    let input_bv = input.iter().map(|r| RayBigVec {
        pos: Vector3::new(r.pos.0.clone().into(), r.pos.1.clone().into(), r.pos.2.clone().into()),
        vel: Vector3::new(r.vel.0.clone().into(), r.vel.1.clone().into(), r.vel.2.clone().into()),
    }).collect_vec();

    if false {
        let mut thetar = (0.0, 2.0*PI, 0.1);
        let mut phir = (0.0, 0.5*PI, 0.1);
        let mut r = Vector3::zero();
        for i in 0..24 {
            // println!("{:?} {:?}", thetar, phir);
            let mut errors = Vec::new();
            for theta in float_loop(thetar.0, thetar.1, thetar.2) {
                for phi in float_loop(phir.0, phir.1, phir.2) {
                    let dir = Vector3::new(theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin());
                    let error = test_dir(&input_v, &dir);
                    // println!("{} {:?}", error, dir);
                    errors.push((error, theta, phi, dir));
                }
            }
            errors.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let best = errors[0];
            // println!("{:?}", errors);
            println!("{:?} {:?}", best, best.3 * Vector3::new(-3.0, 1.0, 2.0).norm());
            thetar = (best.1 - thetar.2 * 1.1, best.1 + thetar.2 * 1.1, thetar.2 / 4.0);
            phir = (best.2 - phir.2 * 1.1, best.2 + phir.2 * 1.1, phir.2 / 4.0);
            r = best.3;
        }

        println!("{}", r / r.x * 213.0);
        // for i in 1..100 {
            // println!("{:?}", r * (i as f64) / r.x);
        // }
        return;
    }

    // Direction is an integer multiple of this
    // let throw_dir = Vector3::new(-3, 1, 2);
    let throw_dir = if demo {
        Vector3::new(
            BigRational::from_i32(-3).unwrap(),
            BigRational::from_i32(1).unwrap(),
            BigRational::from_i32(2).unwrap()
        )
    } else {
        Vector3::new(
            BigRational::from_i32(213).unwrap(),
            BigRational::from_i32(-282).unwrap(),
            BigRational::from_i32(-52).unwrap()
        ) * BigRational::from_i32(-1).unwrap()
    };

    println!("{:?}", throw_dir);
    let ax2 = throw_dir.clone(); //.clone() / (throw_dir.x.pow(2) + throw_dir.y.pow(2) + throw_dir.z.pow(2)).pow(BigRational::new(1, 2));

    let max2sq = ax2.x.pow(2) + ax2.y.pow(2) + ax2.z.pow(2);
    let max2 = BigRational::from_f64(max2sq.to_f64().unwrap().sqrt()).unwrap();
    let ax2 = ax2 / max2.clone();

    let ax0 = ax2.cross(&Vector3::new(BigRational::from_i32(1).unwrap(), BigRational::zero(), BigRational::zero()));

    let max0sq = ax0.x.pow(2) + ax0.y.pow(2) + ax0.z.pow(2);
    let max0 = BigRational::from_f64(max0sq.to_f64().unwrap().sqrt()).unwrap();
    let ax0 = ax0 / max0;

    let ax1 = ax2.cross(&ax0);
    // XXX: normalise

    println!("{} {} ", ax0, ax0.dot(&ax2).to_f64().unwrap());
    println!("{} {} ", ax1, ax1.dot(&ax2).to_f64().unwrap());
    println!("{} {} ", ax2, ax0.dot(&ax1).to_f64().unwrap());
    // return;

    let input_reproj = input_bv.iter().map(|ray| {
        let pos = Vector3::new(
            ray.pos.dot(&ax0),
            ray.pos.dot(&ax1),
            ray.pos.dot(&ax2),
        );
        let vel = Vector3::new(
            ray.vel.dot(&ax0),
            ray.vel.dot(&ax1),
            ray.vel.dot(&ax2),
        );
        RayBigVec { pos, vel}
    }).collect_vec();

    // for (a0, a1) in input_reproj.iter().tuple_combinations() {
        // if let Some(c) = intersection(&a0, &a1) {
            // println!("{} {}", c.x.to_f64().unwrap(), c.y.to_f64().unwrap());
        // }
    // }
    let isct = intersection(&input_reproj[0], &input_reproj[1]).unwrap();

    // for (a0, a1) in input_reproj.iter().tuple_combinations() {
    //     if let Some((c, t0, t1)) = intersection(&a0, &a1) {
    //         println!("{} {}", c.x.to_f64().unwrap(), c.y.to_f64().unwrap());

    //         let d = (a1.pos.clone() + a1.vel.clone() * t1.clone()) - (a0.pos.clone() + a0.vel.clone() * t0.clone());
    //         let d2 = (d.x.pow(2) + d.y.pow(2) + d.z.pow(2));
    //         // println!("# {}", d2.to_f64().unwrap().sqrt() / (t1.clone() - t0.clone()).to_f64().unwrap() / max2.to_f64().unwrap());
    //     }
    // }

    for i in 1..input_reproj.len() {
        if let Some(isct) = intersection(&input_reproj[i], &input_reproj[0]) {
            let throw_pos = input_bv[i].pos.clone() + input_bv[i].vel.clone() * isct.1.clone() - throw_dir.clone() * isct.1.clone();
            println!("{}", throw_pos);
            println!("{}", throw_pos.x.clone() + throw_pos.y.clone() + throw_pos.z.clone());
        }
    }
        // let isct = intersection(&input_reproj[0], &input_reproj[1]).unwrap();

    // let throw_pos = input_bv[0].pos.clone() + input_bv[0].vel.clone() * isct.1.clone() - throw_dir.clone() * isct.1.clone();
    // println!("{}", throw_pos);
    // println!("{}", throw_pos.x.clone() + throw_pos.y.clone() + throw_pos.z.clone());

    // for a0 in input_reproj.iter() {
    //     let p0 = Vector2::new(a0.pos.x.clone(), a0.pos.y.clone());
    //     let v0 = Vector2::new(a0.vel.x.clone(), a0.vel.y.clone());
    //     let t2 = (isct - p0).norm_squared() / v0.norm_squared();
    // }

    // println!("{:?}", input_reproj);

    // For each line:
    // p0+v0*t = pt+vt*t
    // (p0-pt) = (vt-v0)*t

    // For any pair of lines:
    // p0+v0*t + d*vt = p1+v1*(t+vt)    where d = unknown time delta between iscts
    // (p0-p1) + (v0-v1)*t = (v1-n)*vt

    // pt+vt*t + n*vt = p1+v1*(t+vt)
    // (pt-p1) + vt*t + n*vt = p1+v1*(t+vt)

    // p0+v0*t = pt+vt*t
    //  t = (p0-pt)/(vt-v0)
    // p1+v1*(t+d) = pt+vt*(t+d)
    // p1+v1*t+v1*d = pt+vt*t+vt*d
    // (p1-pt) + t(v1-vt) + d(v1-vt) = 0
    // (p1-pt) + (p0-pt)/(vt-v0) (v1-vt) + d(v1-vt) = 0

    // Given t [time intersecting line 0]:
    //  Let C0=p0+v0*t, C1=p1+v1*t
    //  We want C0+vt*d = C1+v1*d
    //  => d = (C0-C1)/(v1-vt)  [time between both intersections]
    //


    // Or
    // Using the reprojected lines:
    // Sort by pZ - this must be the order we hit them in
    // DeltaZ / len(throwdir)
    // Find precise intersection point (reprojected)
    // When line 0 (reproj) crosses that point, that gives t0
    // When line 1 (reproj) crosses that point, that gives t1
    // Difference gives speed


    // Pick any permutation of 2 rocks
    // Construct a ray passing through both
    // See how many it hits

    // Convert every ray to (pos - throw.pos, vel - throw.vel)
    // See if they all pass through 0 at any t>0
    // or
    // Convert every ray to (pos, vel - throw.vel)
    // See if they all pass through any point P
    //  => find intersection of 1 pair, test others to confirm
    //
    // or
    //
    // r0p - tp + t0 * (r0v - tv) = 0
    // r1p - tp + t1 * (r1v - tv) = 0
    // ...
    // r0p + t0 * (r0v - tv) = r1p + t1 * (r1v - tv)

    //
    // Or
    // First find a line which all rays intersect
    // - is there only 1? then it's just timing

    // Test a direction:
    //  For each pair of lines:
    //    Project onto plane
    //    Record intersection
    //  Find mean intersection [excluding outliers]
    //  Optimise min dist between intersections

    // for (a, b) in input.iter().permutations(2) {

    // }

    // Rays intersect at  pa + |va| * (pb - pa).|va|

    // Rays intersect when
    // pa+va*t = pb+vb*t
    // t = (pb-pa)/(va-vb)
    // p = pa + va*(pb-pa)/(va-vb)

    // pa.x+va.x*t = pb.x+vb.x*t
    // pa.y+va.y*t = pb.y+vb.y*t

    // println!("{:?}", input);

return;

    let mut sum = 0;
    for (ra, rb) in input.iter().tuple_combinations() {
        println!("{:?} {:?}", ra, rb);

        let p1 = ra.pos.clone();
        let p2 = add(ra.pos.clone(), ra.vel.clone());
        let p3 = rb.pos.clone();
        let p4 = add(rb.pos.clone(), rb.vel.clone());

        // let dmnop = dot(sub(pm, pn), sub(po, pp));
        let d1343 = dot(sub(&p1, &p3), sub(&p4, &p3));
        let d4321 = dot(sub(&p4, &p3), sub(&p2, &p1));
        let d1321 = dot(sub(&p1, &p3), sub(&p2, &p1));
        let d4343 = dot(sub(&p4, &p3), sub(&p4, &p3));
        let d2121 = dot(sub(&p2, &p1), sub(&p2, &p1));
        if d2121.clone() * d4343.clone() - d4321.clone() * d4321.clone() == BigInt::from(0) {
            println!("Parallel");
            continue;
        }
        let mua = BigRational::new(d1343.clone() * d4321.clone() - d1321.clone() * d4343.clone(), d2121.clone() * d4343.clone() - d4321.clone() * d4321.clone());
        let mub = (BigRational::from(d1343.clone()) + mua.clone() * d4321.clone()) / d4343.clone();

        let ca = addf(&ra.pos, &mulf(&ra.vel, &mua));
        let cb = addf(&rb.pos, &mulf(&rb.vel, &mub));
        let delta = subff(ca.clone(), cb.clone());
        let dist2 = dotf(delta.clone(), delta.clone());
        // println!("  {} {} {}", mua, mub, dist2);

        // if mua >= BigRational::zero() && mub >= BigRational::zero() && dist2 == BigRational::zero() &&
        //     range.0.0 <= ca.0 && ca.0 <= range.0.1 &&
        //     range.1.0 <= ca.1 && ca.1 <= range.1.1 &&
        //     range.2.0 <= ca.2 && ca.2 <= range.2.1
        // {
        //     println!(" OK");
        //     sum += 1;
        // }

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
