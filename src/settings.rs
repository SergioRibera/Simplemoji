use clap::Parser;

use ui::SkinTone;

use crate::recents::RecentType;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
#[allow(clippy::struct_excessive_bools)]
pub struct ArgOpts {
    #[arg(long, short)]
    pub tone: Option<SkinTone>,
    /// Show debug keys
    #[arg(long, short = 'd')]
    pub debug: bool,
    /// The font use for render emojis
    #[arg(long, short)]
    pub font: Option<String>,
    /// The corner radius for the emoji when it is in focus
    #[arg(long, short = 'r')]
    pub corner_radius: Option<u8>,
    #[arg(long, short = 's')]
    pub show_search: bool,
    #[arg(long)]
    pub show_recent: bool,
    #[arg(long, default_value_t = 1)]
    pub recent_rows: u8,
    #[arg(long)]
    pub recent_type: RecentType,
    #[arg(long, default_value_t = 4)]
    pub static_recents: usize,
    /// Use fuzzing search algorithms
    #[arg(long, short = 'z')]
    pub fuzzing_search: bool,
    #[arg(long, short = 'p')]
    pub show_preview: bool,
    #[arg(long, short = 'o')]
    pub close_on_copy: bool,
    /// By default the application closes automatically when it is out of focus, this option disables that behavior
    #[arg(long, short = 'x')]
    pub no_close: bool,
    /// Background color in hex (RGB, RGBA, RRGGBB, RRGGBBAA)
    #[arg(long, short = 'b')]
    pub background_color: Option<String>,
    /// Primary color in hex (RGB, RGBA, RRGGBB, RRGGBBAA)
    #[arg(long, short = 'm')]
    pub primary_color: Option<String>,
    /// This is the command that will be executed to copy the emoji
    #[arg(long, short)]
    pub copy_command: Option<String>,
}
