// Part 1: 41 mins
// Part 1+2: 77 mins

use aocgrid::Grid;
use aocpath::Pathfinder;
use itertools::Itertools;

fn move_cost(p: u8, x: u8, y: u8, nx: u8, ny: u8) -> i64 {
    [1, 10, 100, 1000][p as usize] * (x.abs_diff(nx) + y.abs_diff(ny)) as i64
}

fn run(title: &str, input: &str, part2: bool) {
    // Valid movements:
    //   From room to hallway (but not the entrance to a room)
    //   From hallway to valid room, and only if that room has no invalid members

    let grid = Grid::from(input);

    // Coordinates:
    //   Hallway 0..=10, y=0
    //   Room 2,4,6,8, y=1..=2

    let mut pods = vec![];
    grid.for_each(|x, y, c| {
        match c {
            'A'..='D' => {
                pods.push(((*c as u8 - 'A' as u8), (x - 1) as u8, (y - 1) as u8));
            }
            _ => ()
        }
    });

    println!("{:?}", pods);

    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
    struct Node {
        pods: Vec<(u8, u8, u8)>,
    }

    struct PathContext {
        part2: bool,
    }

    impl aocpath::Callbacks<Node> for PathContext {
        fn get_neighbours(&mut self, node: &Node) -> Vec<(i64, Node)> {
            let mut ret = Vec::new();

            let mut occupied = [[false; 5]; 11];
            let mut bad_room = [false; 11];

            for &(p, x, y) in &node.pods {
                occupied[x as usize][y as usize] = true;

                if [2,4,6,8].contains(&x) && y > 0 && (p * 2) + 2 != x {
                    bad_room[x as usize] = true;
                }
            }

            for i in 0..node.pods.len() {
                let (p, x, y) = node.pods[i];

                if y > 0 && (1..y).all(|ny| !occupied[x as usize][ny as usize]) {
                    // In a room, and not blocked in.

                    for range in [(0..=x).rev().collect_vec(), (x..=10).collect_vec()] {
                        for nx in range {

                            if occupied[nx as usize][0] {
                                break;
                            }

                            if ![2,4,6,8].contains(&nx) {
                                // Reachable hallway spot

                                let mut nn = node.clone();
                                nn.pods[i] = (p, nx, 0);
                                ret.push((move_cost(p, x, y, nx, 0), nn));
                            }
                        }
                    }

                } else if y == 0 {
                    // In a hallway

                    for range in [(0..x).rev().collect_vec(), (x+1..=10).collect_vec()] {
                        for nx in range {

                            if occupied[nx as usize][0] {
                                break;
                            }

                            if [2,4,6,8].contains(&nx) {
                                // Reachable room

                                if p * 2 + 2 == nx {
                                    // Correct room

                                    if !bad_room[nx as usize] {
                                        // Doesn't contain the wrong person

                                        // Must be at least one free space, so pick the lowest
                                        assert!(!occupied[nx as usize][1]);
                                        let ny = if occupied[nx as usize][2] {
                                            1
                                        } else if !self.part2 || occupied[nx as usize][3] {
                                            2
                                        } else if occupied[nx as usize][4] {
                                            3
                                        } else {
                                            4
                                        };

                                        let mut nn = node.clone();
                                        nn.pods[i] = (p, nx, ny);
                                        ret.push((move_cost(p, x, y, nx, ny), nn));

                                    }
                                }
                            }
                        }
                    }
                }
            }

            ret
        }

        fn found_path(&mut self, node: &Node, cost: i64) -> Result<bool, aocpath::PathError> {
            // if cost % 1001 == 0 { println!("{}", cost); }
            if node.pods.iter().all(|(p, x, y)| {
                (*p * 2) + 2 == *x && *y != 0
            }) {
                println!("found path to {:?}, {}", node, cost);
                return Err(aocpath::PathError::Abort);
            }

            Ok(true)
        }

        fn heuristic(&mut self, node: &Node) -> i64 {
            node.pods.iter().map(|&(p, x, y)| {
                if (p * 2) + 2 == x {
                    0
                } else {
                    move_cost(p, x, y, x, 0) + move_cost(p, x, 0, p * 2 + 2, 1)
                }
            }).sum()
        }
    }

    let mut ctx = PathContext { part2 };
    let mut pathfinder = Pathfinder::new();
    let _ = pathfinder.astar(&mut ctx, Node { pods });
}

const INPUT_DEMO: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

const INPUT_DEMO2: &str = "#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########
";

const INPUT2: &str = "#############
#...........#
###D#B#A#C###
  #D#C#B#A#
  #D#B#A#C#
  #C#A#D#B#
  #########
";

fn main() {
    run("demo", INPUT_DEMO, false);
    // run("input", &std::fs::read_to_string("23/input.txt").unwrap(), false);
    // run("demo", INPUT_DEMO2, true);
    run("input", INPUT2, true);
}
