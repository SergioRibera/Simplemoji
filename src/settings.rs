use clap::Parser;

use crate::skin_tone::SkinTone;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct ArgOpts {
    #[arg(long, short)]
    pub tone: Option<SkinTone>,
    #[arg(long)]
    pub show_search: bool,
    #[arg(long)]
    pub show_preview: bool,
}
