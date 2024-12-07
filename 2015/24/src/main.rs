use itertools::Itertools;

fn permutations(mut path: Vec<i32>, rest: Vec<i32>, weight: i32, smallest: &mut i32) -> Vec<Vec<i32>> {
    // println!("- {:?} {:?} {}", path, rest, weight);

    if weight == 0 {
        // println!("> {:?}", path);
        // path.sort();
        *smallest = (*smallest).min(path.len() as i32);
        return vec![path];
    }

    if rest.is_empty()
    // || path.len() as i32 > *smallest
    {
        return vec![];
    }

    let mut ret = Vec::new();
    for i in 0..rest.len() {
        if rest[i] <= weight {
            let mut newpath = path.clone();
            newpath.push(rest[i]);
            let mut newrest = rest[i+1..].to_owned();
            ret.append(&mut permutations(newpath, newrest, weight - rest[i], smallest));
        }
    }
    ret.sort();
    ret.dedup();
    ret
}

fn run(title: &str, input: &str) {
    let data: Vec<i32> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let total: i32 = data.iter().sum();
    let target = total / 4; /* part1: 3, part2: 4 */

    println!("total: {}, target: {}", total, target);

    let mut smallest = i32::MAX;
    let mut groups = permutations(vec![], data.clone(), target, &mut smallest);
    println!("{} {}", groups.len(), groups.iter().filter(|g| g.len() as i32 == smallest).count());

    groups.sort_by_key(|g| (g.len(), g.iter().copied().map_into::<i64>().product::<i64>()));
    for g1 in groups {
        println!("{:?}", g1);

        let rest = data.iter().copied().filter(|i| !g1.contains(i)).collect_vec();
        let mut smallest = i32::MAX;
        let p = permutations(vec![], rest, target, &mut smallest);
        println!("  {}", p.len());
        if p.len() > 0 {
            println!("{} part N: {}", title, g1.iter().copied().map_into::<i64>().product::<i64>());
            break;
        }
    }
}

const INPUT_DEMO: &str = "1
2
3
4
5
7
8
9
10
11";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("24/input.txt").unwrap());
}
