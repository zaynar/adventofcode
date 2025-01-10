// Part 1: 28 mins
// Part 1+2: 101 mins

use std::collections::{HashMap, HashSet, VecDeque};

use aocpath::Pathfinder;
use itertools::Itertools;

fn run(title: &str, input: &str) {
    let mut valves = HashMap::new();
    for line in input.lines() {
        let (valve, rest) = line.strip_prefix("Valve ").unwrap().split_once(" ").unwrap();
        let (flow, rest) = rest.strip_prefix("has flow rate=").unwrap().split_once(";").unwrap();
        let rest = rest.replace("tunnel leads to valve", "tunnels lead to valves");
        let dst = rest.strip_prefix(" tunnels lead to valves ").unwrap().split(", ").map(|s| s.to_owned()).collect_vec();
        valves.insert(valve.to_owned(), (flow.parse::<u32>().unwrap(), dst));
    }

    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
    struct Node {
        pos: String,
        flow: u32,
        opened: Vec<String>,
        left: u32,
        prev: Vec<String>,
    }

    struct PathContext {
        valves: HashMap<String, (u32, Vec<String>)>,
        best: u32,
    }

    impl aocpath::Callbacks<Node> for PathContext {
        fn get_neighbours(&mut self, node: &Node) -> Vec<(i64, Node)> {
            let mut ret = Vec::new();

            if !node.opened.contains(&node.pos) && self.valves[&node.pos].0 != 0 {
                let mut opened = node.opened.clone();
                opened.push(node.pos.clone());
                opened.sort();
                ret.push((1, Node {
                    pos: node.pos.clone(),
                    flow: node.flow + self.valves[&node.pos].0 * (node.left - 1),
                    left: node.left - 1,
                    opened,
                    prev: vec![],
                }));
            }

            for n in &self.valves[&node.pos].1 {
                if !node.prev.contains(n) {
                    let mut prev = node.prev.clone();
                    prev.push(node.pos.clone());
                    prev.sort();
                    ret.push((1, Node {
                        pos: n.clone(),
                        flow: node.flow,
                        left: node.left - 1,
                        opened: node.opened.clone(),
                        prev,
                    }));
                }
            }

            // let (x, y) = node.pos;
            // let s = *self.grid.get(node.pos.0, node.pos.1);
            // let s = if s == 'S' { 'a' as u8 } else { s as u8 };
            // for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            //     if let Some(&c) = self.grid.try_get(x + dx, y + dy) {
            //         let c = if c == 'E' { 'z' as u8 } else { c as u8 };
            //         if c <= 1 + s {
            //             ret.push((1, Node { pos: (x + dx, y + dy) }));
            //         }

            //     }
            // }

            ret
        }

        fn found_path(&mut self, node: &Node, cost: i64) -> Result<bool, aocpath::PathError> {
            if node.flow > self.best {
                println!("found path to {:?}, {}", node, cost);
            }
            self.best = self.best.max(node.flow);

            let remaining = self.valves.iter().filter(|(k, v)| !node.opened.contains(k)).map(|(k, v)| v.0).sum::<u32>() * (node.left - 1);
            if node.flow + remaining < self.best {
                return Ok(false);
            }

            if node.left == 0 {
                // return Err(aocpath::PathError::Abort);
                return Ok(false);
            }
            Ok(true)
        }
    }

    let mut ctx = PathContext { valves, best: 0 };
    let mut pathfinder = Pathfinder::new();
    let _ = pathfinder.dfs(&mut ctx, Node { pos: "AA".to_owned(), flow: 0, opened: vec![], left: 30, prev: vec![] });

    // println!("{} part 1: {}", title, ctx.cost);
    // println!("{:?}", valves);

    println!("{} part 1: {}", title, ctx.best);
}

fn run2(title: &str, input: &str) {
    let mut ids = HashMap::new();
    let mut valves = HashMap::new();
    for line in input.lines() {
        let (valve, rest) = line.strip_prefix("Valve ").unwrap().split_once(" ").unwrap();
        let (flow, rest) = rest.strip_prefix("has flow rate=").unwrap().split_once(";").unwrap();
        let rest = rest.replace("tunnel leads to valve", "tunnels lead to valves");
        let dst = rest.strip_prefix(" tunnels lead to valves ").unwrap().split(", ").map(|s| s.to_owned()).collect_vec();
        valves.insert(valve.to_owned(), (flow.parse::<i64>().unwrap(), dst));
        ids.insert(valve.to_owned(), ids.len() as u8);
    }

    let mut valves2 = HashMap::new();
    for (k, v) in &valves {

        // println!("{} [label=\"{}\"];", k, v.0);
        // for d in &v.1 {
        //     println!("{} -> {};", k, d);
        // }

        if v.0 == 0 && k != "AA" {
            continue;
        }

        let mut open = VecDeque::from([(k, 0_i64)]);
        let mut visited: HashSet<String> = HashSet::new();
        while let Some((n, d)) = open.pop_front() {
            if !visited.insert(n.clone()) {
                continue;
            }

            if (valves[n].0 != 0 && d != 0) {
                valves2.entry(ids[k]).or_insert_with(|| (v.0, vec![])).1.push((ids[n], d));
            }

            for e in &valves[n].1 {
                open.push_back((e, d + 1));
            }
        }

    }

    println!("{:?}", valves2);

    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
    struct NodePart {
        pos: u8,
        // prev: Vec<String>,
        busy: i64,
    }

    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
    struct Node {
        parts: Vec<NodePart>,
        opened: Vec<u8>,
        // flow: u32,
        left: i64,
        turn: usize,
    }

    struct PathContext {
        valves: HashMap<u8, (i64, Vec<(u8, i64)>)>,
        best: i64,
    }

    impl aocpath::Callbacks<Node> for PathContext {
        fn get_neighbours(&mut self, node: &Node) -> Vec<(i64, Node)> {
            let mut ret = Vec::new();

            let part = &node.parts[node.turn];

            let both_busy = node.parts[0].busy.min(node.parts[1].busy);
            if both_busy > 0 {
                ret.push((0, Node {
                    parts: vec![
                        NodePart {
                            pos: node.parts[0].pos.clone(),
                            busy: node.parts[0].busy - both_busy,
                            // prev: part.prev.clone(),
                        },
                        NodePart {
                            pos: node.parts[1].pos.clone(),
                            busy: node.parts[1].busy - both_busy,
                            // prev: part.prev.clone(),
                        },
                    ],
                    // flow: node.flow,
                    left: node.left - both_busy,
                    opened: node.opened.clone(),
                    turn: node.turn,
                }));

                return ret;
            }

            if part.busy > 0 {
                if node.turn == 0 {
                    ret.push((0, Node {
                        parts: vec![
                            NodePart {
                                pos: part.pos.clone(),
                                busy: part.busy - 1,
                                // prev: part.prev.clone(),
                            },
                            node.parts[1].clone(),
                        ],
                        // flow: node.flow,
                        left: node.left,
                        opened: node.opened.clone(),
                        turn: 1,
                    }));
                } else {
                    ret.push((0, Node {
                        parts: vec![
                            node.parts[0].clone(),
                            NodePart {
                                pos: part.pos.clone(),
                                busy: part.busy - 1,
                                // prev: part.prev.clone(),
                            },
                        ],
                        // flow: node.flow,
                        left: node.left - 1,
                        opened: node.opened.clone(),
                        turn: 0,
                    }));
                }

                return ret;
            }

            if !node.opened.contains(&part.pos) && self.valves[&part.pos].0 != 0 {
                let mut opened = node.opened.clone();
                opened.push(part.pos.clone());
                opened.sort();
                if node.turn == 0 {
                    ret.push((-self.valves[&part.pos].0 * (node.left - 1), Node {
                        parts: vec![
                            NodePart {
                                pos: part.pos.clone(),
                                busy: 0,
                                // prev: vec![],
                            },
                            node.parts[1].clone(),
                        ],
                        // flow: node.flow + self.valves[&part.pos].0 * (node.left - 1),
                        left: node.left,
                        opened,
                        turn: 1,
                    }));
                } else {
                    ret.push((-self.valves[&part.pos].0 * (node.left - 1), Node {
                        parts: vec![
                            node.parts[0].clone(),
                            NodePart {
                                pos: part.pos.clone(),
                                busy: 0,
                                // prev: vec![],
                            },
                        ],
                        // flow: node.flow + self.valves[&part.pos].0 * (node.left - 1),
                        left: node.left - 1,
                        opened,
                        turn: 0,
                    }));
                }
            }

            let mut other = self.valves[&part.pos].1.clone();
            other.sort_by_key(|(n, d)| self.valves[n].0 * (node.left - 1 - d));
            other.reverse();
            for (n, d) in &other {
            // for (n, d) in &self.valves[&part.pos].1 {
                if
                // !part.prev.contains(n) &&
                !node.opened.contains(n) && *d < node.left
                && !(node.parts[0].pos == *n || node.parts[1].pos == *n)
                 {
                    // let mut prev = part.prev.clone();
                    // prev.push(part.pos.clone());
                    // prev.sort();
                    if node.turn == 0 {
                        ret.push((0, Node {
                            parts: vec![
                                NodePart {
                                    pos: n.clone(),
                                    busy: d - 1,
                                    // prev,
                                },
                                node.parts[1].clone(),
                            ],
                            // flow: node.flow,
                            left: node.left,
                            opened: node.opened.clone(),
                            turn: 1,
                        }));
                    } else {
                        ret.push((0, Node {
                            parts: vec![
                                node.parts[0].clone(),
                                NodePart {
                                    pos: n.clone(),
                                    busy: d - 1,
                                    // prev,
                                },
                            ],
                            // flow: node.flow,
                            left: node.left - 1,
                            opened: node.opened.clone(),
                            turn: 0,
                        }));
                    }
                }
            }

            if ret.is_empty() && part.busy == 0 {
                ret.push((0, Node {
                    parts: node.parts.clone(),
                    // flow: node.flow,
                    left: if node.turn == 0 { node.left} else { node.left - 1 },
                    opened: node.opened.clone(),
                    turn: (node.turn + 1) % 2,
                }));

            }

            ret
        }

        fn found_path(&mut self, node: &Node, cost: i64) -> Result<bool, aocpath::PathError> {
            // let flow = node.flow
            let flow = -cost as i64;
            if flow > self.best {
                println!("found path to {:?}, {}", node, cost);
            }
            self.best = self.best.max(flow);

            let both_busy = node.parts[0].busy.min(node.parts[1].busy);

            let remaining = self.valves.iter().filter(|(k, v)| !node.opened.contains(k)).map(|(k, v)| v.0).sum::<i64>() * (node.left - 1 - both_busy);
            if flow + remaining < self.best {
                return Ok(false);
            }

            if node.left <= both_busy {
                // return Err(aocpath::PathError::Abort);
                return Ok(false);
            }
            Ok(true)
        }
    }

    let mut ctx = PathContext { valves: valves2, best: 2705-1 };
    // let mut ctx = PathContext { valves: valves2, best: 0 };
    let mut pathfinder = Pathfinder::new();
    let _ = pathfinder.dijkstra(&mut ctx, Node {
        parts: vec![
            NodePart { pos: ids["AA"], busy: 0,
            // prev: vec![]
         },
            NodePart { pos: ids["AA"], busy: 0,
            //  prev: vec![]
            },
        ],
        // flow: 0,
         opened: vec![],
        left: 30 - 4,
        turn: 0
    });

    // println!("{} part 1: {}", title, ctx.cost);
    // println!("{:?}", valves);

    println!("{} part 2: {}", title, ctx.best);
}

const INPUT_DEMO: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

fn main() {
    // run("demo", INPUT_DEMO);
    // run("input", &std::fs::read_to_string("16/input.txt").unwrap());
    run2("demo", INPUT_DEMO);
    run2("input", &std::fs::read_to_string("16/input.txt").unwrap());
}
