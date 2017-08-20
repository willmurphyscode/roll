#[macro_use]
extern crate clap;
extern crate rand; 
#[macro_use]
extern crate nom; 

use std::str::{from_utf8, FromStr};
use std::env::args;

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

named!(parse_int<i64>, 
    map_res!(
      map_res!(
        ws!(nom::digit),
        std::str::from_utf8
      ),
      FromStr::from_str
    )
);


fn parse_dice_spec(s: String) -> Result<DiceSpec,()> {
    Ok(DiceSpec::new(1, 6, 0))
}


fn main() {
    let args = std::env::args();
    if args.len() > 1 {
        let option_arg = args.last();
        match option_arg {
            Some(last_arg) => {
                let val = parse_int(last_arg.as_bytes());
                println!("{:?}", val);
            },
            None => ()
        }
        
    }
}
