use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn flood(grid: &HashMap<(i32, i32), String>, x0: i32, y0: i32) -> Vec<(String, i32)> {
    let mut seen = HashSet::new();
    let mut open = VecDeque::new();
    open.push_back((x0, y0, 0));
    seen.insert((x0, y0));

    let mut ret = Vec::new();

    while let Some((x, y, d)) = open.pop_front() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if seen.insert((x + dx, y + dy)) {
                let dst = grid.get(&(x + dx, y + dy)).unwrap();
                if dst.len() == 3 {
                    ret.push((dst.clone(), d + 1));
                } else if dst == "." {
                    open.push_back((x + dx, y + dy, d + 1));
                }
            }
        }
    }

    ret
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct State {
    pos: String,
    level: i32,

    steps: i32,
    path: Vec<(String, i32, i32)>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(self.pos.cmp(&other.pos))
            .then(self.level.cmp(&other.level))
    }
}

fn alternate(label: &String, level: i32, part2: bool) -> (String, i32) {
    let b = label.as_bytes();
    match b[2] {
        b'i' => (String::from_utf8(b[0..2].to_vec()).unwrap() + "o", if part2 { level + 1 } else { level }),
        b'o' => (String::from_utf8(b[0..2].to_vec()).unwrap() + "i", if part2 { level - 1 } else { level }),
        _ => panic!(),
    }
}

fn run(title: &str, input: &str, part2: bool) {
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut portals = HashMap::new();

    println!("{:?}", data);

    let letters = 'A'..='Z';

    for y in 0..data.len() {
        for x in 0..data[y].len()-1 {
            let a = data[y][x];
            let b = data[y][x+1];
            let label = format!("{}{}", a, b);
            if letters.contains(&a) && letters.contains(&b) {
                if x > 0 && data[y][x-1] == '.' {
                    portals.entry(label.clone() + if x < data[y].len()/2 { "i" } else { "o" } ).or_insert_with(|| Vec::new()).push((x as i32 - 1, y as i32));
                }
                if x+2 < data[y].len() && data[y][x+2] == '.' {
                    portals.entry(label.clone() + if x < data[y].len()/2 { "o" } else { "i" } ).or_insert_with(|| Vec::new()).push((x as i32 + 2, y as i32));
                }
            }
        }
    }

    for y in 0..data.len()-1 {
        for x in 0..data[y].len() {
            let a = data[y][x];
            let b = data[y+1].get(x).unwrap_or(&' ');
            let label = format!("{}{}", a, b);
            // if x == 0 { println!("{} {} {}", x, y, label); }
            if letters.contains(&a) && letters.contains(&b) {
                // println!("{} {} {}", x, y, label);
                if y > 0 && data[y-1][x] == '.' {
                    portals.entry(label.clone() + if y < data.len()/2 { "i" } else { "o" } ).or_insert_with(|| Vec::new()).push((x as i32, y as i32 - 1));
                }
                if y+2 < data.len() && data[y+2][x] == '.' {
                    portals.entry(label.clone() + if y < data.len()/2 { "o" } else { "i" } ).or_insert_with(|| Vec::new()).push((x as i32, y as i32 + 2));
                }
            }
        }
    }

    let mut grid = HashMap::new();

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            let ps: Vec<_> = portals.iter().filter(|(k,v)| v.iter().any(|p| *p == (x as i32, y as i32))).collect();
            if !ps.is_empty() {
                print!("{}", ps.first().unwrap().0.chars().nth(2).unwrap());
                grid.insert((x as i32, y as i32), ps.first().unwrap().0.clone());
            } else {
                print!("{}", data[y][x]);
                grid.insert((x as i32, y as i32), data[y][x].to_string());
            }
        }
        println!();
    }

    // println!("{:?}", grid);
    println!("{:?}", portals);

    let mut edges = HashMap::new();

    for (name,ps) in &portals {
        for p in ps {
            for (dst, d) in flood(&grid, p.0, p.1) {
                edges
                .entry(name)
                .or_insert_with(|| HashSet::new())
                .insert((dst, d));
            }
        }
    }

    println!("Edges: {:?}", edges);

    let mut added = HashMap::new();
    let mut open = BinaryHeap::new();
    open.push(State {
        pos: "AAo".to_owned(),
        level: 0,
        steps: 0,
        path: Vec::new(),
    });

    while let Some(state) = open.pop() {
        // println!("# {:?}", state);
        if state.pos == "ZZi" {
            if state.level == -1 {
                println!("{} part N: {}", title, state.steps - 1);
                break;
            } else {
                // println!("NOT {} part N: {} -- {:?}", title, state.steps - 1, state);
                continue;
            }
        }

        if state.level < 0 {
            continue;
        }

        if state.pos == "AAo" && state.level != 0 {
            continue;
        }

        for (dst, d) in edges.get(&state.pos).unwrap_or(&HashSet::new()) {

            let alt = alternate(&dst, state.level, part2);

            let mut new_state = State {
                pos: alt.0.clone(),
                level: alt.1,
                steps: state.steps + d + 1,
                path: state.path.clone(),
            };

            new_state.path.push((dst.clone(), state.level, *d));
            new_state.path.push((new_state.pos.clone(), new_state.level, 1));

            // println!("EXPLORE {:?} -> {:?} {:?}", state, dst, new_state);

            match added.get(&(new_state.pos.clone(), new_state.level)) {
                Some(n) if *n <= new_state.steps => {
                    // prune
                }
                _ => {
                    added.insert((new_state.pos.clone(), new_state.level), new_state.steps);
                    open.push(new_state);
                }
            }
        }
    }
}

const INPUT_DEMO: &str = "         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z
";

const INPUT_DEMO2: &str = "                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P
";

const INPUT_DEMO3: &str = "             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M
";


fn main() {
    // run("demo", INPUT_DEMO);
    // run("demo 2", INPUT_DEMO2, false);
    // run("input", &std::fs::read_to_string("20/input.txt").unwrap(), false);

    // run("demo", INPUT_DEMO, true);
    // run("demo 2", INPUT_DEMO2, true);
    // run("demo 3", INPUT_DEMO3, true);
    run("input", &std::fs::read_to_string("20/input.txt").unwrap(), true);
}
