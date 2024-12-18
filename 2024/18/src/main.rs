// Part 1: 23 mins
// Part 1+2: 28 mins

use aocgrid::Grid;
use aocpath::Pathfinder;

fn run(title: &str, input: &str, size: usize, fallen: usize) {
    let mut grid = Grid::new_empty(size, size, '.');

    for (i, line) in input.lines().enumerate() {
        let (x, y) = line.split_once(",").unwrap();
        if i < fallen {
            grid.set(x.parse().unwrap(), y.parse().unwrap(), '#');
        }
    }

    // println!("{}", grid);

    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
    struct Node {
        pos: (i32, i32),
    }

    struct PathContext {
        grid: Grid<char>,
        end: (i32, i32),
    }

    impl aocpath::Callbacks<Node> for PathContext {
        fn get_neighbours(&mut self, node: &Node) -> Vec<(i64, Node)> {
            let mut ret = Vec::new();
            let (x, y) = node.pos;
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if x + dx >= 0 && x + dx < self.grid.width() &&
                y + dy >= 0 && y + dy < self.grid.height() &&
                *self.grid.get(x + dx, y + dy) == '.' {
                    ret.push((1, Node { pos: (x + dx, y + dy) }));

                }
            }

            ret
        }

        fn found_path(&mut self, id: &Node, cost: i64) -> Result<(), aocpath::PathError> {
            if id.pos == self.end {
                println!("found path to {:?}, {}", id, cost);
                return Err(aocpath::PathError::Abort);
            }
            Ok(())
        }
    }

    let mut ctx = PathContext { grid: grid.clone(), end: (size as i32 - 1, size as i32 - 1) };
    let mut pathfinder = Pathfinder::new();
    let _ = pathfinder.bfs(&mut ctx, Node { pos: (0, 0) });
}

fn run2(title: &str, input: &str, size: usize) {
    for fallen in 1.. {

        let mut grid = Grid::new_empty(size, size, '.');

        for (i, line) in input.lines().enumerate() {
            let (x, y) = line.split_once(",").unwrap();
            if i < fallen {
                grid.set(x.parse().unwrap(), y.parse().unwrap(), '#');
            }
        }

        // println!("{}", grid);

        #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
        struct Node {
            pos: (i32, i32),
        }

        struct PathContext {
            grid: Grid<char>,
            end: (i32, i32),
        }

        impl aocpath::Callbacks<Node> for PathContext {
            fn get_neighbours(&mut self, node: &Node) -> Vec<(i64, Node)> {
                let mut ret = Vec::new();
                let (x, y) = node.pos;
                for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    if x + dx >= 0 && x + dx < self.grid.width() &&
                    y + dy >= 0 && y + dy < self.grid.height() &&
                    *self.grid.get(x + dx, y + dy) == '.' {
                        ret.push((1, Node { pos: (x + dx, y + dy) }));

                    }
                }

                ret
            }

            fn found_path(&mut self, id: &Node, cost: i64) -> Result<(), aocpath::PathError> {
                if id.pos == self.end {
                    // println!("found path to {:?}, {}", id, cost);
                    return Err(aocpath::PathError::Abort);
                }
                Ok(())
            }
        }

        let mut ctx = PathContext { grid: grid.clone(), end: (size as i32 - 1, size as i32 - 1) };
        let mut pathfinder = Pathfinder::new();
        match pathfinder.bfs(&mut ctx, Node { pos: (0, 0) }) {
            Err(aocpath::PathError::Exhausted) => {
                println!("no path: {} {:?}", fallen, input.lines().nth(fallen - 1));
                return;
            },
            _ => ()
        }
    }
}

const INPUT_DEMO: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

fn main() {
    run("demo", INPUT_DEMO, 7, 12);
    run2("demo", INPUT_DEMO, 7);
    run("input", &std::fs::read_to_string("18/input.txt").unwrap(), 71, 1024);
    run2("input", &std::fs::read_to_string("18/input.txt").unwrap(), 71);
}
