use std::io;
use rand::Rng;

fn main() {
    println!("How many characters do you want generated?");
    let mut characters = String::new();
    let _standard =  vec![15, 14, 13, 12, 10, 8];

    io::stdin()
        .read_line(&mut characters)
        .unwrap_or_else(|error|panic!("problem getting input: {}", error)); 

    let characters = characters.trim()
        .parse()
        .unwrap_or_else(|error|panic!("problem getting input: {}", error));

    for _ in 0..characters {
        println!("{:?}", generate_character())
    }
    
}

fn generate_score() -> i8{
    struct Roll{
    roll: i8, 
    index: usize
}

    let mut rolls = vec![]; 
    for _ in 0..4 {
        rolls.push(rand::thread_rng().gen_range(1..=6));
    }

    let mut lowest_role = 
    Roll {
        roll: 20, 
        index: 0
    };
    
    for i in 0..rolls.len(){
        if rolls[i] < lowest_role.roll {
            lowest_role.roll = rolls[i]; 
            lowest_role.index = i;
        }
    }
    rolls.remove(lowest_role.index);

    rolls.iter().sum()
}

fn generate_character() -> Vec<i8>{
    let mut scores = vec![];
    for _ in 0..6 {
        scores.push(generate_score());
    }
    scores.sort();
    scores.reverse();
    scores
}
