use clap::Parser;

use crate::skin_tone::SkinTone;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct ArgOpts {
    #[arg(long, short)]
    pub tone: Option<SkinTone>,
    #[arg(long, short = 's')]
    pub show_search: bool,
    #[arg(long, short = 'p')]
    pub show_preview: bool,
    /// By default the application closes automatically when it is out of focus, this option disables that behavior.
    #[arg(long, short = 'e')]
    pub no_close: bool,
    /// This is the command that will be executed to copy the emoji
    #[arg(long, short)]
    pub copy_command: Option<String>,
}
