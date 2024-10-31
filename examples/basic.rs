use arglite::ArgParser;

fn main() {
    let mut parser = ArgParser::new();
    
    // Setup parser
    parser.set_description("Example program showing basic arglite usage");
    parser.add_flag("verbose", "Enable verbose output");
    parser.add_flag("config", "Use custom config file");
    parser.add_positional("input", "Input file to process");
    parser.add_positional("output", "Output destination");

    // Parse arguments
    match parser.parse(std::env::args().collect()) {
        Ok(_) => {
            if parser.get("verbose").is_some() {
                println!("Verbose mode enabled");
            }
            
            println!("Input file: {}", parser.get("input").unwrap());
            println!("Output file: {}", parser.get("output").unwrap());
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            parser.print_help();
            std::process::exit(1);
        }
    }
}