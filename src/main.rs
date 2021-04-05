use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, StdinLock};
use std::path::Path;
use clap::{Arg, App};

// Struct to store file specific values
struct FileCounts {
    filename: String,
    w_count: i32,
    c_count: i32,
    l_count: i32,
}

fn main() {
    let matches = App::new("kwc-rs") // Launch options with clap
        .version("1.0")
        .author("KirottuM")
        .about("Counts words, characters and newlines")
        .arg(Arg::new("words")
             .about("Output file word count")
             .short('w')
             .long("words")
             .takes_value(false))
        .arg(Arg::new("characters")
             .about("Output file character count")
             .short('c')
             .long("characters")
             .takes_value(false))
        .arg(Arg::new("newlines")
             .about("Output file newline count")
             .short('l')
             .long("newlines")
             .takes_value(false))
        .arg(Arg::new("verbosity")
             .about("Output more verbose information")
             .short('v')
             .long("verbose")
             .takes_value(false))
        .arg(Arg::new("files")
             .about("Fiĺes to read from")
             .value_name("FILES")
             .multiple(true)
             .index(1))
        .get_matches();

    let mut files: Vec<FileCounts> = Vec::new(); // Vector to store all the counts related to files

    let iterator = matches.values_of("files"); // Get iterator for all the inputted file names
    
    // Loop thru all the file options to add to the vector
    for file in iterator.unwrap() {
        files.push(FileCounts {
            filename: String::from(file), 
            w_count: 0, 
            c_count: 0, 
            l_count: 0
        });
    }

    for file_count in files.iter_mut() {
        // Create path to the file
        let path = Path::new(&file_count.filename);
        let display = path.display();

        // Open file in read only mode
        let file = match File::open(&path) {
            Err(why) => panic!("Could not open {} for reading: {}", display, why),
            Ok(file) => file,
        };
        
        // Loop through lines of the file
        let file_count_new = loop_thru_file(Some(&file), None);

        file_count.w_count = file_count_new.w_count;
        file_count.c_count = file_count_new.c_count;
        file_count.l_count = file_count_new.l_count;
    }
    
    // If no files are inputted, read from stdin
    if files.len() == 0 {
        let mut file_count = FileCounts {
            filename: String::from("stdin"),
            w_count: 0,
            c_count: 0,
            l_count: 0
        };

        let stdin = io::stdin();
        let mut handle = stdin.lock();

        let file_count_new = loop_thru_file(None, Some(&mut handle));
        
        file_count.w_count = file_count_new.w_count;
        file_count.c_count = file_count_new.c_count;
        file_count.l_count = file_count_new.l_count;
    }
    
    let mut w_count_total: i32 = 0;
    let mut c_count_total: i32 = 0;
    let mut l_count_total: i32 = 0;

    for file_count in files.iter()
    {
        w_count_total += file_count.w_count;
        c_count_total += file_count.c_count;
        l_count_total += file_count.l_count;
        println!("{} {} {} {}", file_count.l_count, file_count.w_count, file_count.c_count, file_count.filename);
    }

    if files.len() > 1 {
        println!("{} {} {} Total", l_count_total, w_count_total, c_count_total);
    }
}

fn loop_thru_file(f: Option<&File>, stdin: Option<&mut StdinLock>) -> FileCounts
{
    let mut file_count = FileCounts {
        filename: String::from(""),
        w_count: 0,
        c_count: 0,
        l_count: 0
    };

    // Same code for 2 loops due to type mismatch, sigh
    if let Some(file) = f {
        for line in BufReader::new(file).lines() {
            if let Ok(line_str) = line {
                // Increment both newline and character count due to newline characters
                file_count.l_count += 1; 
                file_count.c_count += 1;
                
                // Set last character to a null character
                let mut last_c = '\0';
                
                // Loop through characters of the current line
                for c in line_str.chars() {
                    file_count.c_count += 1;
                    
                    if (last_c == ' ' || last_c == '\0') && c != ' ' {
                        file_count.w_count += 1; // Increment word count if those conditions are met
                    }

                    last_c = c; // Make sure the last_c character is the last character
                }
            }
        }
    }
    else if let Some(file) = stdin {
        for line in file.lines() {
            if let Ok(line_str) = line {
                // Increment both newline and character count due to newline characters
                file_count.l_count += 1; 
                file_count.c_count += 1;
                
                // Set last character to a null character
                let mut last_c = '\0';
                
                // Loop through characters of the current line
                for c in line_str.chars() {
                    file_count.c_count += 1;
                    
                    if (last_c == ' ' || last_c == '\0') && c != ' ' {
                        file_count.w_count += 1; // Increment word count if those conditions are met
                    }

                    last_c = c; // Make sure the last_c character is the last character
                }
            }
        }
    }

    file_count
}
