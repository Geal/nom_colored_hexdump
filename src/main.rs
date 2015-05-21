#[macro_use] extern crate nom;

use nom::IResult;
use nom::util::{generate_colors,prepare_errors,print_codes,print_offsets};
use std::collections::HashMap;

fn main() {
  named!(err_test, alt!(
    tag!("abcd") |
    error!(12,
      preceded!(tag!("efgh"), error!(42,
          chain!(
                 tag!("ijk")              ~
            res: error!(128, tag!("mnop")) ,
            || { res }
          )
        )
      )
    )
  ));
  let a = &b"efghblah"[..];
  let b = &b"efghijklblahblah"[..];

  let res_a = err_test(a);
  let res_b = err_test(b);

  display_error(a, res_a);
  display_error(b, res_b);
}

pub fn display_error<I,O>(input: &[u8], res: IResult<I,O>) {
  let mut h: HashMap<u32, &str> = HashMap::new();
  h.insert(12, "preceded");
  h.insert(42, "chain");
  h.insert(128, "tag mnop");
  h.insert(0, "tag");

  if let Some(v) = prepare_errors(input, res) {
    let colors = generate_colors(&v);
    println!("parsers: {}", print_codes(colors, h));
    println!("{}",   print_offsets(input, 0, &v));

  } else {
    println!("not an error");
  }
}
