
use cli::CLI;
use discord::connect as discordConnect;


fn main() {
    discordConnect();

    let cli = CLI::from_env();

    match cli.terminal {
        Some(true) => println!("Terminal option is true."),
        _ => ()
    }

    match cli.sub {
        _ => eprintln!("Undefined")
    }
}
