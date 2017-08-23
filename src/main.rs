#![feature(use_extern_macros)]
extern crate rand; 
#[macro_use]
extern crate nom; 

use std::str::{FromStr};

#[derive(Debug, PartialEq)]
struct DiceSpec {
    quantity: usize,
    faces: usize,
    bonus: i32
}

impl DiceSpec {
    pub fn new(quantity: usize, faces: usize, bonus: i32) -> DiceSpec {
        DiceSpec {
            quantity: quantity,
            faces: faces,
            bonus
        }
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

named!(dice_notation_bytes<DiceSpec>, 
    do_parse!(
        quantity: parse_int >>
        d: d_tag >>
        faces: parse_int >>
        bonus: opt!(complete!(bonus)) >>
        ( 
            match bonus {
                Some(b) => DiceSpec::new(quantity, faces, b),
                None => DiceSpec::new(quantity, faces, 0)  
            }   
        )
));

use nom::digit;


fn parse_dice_spec(s: String) -> Result<DiceSpec,()> {
    let slice : &[u8] = s.as_bytes();
    let value = dice_notation_bytes(slice);
    println!("{:?}", value);
    match value {
        nom::IResult::Done(i, dice) => println!("parsed: ({:?} {:?})", i, dice),
        nom::IResult::Incomplete(_) => (),
        nom::IResult::Error(e) => println!("Error: {:?}", e),
    }
    Ok(DiceSpec::new(1, 6, 0))
}


fn main() {
    let args = std::env::args();
    if args.len() > 1 {
        let option_arg = args.last();
        match option_arg {
            Some(last_arg) => {
                parse_dice_spec(last_arg.clone());
            },
            None => ()
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