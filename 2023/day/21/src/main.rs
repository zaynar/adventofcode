use std::{fs, cell::OnceCell, collections::{HashMap, HashSet}};

// For each chunk:
//  Use hash table to convert each chunk to ID
//  Use other hash table to map 5 chunks (+ shape) into new middle chunk
//
// Beyond a certain point, do N steps per iteration
//
// Repeat with recursively bigger chunks:
//
// Subdiv is 2^N x 2^N chunks
// Contains 2x2 areas

#[derive(Hash, PartialEq, Eq, Clone)]
struct ChunkState {
    visited: Vec<Vec<bool>>,
    count: usize,
}

fn get_cell(x: isize, y: isize, w: isize, h: isize, cn: &Vec< &Vec<Vec<bool>> >) -> bool {
    if x < 0 || x > w*3-1 || y < 0 || y > h*3-1 {
        return false;
    }

    cn[((x / w) + 3 * (y / h)) as usize][(y % h) as usize][(x % w) as usize]
}

fn get_cell2(x: isize, y: isize, w: isize, h: isize, v: &Vec<Vec<bool>>) -> bool {
    if x < 0 || x > w*3-1 || y < 0 || y > h*3-1 {
        return false;
    }

    v[y as usize][x as usize]
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Area {
    level: u32, // 0 = contains chunks, else contains areas
    sum: u64,
    sub: [u32; 9]
}

struct Areas {
    plots: Vec<Vec<bool>>,
    w: usize,
    h: usize,

    area_by_id: Vec<Area>,
    area_ids: HashMap<Area, u32>,
    area_top: u32,
    area_top_empty: u32,
    max_level: u32,

    progression: HashMap<(usize, u32, [u32; 9]), u32>,

    chunk_by_id: Vec<ChunkState>,
    chunk_ids: HashMap<ChunkState, u32>,
}

const CHUNK_ID: u32 = 0;

impl Areas {
    fn new(plots: Vec<Vec<bool>>, start: (usize, usize)) -> Self {
        let area_by_id = Vec::new();
        let area_ids = HashMap::new();
        let progression = HashMap::new();

        let mut chunk_by_id: Vec<ChunkState> = Vec::new();
        let mut chunk_ids: HashMap<ChunkState, u32> = HashMap::new();
        // let mut chunks: HashMap<(i32, i32), u32> = HashMap::new();

        let w = plots[0].len();
        let h = plots.len();

        let empty_chunk = ChunkState { visited: plots.iter().map(|row| row.iter().map(|c| false).collect()).collect(), count: 0 };

        let mut start_chunk = empty_chunk.clone();
        start_chunk.visited[start.1][start.0] = true;
        start_chunk.count = 1;

        chunk_by_id.push(empty_chunk.clone());
        chunk_ids.insert(empty_chunk.clone(), 0 + CHUNK_ID);
        chunk_by_id.push(start_chunk.clone());
        chunk_ids.insert(start_chunk, 1 + CHUNK_ID);

        let empty_chunk = 0 + CHUNK_ID;
        let start_chunk = 1 + CHUNK_ID;


        let mut this = Self { plots, w, h, area_by_id, area_ids,
            max_level: 12,
            area_top: 0, area_top_empty: 0, progression,
            chunk_by_id, chunk_ids
         };

        let mut empty = this.add_area(Area { level: 0, sum: 0, sub: [
            empty_chunk, empty_chunk, empty_chunk,
            empty_chunk, empty_chunk, empty_chunk,
            empty_chunk, empty_chunk, empty_chunk
        ]});
        let mut area = this.add_area(Area { level: 0, sum: 1, sub: [
            empty_chunk, empty_chunk, empty_chunk,
            empty_chunk, start_chunk, empty_chunk,
            empty_chunk, empty_chunk, empty_chunk
        ]});
        for level in 1..this.max_level {
            area = this.add_area(Area { level, sum: 1, sub: [
                empty, empty, empty,
                empty, area, empty,
                empty, empty, empty,
            ]});
            empty = this.add_area(Area { level, sum: 0, sub: [
                empty, empty, empty,
                empty, empty, empty,
                empty, empty, empty,
            ]});
        }

        this.area_top = area;
        this.area_top_empty = empty;

        this
    }

    fn add_area(&mut self, area: Area) -> u32 {
        let area_by_id = &mut self.area_by_id;
        *self.area_ids.entry(area.clone()).or_insert_with(|| {
            let id = area_by_id.len() as u32;
            area_by_id.push(area);
            id
        })
    }

    fn update_top(&mut self, steps: usize) {
        self.area_top = self.update_mid(self.max_level,
            &[
                self.area_top_empty, self.area_top_empty, self.area_top_empty,
                self.area_top_empty, self.area_top,       self.area_top_empty,
                self.area_top_empty, self.area_top_empty, self.area_top_empty,
            ], steps);
    }

    fn sum(&self) -> u64 {
        self.area_by_id[self.area_top as usize].sum
    }

    // Calculate new ID for middle area
    fn update_mid(&mut self, level: u32, areas: &[u32; 9], steps: usize) -> u32 {
        // println!("Updating level {}, {:?}", level, areas);

        if let Some(&id) = self.progression.get(&(steps, level, *areas)) {
            // println!("Cached {}", id);
            return id;
        }

        let id = if level == 0 {

            // Create a new chunk based on areas
            // println!("Updating chunks {:?}", areas);

            let v: Vec<_> = areas.iter().map(|&id| &self.chunk_by_id[(id - CHUNK_ID) as usize].visited).collect();

            let mut visited = Vec::from_iter((0..self.h*3).map(|_| Vec::from_iter((0..self.w*3).map(|_| false))));
            for y in 0..self.h*3 {
                for x in 0..self.w*3 {
                    let ix = x as isize;
                    let iy = y as isize;
                    let w = self.w as isize;
                    let h = self.h as isize;
                    visited[y][x] = get_cell(ix, iy, w, h, &v);
                }
            }

            for i in 0..steps {
                let oldv = visited.clone();
                for y in 0..self.h*3 {
                    for x in 0..self.w*3 {
                        let ix = x as isize;
                        let iy = y as isize;
                        let iw = self.w as isize;
                        let ih = self.h as isize;
                        visited[y][x] = self.plots[y % self.h][x % self.w] && (
                            get_cell2(ix-1, iy, iw, ih, &oldv) ||
                            get_cell2(ix+1, iy, iw, ih, &oldv) ||
                            get_cell2(ix, iy-1, iw, ih, &oldv) ||
                            get_cell2(ix, iy+1, iw, ih, &oldv)
                        );
                    }
                }
            }

            let mut new_chunk = ChunkState {
                count: usize::MAX,
                visited: Vec::from_iter((0..self.h).map(|y| Vec::from_iter((0..self.w).map(|x|
                    visited[y + self.h][x + self.w]
                ))))
            };
            new_chunk.count = new_chunk.visited.iter().map(|row| row.iter().filter(|c| **c).count()).sum();
            let c = new_chunk.count;

            let id = *self.chunk_ids.entry(new_chunk.clone()).or_insert_with(|| {
                let id = self.chunk_by_id.len() as u32 + CHUNK_ID;
                self.chunk_by_id.push(new_chunk);
                id
            });
            // println!(" = {} ({})", id, c);
            id

        } else {
            // println!("Updating area mid {:?} lv {}", areas, level);

            let mut area = self.area_by_id[areas[4] as usize].clone();

            // XXX: update all subs
            // area.sub[4] = self.update_mid(level - 1, &area.sub, steps);

            // println!(" <sub = {:?}", area.sub);
            area.sub = [
                self.update_mid(level - 1, &self.get_subs(0, 0, &areas), steps),
                self.update_mid(level - 1, &self.get_subs(1, 0, &areas), steps),
                self.update_mid(level - 1, &self.get_subs(2, 0, &areas), steps),
                self.update_mid(level - 1, &self.get_subs(0, 1, &areas), steps),
                self.update_mid(level - 1, &self.get_subs(1, 1, &areas), steps),
                self.update_mid(level - 1, &self.get_subs(2, 1, &areas), steps),
                self.update_mid(level - 1, &self.get_subs(0, 2, &areas), steps),
                self.update_mid(level - 1, &self.get_subs(1, 2, &areas), steps),
                self.update_mid(level - 1, &self.get_subs(2, 2, &areas), steps),
            ];
            // println!(" >sub = {:?}", area.sub);

            if level == 1 {
                area.sum = area.sub.iter().map(|&a| self.chunk_by_id[(a - CHUNK_ID) as usize].count as u64).sum();
            } else {
                area.sum = area.sub.iter().map(|&a| self.area_by_id[a as usize].sum).sum();
            }

            *self.area_ids.entry(area.clone()).or_insert_with(|| {
                let id = self.area_by_id.len() as u32;
                self.area_by_id.push(area);
                id
            })
        };

        self.progression.insert((steps, level, *areas), id);
        id
    }

    fn get_subs(&self, x: u32, y: u32, areas: &[u32; 9]) -> [u32; 9] {
        let mut s: [&Area; 9] = areas.map(|a| &self.area_by_id[a as usize]);
        // x,y = 0..3

        (-1..=1).map(|dy| (-1..=1).map(|dx| {
            let cx = (x as i32 + 3 + dx);
            let cy = (y as i32 + 3 + dy);
            let ix = cx / 3;
            let iy = cy / 3;
            let dx = (cx + 3) % 3;
            let dy = (cy + 3) % 3;
            // println!("{} {} - {} {}, {} {}, {} {}", x, y, cx, cy, ix, iy, dx, dy);
            if ix < 0 || iy < 0 || ix >= 3 || iy >= 3 {
                0
            } else {
                // println!("  [{}]", s[(ix + iy*3) as usize].sub[(dx + dy*3) as usize]);
                s[(ix + iy*3) as usize].sub[(dx + dy*3) as usize]
            }
        }).collect::<Vec<_>>()).flatten().collect::<Vec<_>>().try_into().unwrap()
    }
}


/*
 Areas are 3x3 sub-areas
To calculate level N:
   Update middle sub-area using its neighbours
   If cached: return new sub-area ID
   Else:
    Calculate new sub-area based on its neighbours
 */

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

    // println!("{:?}", visited);
    // println!("{:?}", visited.iter().map(|row| row.iter().filter(|c| **c).count()).sum::<usize>());

    let big_steps = plots.len();
    let mut areas = Areas::new(plots, *start.get().unwrap());

    let mut i = 0;
    while i < 26501365 {
        if [6, 10, 50, 100, 500, 1000, 5000, 26501365].contains(&i) {
            println!("{}: {:?}", i, areas.sum());
        }
        if i % 100 == 0 {
            println!("...{} {} {} {}", i, areas.sum(), areas.area_by_id.len(), areas.chunk_by_id.len());
        }

        // assert!(big_steps < plots.len());
        // let steps = if i < 1000 || (26501365 - i) % big_steps != 0 { 1 } else { big_steps };
        let steps = if (26501365 - i) % big_steps != 0 { 1 } else { big_steps };
        // let steps = 11;

        areas.update_top(steps);

        i += steps;
    }

    println!("{:?}", areas.sum());
    // println!("{:?}", visited.iter().map(|row| row.iter().filter(|c| **c).count()).sum::<usize>());
}
