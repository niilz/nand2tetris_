use std::fs::File;
use std::io::{ self, Read };
use std::env::args;
use std::io::prelude::*;
use std::path::{ Path, PathBuf };

mod parser;
mod code_writer;

use parser::{ clean_line, parse_line };
use parser::Com::{Arith, Push, Pop };
use code_writer::{ write_arithmetic, write_push, write_pop };

fn main() {

    // Get file-name from command-line
    let args: Vec<String> = args().collect();
    let input_path = if args.len() == 2 {
            &args[1]
        } else {
            panic!("Please specify input file!")
        };

    let path = Path::new(input_path);
    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    let output_file = file_stem.to_string() + ".asm";
    let output_path: PathBuf = [path.parent().unwrap().to_str().unwrap(), &output_file].iter().collect();

    // Read input file (specified through command-line)
    let mut data = String::new(); 
    let mut f = File::open(input_path).expect("Could not open file.");
    f.read_to_string(&mut data).expect("Could not read to string.");
   

    let mut parsed_lines = Vec::new();
    for line in data.lines() {
        let cleaned_line = clean_line(&line);
        if !cleaned_line.is_empty() {
            match parse_line(&line) {
                Some(command) => parsed_lines.push(command),
                None => println!("Could not unwrap the parsed command."),
            }
        }
    }

    let asm_result_vec: Vec<String> = parsed_lines.iter().enumerate().map(|(line, command)| {
            match command {
                &Arith(com) => write_arithmetic(com, line),
                &Push(segment, position) => write_push(segment, position, &file_stem),
                &Pop(segment, position) => write_pop(segment, position, line, &file_stem),
            }
        }).collect();

    // Create the output file.
    println!("{:?}", output_path);
    let mut asm_file = match File::create(&output_path) {
        Ok(file) => file,
        Err(m) => panic!("Could not create file because: {}", m),
    };

    // Concat all asm-vecs values with a new-line.
    let file_content_str = asm_result_vec.join("\n");

    // Write the final asm-string to the file.
    match asm_file.write_all(file_content_str.as_bytes()) {
        Ok(_) => println!("HackFile with path: '{:?}' has been created successfully.", output_path),
        Err(m) => panic!("Coulnd not write to file because: {}", m),
    }
}