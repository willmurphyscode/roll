#![feature(use_extern_macros)]
extern crate rand; 
#[macro_use]
extern crate nom; 

use std::str::{FromStr};

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

named!(d_tag<&[u8]>, tag!("d"));

named!(dice_notation_bytes<(&[u8],&[u8])>, 
    do_parse!(
        quantity: digit >>
        d: d_tag >>
        faces: digit >>
        (quantity, faces)
));

use nom::digit;


fn parse_dice_spec(s: String) -> Result<DiceSpec,()> {
    let slice : &[u8] = s.as_bytes();
    let value = dice_notation_bytes(slice);
    println!("{:?}", value);
    match value {
        nom::IResult::Done(_, (dice, faces)) => println!("parsed: {:?} {:?}", dice, faces),
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
