use std::env;

use crate::r#gen::generate;
mod file;
mod r#gen;

#[cfg(test)]
mod tests {
    mod generate;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Brainfuck Generator");
        println!("usage : cargo run -- <input_file> [-o out_file]");
        return;
    }

    let input = &args[1];
    let mut output: Option<&String> = None;

    for i in 2..args.len() {
        if args[i] == "-o" && i + 1 < args.len() {
            output = Some(&args[i + 1]);
            break;
        }
    }

    let input_txt = file::read(input);
    let code = generate(input_txt.trim());

    match output {
        Some(path) => {
            file::write(path, &code);
            println!("sucess!")
        }
        None => {
            println!("generated code :");
            println!("{}", code);
        }
    }
}
