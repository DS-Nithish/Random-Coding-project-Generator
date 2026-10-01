mod cli;
mod database;
mod models;
mod ui;

use clap::Parser;
use cli::CliArgs;
use colored::*;
use inquire::Select;
use models::{Difficulty, Domain, ProjectIdea};
use std::fs;
use std::path::Path;

fn main() {
    let args = CliArgs::parse();

    ui::print_banner();

    // 1. Handle --list flag
    if args.list {
        list_all_projects();
        return;
    }

    // 2. Determine execution mode (Direct Flags vs Interactive)
    let has_direct_filter = args.level.is_some() || args.domain.is_some();
    let has_action_flag = args.steps || args.code || args.doc || args.export;

    if !args.interactive && (has_direct_filter || has_action_flag) {
        run_flag_mode(&args);
    } else {
        run_interactive_mode();
    }
}

fn list_all_projects() {
    println!("{}", "📚 COMPLETE PROJECT BLUEPRINTS CATALOG:".bold().bright_cyan());
    println!("{}", "═".repeat(75).bright_cyan());
    let all = database::get_all();
    for (i, p) in all.iter().enumerate() {
        println!(
            "{:2}. {} {}  {}",
            i + 1,
            ui::domain_badge(p.domain),
            ui::difficulty_badge(p.difficulty),
            p.title.bold().bright_white()
        );
    }
    println!("{}", "═".repeat(75).bright_cyan());
    println!(
        "{} total projects available in the database.\n",
        all.len().to_string().bold().bright_green()
    );
}

fn run_flag_mode(args: &CliArgs) {
    let domain = args.domain.as_deref().and_then(Domain::parse);
    let difficulty = args.level.as_deref().and_then(Difficulty::parse);

    if args.domain.is_some() && domain.is_none() {
        eprintln!(
            "{}",
            format!("⚠️  Warning: Unknown domain '{}'. Ignoring domain filter.", args.domain.as_ref().unwrap())
                .yellow()
        );
    }
    if args.level.is_some() && difficulty.is_none() {
        eprintln!(
            "{}",
            format!("⚠️  Warning: Unknown level '{}'. Valid levels: beginner, intermediate, advanced.", args.level.as_ref().unwrap())
                .yellow()
        );
    }

    ui::play_randomizer_animation();

    match database::pick_random(domain, difficulty) {
        Some(project) => {
            ui::print_project_card(project);

            if args.steps {
                ui::print_steps(project);
            }
            if args.code {
                ui::print_code(project);
            }
            if args.doc {
                ui::print_documentation(project);
            }
            if args.export {
                export_project(project, &args.output);
            }

            if !args.steps && !args.code && !args.doc && !args.export {
                println!(
                    "{}",
                    "💡 Tip: Use --steps, --code, --doc, or --export to unlock full project deliverables!".bright_yellow()
                );
                println!(
                    "   {} cargo run -- --level {} --domain {} --steps --code --doc\n",
                    "Example:".bright_black(),
                    project.difficulty.as_str().to_lowercase(),
                    project.domain.short_name()
                );
            }
        }
        None => {
            println!(
                "{}",
                "❌ No project matching specified criteria was found.".bold().bright_red()
            );
        }
    }
}

fn run_interactive_mode() {
    let domain_options = vec![
        "🎲 Any Domain (Surprise Me!)",
        "🌐 Web Development",
        "📱 Mobile App Development",
        "🎮 Game Development",
        "🧠 AI & Machine Learning",
        "💻 CLI & System Tools",
        "🛡️  Cybersecurity & Networking",
    ];

    let domain_choice = Select::new("Choose a Project Domain:", domain_options)
        .prompt()
        .unwrap_or("🎲 Any Domain (Surprise Me!)");

    let selected_domain = match domain_choice {
        d if d.contains("Web") => Some(Domain::WebDev),
        d if d.contains("Mobile") => Some(Domain::MobileApp),
        d if d.contains("Game") => Some(Domain::GameDev),
        d if d.contains("AI") => Some(Domain::AiMachineLearning),
        d if d.contains("CLI") => Some(Domain::CliSystems),
        d if d.contains("Cyber") => Some(Domain::Cybersecurity),
        _ => None,
    };

    let diff_options = vec![
        "🎲 Any Skill Level (Surprise Me!)",
        "🟢 Beginner",
        "🟡 Intermediate",
        "🔴 Advanced",
    ];

    let diff_choice = Select::new("Choose Skill Level / Difficulty:", diff_options)
        .prompt()
        .unwrap_or("🎲 Any Skill Level (Surprise Me!)");

    let selected_diff = match diff_choice {
        d if d.contains("Beginner") => Some(Difficulty::Beginner),
        d if d.contains("Intermediate") => Some(Difficulty::Intermediate),
        d if d.contains("Advanced") => Some(Difficulty::Advanced),
        _ => None,
    };

    ui::play_randomizer_animation();

    let mut current_project = match database::pick_random(selected_domain, selected_diff) {
        Some(p) => p,
        None => {
            println!("{}", "❌ No projects found for this filter.".bright_red());
            return;
        }
    };

    ui::print_project_card(current_project);

    // Interactive Action Loop (Step 4 compliance)
    loop {
        let action_options = vec![
            "🗺️  Generate Step-by-Step Implementation Roadmap",
            "📦 Generate Single-File Starter Code",
            "📖 Generate Project Documentation & README",
            "💾 Export Project Bundle to Disk",
            "🎲 Re-Roll / Generate Another Random Project",
            "🚪 Exit",
        ];

        let action = match Select::new("What would you like to do next?", action_options).prompt() {
            Ok(ans) => ans,
            Err(_) => break,
        };

        if action.contains("Roadmap") {
            ui::print_steps(current_project);
        } else if action.contains("Starter Code") {
            ui::print_code(current_project);
        } else if action.contains("Documentation") {
            ui::print_documentation(current_project);
        } else if action.contains("Export") {
            let default_dir = format!("./project_{}", current_project.id.replace('-', "_"));
            export_project(current_project, &default_dir);
        } else if action.contains("Re-Roll") {
            ui::play_randomizer_animation();
            if let Some(next_p) = database::pick_random(selected_domain, selected_diff) {
                current_project = next_p;
                ui::print_project_card(current_project);
            }
        } else {
            println!("{}", "👋 Happy coding! Build something legendary.".bright_magenta().bold());
            break;
        }
    }
}

fn export_project(p: &ProjectIdea, target_dir_str: &str) {
    let target_dir = Path::new(target_dir_str);
    if let Err(e) = fs::create_dir_all(target_dir) {
        eprintln!("{}", format!("❌ Failed to create output directory: {}", e).bright_red());
        return;
    }

    // 1. Write README.md
    let readme_path = target_dir.join("README.md");
    let full_readme = format!(
        "# {}\n\n**Domain**: {}\n**Difficulty**: {}\n**Estimated Duration**: {}\n\n## Description\n{}\n\n## Core Requirements\n{}\n\n## Tech Stack\n{}\n\n{}\n\n## Starter Code Reference\nFile: `{}`\n\n```{}\n{}\n```\n",
        p.title,
        p.domain,
        p.difficulty,
        p.duration,
        p.description,
        p.requirements.iter().map(|r| format!("- [ ] {}", r)).collect::<Vec<_>>().join("\n"),
        p.technologies.iter().map(|t| format!("- {}", t)).collect::<Vec<_>>().join("\n"),
        p.documentation,
        p.starter_code_filename,
        p.starter_code_language,
        p.starter_code
    );

    if let Err(e) = fs::write(&readme_path, full_readme) {
        eprintln!("{}", format!("❌ Failed to write README.md: {}", e).bright_red());
        return;
    }

    // 2. Write starter code file
    let code_path = target_dir.join(&p.starter_code_filename);
    if let Err(e) = fs::write(&code_path, &p.starter_code) {
        eprintln!("{}", format!("❌ Failed to write starter code file: {}", e).bright_red());
        return;
    }

    // 3. Write metadata JSON
    let meta_path = target_dir.join("project_spec.json");
    if let Ok(json_str) = serde_json::to_string_pretty(p) {
        let _ = fs::write(&meta_path, json_str);
    }

    println!();
    println!("{}", "🎉 PROJECT EXPORTED SUCCESSFULLY!".bold().bright_green());
    println!("  📁 Directory:    {}", target_dir.display().to_string().bright_cyan());
    println!("  📄 Documentation: {}", readme_path.display().to_string().bright_white());
    println!("  💻 Starter Code:  {}", code_path.display().to_string().bright_yellow());
    println!("  ⚙️  Spec JSON:    {}", meta_path.display().to_string().bright_magenta());
    println!();
}
