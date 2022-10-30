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

// test this: 2 + +f 102 * 512 ^ 12 -f * 201 + 100 -l * 104165456565646546545645646 +l + 10
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

fn add(mut op1: String, mut op2: String) -> String {
    let op1_negative = op1.chars().nth(0) == Some('-');
    let op2_negative = op2.chars().nth(0) == Some('-');

    if op1_negative {
        op1.remove(0);
        if !op2_negative {
            return sub(op2, op1);
        }
        op2.remove(0);
    } else if op2_negative {
        op2.remove(0);
        return sub(op1, op2);
    }

    let mut r = String::new();

    let y;
    let x = if op1.len() <= op2.len() {
        y = op1.as_bytes();
        op2.as_bytes()
    } else {
        y = op2.as_bytes();
        op1.as_bytes()
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
    if op1_negative && op2_negative {
        format!("-{}", r)
    } else {
        r
    }
}

fn sub(mut op1: String, mut op2: String) -> String {
    let op1_negative = op1.chars().nth(0) == Some('-');
    let op2_negative = op2.chars().nth(0) == Some('-');

    if op1_negative {
        op1.remove(0);
        if !op2_negative {
            return format!("-{}", add(op1, op2));
        }
        op2.remove(0);
        let temp = op1;
        op1 = op2;
        op2 = temp;
    } else if op2_negative {
        op2.remove(0);
        return add(op1, op2);
    }

    let mut r = String::new();
    let mut negative = false;
    let x;
    let y;
    if op1.len() < op2.len() || (op1.len() == op2.len() && op1 <= op2) {
        negative = true;
        x = op2.as_bytes(); //.map(|digit| digit - ('0' as u8))
        y = op1.as_bytes();
    } else {
        y = op2.as_bytes(); //.map(|digit| digit - ('0' as u8))
        x = op1.as_bytes();
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

fn mul(mut op1: String, mut op2: String, logidx: u64) -> String {
    let mut r = String::new();

    let op1_sign: i8 = if op1.chars().nth(0) == Some('-') {
        -1
    } else {
        1
    };
    let op2_sign: i8 = if op2.chars().nth(0) == Some('-') {
        -1
    } else {
        1
    };

    if op1_sign == -1 {
        op1.remove(0);
    }
    if op2_sign == -1 {
        op2.remove(0);
    }
    let y;
    let x = if op1.len() <= op2.len() {
        y = op1.as_bytes();
        op2.as_bytes()
    } else {
        y = op2.as_bytes();
        op1.as_bytes()
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
    if op1_sign * op2_sign == -1 {
        format!("-{}", r)
    } else {
        r
    }
}

fn pow(base: String, mut power: String, logidx: u64) -> String {
    let mut result = format!("1");
    let length_long = base.len() + power.len() >= 30;
    let negative = if power.chars().nth(0) == Some('-') {
        power.remove(0);
        true
    } else { false };
    let temp_power: u128 = power.parse().unwrap();

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

    return if negative {
        format!("1/{}", result)
    } else {
        result
    };
}

fn app(expression: String, params: (bool, bool)) -> (bool, bool) {
    // fullog = false;
    let mut ops = expression
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<String>>();
    let mut logidx: u64 = 0;
    let mut term_idx = 0;
    let mut operator_idx = 0;
    let mut log = params.0;
    let mut fullog = params.1;
    if log {
        println!("\n--------------------------------------------- LOG --------------------------------------------");
    }
    let priorities = [("^", "", ""), ("x", "*", "/"), ("+", "-", "")];

    for current in priorities.iter() {
        log = params.0;
        fullog = params.1;
        let mut i = 0;
        while ops.len() > 1 && i < ops.len() {
            if ops[i] == "+l" {
                log = true;
            } else if ops[i] == "-l" {
                log = false;
                fullog = false;
            } else if ops[i] == "+f" {
                fullog = true;
                log = true;
            } else if ops[i] == "-f" {
                fullog = false;
            } else {
                if operator_idx > 0
                    && (ops[operator_idx] == current.0
                        || (current.1 != "" && current.1 == ops[operator_idx])
                        || (current.2 != "" && current.2 == ops[operator_idx]))
                {
                    rem_redundant(&mut ops[term_idx]);
                    rem_redundant(&mut ops[i]);

                    let length_long = ops[term_idx].len() + ops[i].len() >= 30;
                    if log {
                        print!(
                            "{} {} {} {}",
                            PADDING, ops[term_idx], ops[operator_idx], ops[i]
                        );
                    }

                    ops[term_idx] = match ops[operator_idx].as_str() {
                        "+" => add(ops[term_idx].clone(), ops[i].clone()),
                        "-" => sub(ops[term_idx].clone(), ops[i].clone()),
                        "*" | "x" => mul(
                            ops[term_idx].clone(),
                            ops[i].clone(),
                            if fullog {
                                logidx += 1;
                                logidx
                            } else {
                                0
                            },
                        ),
                        "^" => pow(
                            ops[term_idx].clone(),
                            ops[i].clone(),
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
                            txtstylish(&ops[term_idx], "blue", 'b'),
                            if length_long { "\n" } else { "" }
                        );
                    }
                    ops.remove(i);
                    ops.remove(operator_idx);
                    i = term_idx;
                    operator_idx = 0;
                } else {
                    operator_idx = match ops[i].as_str() {
                        "^" | "*" | "x" | "+" | "-" | "/" => i,
                        _ => {
                            term_idx = i;
                            0
                        }
                    };
                }
            }
            i += 1;
        }
    }
    println!("\n--------------------------------------------- finally --------------------------------------------");
    println!("    = {}", txtstylish(&(ops[term_idx]), "purple", 'b'));
    return (log, fullog);
}
fn main() {
    println!("byteculator v0.1.0: calculator for large numbers \n \t Enter the expression: \n");

    let mut log = true;
    let mut fullog = false;
    loop {
        println!("\n--------------------------------------------- NEW --------------------------------------------");
        let mut expression = String::new();
        io::stdin()
            .read_line(&mut expression)
            .expect("Wrong Expression");
        match expression.as_str().trim() {
            "x" | "" => break,
            _ => {
                let params_status = app(
                    expression.clone(),
                    (
                        log && (expression.chars().nth(0) != Some('-')
                            || expression.chars().nth(1) != Some('l')),
                        fullog,
                    ),
                );
                fullog = params_status.1;
                log = params_status.0
            }
        }
    }
}
