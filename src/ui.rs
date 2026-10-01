use crate::models::{Difficulty, Domain, ProjectIdea};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread::sleep;
use std::time::Duration;

pub fn print_banner() {
    println!();
    println!("{}", " ╔═══════════════════════════════════════════════════════════════════════╗ ".bright_cyan());
    println!("{}", " ║   ⚡  R A N D O M   C O D I N G   P R O J E C T   G E N E R A T O R  ⚡  ║ ".bold().bright_white().on_blue());
    println!("{}", " ╚═══════════════════════════════════════════════════════════════════════╝ ".bright_cyan());
    println!(
        "   {} {} {} {}",
        "✨ Discover".bright_magenta().bold(),
        "• 🛠️  Build".bright_yellow().bold(),
        "• 🚀 Level Up".bright_green().bold(),
        "• 💡 Master Your Craft".bright_cyan().bold()
    );
    println!();
}

pub fn play_randomizer_animation() {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.bright_magenta} {msg}")
            .unwrap(),
    );

    let messages = [
        "Consulting the developer cosmos...",
        "Querying 50+ curated project blueprints...",
        "Evaluating difficulty algorithms...",
        "Spinning the tech roulette...",
        "Synthesizing specifications...",
        "✨ Project Found!",
    ];

    for msg in &messages {
        spinner.set_message(msg.bright_cyan().bold().to_string());
        for _ in 0..6 {
            spinner.tick();
            sleep(Duration::from_millis(35));
        }
    }
    spinner.finish_and_clear();
}

pub fn difficulty_badge(diff: Difficulty) -> ColoredString {
    match diff {
        Difficulty::Beginner => " BEGINNER ".bold().on_green().black(),
        Difficulty::Intermediate => " INTERMEDIATE ".bold().on_yellow().black(),
        Difficulty::Advanced => " ADVANCED ".bold().on_red().bright_white(),
    }
}

pub fn domain_badge(domain: Domain) -> ColoredString {
    match domain {
        Domain::WebDev => " 🌐 WEB DEV ".bold().on_blue().white(),
        Domain::MobileApp => " 📱 MOBILE APP ".bold().on_magenta().white(),
        Domain::GameDev => " 🎮 GAME DEV ".bold().on_purple().white(),
        Domain::AiMachineLearning => " 🧠 AI & ML ".bold().on_cyan().black(),
        Domain::CliSystems => " 💻 CLI & SYSTEMS ".bold().on_bright_black().bright_white(),
        Domain::Cybersecurity => " 🛡️  CYBERSECURITY ".bold().on_red().bright_white(),
    }
}

pub fn print_project_card(p: &ProjectIdea) {
    println!();
    println!("{}", "═".repeat(75).bright_cyan());
    println!(
        " {}  {}  {}",
        domain_badge(p.domain),
        difficulty_badge(p.difficulty),
        format!("⏱️  {}", p.duration).bright_yellow().bold()
    );
    println!("{}", "─".repeat(75).bright_black());
    println!("  📌 {}", p.title.bold().bright_white());
    println!();
    println!("  {}", p.description.white());
    println!();

    println!("  {}", "🛠️  RECOMMENDED TECH STACK:".bold().bright_cyan());
    let mut tech_line = String::from("   ");
    for tech in &p.technologies {
        tech_line.push_str(&format!(" [{}]", tech).bright_green().bold().to_string());
    }
    println!("{}", tech_line);
    println!();

    println!("  {}", "📋 CORE REQUIREMENTS:".bold().bright_cyan());
    for req in &p.requirements {
        println!("   {} {}", "✔".bright_green().bold(), req.bright_white());
    }
    println!("{}", "═".repeat(75).bright_cyan());
    println!();
}

pub fn print_steps(p: &ProjectIdea) {
    println!();
    println!("{}", "╔═══════════════════════════════════════════════════════════════════════╗".bright_yellow());
    println!("{}", "║                  🗺️  STEP-BY-STEP IMPLEMENTATION ROADMAP               ║".bold().bright_yellow());
    println!("{}", "╚═══════════════════════════════════════════════════════════════════════╝".bright_yellow());
    println!();
    for (i, step) in p.steps.iter().enumerate() {
        println!("  {}  {}", format!("Phase {}:", i + 1).bold().bright_magenta(), step.white());
        if i < p.steps.len() - 1 {
            println!("  {}", "  │".bright_black());
            println!("  {}", "  ▼".bright_cyan());
        }
    }
    println!();
}

pub fn print_code(p: &ProjectIdea) {
    println!();
    println!("{}", "╔═══════════════════════════════════════════════════════════════════════╗".bright_green());
    println!(
        "║  📦 STARTER CODE (Single File: {}){}",
        p.starter_code_filename.bold().bright_white(),
        " ".repeat(66_usize.saturating_sub(p.starter_code_filename.len() + 32)) + "║"
    );
    println!("{}", "╚═══════════════════════════════════════════════════════════════════════╝".bright_green());
    println!(
        "  {} {}",
        "Language:".bold().bright_black(),
        p.starter_code_language.to_uppercase().bold().bright_yellow()
    );
    println!("{}", "─".repeat(75).bright_black());
    for (line_idx, line) in p.starter_code.lines().enumerate() {
        println!("{:3} │ {}", format!("{}", line_idx + 1).bright_black(), line.bright_white());
    }
    println!("{}", "─".repeat(75).bright_black());
    println!();
}

pub fn print_documentation(p: &ProjectIdea) {
    println!();
    println!("{}", "╔═══════════════════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║                   📖 PROJECT DOCUMENTATION & README                   ║".bold().bright_cyan());
    println!("{}", "╚═══════════════════════════════════════════════════════════════════════╝".bright_cyan());
    println!();
    for line in p.documentation.lines() {
        if line.starts_with("# ") {
            println!("  {}", line.bold().bright_magenta());
        } else if line.starts_with("## ") {
            println!("  {}", line.bold().bright_cyan());
        } else if line.starts_with("- ") {
            println!("   {}", line.bright_white());
        } else {
            println!("  {}", line.white());
        }
    }
    println!();
}
