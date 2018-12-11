use std::env;
use std::str::FromStr;
extern crate clap;
extern crate regex;
extern crate rand; 

use clap::{Arg, App};
use regex::Regex;
use rand::Rng;

fn main() {
    // First, set required and optional parameters and a hep message
    // TODO: Allow an infinite number of dice but only 1 modifier
    App::new("Rget")
        .author("Alexandre Léonardi <leonardialexandre@free.fr>")
        .about("A simple command line dice roller (one day with pixel art...)")
        .arg(Arg::with_name("dice")
            .required(true)
            .takes_value(true)
            .multiple(true)
            .help("List of dice to roll in the form nDm (n m-sided dice, eg 3D10)"))
        .arg(Arg::with_name("modificator")
            .required(false)
            .takes_value(true)
            .last(true)
            .help("Optional modificator in the form p/mx where x is a number (eg p5 for +5 or m5 for -5, as in 3D10 +5)"))
        .get_matches();

    // We need 1 regexo to catch the dice format nDm and 1 for the modifier +/-p
    let die_re = Regex::new(r"^(\d+)[Dd](\d+)$").unwrap();
    let modificator_re = Regex::new(r"^([pm])(\d+)$").unwrap();

    // Convert arguments to a vector...
    let mut arguments: Vec<_> = env::args().collect();
    //... and remove the first element (the program's name)
    arguments.remove(0);

    // Result of the roll
    let mut result = 0;

    // Variable used to know if a modifier has been used already :
    // it must be the last argument of the command line
    let mut has_modifier_been_used = false;

    // For each argument of the program
    for arg in &arguments {
        // first we check if no modifier has already been used
        if has_modifier_been_used {
            panic!("No argument can be used after a modifier (use --help for details)");
        // then we check if this argument is a die (ie in the form 3d10), and if so we roll the concerned dice and add the result
        } else if die_re.is_match(arg) {
            for die in die_re.captures_iter(arg) {
                let nb = FromStr::from_str(&die[1]).unwrap();
                let sides = FromStr::from_str(&die[2]).unwrap();
                result += roll_dice(nb, sides);
            }
        // otherwise, we check if the argument is a modifier, and if so, we add/substract it to the total
        } else if modificator_re.is_match(arg) {
            has_modifier_been_used = true;
            for part in modificator_re.captures_iter(arg) {
                let symbol = &part[1];
                let n: i32 = FromStr::from_str(&part[2]).unwrap();
                if symbol == "p" {
                    result += n;
                } else if symbol == "m" {
                    result -= n;
                }
            }
        // finally if the argument is nothing of the above, it is of an unrecognized format
        } else {
            panic!("Argument {} is neither a die (eg 2D6) nor a modifier (eg m5 for -5)", arg);
        }
    }
    println!("result: {}", result);
}


fn roll_dice(number: i32, sides: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let mut result = 0;
    for _i in 0..number {
        let n = rng.gen_range(1, sides+1);
        println!("{}", n);
        result += n;
    }
    return result;
}