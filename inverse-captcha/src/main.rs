use std::env;

fn main() {
  let args:Vec<String> = env::args().collect();
  let answer:u32 = resolve_captcha(&args[1]);
  println!("{}", answer);
}

fn resolve_captcha(puzzle:&String) -> u32 {
  let chars: Vec<char> = puzzle.chars().collect();
  let mut sum:u32 = 0;
  for i in 0..chars.len() {
    let next = if (i + 1) == (chars.len()) {0} else {i + 1};
    if chars[next] == chars[i] {
      sum += chars[i].to_digit(10).unwrap();
    }
  }
  sum
}

#[test]
fn it_resolves_sum () {
  let sum = resolve_captcha(&String::from("1122"));
  assert_eq!(sum, 3);
  let sum = resolve_captcha(&String::from("1111"));
  assert_eq!(sum, 4);
  let sum = resolve_captcha(&String::from("91212129"));
  assert_eq!(sum, 9);
}

#[test]
fn it_returns_zero_if_no_digit_matches_next () {
  let sum = resolve_captcha(&String::from("1234"));
  assert_eq!(sum, 0);
}
