use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "random-project-gen",
    author = "Antigravity Team",
    version = "1.0.0",
    about = "⚡ Colorful Random Coding Project Generator CLI ⚡",
    long_about = "A colorful, interactive CLI tool to generate random project ideas across skill levels and domains, with on-demand steps, single-file code, documentation, and export capabilities."
)]
pub struct CliArgs {
    /// Skill level (beginner, intermediate, advanced)
    #[arg(short = 'l', long = "level")]
    pub level: Option<String>,

    /// Domain (web, mobile, game, ai, cli, cyber)
    #[arg(short = 'd', long = "domain")]
    pub domain: Option<String>,

    /// Display the step-by-step implementation roadmap
    #[arg(long = "steps")]
    pub steps: bool,

    /// Display the self-contained starter code
    #[arg(long = "code")]
    pub code: bool,

    /// Display the project documentation and README
    #[arg(long = "doc")]
    pub doc: bool,

    /// Export the project (README, starter code, spec) into a folder
    #[arg(short = 'e', long = "export")]
    pub export: bool,

    /// Output folder path when exporting
    #[arg(short = 'o', long = "output", default_value = "./generated_project")]
    pub output: String,

    /// List all 50 available projects in the catalog
    #[arg(long = "list")]
    pub list: bool,

    /// Force interactive terminal wizard mode
    #[arg(short = 'i', long = "interactive")]
    pub interactive: bool,
}
