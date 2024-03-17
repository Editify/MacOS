use argh::FromArgs;

// MAIN CLI

#[derive(FromArgs, PartialEq, Debug)]
/// Reach new heights.
pub struct CLI {
    /// terminal version
    #[argh(switch, short = 'T')]
    pub terminal: Option<bool>,

    #[argh(subcommand)]
    pub sub: Option<SubCommands>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCommands {
    Alias(Alias),
}



// ALIASES

#[derive(FromArgs, PartialEq, Debug)]
/// Alias for Editify.
#[argh(subcommand, name = "alias")]
pub struct Alias {
    #[argh(subcommand)]
    pub sub: AliasSubCommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum AliasSubCommands {
    AliasInstall(AliasInstall)
}

#[derive(FromArgs, PartialEq, Debug)]
/// Installs alias for editify.
#[argh(subcommand, name = "install")]
pub struct AliasInstall {}



impl CLI {
    pub fn from_env() -> Self { argh::from_env() }
}