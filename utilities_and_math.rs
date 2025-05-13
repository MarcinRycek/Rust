fn main() {
    let card = business_card("first", "last");
    println!("{card}");
    let x = 8.0;
}

fn business_card(first: &str, last: &str) -> String {
    let mut first_initial = first.chars().next().unwrap().to_uppercase().to_string();
    let last_initial = last.chars().next().unwrap().to_uppercase().to_string();
    let last_rest = last.chars().skip(1).collect::<String>().to_lowercase();

    first_initial + ". " + &last_initial + &last_rest
}

fn to_roman(mut number: i32) -> String {
    let mut result: String = Default::default();
    while number > 0 {
        if number >= 1000 {
            result.push('M');
            number -= 1000;
        } else if number >= 900 {
            result.push_str("CM");
            number -= 900;
        } else if number >= 500 {
            result.push_str("D");
            number -= 500;
        } else if number >= 400 {
            result.push_str("CD");
            number -= 400;
        } else if number >= 100 {
            result.push_str("C");
            number -= 100;
        } else if number >= 90 {
            result.push_str("XC");
            number -= 90;
        } else if number >= 50 {
            result.push_str("L");
            number -= 50;
        } else if number >= 40 {
            result.push_str("XL");
            number -= 40;
        } else if number >= 10 {
            result.push_str("X");
            number -= 10;
        } else if number >= 5 {
            result.push_str("V");
            number -= 5;
        } else if number >= 4 {
            result.push_str("IV");
            number -= 4;
        } else if number >= 1 {
            result.push_str("I");
            number -= 1;
        }
    }
    result
}

fn add_strings(a: &str, b: &str) -> String {
    let mut result = String::new();
    let mut a_chars = a.chars().rev().peekable();
    let mut b_chars = b.chars().rev().peekable();
    let mut carry = 0;

    while a_chars.peek().is_some() || b_chars.peek().is_some() || carry != 0 {
        let va = a_chars.next().unwrap_or('0').to_digit(10).unwrap_or(0);
        let vb = b_chars.next().unwrap_or('0').to_digit(10).unwrap_or(0);
        let sum = va + vb + carry;
        carry = sum / 10;
        let ch = char::from_digit(sum % 10, 10).unwrap();
        result.push(ch);
    }

    result.chars().rev().collect()
}

fn f(x: f64) -> f64 {
    x * x - 4.0
}

fn f_prime(x: f64) -> f64 {
    2.0 * x
}

fn newton_method(x0: f64, epsilon: f64, max_iter: u128) -> f64 {
    let mut x = x0;
    let mut iter = 0;

    loop {
        iter += 1;
        let fx = f(x);
        let fpx = f_prime(x);

        if fpx.abs() < epsilon {
            break;
        }

        let x_new = x - fx / fpx;

        if (x_new - x).abs() < epsilon || iter >= max_iter {
            x = x_new;
            break;
        }

        x = x_new;
    }

    x
}
