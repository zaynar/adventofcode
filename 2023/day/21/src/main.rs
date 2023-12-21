use std::{fs, cell::OnceCell};

fn main() {
    let mut start = OnceCell::new();
    let s = &mut start;
    let file = fs::read_to_string("input").unwrap();
    let plots: Vec<Vec<bool>> = file.lines().enumerate().map(|(y, line)| {
        if let Some(x) = line.find("S") {
            start.set((x, y)).unwrap();
        }
        line.chars().map(|c| c != '#').collect()
    }).collect();
    // println!("{:?} {:?}", start, plots);

    let start = start.get().unwrap();
    let mut visited: Vec<Vec<bool>> = plots.iter().map(|row| row.iter().map(|c| false).collect()).collect();
    visited[start.1][start.0] = true;

    // println!("{:?}", visited);
    // println!("{:?}", visited.iter().map(|row| row.iter().filter(|c| **c).count()).sum::<usize>());

    let w = visited[0].len();
    let h = visited.len();

    for i in 0..64 {
        let old_vis = visited.clone();
        for y in 0..h {
            for x in 0..w {
                visited[y][x] = plots[y][x] && (
                    (x > 0 && old_vis[x-1][y]) ||
                    (x < w-1 && old_vis[x+1][y]) ||
                    (y > 0 && old_vis[x][y-1]) ||
                    (y < h-1 && old_vis[x][y+1])
                );
            }
        }
    }

    // println!("{:?}", visited);
    println!("{:?}", visited.iter().map(|row| row.iter().filter(|c| **c).count()).sum::<usize>());
}
