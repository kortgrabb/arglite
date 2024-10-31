fn main() {
    let mut parser = arglite::ArgParser::new();
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