use core::num;
use std::{io::{self, Read}, num::ParseIntError};

//Create an enum with variants for all possible commands
enum Commands {
    Invert, // -
    PushPop, // ;
    Split, // %
    NextHead, // >
    Output, // !
    StartL(Option<usize>), // [
    EndL(Option<usize>), // ]
    Reset, // #
    Pass // All other characters
}

//Create variables for the HYDRA tape, its pointer, and the code/data pointer
static mut HYDRA: Vec<String> = Vec::new();
static mut HP: usize = 0;
static mut DP: usize = 0;
fn main() {
    //Initialize HYDRA
    unsafe {
        HYDRA.push(String::from("1"));
    }
    //Get code input
    let mut code:String = String::new();
    io::stdin()
    .read_line(&mut code)
    .ok()
    .expect("Could not read code");
    //Convert code into an indexable vector of characters
    let code: Vec<char> = code.chars().collect::<Vec<char>>();
    //Initialize an enum vector that stores different command variants
    let mut cmds: Vec<Commands> = Vec::new();
    //Loop through code vector to add corresponding Commands to the cmds vector
    for c in 0..code.len() {
        cmds.push(match code.get(c).unwrap() {
            '-' => Commands::Invert,
            ';' => Commands::PushPop,
            '%' => Commands::Split,
            '>' => Commands::NextHead,
            '!' => Commands::Output,
            '[' => {
                //Create variables to count the number of loops and to store the end index of the ongoing loop
                let mut runningloop: Option<usize> = None;
                let mut loopcount: i32 = 0;
                //Loop through the code starting from the parent for loop's current index
                for i in c..code.len() {
                    //Increment, decrement, or leave alone the loopcount based on which character is being read
                    loopcount = match code.get(i).unwrap() {
                        '[' => loopcount + 1,
                        ']' => loopcount - 1,
                        _ => loopcount
                    };
                    //If the loopcount is 0, store the current index in runningloop and break the current for loop
                    if loopcount == 0 {
                        runningloop = Some(i);
                        break;
                    }
                }
                //Return a Commands enum that stores the index of its corresponding end
                Commands::StartL(runningloop)
            },
            ']' => {
                //See above match arm's comments for details
                let mut runningloop: Option<usize> = None;
                let mut loopcount: i32 = 0;
                for i in (0..=c).rev() {
                    loopcount = match code.get(i).unwrap() {
                        '[' => loopcount - 1,
                        ']' => loopcount + 1,
                        _ => loopcount
                    };

                    if loopcount == 0 {
                        runningloop = Some(i);
                        break;
                    }
                }
                Commands::EndL(runningloop)
            },
            '#' => Commands::Reset,
            _ => Commands::Pass
        });
    }

    drop(code);

    //Loop through commands enum and execute code accordingly
    unsafe {
        while DP > cmds.len() {
            match cmds.get(DP).unwrap() {
                Commands::Invert => {invert();},
                Commands::PushPop => {push_pop();},
                Commands::Split => {split();},
                Commands::NextHead => {next_head();},
                Commands::Output => {output();},
                Commands::StartL(ptr) | Commands::EndL(ptr) => {jump(&ptr);},
                Commands::Reset => {reset();},
                _ => {DP += 1;}
            }
        }
    }
}
//Replaces the first character of the current HYDRA head with its opposite counterpart (e.g. 1 -> 0 and 0 -> 1).
unsafe fn invert() {
    match HYDRA.get(HP).unwrap().chars().nth(0).unwrap() {
        '0' => {HYDRA[HP].replace_range(0..1, "1")},
        '1' => {HYDRA[HP].replace_range(0..1, "0")},
        _  => {panic!("Tape not in binary");}
    };
    DP += 1;
}
//If the first character of the current HYDRA head is 0, it gets removed. If it's 1, a 1 is appended to the end of the HYDRA head.
unsafe fn push_pop() {
    match HYDRA.get(HP).unwrap().chars().nth(0).unwrap() {
        '0' => {HYDRA[HP].remove(0);},
        '1' => {HYDRA[HP].push_str("1");},
        _ => {panic!("Tape not in binary");}
    }
    DP += 1;
}
//Creates a new HYDRA head that's a copy of the current one.
unsafe fn split() {
    HYDRA.push(HYDRA.get(HP).unwrap().clone());
    DP += 1;
}
//Moves the HYDRA pointer to the next head.
unsafe fn next_head() {
    if HP + 1 >= HYDRA.len() {
        HP = 0;
    } else {
        HP += 1;
    }
    DP += 1;
}
//Outputs the value at the current HYDRA head as a string 
unsafe fn output() {
    //Converts the current HYDRA head into a Result<Vec<u8>> by mapping chunks of 8 binary characters at a time into a u8 and collecting them all in a vector.
    let num_value: Result<Vec<u8>, ParseIntError> = (0..HYDRA.get(HP).unwrap().len())
        .step_by(8)
        .map(|x| u8::from_str_radix(&HYDRA.get(HP).unwrap()[x..x+7], 2))
        .collect();
    //Outputs the newly converted HYDRA head as a string from a utf8.
    print!("{:#?}", String::from_utf8(num_value.ok().unwrap()));
    DP += 1;
}
//Jumps to the position in code specified in the Command pointer if its value is not null
unsafe fn jump(pos: &Option<usize>) {
    if let Some(x) = *pos {
        DP = x;
    }
}
//Resets the current head to 1.
unsafe fn reset() {
    HYDRA[HP] = String::from("1");
}