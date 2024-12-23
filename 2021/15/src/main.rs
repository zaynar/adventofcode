// Part 1: 5 mins
// Part 1+2: 9 mins

use aocgrid::Grid;
use aocpath::Pathfinder;

fn run(title: &str, input: &str) {
    let grid0: Grid<i32> = Grid::from(input).map(|c| c.to_digit(10).unwrap() as i32);

    let mut grid: Grid<i32>  = Grid::new_empty(grid0.width() as usize * 5, grid0.height() as usize * 5, 0);
    grid.for_each_mut(|x, y, c| {
        let (w, h) = (grid0.width(), grid0.height());
        *c = (*grid0.get(x % w, y % h) as i32 + x / w + y / h - 1) % 9 + 1;
    });

    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
    struct Node {
        pos: (i32, i32),
    }

    struct PathContext {
        grid: Grid<i32>,
        target: (i32, i32),
    }

    impl aocpath::Callbacks<Node> for PathContext {
        fn get_neighbours(&mut self, node: &Node) -> Vec<(i64, Node)> {

            let mut ret = vec![];
            let (x, y) = node.pos;
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if let Some(n) = self.grid.try_get(x + dx, y + dy) {
                    ret.push((*n as i64, Node { pos: (x + dx, y + dy) }));

                }
            }

            ret
        }

        fn found_path(&mut self, id: &Node, cost: i64) -> Result<bool, aocpath::PathError> {
            if id.pos == self.target {
                println!("found path to {:?}, cost {}", id, cost);
                return Err(aocpath::PathError::Abort);
            }
            Ok(true)
        }
    }

    let mut ctx = PathContext { grid: grid.clone(), target: (grid.width() - 1, grid.height() - 1) };
    let mut pathfinder = Pathfinder::new();
    let _ = pathfinder.dijkstra(&mut ctx, Node { pos: (0, 0) });

    println!("{} part 1: {}", title, "TODO");

    println!("{} part 2: {}", title, "TODO");
}

const INPUT_DEMO: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("15/input.txt").unwrap());
}
