// Part 1: 16 mins
// Part 1+2: 19 mins

use aocgrid::Grid;
use aocpath::Pathfinder;

#[derive(Debug, Clone)]
struct Blizzard {
    x: i32,
    y: i32,
    d: (i32, i32),
}

fn busy(blizzards: &Vec<Blizzard>, w: i32, h: i32, t: i32, x: i32, y: i32) -> bool {
    for b in blizzards {
        let (bx, by) = match b.d {
            (1, 0) => ((b.x - 1 + t).rem_euclid(w - 2) + 1, b.y),
            (-1, 0) => ((b.x - 1 - t).rem_euclid(w - 2) + 1, b.y),
            (0, 1) => (b.x, (b.y - 1 + t).rem_euclid(h - 2) + 1),
            (0, -1) => (b.x, (b.y - 1 - t).rem_euclid(h - 2) + 1),
            _ => panic!()
        };

        if (bx, by) == (x, y) {
            return true;
        }
    }

    false
}

fn run(title: &str, input: &str) {
    let grid = Grid::from(input);

    let mut blizzards = vec![];
    grid.for_each(|x, y, c| {
        match c {
            '>' => blizzards.push(Blizzard { x, y, d: (1, 0) }),
            '<' => blizzards.push(Blizzard { x, y, d: (-1, 0) }),
            'v' => blizzards.push(Blizzard { x, y, d: (0, 1) }),
            '^' => blizzards.push(Blizzard { x, y, d: (0, -1) }),
            _ => ()
        }
    });

    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
    struct Node {
        x: i32,
        y: i32,
        t: i32,
    }

    struct PathContext {
        blizzards: Vec<Blizzard>,
        grid: Grid<char>,
        goal: i32,
        best: i64,
    }

    impl aocpath::Callbacks<Node> for PathContext {
        fn get_neighbours(&mut self, node: &Node) -> Vec<(i64, Node)> {
            let mut ret = Vec::new();

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)] {
                let x = node.x + dx;
                let y = node.y + dy;
                if y >= 0 && y < self.grid.height()
                    && *self.grid.get(x, y) != '#'
                    && !busy(&self.blizzards, self.grid.width(), self.grid.height(), node.t + 1, x, y)
                {
                    ret.push((1, Node {
                        x,
                        y,
                        t: node.t + 1,
                    }));
                }
            }

            ret
        }

        fn found_path(&mut self, node: &Node, cost: i64) -> Result<bool, aocpath::PathError> {
            // println!("{:?}", node);
            if node.y == self.goal {
                println!("found path to {:?}, {}", node, cost);
                self.best = cost;
                return Err(aocpath::PathError::Abort);
            }

            Ok(true)
        }
    }

    let mut ctx = PathContext { blizzards: blizzards.clone(), grid: grid.clone(), goal: grid.height() - 1, best: 0 };
    let mut pathfinder = Pathfinder::new();
    let _ = pathfinder.bfs(&mut ctx, Node { x: 1, y: 0, t: 0 });

    println!("{} part 1: {}", title, ctx.best);
    let mut part2 = ctx.best;

    let mut ctx = PathContext { blizzards: blizzards.clone(), grid: grid.clone(), goal: 0, best: 0 };
    let mut pathfinder = Pathfinder::new();
    let _ = pathfinder.bfs(&mut ctx, Node { x: grid.width() - 2, y: grid.height() - 1, t: part2 as i32 });

    part2 += ctx.best;

    let mut ctx = PathContext { blizzards: blizzards.clone(), grid: grid.clone(), goal: grid.height() - 1, best: 0 };
    let mut pathfinder = Pathfinder::new();
    let _ = pathfinder.bfs(&mut ctx, Node { x: 1, y: 0, t: part2 as i32 });

    part2 += ctx.best;

    println!("{} part 2: {}", title, part2);
}

const INPUT_DEMO: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("24/input.txt").unwrap());
}
