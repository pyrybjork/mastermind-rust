use std::io;
use std::io::Write;
use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "--help"  {
            help();
            return;
        }
        if args[1] == "-h"  {
            help();
        }
    }

    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut solution = [0u8; 4];

    for i in 0..4 {
        solution[i] = numbers.remove(rng.gen_range(0..numbers.len()))
    }

    let mut board = [[0u8; 4]; 9];
    let mut correct_num = [0u8; 9];
    let mut correct_place = [0u8; 9];
    let mut new_row: [u8; 4];

    for i in 0..9 {
        print_board(board, correct_num, correct_place);
        new_row = get_new_valid_row();
        board[i] = new_row.clone();
        correct_num[i] = calculate_correct_nums(&solution, &new_row); 
        correct_place[i] = calculate_correct_places(&solution, &new_row);

        if new_row == solution {
            println!("You cracked the code!");
            break;
        } else if i == 8 {
            println!("You ran out of tries. The correct solution was {:?}", solution);
        }
    }
    
}

fn help() {
    println!("Mastermind is a game where you try to solve a 4 digit code consisting of numbers from 1 to 8. The code cannot have duplicates.");
    println!("After each guess you are told how many of the numbers in your guess was correct numbers, but in the wrong places and how many of the numbers were exact match for the correct solution.");
}

fn calculate_correct_nums(solution: &[u8; 4], new_row: &[u8; 4]) -> u8 {
    let mut correct_nums: u8 = 0;
    for i in 0..4 {
        if solution.contains(&new_row[i]) && solution[i] != new_row[i] {
            correct_nums += 1;
        }
    }
    correct_nums
}

fn calculate_correct_places(solution: &[u8; 4], new_row: &[u8; 4]) -> u8 {
    let mut correct_places: u8 = 0;
    for i in 0..4 {
        if solution[i] == new_row[i] {
            correct_places += 1;
        }
    }
    correct_places
}

fn get_new_valid_row() -> [u8; 4] {
    let mut row_out: [u8; 4];
    'outer: loop {
        row_out = [0; 4];
        let input: String = get_input("new line : ");
        if input.split(',').count() >= 5{
            continue;
        }
        for (i, x) in input.split(',').enumerate() {
            let trimmed: String = x.split_whitespace().collect();
            row_out[i] = trimmed.parse().unwrap_or_else(|_| 0);
        }
        if !row_out.contains(&0u8) {
            for i in row_out {
                if i > 8 {
                    println!("Invalid input. Only use numbers from 1 to 8");
                    continue 'outer;
                }
                if row_out.iter().filter(|&n| *n == i).count() >= 2 {
                    println!("Invalid input. You cant have duplicates");
                    continue 'outer;
                }
            }
            break;
        }
        println!("Invalid input. Use format 1, 2, 3, 4 with numbers from 1 to 8");
    }
    row_out
}

fn print_board(board: [[u8; 4]; 9], correct_num: [u8; 9], correct_place: [u8; 9]) {
    for i in 0..9 {
        if i == 0 {
            println!("\ncorrect {:?} {:?} {:?} correct", correct_num[i], board[i], correct_place[i]);
        } else if i == 1 {
            println!(" number {:?} {:?} {:?} place", correct_num[i], board[i], correct_place[i]);
        } else {
            println!("        {:?} {:?} {:?}", correct_num[i], board[i], correct_place[i]);
        }
    }
}

fn get_input(prompt: &str) -> String{
    print!("{}",prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}
