// Part 1: 7 mins
// Part 1+2: 7 mins

use aocgrid::Grid;

fn run(title: &str, input: &str) {

    let mut grid = Grid::from(input);

    for i in 0.. {
        let start = grid.clone();

        grid = grid.map_coords(|x, y, c| {
            if c == '>' && *grid.get((x + 1) % grid.width(), y) == '.' {
                '.'
            } else if c == '.' && *grid.get((x + grid.width() - 1) % grid.width(), y) == '>' {
                '>'
            } else {
                c
            }
        });

        grid = grid.map_coords(|x, y, c| {
            if c == 'v' && *grid.get(x, (y + 1) % grid.height()) == '.' {
                '.'
            } else if c == '.' && *grid.get(x, (y + grid.height() - 1) % grid.height()) == 'v' {
                'v'
            } else {
                c
            }
        });

        if grid == start {
            println!("{} part 1: {}", title, i + 1);
            break;
        }
    }

    // println!("{}", grid);


}

const INPUT_DEMO: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("25/input.txt").unwrap());
}
