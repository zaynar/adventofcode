// Part 1: 6 mins
// Part 1+2: 10 mins

use aocgrid::Grid;
use aocpath::Pathfinder;

fn run(title: &str, input: &str) {
    let data = Grid::from(input);
    let start = data.find(&'S').unwrap();
    let end = data.find(&'E').unwrap();

    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
    struct Node {
        pos: (i32, i32),
    }

    struct PathContext {
        grid: Grid<char>,
        end: (i32, i32),
        cost: i64,
    }

    impl aocpath::Callbacks<Node> for PathContext {
        fn get_neighbours(&mut self, node: &Node) -> Vec<(i64, Node)> {
            let mut ret = Vec::new();
            let (x, y) = node.pos;
            let s = *self.grid.get(node.pos.0, node.pos.1);
            let s = if s == 'S' { 'a' as u8 } else { s as u8 };
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if let Some(&c) = self.grid.try_get(x + dx, y + dy) {
                    let c = if c == 'E' { 'z' as u8 } else { c as u8 };
                    if c <= 1 + s {
                        ret.push((1, Node { pos: (x + dx, y + dy) }));
                    }

                }
            }

            ret
        }

        fn found_path(&mut self, id: &Node, cost: i64) -> Result<bool, aocpath::PathError> {
            if id.pos == self.end {
                // println!("found path to {:?}, {}", id, cost);
                self.cost = cost;
                return Err(aocpath::PathError::Abort);
            }
            Ok(true)
        }
    }

    let mut ctx = PathContext { grid: data.clone(), end, cost: 0 };
    let mut pathfinder = Pathfinder::new();
    let _ = pathfinder.bfs(&mut ctx, Node { pos: start });

    println!("{} part 1: {}", title, ctx.cost);

    let mut best = i64::MAX;
    data.for_each(|x, y, c| {
        if *c == 'S' || *c == 'a' {
            let mut ctx = PathContext { grid: data.clone(), end, cost: i64::MAX };
            let mut pathfinder = Pathfinder::new();
            let _ = pathfinder.bfs(&mut ctx, Node { pos: (x, y) });
            best = best.min(ctx.cost);
        }
    });

    println!("{} part 2: {}", title, best);
}

const INPUT_DEMO: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("12/input.txt").unwrap());
}
