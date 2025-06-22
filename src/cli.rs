use clap::{Parser, ArgGroup};

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(group(
    ArgGroup::new("keep_policy")
        .args(["keep_newest", "keep_oldest", "keep_shortest", "keep_regex"])
        .multiple(false)
))]
pub struct Args {
    pub path: String,

    #[arg(short, long)]
    pub open: bool,

    #[arg(long)]
    pub interactive: bool,

    /// Keep newest file in group
    #[arg(long)]
    pub keep_newest: bool,

    /// Keep oldest file in group
    #[arg(long)]
    pub keep_oldest: bool,

    /// Keep file with shortest path
    #[arg(long)]
    pub keep_shortest: bool,

    /// Keep file matching regex
    #[arg(long, value_name = "PATTERN")]
    pub keep_regex: Option<String>,
}
