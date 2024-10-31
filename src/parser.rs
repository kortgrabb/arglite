use std::collections::HashMap;
use crate::error::ArgLiteError;

pub struct ArgParser {
    args: HashMap<String, String>,
    flags: Vec<String>,
    positional_args: Vec<String>,
}

impl ArgParser {
    pub fn new() -> Self {
        Self {
            args: HashMap::new(),
            flags: Vec::new(),
            positional_args: Vec::new(),
        }
    }

    /// Add a flag (e.g., `--verbose`).
    pub fn add_flag(&mut self, flag: &str) {
        self.flags.push(flag.to_string());
    }

    /// Add a positional argument (e.g., `filename`).
    pub fn add_positional(&mut self, arg_name: &str) {
        self.positional_args.push(arg_name.to_string());
    }

    /// Parse the command-line arguments.
    pub fn parse(&mut self, args: Vec<String>) -> Result<(), ArgLiteError> {
        let mut positional_index = 0;
    
        let mut iter = args.into_iter().skip(1); // Skip executable name
        while let Some(arg) = iter.next() {
            if arg.starts_with("--") {
                let flag = arg.trim_start_matches("--").to_string();
                if self.flags.contains(&flag) {
                    self.args.insert(flag, "true".to_string());
                } else {
                    return Err(ArgLiteError::UnknownFlag(flag));
                }
            } else {
                if positional_index < self.positional_args.len() {
                    self.args.insert(self.positional_args[positional_index].clone(), arg);
                    positional_index += 1;
                } else {
                    return Err(ArgLiteError::ParseError(format!("Unexpected argument: {}", arg)));
                }
            }
        }
    
        // Check for missing required positional arguments
        if positional_index < self.positional_args.len() {
            return Err(ArgLiteError::MissingArgument(self.positional_args[positional_index].clone()));
        }
    
        Ok(())
    }
    

    /// Get the value of a parsed argument.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.args.get(key)
    }
}
