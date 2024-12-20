// Part 1: 13 mins
// Part 1+2: 27 mins

use aocgrid::Grid;
use aocpath::Pathfinder;

fn run(title: &str, depth: i32, target: (i32, i32)) {
    /*

    index(0, 0) = 0
    index(target) = 0
    index(x, 0) = x * 16807
    index(0, y) = y * 48271
    index(x, y) = erosion(x-1, y) * erosion(x, y-1)
    erosion(x, y) = (index(x, y) + depth) % 20183

    type = erosion % 3

    risk = sum over 0-target

     */

    let w = target.0 * 128;
    let h = target.1 * 2;

    let mut index = Grid::new_empty(w, h, 0);
    for y in 0..h {
        for x in 0..w {
            let i = if (x, y) == (0, 0) || (x, y) == target {
                0
            } else if x == 0 {
                y * 48271
            } else if y == 0 {
                x * 16807
            } else {
                ((*index.get(x - 1, y) + depth) % 20183) *
                ((*index.get(x, y - 1) + depth) % 20183)
            };
            index.set(x, y, i);
        }
    }

    // for y in 0..h {
    //     for x in 0..w {
    //         print!("{:8}", index[x + y*w]);
    //     }
    //     println!();
    // }

    // for y in 0..h {
    //     for x in 0..w {
    //         print!("{}", b".=|"[((index[x + y*w] + depth) % 20183) % 3] as char);
    //     }
    //     println!();
    // }

    let mut part1 = 0;
    index.for_each(|x, y, n| {
        if x <= target.0 && y <= target.1 {
            part1 += ((*n + depth) % 20183) % 3;
        }
    });
    println!("{} part 1: {}", title, part1);

    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
    struct Node {
        pos: (i32, i32),
        tool: i32, // 0 = neither, 1 = torch, 2 = gear
    }

    struct PathContext {
        grid: Grid<i32>,
        target: (i32, i32),
        depth: i32,
    }

    impl aocpath::Callbacks<Node> for PathContext {
        fn get_neighbours(&mut self, node: &Node) -> Vec<(i64, Node)> {
            let ty = ((self.grid.get(node.pos.0, node.pos.1) + self.depth) % 20183) % 3;

            let mut ret = Vec::new();
            if node.pos.0 > 0 {
                ret.push((1, Node { pos: (node.pos.0 - 1, node.pos.1), tool: node.tool }));
            }
            if node.pos.1 > 0 {
                ret.push((1, Node { pos: (node.pos.0, node.pos.1 - 1), tool: node.tool }));
            }
            ret.push((1, Node { pos: (node.pos.0 + 1, node.pos.1), tool: node.tool }));
            ret.push((1, Node { pos: (node.pos.0, node.pos.1 + 1), tool: node.tool }));

            let new_tool = if (node.tool + 1) % 3 == ty { (node.tool + 2) % 3 } else { (node.tool + 1) % 3 };
            ret.push((7, Node { pos: node.pos, tool: new_tool }));

            ret
        }

        fn found_path(&mut self, id: &Node, cost: i64) -> Result<bool, aocpath::PathError> {
            if id.pos == self.target && id.tool == 1 {
                println!("found path to {:?}, cost {}", id, cost);
                return Err(aocpath::PathError::Abort);
            }

            let ty = ((self.grid.get(id.pos.0, id.pos.1) + self.depth) % 20183) % 3;
            if ty == id.tool {
                Ok(false)
            } else {
                Ok(true)
            }
        }
    }

    let mut ctx = PathContext { grid: index.clone(), target, depth };
    let mut pathfinder = Pathfinder::new();
    let _ = pathfinder.dijkstra(&mut ctx, Node { pos: (0, 0), tool: 1 });

    println!("{} part 2: {}", title, "TODO");
}

fn main() {
    run("demo", 510, (10, 10));
    run("input", 10914, (9,739));
}
