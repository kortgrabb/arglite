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
        parser.add_flag("verbose");
        parser.add_positional("filename");

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
    fn test_missing_positional_arg() {
        let mut parser = ArgParser::new();
        parser.add_positional("filename");

        let args = vec!["arglite".to_string()];

        let result = parser.parse(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_flag() {
        let mut parser = ArgParser::new();
        parser.add_flag("verbose");

        let args = vec![
            "arglite".to_string(),
            "--unknown".to_string(),
        ];

        let result = parser.parse(args);
        assert!(matches!(result, Err(ArgLiteError::UnknownFlag(_))));
    }
}
