// Part 1: 60 mins
// Part 1+2: 165 mins

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn numpad_paths2(from: char, to: char) -> HashSet<String> {
    HashSet::from_iter(numpad_paths(from, to).iter().map(|s| s.iter().collect::<String>()))
}

fn dir_paths2(from: char, to: char) -> HashSet<String> {
    HashSet::from_iter(dir_paths(from, to).iter().map(|s| s.iter().collect::<String>()))
}

fn numpad_paths(from: char, to: char) -> HashSet<Vec<char>> {
    let numpad: HashMap<char, (i32, i32)> = HashMap::from([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ]);

    let dir = HashMap::from([
        ((-1, 0), '<'),
        ((1, 0), '>'),
        ((0, -1), '^'),
        ((0, 1), 'v'),
    ]);

    let mut ret = HashSet::new();

    let mut open = vec![(from, vec![])];
    while let Some((cur, path)) = open.pop() {

        if cur == to {
            ret.insert(path);
            continue;
        }

        for other in numpad.keys() {
            if numpad[&to].0.abs_diff(numpad[other].0) < numpad[&to].0.abs_diff(numpad[&cur].0) ||
                numpad[&to].1.abs_diff(numpad[other].1) < numpad[&to].1.abs_diff(numpad[&cur].1) {
                if let Some(d) = dir.get(&(numpad[other].0 - numpad[&cur].0, numpad[other].1 - numpad[&cur].1))
                {
                    let mut p2 = path.clone();
                    p2.push(*d);
                    open.push((*other, p2));
                }
            }
        }

    }


    // let mut dx = numpad[&to].0 - numpad[&from].0;
    // let mut dy = numpad[&to].1 - numpad[&from].1;

    // let mut r = Vec::new();
    // while dy < 0 {
    //     r.push('^');
    //     dy += 1;
    // }
    // while dy > 0 {
    //     r.push('v');
    //     dy -= 1;
    // }
    // while dx < 0 {
    //     r.push('<');
    //     dx += 1;
    // }
    // while dx > 0 {
    //     r.push('>');
    //     dx -= 1;
    // }

    // let mut ret = HashSet::new();

    // for p in r.iter().copied().permutations(r.len()) {
    //     ret.insert(p);
    // }

    ret
}

fn dir_paths0(from: char, to: char) -> HashSet<Vec<char>> {
    let dirpad = HashMap::from([
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);

    let mut ret = HashSet::new();

    {
        let mut dx = dirpad[&to].0 - dirpad[&from].0;
        let mut dy = dirpad[&to].1 - dirpad[&from].1;

        let mut r = Vec::new();
        while dy < 0 {
            r.push('^');
            dy += 1;
        }
        while dy > 0 {
            r.push('v');
            dy -= 1;
        }
        while dx < 0 {
            r.push('<');
            dx += 1;
        }
        while dx > 0 {
            r.push('>');
            dx -= 1;
        }
        ret.insert(r);
    }

    if dirpad[&from].1 != 3 && dirpad[&to].1 != 1 {
        let mut dx = dirpad[&to].0 - dirpad[&from].0;
        let mut dy = dirpad[&to].1 - dirpad[&from].1;

        let mut r = Vec::new();
        while dx < 0 {
            r.push('<');
            dx += 1;
        }
        while dx > 0 {
            r.push('>');
            dx -= 1;
        }
        while dy < 0 {
            r.push('^');
            dy += 1;
        }
        while dy > 0 {
            r.push('v');
            dy -= 1;
        }
        ret.insert(r);
    }

    ret
}

fn dir_paths(from: char, to: char) -> HashSet<Vec<char>> {
    let numpad: HashMap<char, (i32, i32)> = HashMap::from([
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);

    let dir = HashMap::from([
        ((-1, 0), '<'),
        ((1, 0), '>'),
        ((0, -1), '^'),
        ((0, 1), 'v'),
    ]);

    let mut ret = HashSet::new();

    let mut open = vec![(from, vec![])];
    while let Some((cur, path)) = open.pop() {

        if cur == to {
            ret.insert(path);
            continue;
        }

        for other in numpad.keys() {
            if numpad[&to].0.abs_diff(numpad[other].0) < numpad[&to].0.abs_diff(numpad[&cur].0) ||
                numpad[&to].1.abs_diff(numpad[other].1) < numpad[&to].1.abs_diff(numpad[&cur].1) {
                if let Some(d) = dir.get(&(numpad[other].0 - numpad[&cur].0, numpad[other].1 - numpad[&cur].1))
                {
                    let mut p2 = path.clone();
                    p2.push(*d);
                    open.push((*other, p2));
                }
            }
        }

    }

    ret
}

fn dir_path(from: char, to: char) -> Vec<char> {
    let dirpad = HashMap::from([
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);

    let mut dx = dirpad[&to].0 - dirpad[&from].0;
    let mut dy = dirpad[&to].1 - dirpad[&from].1;

    let mut r = Vec::new();
    while dy < 0 {
        r.push('^');
        dy += 1;
    }
    while dy > 0 {
        r.push('v');
        dy -= 1;
    }
    while dx < 0 {
        r.push('<');
        dx += 1;
    }
    while dx > 0 {
        r.push('>');
        dx -= 1;
    }
    r
}

/*



*/

fn expand(paths: &[HashSet<Vec<char>>], pos: char, v: Vec<char>) -> HashSet<Vec<char>> {
    if paths.is_empty() {
        // println!("- {}", v.iter().collect::<String>());
        return HashSet::from([v]);
    }

    let mut r = HashSet::new();
    for p in &paths[0] {
        let mut v2 = v.clone();
        v2.append(&mut p.clone());
        r = HashSet::from_iter(r.union(&expand(&paths[1..], pos, v2)).cloned());
    }

    r
}

fn expand2(paths: &[HashSet<String>], pos: char, v: String) -> HashSet<String> {
    if paths.is_empty() {
        // println!("- {}", v.iter().collect::<String>());
        return HashSet::from([v]);
    }

    let mut r = HashSet::new();
    for p in &paths[0] {
        let mut v2 = v.clone();
        v2.push_str(&p);
        r = HashSet::from_iter(r.union(&expand2(&paths[1..], pos, v2)).cloned());
    }

    r
}

fn run(title: &str, input: &str) {

    let mut part1 = 0;

    for line in input.lines() {

        println!("# {}", line);

        let mut fin = Vec::new();

        let mut paths0 = Vec::new();

        let mut pos0 = 'A';
        for c in line.chars() {
            paths0.push(numpad_paths(pos0, c));
            paths0.push(HashSet::from([vec!['A']]));
            pos0 = c;
        }
        println!("{:?}", paths0);

        let paths1 = expand(&paths0, 'A', vec![]);
        for p1 in paths1 {
            // println!("  {}", p1.iter().collect::<String>());

            let mut pos1 = 'A';
            let mut paths2 = Vec::new();
            for c in p1 {
                paths2.push(dir_paths(pos1, c));
                paths2.push(HashSet::from([vec!['A']]));
                pos1 = c;
            }

            let paths3 = expand(&paths2, 'A', vec![]);
            for p3 in paths3 {
                // println!("    {}", p3.iter().collect::<String>());

                let mut pos2 = 'A';
                let mut paths4 = Vec::new();
                for c in p3 {
                    paths4.push(dir_paths(pos2, c));
                    paths4.push(HashSet::from([vec!['A']]));
                    pos2 = c;
                }

                let paths5 = expand(&paths4, 'A', vec![]);
                for p5 in paths5 {
                    // println!("      {}", p5.iter().collect::<String>());
                    fin.push(p5.iter().collect::<String>());
                }
            }
        }

        let min = fin.iter().map(|n| n.len()).min().unwrap();
        let max = fin.iter().map(|n| n.len()).max().unwrap();
        println!("{}", min);
        part1 += min * usize::from_str_radix(&line[0..3], 10).unwrap();
        // assert_eq!(min, max);

        println!("====");
    }

    println!("{} part 1: {}", title, part1);
}

fn seg_cost(p: &Vec<char>, reps: usize, cache: &mut HashMap<(Vec<char>, usize), usize>) -> usize {
    if reps == 0 {
        return p.len();
    }

    if let Some(r) = cache.get(&(p.clone(), reps)) {
        return *r;
    }

    let dirmap = HashMap::from([
        (('<', '<'), "A"),
        (('<', '>'), ">>A"),
        (('<', '^'), "^>A"),
        (('<', 'v'), ">A"),
        (('<', 'A'), "^>>A"),
        (('>', '<'), "<<A"),
        (('>', '>'), "A"),
        (('>', '^'), "^<A"),
        (('>', 'v'), "<A"),
        (('>', 'A'), "^A"),
        (('^', '<'), "v<A"),
        (('^', '>'), "v>A"),
        (('^', '^'), "A"),
        (('^', 'v'), "vA"),
        (('^', 'A'), ">A"),
        (('v', '<'), "<A"),
        (('v', '>'), ">A"),
        (('v', '^'), "^A"),
        (('v', 'v'), "A"),
        (('v', 'A'), "^>A"),
        (('A', '<'), "v<<A"),
        (('A', '>'), "vA"),
        (('A', '^'), "<A"),
        (('A', 'v'), "v<A"),
        (('A', 'A'), "A"),
    ]);

    let mut cost = 0;
    let mut pos1 = 'A';
    for &c in p {
        // paths2.append(&mut dir_path(pos1, *c));
        // paths2.push('A');
        cost += seg_cost(&dirmap[&(pos1, c)].chars().collect_vec(), reps - 1, cache);
        pos1 = c;
    }

    cache.insert((p.clone(), reps), cost);

    cost
}

fn seg_cost2(p: &String, reps: usize, cache: &mut HashMap<(String, usize), usize>) -> usize {
    assert_eq!(p.chars().last(), Some('A'));

    if reps == 0 {
        return p.len();
    }

    if let Some(r) = cache.get(&(p.clone(), reps)) {
        return *r;
    }

    let mut cost = 0;
    let mut pos1 = 'A';

    for c in p.chars() {
        cost += dir_paths2(pos1, c).iter().map(|s|
            seg_cost2(&(s.clone() + "A"), reps - 1, cache)
        ).min().unwrap();
        pos1 = c;
    }

    cache.insert((p.clone(), reps), cost);

    cost
}


// At level 0:
//   Input is button seq (iterate over ~10 options for numpad)

// At level N:
//   Bot 0 goes [A->a] A [a->b] A [b->c] A ...
//   where [...] is a sequence of 0-3 buttons (see dirmap)

/*

<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
  v <<   A >>  ^ A   <   A > A  v  A   <  ^ AA > A   < v  AAA >  ^ A
         <       A       ^   A     >        ^^   A        vvv      A
                 0           2                   9                 A

So each segment goes [A]xyzA -> [A]mnopqrstA

*/

// Len N+1 =

// At the last level:
//   Input is button seq
//   For each button,


fn run2(title: &str, input: &str) {

    // for a in "<>^vA".chars() {
    //     for b in "<>^vA".chars() {
    //         println!("  (('{}', '{}'), \"{}A\"),", a, b, dir_path(a, b).iter().collect::<String>());
    //     }
    // }
    for a in "<>^vA".chars() {
        for b in "<>^vA".chars() {
            println!("  (('{}', '{}'), {:?}),", a, b, dir_paths(a, b));
        }
    }
    // for a in "<>^vA".chars() {
    //     println!("  ('{}', \"{}{}\"),", a, dir_path('A', a).iter().collect::<String>(), dir_path(a, 'A').iter().collect::<String>());
    // }

    let mut part2 = 0;

    let dirmap = HashMap::from([
        (('<', '<'), "A"),
        (('<', '>'), ">>A"),
        (('<', '^'), "^>A"),
        (('<', 'v'), ">A"),
        (('<', 'A'), "^>>A"),
        (('>', '<'), "<<A"),
        (('>', '>'), "A"),
        (('>', '^'), "^<A"),
        (('>', 'v'), "<A"),
        (('>', 'A'), "^A"),
        (('^', '<'), "v<A"),
        (('^', '>'), "v>A"),
        (('^', '^'), "A"),
        (('^', 'v'), "vA"),
        (('^', 'A'), ">A"),
        (('v', '<'), "<A"),
        (('v', '>'), ">A"),
        (('v', '^'), "^A"),
        (('v', 'v'), "A"),
        (('v', 'A'), "^>A"),
        (('A', '<'), "v<<A"),
        (('A', '>'), "vA"),
        (('A', '^'), "<A"),
        (('A', 'v'), "v<A"),
        (('A', 'A'), "A"),
    ]);

/*
  (('<', '<'), [""]),
  (('<', '>'), [">>"]),
  (('<', '^'), [">^"]),
  (('<', 'v'), [">"]),
  (('<', 'A'), [">>^", ">^>"]),

  (('>', '<'), ["<<"]),
  (('>', '>'), [""]),
  (('>', '^'), ["^<", "<^"]),
  (('>', 'v'), ["<"]),
  (('>', 'A'), ["^"]),

  (('^', '<'), ["v<"]),
  (('^', '>'), ["v>", ">v"]),
  (('^', '^'), [""]),
  (('^', 'v'), ["v"]),
  (('^', 'A'), [">"]),

  (('v', '<'), ["<"]),
  (('v', '>'), [">"]),
  (('v', '^'), ["^"]),
  (('v', 'v'), [""]),
  (('v', 'A'), ["^>", ">^"]),

  (('A', '<'), ["v<<", "<v<"]),
  (('A', '>'), ["v"]),
  (('A', '^'), ["<"]),
  (('A', 'v'), ["<v", "v<"]),
  (('A', 'A'), [""]),
  */

    for line in input.lines() {

        println!("# {}", line);

        let mut paths0 = Vec::new();

        let mut pos0 = 'A';
        for c in line.chars() {
            paths0.push(numpad_paths(pos0, c));
            paths0.push(HashSet::from([vec!['A']]));
            pos0 = c;
        }
        println!("{:?}", paths0);

        let paths1 = expand(&paths0, 'A', vec![]);
        println!("{:?}", paths1.iter().map(|s| s.iter().collect::<String>()).collect_vec());

        continue;

        let mut cache = HashMap::new();

        let mut best_cost = usize::MAX;

        let total_reps = 2;
        let manual_reps = 2;

        let mut pathsn = paths1;
        for i in 0..manual_reps {
            println!("{} {} {}", i, pathsn.len(), pathsn.iter().next().unwrap().len());

            if i > 1 {
                pathsn = HashSet::from([pathsn.iter().sorted_by_key(|p| p.len()).next().unwrap().clone()]);
            }

            pathsn = HashSet::from_iter(pathsn.iter().map(|p| {

                // // println!("expand {:?}", p.iter().collect::<String>());
                // let mut pos1 = 'A';
                // let mut paths2 = Vec::new();
                // for c in p {
                //     // paths2.append(&mut dir_path(pos1, *c));
                //     // paths2.push('A');
                //     paths2.append(&mut dirmap[&(pos1, *c)].chars().collect_vec());
                //     pos1 = *c;
                // }
                // paths2

                let mut pos1 = 'A';
                let mut paths2 = Vec::new();
                for &c in p {
                    paths2.push(dir_paths(pos1, c));
                    paths2.push(HashSet::from([vec!['A']]));
                    pos1 = c;
                }
                expand(&paths2, 'A', vec![])

            }
            ).concat());
        }

        for p in pathsn {
            let mut segs = Vec::new();
            let mut pos1 = 'A';
            for c in p {
                // paths2.append(&mut dir_path(pos1, *c));
                // paths2.push('A');
                segs.push(dirmap[&(pos1, c)].chars().collect_vec());
                pos1 = c;
            }
            let cost = segs.iter().map(|seg| seg_cost(seg, total_reps - manual_reps - 1, &mut cache)).sum();
            // println!("segs {:?}", segs);
            // println!("  cost {}", cost);
            best_cost = best_cost.min(cost);
        }
        //     }

        // let min = pathsn.iter().map(|n| n.len()).min().unwrap();
        // println!("{}", min);
        part2 += best_cost * usize::from_str_radix(&line[0..3], 10).unwrap();
        // assert_eq!(min, max);

        println!("====");
    }

    // 250219886362000 too high
    // 249461848149792 too high
    // 100012495087834 too low
    println!("{} part 2: {}", title, part2);
}


fn run3(title: &str, input: &str) {

    // for a in "<>^vA".chars() {
    //     for b in "<>^vA".chars() {
    //         println!("  (('{}', '{}'), \"{}A\"),", a, b, dir_path(a, b).iter().collect::<String>());
    //     }
    // }
    for a in "<>^vA".chars() {
        for b in "<>^vA".chars() {
            println!("  (('{}', '{}'), {:?}),", a, b, dir_paths(a, b).iter().map(|n| n.iter().collect::<String>()).collect_vec());
        }
    }
    // for a in "<>^vA".chars() {
    //     println!("  ('{}', \"{}{}\"),", a, dir_path('A', a).iter().collect::<String>(), dir_path(a, 'A').iter().collect::<String>());
    // }

    let mut part2 = 0;

    for line in input.lines() {

        println!("# {}", line);

        let mut paths0 = Vec::new();

        let mut pos0 = 'A';
        for c in line.chars() {
            paths0.push(numpad_paths2(pos0, c));
            paths0.push(HashSet::from(["A".to_owned()]));
            pos0 = c;
        }
        println!("{:?}", paths0);

        let paths1 = expand2(&paths0, 'A', "".to_owned());

        let mut cache = HashMap::new();
        let mut best_cost = usize::MAX;
        for p1 in paths1 {
            println!("p1: {}", p1);
            for i in 0..25 {
                println!("{} {}", i, seg_cost2(&p1, i, &mut cache));
            }
            best_cost = best_cost.min(seg_cost2(&p1, 25, &mut cache));
        }

        part2 += best_cost * usize::from_str_radix(&line[0..3], 10).unwrap();

        // let mut pathsn = paths1;

        // for i in 0..1 {
        //     println!("{:?}", pathsn);

        //     let mut out = HashSet::new();
        //     for p1 in pathsn {
        //         println!("p{}: {}", i, p1);

        //         let mut pos1 = 'A';
        //         let mut paths2 = Vec::new();
        //         for c in p1.chars() {
        //             paths2.push(dir_paths2(pos1, c));
        //             paths2.push(HashSet::from(["A".to_owned()]));
        //             pos1 = c;
        //         }
        //         println!("{}--> {:?}", i, paths2);
        //         let p2 = expand2(&paths2, 'A', "".to_owned());

        //         println!("{}--> {:#?}", i, p2);
        //         for p in p2 {
        //             out.insert(p);
        //         }
        //     }

        //     pathsn = out;
        // }

        /*
        let paths1 = expand(&paths0, 'A', vec![]);
        println!("{:?}", paths1.iter().map(|s| s.iter().collect::<String>()).collect_vec());

        continue;

        let mut cache = HashMap::new();

        let mut best_cost = usize::MAX;

        let total_reps = 2;
        let manual_reps = 2;

        let mut pathsn = paths1;
        for i in 0..manual_reps {
            println!("{} {} {}", i, pathsn.len(), pathsn.iter().next().unwrap().len());

            if i > 1 {
                pathsn = HashSet::from([pathsn.iter().sorted_by_key(|p| p.len()).next().unwrap().clone()]);
            }

            pathsn = HashSet::from_iter(pathsn.iter().map(|p| {

                // // println!("expand {:?}", p.iter().collect::<String>());
                // let mut pos1 = 'A';
                // let mut paths2 = Vec::new();
                // for c in p {
                //     // paths2.append(&mut dir_path(pos1, *c));
                //     // paths2.push('A');
                //     paths2.append(&mut dirmap[&(pos1, *c)].chars().collect_vec());
                //     pos1 = *c;
                // }
                // paths2

                let mut pos1 = 'A';
                let mut paths2 = Vec::new();
                for &c in p {
                    paths2.push(dir_paths(pos1, c));
                    paths2.push(HashSet::from([vec!['A']]));
                    pos1 = c;
                }
                expand(&paths2, 'A', vec![])

            }
            ).concat());
        }

        for p in pathsn {
            let mut segs = Vec::new();
            let mut pos1 = 'A';
            for c in p {
                // paths2.append(&mut dir_path(pos1, *c));
                // paths2.push('A');
                segs.push(dirmap[&(pos1, c)].chars().collect_vec());
                pos1 = c;
            }
            let cost = segs.iter().map(|seg| seg_cost(seg, total_reps - manual_reps - 1, &mut cache)).sum();
            // println!("segs {:?}", segs);
            // println!("  cost {}", cost);
            best_cost = best_cost.min(cost);
        }
        //     }

        // let min = pathsn.iter().map(|n| n.len()).min().unwrap();
        // println!("{}", min);
        part2 += best_cost * usize::from_str_radix(&line[0..3], 10).unwrap();
        // assert_eq!(min, max);

        println!("====");
        */

        // break;
    }

    // 250219886362000 too high
    // 249461848149792 too high
    // 100012495087834 too low
    println!("{} part 2: {}", title, part2);
}


const INPUT_DEMO: &str = "029A
980A
179A
456A
379A
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("21/input.txt").unwrap());
    // run2("demo", INPUT_DEMO);
    run3("input", &std::fs::read_to_string("21/input.txt").unwrap());
    // run("demo", "459A");
}
