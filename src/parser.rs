use std::collections::HashMap;
use crate::error::ArgLiteError;

pub struct ArgParser {
    args: HashMap<String, String>,
    flags: Vec<(String, String)>,  // (flag_name, description)
    positional_args: Vec<(String, String)>,  // (arg_name, description)
    program_description: Option<String>,
}

impl ArgParser {
    pub fn new() -> Self {
        Self {
            args: HashMap::new(),
            flags: Vec::new(),
            positional_args: Vec::new(),
            program_description: None,
        }
    }

    /// Set the program description shown in help text
    pub fn set_description(&mut self, desc: &str) {
        self.program_description = Some(desc.to_string());
    }

    /// Add a flag with description
    pub fn add_flag(&mut self, flag: &str, description: &str) {
        self.flags.push((flag.to_string(), description.to_string()));
    }

    /// Add a positional argument with description
    pub fn add_positional(&mut self, arg_name: &str, description: &str) {
        self.positional_args.push((arg_name.to_string(), description.to_string()));
    }

    pub fn parse(&mut self, args: Vec<String>) -> Result<(), ArgLiteError> {
        // Add built-in help flag if not already added
        if !self.flags.iter().any(|(f, _)| f == "help") {
            self.add_flag("help", "Show this help message");
        }

        let mut positional_index = 0;
    
        let mut iter = args.into_iter().skip(1);
        while let Some(arg) = iter.next() {
            if arg.starts_with("--") {
                let flag = arg.trim_start_matches("--").to_string();
                if flag == "help" {
                    self.print_help();
                    std::process::exit(0);
                }
                if self.flags.iter().any(|(f, _)| f == &flag) {
                    self.args.insert(flag, "true".to_string());
                } else {
                    return Err(ArgLiteError::UnknownFlag(flag));
                }
            } else {
                if positional_index < self.positional_args.len() {
                    self.args.insert(self.positional_args[positional_index].0.clone(), arg);
                    positional_index += 1;
                } else {
                    return Err(ArgLiteError::ParseError(format!("Unexpected argument: {}", arg)));
                }
            }
        }
    
        // Check for missing required positional arguments
        if positional_index < self.positional_args.len() {
            return Err(ArgLiteError::MissingArgument(self.positional_args[positional_index].0.clone()));
        }
    
        Ok(())
    }

    /// Print formatted help message
    pub fn print_help(&self) {
        let program_name = std::env::args().next().unwrap_or_else(|| "program".to_string());
        
        // Print program description if available
        if let Some(desc) = &self.program_description {
            println!("{}\n", desc);
        }

        // Usage section
        println!("USAGE:");
        let mut usage = format!("    {}", program_name);
        for (flag, _) in &self.flags {
            usage.push_str(&format!(" [--{}]", flag));
        }
        for (arg, _) in &self.positional_args {
            usage.push_str(&format!(" <{}>", arg));
        }
        println!("{}\n", usage);

        // Flags section
        if !self.flags.is_empty() {
            println!("FLAGS:");
            for (flag, desc) in &self.flags {
                println!("    --{:<20} {}", flag, desc);
            }
            println!();
        }

        // Arguments section
        if !self.positional_args.is_empty() {
            println!("ARGS:");
            for (arg, desc) in &self.positional_args {
                println!("    {:<20} {}", arg, desc);
            }
        }
    }

    /// Get the value of a parsed argument.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.args.get(key)
    }

    // print help message
    pub fn help(&self) {
        let mut help_message = "Usage: program".to_string();
        
        for flag in &self.flags {
            help_message.push_str(&format!(" [--{}]", flag.0));
        }

        for arg in &self.positional_args {
            help_message.push_str(&format!(" <{}>", arg.0));
        }

        println!("{}", help_message);
    }
}
