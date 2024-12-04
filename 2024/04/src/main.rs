fn is_word(grid: &Vec<Vec<char>>, word: &Vec<char>, x0: i32, y0: i32, dx: i32, dy: i32) -> bool {
    (0..word.len() as i32).all(|i| {
        let x = x0 + dx*i;
        let y = y0 + dy*i;
        0 <= x && x < grid[0].len() as i32 &&
        0 <= y && y < grid.len() as i32 &&
        grid[y as usize][x as usize] == word[i as usize]
})
}

fn is_x_mas(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    grid[y][x] == 'A' && (
        (grid[y-1][x-1] == 'M' && grid[y+1][x+1] == 'S') ||
        (grid[y-1][x-1] == 'S' && grid[y+1][x+1] == 'M')
    ) && (
        (grid[y+1][x-1] == 'M' && grid[y-1][x+1] == 'S') ||
        (grid[y+1][x-1] == 'S' && grid[y-1][x+1] == 'M')
    )
}

fn run(title: &str, input: &str) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let word = "XMAS".chars().collect();

    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, 1), (-1, -1), (1, -1), (1, 1)] {
                if is_word(&grid, &word, x as i32, y as i32, dx, dy) {
                    count += 1;
                }
            }
        }
    }

    let mut count2 = 0;
    for y in 1..grid.len()-1 {
        for x in 1..grid[0].len()-1 {
            if is_x_mas(&grid, x, y) {
                count2 += 1;
            }
        }
    }

    println!("{} part 1: {}", title, count);

    println!("{} part 2: {}", title, count2);
}

const INPUT_DEMO: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() {
    run("demo", INPUT_DEMO);
    run("input", &std::fs::read_to_string("04/input.txt").unwrap());
}
