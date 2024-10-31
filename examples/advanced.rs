use arglite::{ArgParser, ArgLiteError};

fn main() -> Result<(), ArgLiteError> {
    let mut parser = ArgParser::new();
    
    // Setup arguments
    parser.add_flag("verbose");
    parser.add_flag("help");
    parser.add_positional("input");
    parser.add_positional("output");

    // Get actual command line arguments
    let args: Vec<String> = std::env::args().collect();
    parser.parse(args)?;

    // Handle help flag
    if parser.get("help").is_some() {
        println!("Usage: program [--verbose] [--help] <input> <output>");
        return Ok(());
    }

    // Use the arguments
    let input = parser.get("input").unwrap();
    let output = parser.get("output").unwrap();
    let verbose = parser.get("verbose").is_some();

    println!("Processing {} -> {}", input, output);
    if verbose {
        println!("Verbose mode enabled");
    }

    Ok(())
}
