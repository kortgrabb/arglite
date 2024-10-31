pub mod parser;
pub mod error;

pub use parser::ArgParser;
pub use error::ArgLiteError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_args() {
        let mut parser = ArgParser::new();
        parser.set_description("A test program");
        parser.add_flag("verbose", "Enable verbose output");
        parser.add_positional("filename", "Input file to process");

        let args = vec![
            "arglite".to_string(),
            "--verbose".to_string(),
            "example.txt".to_string(),
        ];

        let result = parser.parse(args);
        assert!(result.is_ok());
        assert_eq!(parser.get("verbose"), Some(&"true".to_string()));
        assert_eq!(parser.get("filename"), Some(&"example.txt".to_string()));
    }

    #[test]
    fn test_help_flag() {
        let mut parser = ArgParser::new();
        parser.add_flag("verbose", "Enable verbose output");
        parser.add_positional("filename", "Input file to process");

        let args = vec![
            "arglite".to_string(),
            "--help".to_string(),
        ];

        // Note: This will call exit(0) when running normally
        let result = parser.parse(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_missing_positional_arg() {
        let mut parser = ArgParser::new();
        parser.add_positional("filename", "Input file to process");

        let args = vec!["arglite".to_string()];

        let result = parser.parse(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_flag() {
        let mut parser = ArgParser::new();
        parser.add_flag("verbose", "Enable verbose output");

        let args = vec![
            "arglite".to_string(),
            "--unknown".to_string(),
        ];

        let result = parser.parse(args);
        assert!(matches!(result, Err(ArgLiteError::UnknownFlag(_))));
    }

    #[test]
    fn test_unexpected_arg() {
        let mut parser = ArgParser::new();
        parser.add_positional("filename", "Input file to process");

        let args = vec![
            "arglite".to_string(),
            "example.txt".to_string(),
            "unexpected".to_string(),
        ];

        let result = parser.parse(args);
        assert!(matches!(result, Err(ArgLiteError::ParseError(_))));
    }
}
