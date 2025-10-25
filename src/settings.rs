use clap::Parser;

use ui::SkinTone;

use crate::recents::RecentType;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
#[allow(clippy::struct_excessive_bools)]
pub struct ArgOpts {
    /// The skin tone to apply to emojis.
    #[arg(long, short)]
    pub tone: Option<SkinTone>,
    /// Show debug keys and additional internal information.
    #[arg(long, short = 'd')]
    pub debug: bool,
    /// The font used to render emojis.
    #[arg(long, short)]
    pub font: Option<String>,
    /// The corner radius (in pixels) for emojis when they are in focus.
    #[arg(long, short = 'r')]
    pub corner_radius: Option<u8>,
    /// Display the search bar in the UI.
    #[arg(long, short = 's')]
    pub show_search: bool,
    /// Show the "recent emojis" section.
    #[arg(long, default_value_t = true)]
    pub show_recent: bool,
    /// The number of rows dedicated to recent emojis.
    #[arg(long, default_value_t = 1)]
    pub recent_rows: u8,
    /// The strategy used to manage the recent emojis list.
    #[arg(long)]
    pub recent_type: Option<RecentType>,
    /// The number of static recents that always appear in the list.
    #[arg(long, default_value_t = 4)]
    pub static_recents: usize,
    /// Enable fuzzy search algorithms.
    #[arg(long, short = 'z')]
    pub fuzzing_search: bool,
    /// Show an emoji preview when selecting.
    #[arg(long, short = 'p')]
    pub show_preview: bool,
    /// Automatically close the picker after copying an emoji.
    #[arg(long, short = 'o')]
    pub close_on_copy: bool,
    /// Prevent the application from closing when it loses focus.
    #[arg(long, short = 'x')]
    pub no_close: bool,
    /// The background color of the UI, in hexadecimal format.
    #[arg(long, short = 'b')]
    pub background_color: Option<String>,
    /// The primary accent color of the UI, in hexadecimal format.
    #[arg(long, short = 'm')]
    pub primary_color: Option<String>,
    /// The command that will be executed to copy an emoji.
    ///
    /// For example: `--copy-command "xclip -selection clipboard"`.
    #[arg(long, short)]
    pub copy_command: Option<String>,
}
