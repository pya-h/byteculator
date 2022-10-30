/*       ***************************************
 *                                         *
 *  APP           : ByteCulator                 *
 *    Programmer    : pya.h  [fr3E.k!]              *
 *    Last Update   : 2022/10/30 Sun.  02:00 PM   *
 *                                                     *
**********************************************************/
extern crate colored;

use colored::ColoredString;
use colored::Colorize;
use std::io;

const PADDING: &str = "\t \t \t";

fn txtstylish(text: &String, color: &str, style: char) -> ColoredString {
  let mut txt = format!("{}", text).white();
  txt = match color {
      "blue" => txt.blue(),
      "green" => txt.green(),
      "yellow" => txt.yellow(),
      "purple" => txt.purple(),
      _ => txt,
  };

  return match style {
      'b' => txt.bold(),
      'i' => txt.italic(),
      'u' => txt.underline(),
      '-' => txt.strikethrough(),
      _ => txt,
  };
}
// EDIT THIS WHEN WORKING WHIT FLOATING POINTS
fn rem_redundant(str_num: &mut String) {
  if str_num.len() > 1 {
      let fst_digit_idx = if str_num.chars().nth(0) == Some('-') {
          1
      } else {
          0
      };
      while fst_digit_idx < str_num.len() && str_num.chars().nth(fst_digit_idx) == Some('0') {
          str_num.remove(fst_digit_idx);
      }
  }
}

fn add(op1: String, op2: String) -> String {
  let mut fstop = op1.clone();
  let mut sndop = op2.clone();
  let fstop_negative = fstop.chars().nth(0) == Some('-');
  let sndop_negative = sndop.chars().nth(0) == Some('-');

  if fstop_negative {
      fstop.remove(0);
      if !sndop_negative {
          return sub(sndop, fstop);
      }
      sndop.remove(0);
  } else if sndop_negative {
      sndop.remove(0);
      return sub(fstop, sndop);
  }

  let mut r = String::new();

  let y;
  let x = if fstop.len() <= sndop.len() {
      y = fstop.as_bytes();
      sndop.as_bytes()
  } else {
      y = sndop.as_bytes();
      fstop.as_bytes()
  };

  let mut i = x.len();
  let mut j = y.len();
  let offset: u8 = '0' as u8;
  let mut carry = 0;
  let mut digit: u8;

  while j > 0 {
      i -= 1;
      j -= 1;

      digit = carry + x[i] - offset + y[j] - offset;
      carry = digit / 10;
      digit %= 10;
      r = format!("{}{}", digit, r);
  }
  while i > 0 {
      i -= 1;

      digit = carry + x[i] - offset;
      carry = digit / 10;
      digit %= 10;
      r = format!("{}{}", digit, r);
  }
  if carry > 0 {
      r = format!("{}{}", carry, r);
  }
  // find the best and fastest way tp concat atrings
  rem_redundant(&mut r);
  if fstop_negative && sndop_negative {
      format!("-{}", r)
  } else {
      r
  }
}

fn sub(op1: String, op2: String) -> String {
  let mut fstop = op1.clone();
  let mut sndop = op2.clone();

  let fstop_negative = fstop.chars().nth(0) == Some('-');
  let sndop_negative = sndop.chars().nth(0) == Some('-');

  if fstop_negative {
      fstop.remove(0);
      if !sndop_negative {
          return format!("-{}", add(fstop, sndop));
      }
      sndop.remove(0);
      let temp = fstop;
      fstop = sndop;
      sndop = temp;
  } else if sndop_negative {
      sndop.remove(0);
      return add(fstop, sndop);
  }

  let mut r = String::new();
  let mut negative = false;
  let x;
  let y;
  if fstop.len() < sndop.len() || (fstop.len() == sndop.len() && fstop <= sndop) {
      negative = true;
      x = sndop.as_bytes(); //.map(|digit| digit - ('0' as u8))
      y = fstop.as_bytes();
  } else {
      y = sndop.as_bytes(); //.map(|digit| digit - ('0' as u8))
      x = fstop.as_bytes();
  }
  let mut i = x.len();
  let mut j = y.len();

  let mut borrow = 0;
  while j > 0 {
      i -= 1;
      j -= 1;

      let mut digit = x[i] as i8 - y[j] as i8 - borrow;
      borrow = 0;
      while digit < 0 {
          borrow += 1;
          digit += 10;
      }
      r = format!("{}{}", digit, r);
  }

  while i > 0 {
      i -= 1;

      let mut digit = x[i] as i8 - borrow;
      borrow = 0;
      while digit < 0 {
          borrow += 1;
          digit += 10;
      }
      r = format!("{}{}", digit - ('0' as i8), r);
  }
  rem_redundant(&mut r);

  // find the best and fastest way tp concat atrings
  if !negative {
      r
  } else {
      format!("-{}", r)
  }
}

fn mul(op1: String, op2: String, logidx: u64) -> String {
  let mut r = String::new();
  let mut fstop = op1.clone();
  let mut sndop = op2.clone();

  let fstop_sign: i8 = if fstop.chars().nth(0) == Some('-') {
      -1
  } else {
      1
  };
  let sndop_sign: i8 = if sndop.chars().nth(0) == Some('-') {
      -1
  } else {
      1
  };

  if fstop_sign == -1 {
      fstop.remove(0);
  }
  if sndop_sign == -1 {
      sndop.remove(0);
  }
  let y;
  let x = if fstop.len() <= sndop.len() {
      y = fstop.as_bytes();
      sndop.as_bytes()
  } else {
      y = sndop.as_bytes();
      fstop.as_bytes()
  };

  let ascii_offset = '0' as u8;
  let mut offset_zeros = String::new();
  let mut carry = 0;

  if logidx > 0 {
      println!(
          " = #{}\n{}--------------------------------- {} -----------------------------",
          logidx,
          PADDING,
          txtstylish(&format!("LOG #{}", logidx), "yellow", 'i')
      );
      print!(
          "{}  {}\n{}{}\n",
          format!("{}{}", PADDING, " ".repeat(y.len() + 1)),
          txtstylish(&op1, "yellow", 'i'),
          format!("{}{}", PADDING, " ".repeat(y.len() + 1)),
          txtstylish(
              &format!("x {}{}", " ".repeat(x.len() - y.len()), op2),
              "yellow",
              'u'
          )
      );
  }

  // NOTE: REMOVE ZERO DIGIT MULTIPLY
  let mut log_row_padding = y.len();
  for dy in y.iter().rev() {
      let dy = dy - ascii_offset;
      if dy != 0 {
          let mut row = offset_zeros.clone();
          for dx in x.iter().rev() {
              let dx = dx - ascii_offset;
              let mut digit: u8 = dx * dy + carry;
              carry = digit / 10;
              digit %= 10;
              row = format!("{}{}", digit, row);
          }
          if carry > 0 {
              row = format!("{}{}", carry, row);
          }
          if logidx > 0 {
              println!(
                  "{} {}{} {}",
                  PADDING,
                  if offset_zeros.len() > 0 { "+" } else { " " },
                  {
                      log_row_padding -= 1;
                      " ".repeat(log_row_padding)
                  },
                  row
              );
          }
          r = add(r, row);
      }
      offset_zeros.push('0');
  }
  if logidx > 0 {
      println!(
          "\n{}--------------------------------- {} -----------------------------",
          PADDING,
          txtstylish(&format!("#{} END", logidx), "yellow", 'i')
      );
  }
  if fstop_sign * sndop_sign == -1 {
      format!("-{}", r)
  } else {
      r
  }
}

fn pow(base: String, power: String, logidx: u64) -> String {
  let mut result = format!("1");
  let length_long = base.len() + power.len() >= 30;

  let temp_power: u64 = power.parse().unwrap();

  if logidx > 0 {
      println!(
          " = #{}\n{}--------------------------------- {} -----------------------------",
          logidx,
          PADDING,
          txtstylish(&format!("LOG #{}", logidx), "yellow", 'i')
      );
  }
  for i in 0..temp_power {
      if logidx > 0 {
          print!("{} {} ^ {}", PADDING, base, i + 1);
      }
      result = mul(result, base.clone(), 0);
      if logidx > 0 {
          println!(
              "{} = {}{}",
              if length_long { "\n\t" } else { "" },
              txtstylish(&result, "blue", 'b'),
              if length_long { "\n" } else { "" }
          );
      }
  }

  if logidx > 0 {
      println!(
          "\n{}--------------------------- {} -----------------------------",
          PADDING,
          txtstylish(&format!("#{} END", logidx), "yellow", 'i')
      );
  }

  result
}

fn app(expression: String, mut log: bool, mut fullog: bool) -> [bool; 2] {
  // fullog = false;
  let ops = expression.split_whitespace().map(str::to_string);
  let mut logidx: u64 = 0;
  let mut result = String::new();
  let mut operator = String::new();
  if log {
      println!("\n--------------------------------------------- LOG --------------------------------------------");
  }
  for mut x in ops {
      if x == "+l" {
          log = true;
      } else if x == "-l" {
          log = false;
          fullog = false;
      } else if x == "+f" {
          fullog = true;
          log = true;
      } else if x == "-f" {
          fullog = false;
      } else {
          rem_redundant(&mut x);
          if !result.is_empty() {
              if !operator.is_empty() {
                  let length_long = result.len() + x.len() >= 30;
                  if log {
                      print!("{} {} {} {}", PADDING, result, operator, x);
                  }

                  result = match operator.as_str() {
                      "+" => add(result, x),
                      "-" => sub(result, x),
                      "*" | "x" => mul(
                          result,
                          x,
                          if fullog {
                              logidx += 1;
                              logidx
                          } else {
                              0
                          },
                      ),
                      "^" => pow(
                          result,
                          x,
                          if fullog {
                              logidx += 1;
                              logidx
                          } else {
                              0
                          },
                      ),
                      _ => break,
                  };

                  if log {
                      println!(
                          "{} = {}{}",
                          if length_long { "\n\t" } else { "" },
                          txtstylish(&result, "blue", 'b'),
                          if length_long { "\n" } else { "" }
                      );
                  }
                  operator = String::new();
              } else {
                  operator = x;
              }
          } else {
              result = x;
          }
      }
  }
  println!("\n--------------------------------------------- finally --------------------------------------------");
  println!("    = {}", txtstylish(&result, "purple", 'b'));
  return [log, fullog];
}
fn main() {
  println!("byteculator v0.1.0: calculator for large numbers \n \t Enter the expression: \n");
  let mut log = true;
  let mut fullog = false;
  loop {
      let mut expression = String::new();
      io::stdin()
          .read_line(&mut expression)
          .expect("Wrong Expression");
      match expression.as_str().trim() {
          "x" | "" => break,
          _ => {
              let params_status = app(
                  expression.clone(),
                  log && (expression.chars().nth(0) != Some('-')
                      || expression.chars().nth(1) != Some('l')),
                  fullog,
              );
              fullog = params_status[1];
              log = params_status[0];
          }
      }
  }
}
