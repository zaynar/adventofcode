#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct State {
    hp: i32,
    boss_hp: i32,
    spent: i32,

    armor: i32,
    mana: i32,

    boss_damage: i32,

    shield: i32,
    poison: i32,
    recharge: i32,

    ended: bool,
}

#[derive(Debug)]
enum Spell {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

fn apply_effects(mut state: State) -> State {
    if state.shield > 0 {
        // println!("shield {}", state.shield);
        state.shield -= 1;
        if state.shield == 0 {
            state.armor -= 7;
        }
    }

    if state.poison > 0 {
        // println!("poison {}", state.poison);
        state.boss_hp -= 3;
        state.poison -= 1;
        if state.boss_hp <= 0 {
            state.ended = true;
            return state;
        }
    }

    if state.recharge > 0 {
        // println!("recharge {}", state.recharge);
        state.mana += 101;
        state.recharge -= 1;
    }

    state
}

fn step(mut state: State, spell: Spell) -> State {
    if state.ended {
        return state;
    }

    // println!("Your turn; {} hp, {} armour, {} mana, {} boss hp", state.hp, state.armor, state.mana, state.boss_hp);

    if true { // part 2
        state.hp -= 1;
        if state.hp <= 0 {
            state.ended = true;
            return state;
        }
    }

    state = apply_effects(state);
    if state.ended {
        return state;
    }

    // println!(">>> cast {:?}", spell);
    let prev_mana = state.mana;
    match spell {
        Spell::Missile => {
            state.mana -= 53;
            state.boss_hp -= 4;
        },
        Spell::Drain => {
            state.mana -= 73;
            state.boss_hp -= 2;
            state.hp += 2;
        }
        Spell::Shield if state.shield == 0 => {
            state.mana -= 113;
            state.shield = 6;
            state.armor += 7;
        }
        Spell::Poison if state.poison == 0 => {
            state.mana -= 173;
            state.poison = 6;
        }
        Spell::Recharge if state.recharge == 0 => {
            state.mana -= 229;
            state.recharge = 5;
        }
        _ => {
            state.hp = -100;
            state.ended = true;
            return state;
        }
    }
    state.spent += prev_mana - state.mana;

    if state.mana < 0 {
        state.hp = -200;
        state.ended = true;
        return state;
    }

    if state.boss_hp <= 0 {
        state.ended = true;
        return state;
    }

    // println!("Boss turn; {} hp, {} armour, {} mana, {} boss hp", state.hp, state.armor, state.mana, state.boss_hp);

    state = apply_effects(state);
    if state.ended {
        return state;
    }

    let hit = (state.boss_damage - state.armor).max(1);
    // println!("boss hits for {}", hit);
    state.hp -= hit;
    if state.hp <= 0 {
        state.ended = true;
        return state;
    }

    state
}

fn main() {
    /*
    let demo = State {
        hp: 10,
        boss_hp: 13,
        spent: 0,
        armor: 0,
        mana: 250,
        boss_damage: 8,
        shield: 0,
        poison: 0,
        recharge: 0,
        ended: false,
    };

    println!("{:?}", demo);
    let demo = step(demo, Spell::Poison);
    println!("{:?}", demo);
    let demo = step(demo, Spell::Missile);
    println!("{:?}", demo);

    let mut demo = State {
        hp: 10,
        boss_hp: 14,
        spent: 0,
        armor: 0,
        mana: 250,
        boss_damage: 8,
        shield: 0,
        poison: 0,
        recharge: 0,
        ended: false,
    };

    println!("--------");

    println!("{:?}", demo);
    for spell in [Spell::Recharge, Spell::Shield, Spell::Drain, Spell::Poison, Spell::Missile] {
        demo = step(demo, spell);
        println!("{:?}", demo);
    }
    */

    let initial = State {
        hp: 50,
        boss_hp: 51,
        spent: 0,
        armor: 0,
        mana: 500,
        boss_damage: 9,
        shield: 0,
        poison: 0,
        recharge: 0,
        ended: false,
    };

    let mut states = vec![initial];

    while !states.is_empty() {
        states.sort();
        states.sort_by_key(|s| -s.spent);
        states.dedup();

        let state = states.pop().unwrap();

        // println!("{} {:?}", states.len(), state);

        if state.mana < 0 || state.hp <= 0 {
            // failed
            continue;
        }

        if state.boss_hp <= 0 {
            // won
            println!("{:?}", state);
            break;
        }

        if state.ended {
            println!("ended??? {:?}", state);
            panic!();
        }

        for spell in [Spell::Missile, Spell::Drain, Spell::Shield, Spell::Poison, Spell::Recharge] {
            states.push(step(state.clone(), spell));
        }
    }
}
