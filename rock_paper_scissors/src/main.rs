use rand::Rng;
use std::io;

fn main() {
    let user_weapon = get_user_weapon();
    let user_index = get_index(&user_weapon);

    println!("The user's weapon is {}", user_weapon);

    let rng = rand::thread_rng().gen_range(0..2);
    let weapon_chamber = ["rock", "paper", "scissors"];
    let comp_weapon = weapon_chamber[rng];
    println!("The computer's weapon is {}", &comp_weapon);

    if rng == user_index {
        return println!("It's a tie.");
    }
}

fn get_user_weapon() -> String {
    loop {
        let mut user_weapon = String::new();

        println!("Enter your weapon of choice.");
        io::stdin()
            .read_line(&mut user_weapon)
            .expect("Something went wrong.");
        user_weapon = user_weapon.trim().to_lowercase();

        if is_valid_input(&user_weapon) {
            return user_weapon;
        }
        println!("That's not a proper weapon. Try again.")
    }
}

fn is_valid_input(weapon: &String) -> bool {
    if weapon == "rock" {
        return true;
    }
    if weapon == "paper" {
        return true;
    }
    if weapon == "scissors" {
        return true;
    }
    false
}

fn get_index(weapon: &String) -> usize {
    if weapon == "rock" {
        return 0;
    }
    if weapon == "paper" {
        return 1;
    }
    2
}
