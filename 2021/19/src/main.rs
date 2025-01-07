// Part 1: 69 mins
// Part 1+2: 74 mins

use rayon::prelude::*;

use std::collections::HashSet;

use itertools::Itertools;

fn transform(p: (i32, i32, i32), t: ((i32, i32, i32), usize)) -> (i32, i32, i32)
{
    let (origin, key) = t;

    let (x, y, z) = p;

    // let (x, y, z) = (x + origin.0, y + origin.1, z + origin.2);

    let mut p = vec![x, y, z];

    let (ix, key) = (key % 3, key / 3);
    let (iy, key) = (key % 2, key / 2);
    let (iz, key) = (key % 1, key / 1);
    let (nx, key) = (key % 2, key / 2);
    let (ny, key) = (key % 2, key / 2);
    let (nz, key) = (key % 2, key / 2);

    let x = p.remove(ix);
    let x = if nx == 1 { -x } else { x };
    let y = p.remove(iy);
    let y = if ny == 1 { -y } else { y };
    let z = p.remove(iz);
    let z = if nz == 1 { -z } else { z };

    (x + origin.0, y + origin.1, z + origin.2)
    // (x, y, z)
}

fn score(old: &Vec<(i32, i32, i32)>, new: &Vec<(i32, i32, i32)>, key: usize) -> (usize, (i32, i32, i32)) {
    // Try matching old[0] against every point in new

    let old_set: HashSet<(i32, i32, i32)> = HashSet::from_iter(old.iter().copied());
    // println!(" old {:?}", old_set);

    old.iter().map(|on| {
    new.iter().map(|n| {
        // let o = transform(old[0], ((0, 0, 0), key));
        // let origin = (o.0 - n.0, o.1 - n.1, o.2 - n.2);
        let o = transform(*n, ((0, 0, 0), key));
        let origin = (on.0 - o.0, on.1 - o.1, on.2 - o.2);

        let t = (origin, key);

        // println!("{:?} k {} -> {:?}; -{:?} = {:?}", old[0], key, o, n, origin);

        // rotate(n + origin) = old
        // rotate(n) = old - origin

        assert_eq!(transform(*n, t), *on);

        let count = new.iter().filter(|p| {
            // println!(" -- {:?} -> {:?}", **p, transform(**p, t));
            old_set.contains(&transform(**p, t))
        }).count();
        // println!(" # {}", count);

        (count, origin)
    }).max().unwrap()
    }).max().unwrap()
}

fn run(title: &str, input: &str) {
    let data: Vec<Vec<(i32, i32, i32)>> = input.split("\n\n").map(|s| {
        let (id, rest) = s.split_once("\n").unwrap();
        rest.lines().map(|l| {
            l.split(",").map(|n| n.parse().unwrap()).collect_tuple().unwrap()
        }).collect()
    }).collect();

    // let mut offsets = vec![((0, 0, 0), 0)];
    let mut old: Vec<(i32, i32, i32)> = data[0].clone();

    let mut offsets: Vec<Option<(i32, i32, i32)>> = vec![None; data.len()];
    offsets[0] = Some((0, 0, 0));

    // let mut pairs = vec![];

    while offsets.iter().any(|o| o.is_none()) {
        for i in 0..data.len() {
            if offsets[i].is_some() {
                continue;
            }

            let best = (0..48).into_par_iter().map(|key| {
            // let best = (0..1).map(|key| {
                // (score(&data[j], &data[i], key), key)
                // println!("{:?} {}", score(&data[j], &data[i], key), key);
                (score(&old, &data[i], key), key)
            }).max().unwrap();
            let ((score, origin), key) = best;
            if score >= 12 {
                // println!("{} {} best {:?}", j, i, best);
                // pairs.push((j, i, origin, key));

                offsets[i] = Some((origin));
                old.append(
                    &mut data[i].iter().map(|p| transform(*p, (origin, key))).collect_vec()
                );
                old.sort();
                old.dedup();
                println!("Points: {}", old.len());
            }
        }
    }

    println!("{} part 1: {}", title, old.len());

    println!("{} part 2: {}", title, offsets.iter().tuple_combinations().map(|(a, b)| {
        let a = a.unwrap();
        let b = b.unwrap();
        (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
    }).max().unwrap());
}

const INPUT_DEMO0: &str = "--- scanner 0 ---
100,200,300
400,600,800

--- scanner 1 ---
1100,1200,1300
1400,1600,1800
";

const INPUT_DEMO: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";

fn main() {
    // run("demo", INPUT_DEMO0);
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("19/input.txt").unwrap());
}
