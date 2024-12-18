// Part 1: 10 mins
// Part 1+2: 34 mins

use std::collections::HashSet;

use aocgrid::Grid;
use aocpath::Pathfinder;

fn run(title: &str, input: &str) {
    let grid = Grid::from(input);
    // grid.print();

    let start = grid.find(&'S').unwrap();
    let end = grid.find(&'E').unwrap();

    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
    struct Node {
        pos: (i32, i32),
        dir: (i32, i32),
    }

    struct PathContext {
        grid: Grid<char>,
        end: (i32, i32),
        best: Option<(Node, i64)>,
    }

    impl aocpath::Callbacks<Node> for PathContext {
        fn get_neighbours(&mut self, node: &Node) -> Vec<(i64, Node)> {
            let mut ret = Vec::new();
            ret.push((1000, Node { pos: node.pos, dir: (-node.dir.1, node.dir.0) }));
            ret.push((1000, Node { pos: node.pos, dir: (node.dir.1, -node.dir.0) }));

            let newpos = (node.pos.0 + node.dir.0, node.pos.1 + node.dir.1);
            if *self.grid.get(newpos.0, newpos.1) != '#' {
                ret.push((1, Node { pos: newpos, dir: node.dir }));
            }

            ret
        }

        fn found_path(&mut self, id: &Node, cost: i64) -> Result<(), aocpath::PathError> {
            if id.pos == self.end && self.best.is_none() {
                println!("found path to {:?}", id);
                self.best = Some((id.clone(), cost));
                return Err(aocpath::PathError::Abort);
            }
            Ok(())
        }
    }

    let mut ctx = PathContext { grid: grid.clone(), end, best: None };
    let mut pathfinder = Pathfinder::new();
    let _ = pathfinder.dijkstra_all(&mut ctx, Node { pos: start, dir: (1, 0) });

    let path = pathfinder.get_path(ctx.best.unwrap().0);
    let path_coords: HashSet<(i32, i32)> = HashSet::from_iter(path.iter().map(|n| n.pos));

    // Really should repeat this for dir in [(1,0),(-1,0),(0,1),(0,-1)]
    let preds = pathfinder.get_all_preds(ctx.best.unwrap().0);
    let pred_coords: HashSet<(i32, i32)> = HashSet::from_iter(preds.iter().map(|n| n.pos));

    // println!("{}", grid.map_coords(|x, y, c| if path_coords.contains(&(x, y)) { 'O' } else { c }));
    // println!("{}", grid.map_coords(|x, y, c| if pred_coords.contains(&(x, y)) { 'O' } else { c }));

    println!("{} part 1: {:?}", title, ctx.best.unwrap().1);
    println!("{} part 2: {:?}", title, pred_coords.len());
}


const INPUT_DEMO: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("16/input.txt").unwrap());
}
