use rand::Rng;
use std::io;

fn main() {
    play();
}
fn dice() -> i32 {
    println!("Rolling the Dice!");

    let roll_dice1 = rand::thread_rng().gen_range(1..=6);
    let roll_dice2 = rand::thread_rng().gen_range(1..=6);

    println!("The rolled dice are : {roll_dice1}, and {roll_dice2}");
    return roll_dice1 + roll_dice2;
}
fn play() {
    let mut sum_dice = dice();
    let mut points = 0;
    loop {
        if sum_dice == 7 || sum_dice == 11 {
            println!("You Win!");
            println!("Your Total points {points}");
            break;
        } else if sum_dice == 2 || sum_dice == 3 || sum_dice == 12 {
            println!("You Lose");
            println!("Your Total points {points}");
            break;
        } else {
            points += sum_dice;
            println!("Your Total points {points}");
            println!("Want to play again? (y/n)");
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read your input, please enter y/n in small letter");

            let user_input = user_input.trim();
            if user_input == "y" {
                sum_dice = dice();
            } else {
                println!("Game Over! Your Total Points {points}");
                break;
            }
        }
    }
}
