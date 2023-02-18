use std::env;
use rand::Rng;
use termion::{color, style};

fn main() {
    let mut password_chars = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"); // 62
    let symbols = "~`!@#$%^&*()_-+={[}]|\\:;\"'<,>.?/"; // 32

    let args: Vec<String> = env::args().collect();

    let (length, use_symbols) = sanitize_args(args);

    if use_symbols { password_chars.push_str(symbols); }

    let password = generate_password(password_chars, length);

    let stylized_password = stylize_password(password);

    // stdout output - without buffering
    print!("your \u{1F511} password is: {}\n", stylized_password);
}

fn sanitize_args(args: Vec<String>) -> (usize, bool) {
    // set defaults in case user does not enter options
    let mut length: usize = 13;
    let mut use_symbols = true;

    // handle when user does enter length
    if args.len() > 1 {
        // if invalid length entered, default to length 13
        length = match args[1].parse() {
            Ok(num) => {
                num
            }, 
            Err(_) => {
                println!("Defaulting to 13...");
                13
            }
        }
    }

    // handle when user does enter y|n to use symbols
    if args.len() > 2 {
        // handle y, n, and invalid entry
        use_symbols = match args[2].as_str() {
            "y" => true,
            "n" => false,
            _ => true,
        };
    }

    (length, use_symbols)
}

fn generate_password(password_chars: String, length: usize) -> String {
    let mut password = String::new();

    for _ in 0..length {
        let rand_num = rand::thread_rng().gen_range(0, password_chars[..].len());

        let rand_password_char = &password_chars[rand_num..rand_num + 1];

        password.push_str(rand_password_char);
    }

    password
}

fn stylize_password(password: String) -> String {
    format!("{}{}{}{}{}",
        style::Underline,
        color::Fg(color::Green),
        password,
        color::Fg(color::Reset),
        style::NoUnderline,
    )
}
