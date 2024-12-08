use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Disk {
    size: u16,
    used: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    goal: (i32, i32),
    disks: Vec<Disk>,
}

fn print(state: &State) {
    let w = 38;
    let h = 26;

    println!("---");
    for y in 0..h {
        for x in 0..w {
            let disk = state.disks[x + y*w];
            assert!(disk.used <= disk.size);
            if state.goal == (x as i32, y as i32) {
                print!("G");
            } else if disk.used == 0 {
                print!("_");
            } else if disk.size > 500 && disk.size - disk.used < 50 {
                print!("#");
            } else if disk.used > 50 {
                print!(".");
            } else {
                print!("?");
            }
        }
        println!();
    }
}

fn mv(state: &State, src: (i32, i32), dst: (i32, i32)) -> State {
    let w = 38;
    let h = 26;

    assert!((src.0 - dst.0).abs() + (src.1 - dst.1).abs() == 1);

    let srcd = state.disks[(src.0 + src.1 * w) as usize];
    let dstd = state.disks[(dst.0 + dst.1 * w) as usize];
    assert!(srcd.used > 0);
    assert!(dstd.size - dstd.used >= srcd.used);

    let mut new = state.clone();
    new.disks[(src.0 + src.1 * w) as usize].used = 0;
    new.disks[(dst.0 + dst.1 * w) as usize].used += srcd.used;
    if src == state.goal {
        new.goal = dst;
    }

    new
}

fn run(title: &str, input: &str) {
    let data: HashMap<(i32, i32), Disk> = input
        .lines()
        .skip(2)
        .map(|line| {
            let ts = line.split_whitespace().collect_vec();
            let ns = ts[0].split("-").collect_vec();
            let x = ns[1].strip_prefix("x").unwrap().parse().unwrap();
            let y = ns[2].strip_prefix("y").unwrap().parse().unwrap();
            let mut size = ts[1].strip_suffix("T").unwrap().parse().unwrap();
            let mut used = ts[2].strip_suffix("T").unwrap().parse().unwrap();

            // if size > 500 {
            //     size = 500;
            //     used = 500;
            // } else if used > 0 {
            //     size = 100;
            //     used = 60;
            // } else {
            //     size = 100;
            //     used = 0;
            // }

            ((x, y), Disk { size, used })
        })
        .collect();

    // println!("{:?}", data);

    let viable = data.values().permutations(2).filter(|disks| {
        disks[0].used != 0 && (disks[1].size - disks[1].used) >= disks[0].used
    }).count();

    println!("{} part 1: {}", title, viable);

    let (maxx, maxy) = data.keys().max().unwrap();
    let (w, h) = (maxx + 1, maxy + 1);

    println!("{} {}", w, h);

    // for y in 0..h {
    //     for x in 0..w {
    //         let disk = data[&(x, y)];
    //         if disk.used == 0 {
    //             print!("_");
    //         } else if disk.size > 500 && disk.size - disk.used < 50 {
    //             print!("#");
    //         } else if disk.used > 50 {
    //             print!(".");
    //         } else {
    //             print!("?");
    //         }
    //     }
    //     println!();
    // }

    let mut open = VecDeque::new();
    let initial = State { goal: (w - 1, 0), disks: (0..h).map(|y| (0..w).map(|x| data[&(x, y)]).collect()).concat() };

    let mut state = initial;
    print(&state);
    let mut steps = 0;
    for i in 0..11 {
        state = mv(&state, (15-i, 23), (15-i+1, 23));
        steps += 1;
    }
    print(&state);
    for i in 0..22 {
        state = mv(&state, (5, 23-i-1), (5, 23-i));
        steps += 1;
    }
    print(&state);
    for i in 0..31 {
        state = mv(&state, (5+i+1, 1), (5+i, 1));
        steps += 1;
    }
    print(&state);
    state = mv(&state, (36, 0), (36, 1));
    state = mv(&state, (37, 0), (36, 0));
    steps += 2;
    for i in 0..36 {
        let gx = state.goal.0;
        state = mv(&state, (gx+1, 1), (gx+1, 0));
        state = mv(&state, (gx, 1), (gx+1, 1));
        state = mv(&state, (gx-1, 1), (gx, 1));
        state = mv(&state, (gx-1, 0), (gx-1, 1));
        state = mv(&state, (gx, 0), (gx-1, 0));
        steps += 5;
    }
    print(&state);

    println!("{}", steps);

    return;



    open.push_back((initial, 0));

    let mut closed = HashSet::new();

    while let Some((node, steps)) = open.pop_front() {
        if node.goal == (0, 0) {
            println!("{} part 2: {}", title, steps);
            break;
        }

        // if steps < 2 {
        //     println!("{}", steps);
        //     for y in 0..h {
        //         for x in 0..w {
        //             let disk = data[&(x, y)];
        //             if disk.used == 0 {
        //                 print!("_");
        //             } else if disk.size > 500 && disk.size - disk.used < 50 {
        //                 print!("#");
        //             } else if disk.used > 50 {
        //                 print!(".");
        //             } else {
        //                 print!("?");
        //             }
        //         }
        //         println!();
        //     }
        // } else {
        //     break;
        // }

        if open.len() % 10_000 == 0 || closed.len() % 10_000 == 0 {
            println!("{} {} {}...", steps, open.len(), closed.len());
        }

        // if steps == 1 && node.disks[(1 + 0*w) as usize].used != 0 {
        //     continue;
        // }
        // if steps == 2 && node.goal != (1, 0) {
        //     continue;
        // }
        // if steps == 3 && node.disks[(2 + 1*w) as usize].used != 0 {
        //     continue;
        // }
        // if steps == 4 && node.disks[(1 + 1*w) as usize].used != 0 {
        //     continue;
        // }
        // if steps == 5 && node.disks[(0 + 1*w) as usize].used != 0 {
        //     continue;
        // }
        // if steps == 6 && node.disks[(0 + 0*w) as usize].used != 0 {
        //     continue;
        // }
        // if steps == 7 && node.disks[(1 + 0*w) as usize].used != 0 {
        //     continue;
        // }

        // println!("{} {:?}", steps, node);

        // if steps > 10 {
        //     break;
        // }

        // if !closed.insert(node.clone()) {
        //     continue;
        // }

        // println!(">>>>");
        for y in 0..h {
            for x in 0..w {
                // let range = 10;
                // if (x - node.goal.0).abs() > range || (y - node.goal.1).abs() > range {
                //     continue;
                // }

                for n in [(x-1, y), (x+1, y), (x, y-1), (x, y+1)] {
                    if n.0 >= 0 && n.0 < w && n.1 >= 0 && n.1 < h {
                        let src = node.disks[(x + y * w) as usize];
                        let dst = node.disks[(n.0 + n.1 * w) as usize];
                        if src.used > 0 && dst.size - dst.used >= src.used {
                            let mut new = node.clone();
                            new.disks[(x + y * w) as usize].used = 0;
                            new.disks[(n.0 + n.1 * w) as usize].used += src.used;
                            if new.goal == (x, y) {
                                // println!("!! {:?} {:?} {:?}", new.goal, n, new);
                                new.goal = n;
                            }
                            // println!("{:?}", new);
                            if closed.insert(new.clone()) {
                                // println!("{:?}({}/{}) -> {:?}({}/{})", (x, y), src.used, src.size, n, dst.used, dst.size);
                                open.push_back((new, steps + 1));
                            }
                        }
                    }
                }
            }
        }
        // println!("<<<<");
    }

    // State:
    //   Node containing G
    //   Size, used in each cell
    // Actions:
    //   Move from A to neighbour
}

const INPUT_DEMO: &str = "
Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%
";

fn main() {
    // run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("22/input.txt").unwrap());
}
