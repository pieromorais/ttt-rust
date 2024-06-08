#![allow(unused)]

use std::{io, string, vec};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Game {
    player1_score: usize,
    player2_score: usize,
    matrix_hash: [i8;9],
    turn_owner: i8, // player can be 0 or 1
    round: usize

}

impl Game {

    fn new() -> Self{
        Self {
            player1_score: 0,
            player2_score: 0,
            matrix_hash: [-5;9],
            turn_owner: 0,
            round: 1
       }
    }
    fn print_grid(self) {
        for i in 0..9 {
            if i == 2 || i == 5 || i == 8 {
                print!("{}", (i+1));
                println!("");
            }else {;
                print!("{} ", (i+1));
            };
        }
    }

    fn print_hash(self) {

        let options_mapped = HashMap::from([
            (0, "O"),
            (1, "X"),
            (-5, "P")
        ]);


        for i in 0..9 {
            if i == 2 || i == 5 || i == 8 {
                print!("{}", options_mapped[&self.matrix_hash[i]]);
                println!("");
            }else {;
                print!("{} ", options_mapped[&self.matrix_hash[i]]);
            };
        }
    }

    fn check(&mut self) -> bool{
        self.round += 1;
        // check rows
        for i in 0..3 {
            if (self.matrix_hash[3*i] + self.matrix_hash[3*i+1] + self.matrix_hash[3*i+2]) == 0 || (self.matrix_hash[3*i] + self.matrix_hash[3*i+1] + self.matrix_hash[3*i+2]) == 3{
                println!("player {} won!", self.turn_owner + 1);
                return true;
            }; 

            if (self.matrix_hash[i] + self.matrix_hash[i+3] + self.matrix_hash[i+6]) == 0 || (self.matrix_hash[i] + self.matrix_hash[i+3] + self.matrix_hash[i+6]) == 3 {
                println!("player {} won!", self.turn_owner + 1);
                return true;
            };

            if (self.matrix_hash[0] + self.matrix_hash[4] + self.matrix_hash[8]) == 0 || (self.matrix_hash[0] + self.matrix_hash[4] + self.matrix_hash[8]) == 3 {
                println!("player {} won!", self.turn_owner + 1);
                return true;
            };
            if (self.matrix_hash[6] + self.matrix_hash[4] + self.matrix_hash[2]) == 0 || (self.matrix_hash[6] + self.matrix_hash[4] + self.matrix_hash[2]) == 3  {
                println!("player {} won!", self.turn_owner + 1);
                return true;
            };
            if self.round == 9{
                println!("Draw!");
                return true;
            }
        } 
        false
    }      

    fn pick_your_spot(&mut self) {

        let mut ps : String = String::new();
        println!("Pick your position");        
        
        loop {

            io::stdin().read_line(&mut ps);
            
            let p : usize = ps.trim().parse().expect("Error parsing from string to usize");
            if p < 1 || p > 9 || self.matrix_hash[p-1] != -5{
                println!("number not allowed");
                continue;
            }else {
                self.matrix_hash[p-1] = self.turn_owner;
                break;
            }
        }

    }

    fn change_turn(&mut self) {
        self.turn_owner ^= 1;
    }
}

fn main() {

    let mut game : Game = Game::new();

    println!("Welcome to ttt!");

    loop {
        println!("Turn: player {}", game.turn_owner + 1);
        game.print_grid();
        game.pick_your_spot();
        game.print_hash();
        
        if game.check() {
            break;
        }

        game.change_turn();
    }

   // game.print_hash();
   // game.check();
   // game.pick_your_spot();
   // game.print_hash();
   // game.pick_your_spot();
   // game.print_hash();
}





// 0 1 2
// 3 4 5
// 6 7 8
