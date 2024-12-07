use itertools::Itertools;

fn run(mut hp: i32, damage: i32, armour: i32) -> bool {
    let mut boss_hp = 109;
    let boss_damage = 8;
    let boss_armour = 2;

    // let mut boss_hp = 12;
    // let boss_damage = 7;
    // let boss_armour = 2;

    loop {
        let hit = (damage - boss_armour).max(1);
        boss_hp -= hit;
        // println!("boss {}", boss_hp);
        if boss_hp <= 0 {
            return true;
        }

        let hit = (boss_damage - armour).max(1);
        hp -= hit;
        // println!("you {}", hp);
        if hp <= 0 {
            return false;
        }
    }
}

fn main() {

    let mut best = i32::MAX;
    let mut worst = 0;
    for weapon in [
        (  8,     4,       0),
        ( 10,     5,       0),
        ( 25,     6,       0),
        ( 40,     7,       0),
        ( 74,     8,       0),
    ] {
        for armour in [
            (  0,     0,       0),
            ( 13,     0,       1),
            ( 31,     0,       2),
            ( 53,     0,       3),
            ( 75,     0,       4),
            (102,     0,       5),
        ] {
            for (ring0, ring1) in [
                (  0,     0,       0),
                (  0,     0,       0),
                ( 25,     1,       0),
                ( 50,     2,       0),
                (100,     3,       0),
                ( 20,     0,       1),
                ( 40,     0,       2),
                ( 80,     0,       3),
            ].iter().tuple_combinations() {
                let hp = 100;
                let cost = weapon.0 + armour.0 + ring0.0 + ring1.0;
                let damage = weapon.1 + armour.1 + ring0.1 + ring1.1;
                let armour_val = weapon.2 + armour.2 + ring0.2 + ring1.2;

                if run(hp, damage, armour_val) {
                    if cost < best {
                        println!("{} {} {} {:?} {:?} {:?} {:?}", cost, damage, armour_val, weapon, armour, ring0, ring1);
                    }
                    best = best.min(cost);
                } else {
                    worst = worst.max(cost);
                }
            }
        }
    }

    println!("part 1: {}", best);
    println!("part 2: {}", worst);
}
