

fn usage() {
    println!("Usage: string_art [options] <input file>");
    println!("Options:");
    println!("  --help           prints this help message.");
    println!("  --full-thread    gives a result that can require multiple threads but is more accurate.");
}

#[derive(Debug)]   // DEBUG
struct Config {
    input_file: String,
    full_thread: bool,
}

impl Config {
    fn create(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(String::from("Not enough arguments"));
        }

        let mut input_file = Option::None;
        let mut full_thread = false;
        
        for arg in args {
            if arg == "--help" {
                usage();
                std::process::exit(0);
            } else if arg == "--full-thread" {
                full_thread = true;
            } else if input_file.is_none() {
                input_file = Some(arg.clone());
            } else {
                return Err(String::from("Too many arguments"));
            }
        }

        if input_file.is_none() {
            return Err(String::from("No input file"));
        }

        Ok(Config {
            input_file: input_file.unwrap(),
            full_thread,
        })

    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::create(&args).unwrap_or_else(|err| {
        eprintln!("string_art: {}", err);
        usage();
        std::process::exit(1);
    });

    dbg!(config);   // DEBUG
}
