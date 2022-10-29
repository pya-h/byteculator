/*       ***************************************
	*													               *
	*	APP           : ByteCulator		   			    *
	*		Programmer    : pya.h	[fr3E.k!]						  *
 *		Last Update   : 2012/12/29 Sat.	04 : 17 PM	 *
 *                                                     *
**********************************************************/

use colored::Colorize;
use std::io;

const PADDING: &str = "\t \t \t";
// EDIT THIS WHEN WORKING WHIT FLOATING POINTS
fn rem_redundant(mut str_num: String) -> String {
    let fst_digit_idx = if str_num.chars().nth(0) == Some('-') {
        1
    } else {
        0
    };
    while fst_digit_idx < str_num.len() && str_num.chars().nth(fst_digit_idx) == Some('0') {
        str_num.remove(fst_digit_idx);
    }
    return str_num;
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
    let mut digit;

    while j > 0 {
        i -= 1;
        j -= 1;

        digit = carry + x[i] - offset + y[j] - offset;
        carry = 0;
        while digit >= 10 {
            carry += 1;
            digit -= 10;
        }
        r = format!("{}{}", digit, r);
    }
    while i > 0 {
        i -= 1;

        digit = carry + x[i] - offset;
        carry = 0;
        while digit >= 10 {
            carry += 1;
            digit -= 10;
        }
        r = format!("{}{}", digit, r);
    }
    if carry > 0 {
        r = format!("{}{}", carry, r);
    }
    // find the best and fastest way tp concat atrings
    r = rem_redundant(r);
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
    r = rem_redundant(r);

    // find the best and fastest way tp concat atrings
    if !negative {
        r
    } else {
        format!("-{}", r)
    }
}

fn mul(op1: String, op2: String) -> String {
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

    for dy in y.iter().rev() {
        let dy = dy - ascii_offset;
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
        r = add(r, row);
        offset_zeros.push('0');
    }
    if fstop_sign * sndop_sign == -1 {
        format!("-{}", r)
    } else {
        r
    }
}

fn pow(base: String, power: String, log: bool) -> String {
    let mut result = format!("1");
    let length_long = base.len() + power.len() >= 30;

    let temp_power: u64 = power.parse().unwrap();

    if log {
        println!(
            "\n{}--------------------------------- {} -----------------------------",
            PADDING,
            format!("{} ^ {}", base, power).yellow().italic()
        );
    }
    for i in 0..temp_power {
        if log {
            print!("{} {} {} {}", PADDING, base, "^", i + 1);
        }
        result = mul(result, base.clone());
        if log {
            println!(
                "{} = {}{}",
                if length_long { "\n\t" } else { "" },
                format!("{}", result).bold().blue(),
                if length_long { "\n" } else { "" }
            );
        }
    }

    if log {
        println!(
            "\n{}--------------------------- {} -----------------------------",
            PADDING,
            format!("END OF {} ^ {}", base, power).yellow().italic()
        );
    }

    result
}
fn main() {
    println!("bigcalc v1.0.0: calculator for large numbers \n \t Enter the expression: \n");
    let mut expression = String::new();

    io::stdin()
        .read_line(&mut expression)
        .expect("Wrong Expression");

    let ops = expression.split_whitespace().map(str::to_string);
    let mut fullog = false;
    let mut result = String::new();
    let mut operator = String::new();
    println!("\n--------------------------------------------- LOG --------------------------------------------");
    for x in ops {
        if x == "+fl" {
            fullog = true;
        } else if x == "-fl" {
            fullog == false;
        } else {
            let x = rem_redundant(x);
            if !result.is_empty() {
                if !operator.is_empty() {
                    let length_long = result.len() + x.len() >= 30;
                    print!("{} {} {} {}", PADDING, result, operator, x);
                    result = match operator.as_str() {
                        "+" => add(result, x),
                        "-" => sub(result, x),
                        "*" => mul(result, x),
                        "^" => pow(result, x, fullog),
                        _ => String::new(),
                    };

                    println!(
                        "{} = {}{}",
                        if length_long { "\n\t" } else { "" },
                        format!("{}", result).bold().blue(),
                        if length_long { "\n" } else { "" }
                    );
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
    println!("    = {}", format!("{}", result).purple().bold());
}
