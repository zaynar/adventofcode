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
    sub: [u32; 9]
}

struct Areas {
    area_by_id: Vec<Area>,
    area_ids: HashMap<Area, u32>,
    area_top: u32,
    area_top_empty: u32,
    max_level: u32,

    progression: HashMap<(usize, u32), u32>,
}

impl Areas {
    fn new() -> Self {
        let area_by_id = Vec::new();
        let area_ids = HashMap::new();
        let progression = HashMap::new();

        let mut this = Self { area_by_id, area_ids,
            max_level: 24,
            area_top: 0, area_top_empty: 0, progression };

        let empty_chunk = 0;
        let start_chunk = 1;

        let mut empty = this.add_area(Area { level: 0, sub: [
            empty_chunk, empty_chunk, empty_chunk,
            empty_chunk, empty_chunk, empty_chunk,
            empty_chunk, empty_chunk, empty_chunk
        ]});
        let mut area = this.add_area(Area { level: 0, sub: [
            empty_chunk, empty_chunk, empty_chunk,
            empty_chunk, start_chunk, empty_chunk,
            empty_chunk, empty_chunk, empty_chunk
        ]});
        for level in 1..this.max_level {
            area = this.add_area(Area { level, sub: [
                empty, empty, empty,
                empty, area, empty,
                empty, empty, empty,
            ]});
            empty = this.add_area(Area { level, sub: [
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
        self.area_top = self.update(self.max_level,
            &[
                self.area_top_empty, self.area_top_empty, self.area_top_empty,
                self.area_top_empty, self.area_top,       self.area_top_empty,
                self.area_top_empty, self.area_top_empty, self.area_top_empty,
            ], steps);
    }

    // Calculate new ID for middle area
    fn update(&mut self, level: u32, areas: &[u32; 9], steps: usize) -> u32 {
        println!("Updating level {}, {:?}", level, areas);

        if let Some(&id) = self.progression.get(&(steps, areas[4])) {
            return id;
        }

        let id = if level == 0 {
            todo!("chunks")
        } else {
            let mut area = self.area_by_id[areas[4] as usize].clone();
            area.sub[4] = self.update(level - 1, &area.sub, steps);

            *self.area_ids.entry(area.clone()).or_insert_with(|| {
                let id = self.area_by_id.len() as u32;
                self.area_by_id.push(area);
                id
            })
        };

        self.progression.insert((steps, areas[4]), id);
        id
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
    let file = fs::read_to_string("input-demo").unwrap();
    let plots: Vec<Vec<bool>> = file.lines().enumerate().map(|(y, line)| {
        if let Some(x) = line.find("S") {
            start.set((x, y)).unwrap();
        }
        line.chars().map(|c| c != '#').collect()
    }).collect();
    // println!("{:?} {:?}", start, plots);

    // println!("{:?}", visited);
    // println!("{:?}", visited.iter().map(|row| row.iter().filter(|c| **c).count()).sum::<usize>());

    let mut chunk_by_id: Vec<ChunkState> = Vec::new();
    let mut chunk_ids: HashMap<ChunkState, u32> = HashMap::new();
    let mut chunks: HashMap<(i32, i32), u32> = HashMap::new();
    let mut chunk_progression: HashMap<(usize, [u32; 9]), u32> = HashMap::new();
    let mut chunk_history: HashMap<(i32, i32), Vec<u32>> = HashMap::new();
    let mut chunks_cyclic: HashMap<(i32, i32), u32> = HashMap::new();

    let w = plots[0].len();
    let h = plots.len();

    let empty = ChunkState { visited: plots.iter().map(|row| row.iter().map(|c| false).collect()).collect(), count: 0 };

    let start = start.get().unwrap();
    let mut start_chunk = empty.clone();
    start_chunk.visited[start.1][start.0] = true;
    start_chunk.count = 1;

    chunk_by_id.push(empty.clone());
    chunk_ids.insert(empty.clone(), 0);
    chunk_by_id.push(start_chunk.clone());
    chunk_ids.insert(start_chunk, 1);
    chunks.insert((0, 0), 1);

    let mut areas = Areas::new();

    // let mut area_by_id: Vec<Area> = Vec::new();
    // let mut area_ids: HashMap<Area, u32> = HashMap::new();
    // let max_levels = 22;
    // {
    //     let mut p = 0;
    //     for i in 0..max_levels {
    //         let area = Area { level: i, sub: [p, p, p, p] };
    //         let id = area_by_id.len() as u32;
    //         area_ids.insert(area.clone(), id);
    //         area_by_id.push(area);
    //         p = id;
    //     }
    // }

    // let mut area = update_area(0,  &mut area_ids, &area_by_id);

    // let mut hist = Vec::new();
    let mut i = 0;
    while i < 26501365 {
        if [6, 10, 50, 100, 500, 1000, 5000, 26501365].contains(&i) {
            let sum: usize = chunks.values().chain(chunks_cyclic.values()).map(|&c|
                chunk_by_id.get(c as usize).unwrap().count
            ).sum();
            println!("{}: {:?}", i, sum);
        }
        if i % 100 == 0 {
            println!("...{} {} {} {}", i, chunks.len(), chunk_by_id.len(), chunks_cyclic.len());
        }

        // if (5000 - i) % 131 == 0 {
        //     let sum: usize = chunks.values().chain(chunks_cyclic.values()).map(|&c|
        //         chunk_by_id.get(c as usize).unwrap().count
        //     ).sum();
        //     println!("# {}: {:?}", i, sum);
        //     hist.push(sum as isize);
        //     let hl = hist.len();
        //     if hl > 3 {
        //         println!("## {} {}", hist[hl-1]-hist[hl-2], (hist[hl-1]-hist[hl-2])-(hist[hl-2]-hist[hl-3]));
        //     }
        // }

        let big_steps = plots.len();
        // assert!(big_steps < plots.len());
        let steps = if i < 1000 || (5000 - i) % big_steps != 0 { 1 } else { big_steps };
        // let steps = 1;

        areas.update_top(steps);

        let old_chunks = chunks.clone();
        let mut expand = HashSet::new();
        let mut remove = HashSet::new();
        for (&(cx, cy), chunk) in chunks.iter_mut() {
            let cn = [
                old_chunks.get(&(cx-1, cy-1)).copied().unwrap_or(0),
                old_chunks.get(&(cx  , cy-1)).copied().unwrap_or(0),
                old_chunks.get(&(cx+1, cy-1)).copied().unwrap_or(0),
                old_chunks.get(&(cx-1, cy  )).copied().unwrap_or(0),
                old_chunks.get(&(cx  , cy  )).copied().unwrap_or(0),
                old_chunks.get(&(cx+1, cy  )).copied().unwrap_or(0),
                old_chunks.get(&(cx-1, cy+1)).copied().unwrap_or(0),
                old_chunks.get(&(cx  , cy+1)).copied().unwrap_or(0),
                old_chunks.get(&(cx+1, cy+1)).copied().unwrap_or(0),
            ];

            *chunk = *chunk_progression.entry((steps, cn)).or_insert_with(|| {
                let v: Vec<_> = cn.iter().map(|&c| &chunk_by_id[c as usize].visited).collect();

                let mut visited = Vec::from_iter((0..h*3).map(|_| Vec::from_iter((0..w*3).map(|_| false))));
                for y in 0..h*3 {
                    for x in 0..w*3 {
                        let ix = x as isize;
                        let iy = y as isize;
                        let w = w as isize;
                        let h = h as isize;
                        visited[y][x] = get_cell(ix, iy, w, h, &v);
                    }
                }

                for i in 0..steps {
                    let oldv = visited.clone();
                    for y in 0..h*3 {
                        for x in 0..w*3 {
                            let ix = x as isize;
                            let iy = y as isize;
                            let iw = w as isize;
                            let ih = h as isize;
                            visited[y][x] = plots[y % h][x % w] && (
                                get_cell2(ix-1, iy, iw, ih, &oldv) ||
                                get_cell2(ix+1, iy, iw, ih, &oldv) ||
                                get_cell2(ix, iy-1, iw, ih, &oldv) ||
                                get_cell2(ix, iy+1, iw, ih, &oldv)
                            );
                        }
                    }
                }
                let mut new_chunk = empty.clone();
                for y in 0..h {
                    for x in 0..w {
                        new_chunk.visited[y][x] = visited[y + h][x + w];
                    }
                }
                new_chunk.count = new_chunk.visited.iter().map(|row| row.iter().filter(|c| **c).count()).sum();

                *chunk_ids.entry(new_chunk.clone()).or_insert_with(|| {
                    let id = chunk_by_id.len() as u32;
                    chunk_by_id.push(new_chunk);
                    id
                })
            });

            if *chunk != 0 {
                if cn[0] == 0 { expand.insert((cx-1, cy-1)); }
                if cn[1] == 0 { expand.insert((cx  , cy-1)); }
                if cn[2] == 0 { expand.insert((cx+1, cy-1)); }
                if cn[3] == 0 { expand.insert((cx-1, cy  )); }

                if cn[5] == 0 { expand.insert((cx+1, cy  )); }
                if cn[6] == 0 { expand.insert((cx-1, cy+1)); }
                if cn[7] == 0 { expand.insert((cx  , cy-1)); }
                if cn[8] == 0 { expand.insert((cx+1, cy+1)); }
            }

            let hist = chunk_history.entry((cx, cy)).or_insert_with(|| Vec::new());
            hist.push(*chunk);
            let hl = hist.len();
            if i % 2 == 1 && *chunk != 0 && hl > 2 && (0..1).all(|i| hist[hl-2*i-1] == hist[hl-2*i-3]) {
                remove.insert((cx, cy));
            }
        }

        for (cx, cy) in expand {
            if !chunks_cyclic.contains_key(&(cx, cy)) {
                chunks.entry((cx, cy)).or_insert(0);
            }
        }

        for (cx, cy) in remove {
            chunks_cyclic.insert((cx, cy), chunks.remove(&(cx, cy)).unwrap());
            chunk_history.remove(&(cx, cy));
        }

        i += steps;
    }

    let sum: usize = chunks.values().chain(chunks_cyclic.values()).map(|&c|
        chunk_by_id.get(c as usize).unwrap().count
    ).sum();
    println!("{:?}", sum);
    // println!("{:?}", visited.iter().map(|row| row.iter().filter(|c| **c).count()).sum::<usize>());
}
