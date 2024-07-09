use cli::CLI;
use discord::connect as discordConnect;

fn main() {
    let _ = discordConnect();

    let cli = CLI::from_env();

    let _ = match cli {
        _terminal => {
            cli::terminal().expect("Terminal failed");
            Ok::<(), Box<dyn std::error::Error>>(())
        }
        _ => unreachable!(),
    };

    // TODO: WINDOW!
}
