use std::env;

pub fn echo() { let arg : Vec<String> = env::args().collect();
  println!(".intel_syntax noprefix\n");
  println!(".globl main\n");
  println!("main:\n");
  println!("  mov rax, {}\n", &arg[1].parse::<i32>().unwrap() );
  println!("  ret\n");
}

