fn pdisk(disk: &Vec<Option<usize>>) -> String {
    disk.iter().map(|b| if let Some(n) = b { if *n < 10 { (*n as u8 + '0' as u8) as char } else { '?' } } else { '.' }).collect::<String>()
}

fn run(title: &str, input: &str) {
    let mut disk = Vec::new();

    for (i, n) in input.trim().chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
        if i % 2 == 0 {
            for j in 0..n {
                disk.push(Some(i / 2));
            }
        } else {
            for j in 0..n {
                disk.push(None);
            }
        }
    }

    // println!("{:?}", pdisk(&disk));

    let mut defrag = Vec::new();
    {
        let mut i = 0;
        while i < disk.len() {
            while disk.last().unwrap().is_none() {
                disk.pop();
            }
            if i >= disk.len() {
                break;
            }

            if let Some(n) = disk[i] {
                // println!("{} cp {} {}", i, n, pdisk(&disk));
                defrag.push(Some(n));
            } else {
                let pop = disk.pop().unwrap();
                // println!("{} pop {:?} {}", i, pop, pdisk(&disk));
                defrag.push(pop);
            }
            i += 1;
        }
    }

    // println!("{:?}", pdisk(&defrag));

    println!("{} part 1: {}", title, defrag.iter().enumerate().map(|(i, b)| if let Some(n) = b { i * n } else { 0 }).sum::<usize>());
}

fn run2(title: &str, input: &str) {

    let mut disk = Vec::new();

    let mut files = Vec::new();
    let mut free = Vec::new();
    let mut offset = 0;
    for (i, n) in input.trim().chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
        if i % 2 == 0 {
            for j in 0..n {
                disk.push(true);
            }
            files.push((offset, n, i / 2));
        } else {
            for j in 0..n {
                disk.push(false);
            }
            free.push((offset, n));
        }
        offset += n;
    }

    // println!("{:?}", files);
    // println!("{:?}", free);

    for j in (0..files.len()).rev() {
        // println!("{:?}", disk.iter().map(|p| if *p { '#' } else { '.' }).collect::<String>());

        let file = files[j];

        for i in 0..file.0 {
            if !disk[i as usize .. i as usize + file.1 as usize].iter().any(|p| *p) {
                disk[file.0 as usize .. file.0 as usize + file.1 as usize].iter_mut().for_each(|n| *n = false);
                disk[i as usize .. i as usize + file.1 as usize].iter_mut().for_each(|n| *n = true);
                files[j].0 = i;
                break;
            }

        }
    }

    files.sort();
    // println!("{:?}", files);

    println!("{} part 2: {:?}", title, files.iter().map(|(i, sz, id)| (0..*sz).map(|j| (i + j) as usize * id).sum::<usize>()).sum::<usize>());
}

fn main() {
    run("demo", "2333133121414131402\n");
    run("input", &std::fs::read_to_string("09/input.txt").unwrap());
    run2("demo", "2333133121414131402\n");
    run2("input", &std::fs::read_to_string("09/input.txt").unwrap());
}
