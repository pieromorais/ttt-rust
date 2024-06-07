#![allow(unused)]

use std::{string, vec};
use std::collections::HashMap;

//#[derive(Debug)]

//struct Game {
//    field: Type
//
//}
//
//impl Game {
//    
//}

fn main() {
    
    let mut game_choices : [i32; 9] = [-1; 9];
    println!("Tic tac toe!");
    print_hash(&mut game_choices);

}

fn print_hash(choices : &mut [i32;9]) {

    let options_mapped = HashMap::from([
        (1, "O"),
        (2, "X")
    ]);


    for i in 0..9 {
        if i == 2 || i == 5 || i == 8 {
            print!("{}", choices[i]);
            println!("");
        }else {;
            print!("{} ", choices[i]);
        };
    }
}

fn check(game : &mut [i32;9]) {
    // check rows
    for i in 0..3 {
        if (game[3*i] + game[3*i+1] + game[3*i+2]) == 000 {
            println!("player");
        }; 
    } 
}

// 0 1 2
// 3 4 5
// 6 7 8
