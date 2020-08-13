use std::{
  env,
  slice,
  iter
};


pub fn echo() {
  let arg : Vec<String> = env::args().collect();
  println!(".intel_syntax noprefix\n");
  println!(".globl main\n");
  println!("main:\n");
  println!("  mov rax, {}\n", &arg[1].parse::<i32>().unwrap() );
  println!("  ret\n");
}

pub fn twop() {
  let arg : Vec<char> = env::args().collect();

  let mut read : std::slice::Iter<char> = arg.iter();

  println!(".intel_syntax noprefix\n");
  println!(".globl main\n");
  println!("main:\n");
  println!("  mov rax, {}\n", &arg[1].parse::<i32>().unwrap() );

  for arg.len() in _ {

  if &read == ''

  }

  println!("  ret\n");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_echo() {
    assert_eq!();
  }
}