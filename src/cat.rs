use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

pub struct CatOptions {
    pub number_lines: bool,
    pub number_nonblank_lines: bool,
    pub show_ends: bool,
    pub show_tabs: bool,
    pub squeeze_blank: bool,
}

impl Default for CatOptions {
    fn default() -> Self {
        CatOptions {
            number_lines: false,
            number_nonblank_lines: false,
            show_ends: false,
            show_tabs: false,
            squeeze_blank: false,
        }
    }
}

trait FileReader {
    fn read_file(&self, path: &Path, options: &CatOptions) -> io::Result<()>;
}

trait StdinReader {
    fn read_stdin(&self, options: &CatOptions) -> io::Result<()>;
}

trait LineProcessor {
    fn process_line(&self, line: &str, line_number: &mut usize, options: &CatOptions);
}

struct StandardFileReader;

impl FileReader for StandardFileReader {
    fn read_file(&self, path: &Path, options: &CatOptions) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let processor = StandardLineProcessor;
        let mut line_number = 1;

        for line_result in reader.lines() {
            let line = line_result?;
            processor.process_line(&line, &mut line_number, options);
        }

        Ok(())
    }
}

struct StandardStdinReader;

impl StdinReader for StandardStdinReader {
    fn read_stdin(&self, options: &CatOptions) -> io::Result<()> {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let processor = StandardLineProcessor;
        let mut line_number = 1;

        for line_result in reader.lines() {
            let line = line_result?;
            processor.process_line(&line, &mut line_number, options);
        }

        Ok(())
    }
}

struct StandardLineProcessor;

impl LineProcessor for StandardLineProcessor {
    fn process_line(&self, line: &str, line_number: &mut usize, options: &CatOptions) {
        let is_blank = line.trim().is_empty();
        
        // Skip blank lines if number_nonblank_lines is true and the line is blank
        if options.number_nonblank_lines && is_blank {
            println!("{}", format_line(line, None, options));
        } else if options.number_lines || (options.number_nonblank_lines && !is_blank) {
            println!("{}", format_line(line, Some(*line_number), options));
            *line_number += 1;
        } else {
            println!("{}", format_line(line, None, options));
        }
    }
}

fn format_line(line: &str, line_number: Option<usize>, options: &CatOptions) -> String {
    let mut result = String::new();
    
    // Add line number if specified
    if let Some(num) = line_number {
        result.push_str(&format!("{:6}\t", num));
    }
    
    // Process line content
    let mut processed_line = line.to_string();
    
    // Replace tabs with visible representation if show_tabs is enabled
    if options.show_tabs {
        processed_line = processed_line.replace('\t', "^I");
    }
    
    result.push_str(&processed_line);
    
    // Add $ at the end of line if show_ends is enabled
    if options.show_ends {
        result.push('$');
    }
    
    result
}

struct CatCommand {
    file_reader: Box<dyn FileReader>,
    stdin_reader: Box<dyn StdinReader>,
}

impl CatCommand {
    fn new() -> Self {
        CatCommand {
            file_reader: Box::new(StandardFileReader),
            stdin_reader: Box::new(StandardStdinReader),
        }
    }
    
    fn run(&self, files: &[String], options: &CatOptions) -> io::Result<()> {
        if files.is_empty() {
            // Read from stdin if no files provided
            self.stdin_reader.read_stdin(options)?;
        } else {
            // Process each file in order
            for file_path in files {
                let path = Path::new(file_path);
                if !path.exists() {
                    eprintln!("cat: {}: No such file or directory", file_path);
                    continue;
                }
                
                if let Err(e) = self.file_reader.read_file(path, options) {
                    eprintln!("cat: {}: {}", file_path, e);
                }
            }
        }
        
        Ok(())
    }
}

pub fn run(files: &[String], options: &CatOptions) -> io::Result<()> {
    let command = CatCommand::new();
    command.run(files, options)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut options = CatOptions::default();
    let mut files = Vec::new();
    
    // Parse command line arguments
    for arg in args.iter().skip(1) {
        if arg.starts_with('-') && arg.len() > 1 {
            // Handle option flags
            for flag in arg.chars().skip(1) {
                match flag {
                    'n' => options.number_lines = true,
                    'b' => {
                        options.number_nonblank_lines = true;
                        options.number_lines = false;  // -b overrides -n
                    },
                    'E' => options.show_ends = true,
                    'T' => options.show_tabs = true,
                    'A' => {
                        options.show_ends = true;
                        options.show_tabs = true;
                    },
                    's' => options.squeeze_blank = true,
                    _ => eprintln!("cat: invalid option -- '{}'", flag),
                }
            }
        } else {
            // Add file to the list
            files.push(arg.clone());
        }
    }
    
    if let Err(e) = run(&files, &options) {
        eprintln!("cat: Error: {}", e);
        std::process::exit(1);
    }
}
