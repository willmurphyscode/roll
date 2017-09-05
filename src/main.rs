#![feature(use_extern_macros)]
extern crate rand; 
#[macro_use]
extern crate nom; 

use std::str::{FromStr};
use rand::distributions::{IndependentSample, Range};

#[derive(Debug, PartialEq)]
struct DiceSpec {
    quantity: usize,
    faces: usize,
    bonus: i32
}

impl DiceSpec {
    pub fn new(quantity: usize, faces: usize, bonus: i32) -> DiceSpec {
        DiceSpec {
            quantity,
            faces,
            bonus
        }
    }
    pub fn roll(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let between = Range::new(1i32, self.faces as i32 + 1);
        let mut result = self.bonus; 
        for _ in 0..self.quantity {
            let die = between.ind_sample(&mut rng);
            result = result + die; 
        }
        result
    }
}

named!(parse_int<usize>, 
    map_res!(
      map_res!(
        ws!(nom::digit),
        std::str::from_utf8
      ),
      FromStr::from_str
    )
);

named!(sign<&str>, 
    map_res!(
            alt!(tag!("+") | tag!("-")), 
            std::str::from_utf8));

named!(d_tag<&[u8]>, tag!("d"));

named!(bonus<i32>, 
    do_parse!(
        sign: ws!(sign) >>
        value: parse_int >>
        (
            match sign {
                "-" => (-1 * (value as i32)),
                "+" => (value as i32),
                _ => 0i32
            }
        )
    ));

named!(int_or_1<usize>, 
    do_parse!(
        val: opt!(parse_int) >>
        (
            match val {
                Some(value) => value,
                None => 1usize
            }
        )
));

named!(dice_notation_bytes<DiceSpec>, 
    do_parse!(
        quantity: int_or_1 >>
        d_tag >>
        faces: parse_int >>
        bonus: opt!(complete!(bonus)) >>
        ( 
            match bonus {
                Some(b) => DiceSpec::new(quantity, faces, b),
                None => DiceSpec::new(quantity, faces, 0)  
            }   
        )
));

fn parse_dice_spec(s: String) -> Result<DiceSpec,()> {
    let slice : &[u8] = s.as_bytes();
    let value = dice_notation_bytes(slice);
    match value {
        nom::IResult::Done(_, dice) => Ok(dice),
        nom::IResult::Incomplete(_) => Err(()),
        nom::IResult::Error(_) => Err(()),
    }
}

fn main() {
    let args = std::env::args();
    let mut argvec : Vec<String> = Vec::new(); 
    if args.len() > 1 {
        for arg in args.skip(1) {
            argvec.push(arg);
        }
        let arg_string = argvec.concat();
        let dice_result = parse_dice_spec(arg_string.clone());
        match dice_result {
            Ok(dice) => {
                let result = dice.roll();
                println!("Rolled {} and got {}", arg_string, result);
            },
            Err(_) => println!("Could not parse: {}.", arg_string)
        }
    }
}

#[test]
fn it_parses_dice_notation_just_the_dice() {
    let expected : nom::IResult<&[u8], DiceSpec> = 
        nom::IResult::Done(&b""[..], DiceSpec::new(3, 6, 0));
    let input = "3d6";
    assert_eq!(expected, dice_notation_bytes(input.as_bytes()));
}


#[test]
fn it_parses_dice_notation_with_bonus() {
    let expected : nom::IResult<&[u8], DiceSpec> = 
        nom::IResult::Done(&b""[..], DiceSpec::new(3, 6, 4)); 
    let input = "3d6 + 4"; 
    assert_eq!(expected, dice_notation_bytes(input.as_bytes()));
}

#[test]
fn it_parses_dice_notation_with_penalty() {
    let expected : nom::IResult<&[u8], DiceSpec> = 
        nom::IResult::Done(&b""[..], DiceSpec::new(3, 6, -4)); 
    let input = "3d6 - 4"; 
    assert_eq!(expected, dice_notation_bytes(input.as_bytes()));
}