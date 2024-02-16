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
    #[arg(long, short)]
    /// This is the command that will be executed to copy the emoji
    pub copy_command: Option<String>,
}
