use std::{fs, cell::OnceCell, collections::{HashMap, HashSet}};

// For each chunk:
//  Use hash table to convert each chunk to ID
//  Use other hash table to map 5 chunks (+ shape) into new middle chunk
//
// Repeat with recursively bigger chunks

#[derive(Hash, PartialEq, Eq, Clone)]
struct ChunkState {
    visited: Vec<Vec<bool>>,
    count: usize,
}

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
    let mut chunk_progression: HashMap<[u32; 5], u32> = HashMap::new();
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

    for i in 0..26501365 {
        if [6, 10, 50, 100, 500, 1000, 5000, 26501365].contains(&i) {
            let sum: usize = chunks.values().chain(chunks_cyclic.values()).map(|&c|
                chunk_by_id.get(c as usize).unwrap().count
            ).sum();
            println!("{}: {:?}", i, sum);
        }
        if i % 100 == 0 {
            println!("...{} {} {} {}", i, chunks.len(), chunk_by_id.len(), chunks_cyclic.len());
        }
        let old_chunks = chunks.clone();
        let mut expand = HashSet::new();
        let mut remove = HashSet::new();
        for (&(cx, cy), chunk) in chunks.iter_mut() {
            let cn = [
                old_chunks.get(&(cx-1, cy)).copied().unwrap_or(0),
                old_chunks.get(&(cx+1, cy)).copied().unwrap_or(0),
                old_chunks.get(&(cx, cy-1)).copied().unwrap_or(0),
                old_chunks.get(&(cx, cy+1)).copied().unwrap_or(0),
                *chunk
            ];

            *chunk = *chunk_progression.entry(cn).or_insert_with(|| {
                let v: Vec<_> = cn.iter().map(|&c| &chunk_by_id[c as usize].visited).collect();

                let mut new_chunk = empty.clone();
                for y in 0..h {
                    for x in 0..w {
                        let pw = if x > 0   { v[4][y][x-1] } else { v[0][y][w-1] };
                        let pe = if x < w-1 { v[4][y][x+1] } else { v[1][y][0] };
                        let pn = if y > 0   { v[4][y-1][x] } else { v[2][h-1][x] };
                        let ps = if y < h-1 { v[4][y+1][x] } else { v[3][0][x] };
                        new_chunk.visited[y][x] = plots[y][x] && (
                            pw || pe || pn || ps
                        );
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
                if cn[0] == 0 { expand.insert((cx-1, cy)); }
                if cn[1] == 0 { expand.insert((cx+1, cy)); }
                if cn[2] == 0 { expand.insert((cx, cy-1)); }
                if cn[3] == 0 { expand.insert((cx, cy+1)); }
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
    }

    let sum: usize = chunks.values().chain(chunks_cyclic.values()).map(|&c|
        chunk_by_id.get(c as usize).unwrap().count
    ).sum();
    println!("{:?}", sum);
    // println!("{:?}", visited.iter().map(|row| row.iter().filter(|c| **c).count()).sum::<usize>());
}
