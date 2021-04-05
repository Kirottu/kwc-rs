use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Seek, BufReader, StdinLock};
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
        .about("Counts words, characters, bytes and newlines")
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
        .arg(Arg::new("lines")
             .about("Output file newline count")
             .short('l')
             .long("lines")
             .takes_value(false))
        .arg(Arg::new("bytes")
             .about("Output file byte count")
             .short('c')
             .long("bytes")
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
            Err(why) => {
                println!("Could not open {} for reading: {}", display, why);
                std::process::exit(why.raw_os_error().unwrap());
            },
            Ok(file) => file,
        };
        
        // Loop through lines of the file
        let file_count_new = loop_thru_file(Some(&file), None);
        
        // Set the counts to the values from the loop
        file_count.w_count = file_count_new.w_count;
        file_count.c_count = file_count_new.c_count;
        file_count.l_count = file_count_new.l_count;
        file_count.b_count = file_count_new.b_count;
    }
    
    // If no files are inputted, read from stdin
    if files.len() == 0 {
        // Stdin reference and lock
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        
        // Loop through the lines of stdin
        let file_count = loop_thru_file(None, Some(&mut handle));
        
        // Print results
        if matches.is_present("lines") {print!(" {} ", file_count.l_count);}
        if matches.is_present("words") {print!(" {} ", file_count.w_count);}
        if matches.is_present("characters") {print!(" {} ", file_count.c_count);}
        if matches.is_present("bytes") {print!(" {} ", file_count.b_count);}
        println!("{}", file_count.filename);
    }
    else { 
        // Variables to store the total values from all files 
        let mut w_count_total: i32 = 0;
        let mut c_count_total: i32 = 0;
        let mut l_count_total: i32 = 0;
        let mut b_count_total: i32 = 0;
        
        // Loop through each file to output its values, and increment total values
        for file_count in files.iter()
        {
            w_count_total += file_count.w_count;
            c_count_total += file_count.c_count;
            l_count_total += file_count.l_count;
            b_count_total += file_count.b_count;
            
            if matches.is_present("lines") {print!(" {} ", file_count.l_count);}
            if matches.is_present("words") {print!(" {} ", file_count.w_count);}
            if matches.is_present("characters") {print!(" {} ", file_count.c_count);}
            if matches.is_present("bytes") {print!(" {} ", file_count.b_count);}
            println!("{}", file_count.filename);
        }
        
        // Do not print total in case there is only 1 file
        if files.len() > 1 {
            if matches.is_present("lines") {print!(" {} ", l_count_total);}
            if matches.is_present("words") {print!(" {} ", w_count_total);}
            if matches.is_present("characters") {print!(" {} ", c_count_total);}
            if matches.is_present("bytes") {print!(" {} ", b_count_total);}
            println!("Total");
        }
    }
}

fn loop_thru_string(line: &str, file_count: &mut FileCount)
{
    // Increment both newline and character count due to newline characters
    file_count.c_count += 1;
    file_count.l_count += 1;

    // Set last character to a null character
    let mut last_c = '\0';
    
    // Loop through characters of the current line
    for c in line.chars() {
        file_count.c_count += 1;
        
        if (last_c == ' ' || last_c == '\0') && c != ' ' {
            file_count.w_count += 1; // Increment word count if those conditions are met
        }

        last_c = c; // Make sure the last_c character is the last character
    }
}

fn loop_thru_file(f: Option<&File>, stdin: Option<&mut StdinLock>) -> FileCount
{
    let mut file_count = FileCount::new("", 0, 0, 0, 0);

    // Same code for 2 loops due to type mismatch, sigh
    if let Some(mut file) = f {
        for line in BufReader::new(file).lines() { // Loop through the lines
            if let Ok(line_str) = line {
                loop_thru_string(&line_str, &mut file_count); // Count a single line
            }
        }
        // Reset read position for byte calculation
        match file.seek(io::SeekFrom::Start(0)) {
            Err(why) => {
                println!("File read position setting failed: {}", why);
                std::process::exit(why.raw_os_error().unwrap());
            },
            Ok(_) => ()
        } 

        for byte in file.bytes() { // Count the bytes
            if let Ok(_) = byte { // Error checking
                file_count.b_count += 1;
            }
        }
    }
    else if let Some(file) = stdin {
        let mut buf = String::new(); // Buffer to store stdin due to StdinLock not having Seek trait

        match file.read_to_string(&mut buf) { // Read stdin into the buffer
            Err(why) => {
                println!("Stdin reading to buffer failed: {}", why);
                std::process::exit(why.raw_os_error().unwrap());
            },
            Ok(_) => ()
        }
        for line in buf.lines() { // Loop through the lines
            loop_thru_string(&line, &mut file_count); // Count a single line
        }

        for _ in buf.bytes() { // Count the bytes
            file_count.b_count += 1;
        }
    }
    
    // Return the calculated values
    file_count
}
