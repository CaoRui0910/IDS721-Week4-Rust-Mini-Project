use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Rui",
    about = "Command-line interface for finding your zodiac sign by your date of birth"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Rui")]
    Constellation {
        month: u64,
        day: u64,
    },
}

//invoke lib.rs using onnx_demo namespace
//use onnx_demo::run;
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Constellation{month, day}) => {
            if ((month == 3) & (day >=21)) | ((month == 4) & (day <= 19)){
                println!("{}", "Aries");
            }else if ((month == 4) & (day >= 20)) | ((month == 5) & (day <= 20)){
                println!("{}", "Taurus");
            }else if ((month == 5) & (day >= 21)) | ((month == 6) & (day <= 21)){
                println!("{}", "Gemini");
            }else if ((month == 6) & (day >= 22)) | ((month == 7) & (day <= 22)){
                println!("{}", "Cancer");
            }else if ((month == 7) & (day >= 23)) | ((month == 8) & (day <= 22)){
                println!("{}", "Leo");
            }else if ((month == 8) & (day >= 23)) | ((month == 9) & (day <= 22)){
                println!("{}", "Virgo");
            }else if ((month == 9) & (day >= 23)) | ((month == 10) & (day <= 23)){
                println!("{}", "Libra");
            }else if ((month == 10) & (day >= 24)) | ((month == 11) & (day <= 21)){
                println!("{}", "Scorpio");
            }else if ((month == 11) & (day >= 22)) | ((month == 12) & (day <= 21)){
                println!("{}", "Sagittarius");
            }else if ((month == 12) & (day >= 22)) | ((month == 1) & (day <= 19)){
                println!("{}", "Capricorn");
            }else if ((month == 1) & (day >= 20)) | ((month == 2) & (day <= 18)){
                println!("{}", "Aquarius");
            }else if ((month == 2) & (day >= 19)) | ((month == 3) & (day <= 20)){
                println!("{}", "Pisces");
            }
        }
        None => println!("No subcommand was used"),
    }
}