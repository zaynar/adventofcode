use std::fs;

// const ERRORS: usize = 0; // part 1
const ERRORS: usize = 1; // part 2

fn vert_reflect(grid: &Vec<Vec<u8>>) -> Option<usize> {
    let w = grid[0].len();
    let h = grid.len();
    for r in 1..w {
        let c = r.min(w - r);
        let errs: usize = (0..h).map(|y| {
            (0..c).filter(|i| grid[y][r-i-1] != grid[y][r+i]).count()
        }).sum();
        if errs == ERRORS {
            return Some(r);
        }
    }
    return None;
}

fn horiz_reflect(grid: &Vec<Vec<u8>>) -> Option<usize> {
    let w = grid[0].len();
    let h = grid.len();
    for r in 1..h {
        let c = r.min(h - r);
        let errs: usize = (0..w).map(|x| {
            (0..c).filter(|i| grid[r-i-1][x] != grid[r+i][x]).count()
        }).sum();
        if errs == ERRORS {
            return Some(r);
        }
    }
    return None;
}

fn main() {
    let mut grids = Vec::new();
    grids.push(Vec::new());

    for line in fs::read_to_string("input").unwrap().lines() {
        if line.is_empty() {
            grids.push(Vec::new());
        } else {
            grids.last_mut().unwrap().push(line.as_bytes().to_vec());
        }
    }

    let mut sum = 0;
    for grid in &grids {
        println!("{:?}", grid);
        println!("{:?} {:?}", vert_reflect(grid), horiz_reflect(grid));
        sum += vert_reflect(grid).unwrap_or(0);
        sum += horiz_reflect(grid).unwrap_or(0) * 100;
    }
    println!("{}", sum);

}
