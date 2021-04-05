use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, StdinLock};
use std::path::Path;
use clap::{Arg, App};

// Struct to store file specific values
#[derive(Debug)]
struct FileCount {
    filename: String,
    w_count: i32,
    c_count: i32,
    l_count: i32,
    b_count: i32,
}

impl FileCount {
    // Function for quickly creating a FileCount instance
    fn new(filename: &str, w_count: i32, c_count: i32, l_count: i32, b_count: i32) -> FileCount
    {
        FileCount {
            filename: String::from(filename),
            w_count: w_count,
            c_count: c_count,
            l_count: l_count,
            b_count: b_count
        }
    }
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
             .short('m')
             .long("chars")
             .takes_value(false))
        .arg(Arg::new("newlines")
             .about("Output file newline count")
             .short('l')
             .long("lines")
             .takes_value(false))
        .arg(Arg::new("bytes")
             .about("Output file byte count")
             .short('c')
             .long("bytes")
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

    let mut files: Vec<FileCount> = Vec::new(); // Vector to store all the counts related to files
    
    // Get iterator for all the inputted file names, also do error handling in case no file names
    // were inputted
    let iterator = match matches.values_of("files") { 
        Some(iterator) => iterator,
        None => clap::Values::default(),
    }; 
    
    // Loop thru all the file options to add to the vector
    for file in iterator {
        files.push(FileCount::new(file, 0, 0, 0, 0));
    }
    
    // Loop through all the items in the vector to count their stats
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
        
        // Set the counts to the values from the loop
        file_count.w_count = file_count_new.w_count;
        file_count.c_count = file_count_new.c_count;
        file_count.l_count = file_count_new.l_count;
    }
    
    // If no files are inputted, read from stdin
    if files.len() == 0 {
        // Default FileCount struct
        let mut file_count = FileCount::new("stdin", 0, 0, 0, 0);
        
        // Stdin reference and lock
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        
        // Loop through the lines of stdin
        let file_count_new = loop_thru_file(None, Some(&mut handle));
        
        // Set the counts to the values from the loop
        file_count.w_count = file_count_new.w_count;
        file_count.c_count = file_count_new.c_count;
        file_count.l_count = file_count_new.l_count;

        println!("{} {} {} {}", file_count.l_count, file_count.w_count, file_count.c_count, file_count.filename);
    }
    else { 
        // Variables to store the total values from all files 
        let mut w_count_total: i32 = 0;
        let mut c_count_total: i32 = 0;
        let mut l_count_total: i32 = 0;
        
        // Loop through each file to output its values, and increment total values
        for file_count in files.iter()
        {
            w_count_total += file_count.w_count;
            c_count_total += file_count.c_count;
            l_count_total += file_count.l_count;
            println!("{} {} {} {}", file_count.l_count, file_count.w_count, file_count.c_count, file_count.filename);
        }
        
        // Do not print total in case there is only 1 file
        if files.len() > 1 {
            println!("{} {} {} Total", l_count_total, w_count_total, c_count_total);
        }
    }
}

fn loop_thru_string(line: &str, counts: &mut (i32, i32, i32, i32))
{
    // Increment both newline and character count due to newline characters
    counts.2 += 1; 
    counts.1 += 1;
    
    // Set last character to a null character
    let mut last_c = '\0';
    
    // Loop through characters of the current line
    for c in line.chars() {
        counts.1 += 1;
        
        if (last_c == ' ' || last_c == '\0') && c != ' ' {
            counts.0 += 1; // Increment word count if those conditions are met
        }

        last_c = c; // Make sure the last_c character is the last character
    }
}

fn loop_thru_file(f: Option<&File>, stdin: Option<&mut StdinLock>) -> FileCount
{
    let mut counts = (0, 0, 0, 0);

    // Same code for 2 loops due to type mismatch, sigh
    if let Some(file) = f {
        for line in BufReader::new(file).lines() {
            if let Ok(line_str) = line {
                loop_thru_string(&line_str, &mut counts);
            }
        }
        for _ in file.bytes() {
            counts.3 += 1;
        }
    }
    else if let Some(file) = stdin {
        for line in file.lines() {
            if let Ok(line_str) = line {
                loop_thru_string(&line_str, &mut counts);
            }
        }
        for _ in file.bytes() {
            counts.3 += 1;
        }
    }
    FileCount::new("", counts.0, counts.1, counts.2, counts.3)
}
