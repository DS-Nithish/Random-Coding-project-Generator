mod ai;
mod cli;
mod database;
mod github;
mod models;
mod ui;

use ai::AiProvider;
use clap::Parser;
use cli::CliArgs;
use colored::*;
use inquire::{Select, Text};
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
    let has_action_flag = args.steps || args.code || args.doc || args.export || args.save_md.is_some();
    let has_source_flag = args.ai || args.github || args.topic.is_some();

    if !args.interactive && (has_direct_filter || has_action_flag || has_source_flag) {
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

    let project_result: Result<ProjectIdea, String> = if args.ai {
        let provider = args.provider.as_deref().and_then(AiProvider::parse);
        let provider_name = provider.map(|p| p.as_str()).unwrap_or("AI");
        ui::play_ai_animation(provider_name);
        ai::generate_ai_project(
            provider,
            domain,
            difficulty,
            args.topic.as_deref(),
            args.api_key.as_deref(),
        )
    } else if args.github {
        ui::play_github_animation();
        github::fetch_github_project(
            domain,
            difficulty,
            args.topic.as_deref(),
            args.github_token.as_deref(),
        )
    } else {
        ui::play_randomizer_animation();
        database::pick_random(domain, difficulty)
            .cloned()
            .ok_or_else(|| "❌ No project matching specified criteria was found in local catalog.".to_string())
    };

    match project_result {
        Ok(project) => {
            ui::print_project_card(&project);

            if args.steps {
                ui::print_steps(&project);
            }
            if args.code {
                ui::print_code(&project);
            }
            if args.doc {
                ui::print_documentation(&project);
            }
            if args.export {
                export_project(&project, &args.output);
            }
            if let Some(ref md_arg) = args.save_md {
                let default_name = format!("{}.md", project.id.replace('-', "_"));
                let file_path = if md_arg.trim().is_empty() {
                    default_name
                } else {
                    md_arg.clone()
                };
                save_markdown_file(&project, &file_path);
            }

            if !args.steps && !args.code && !args.doc && !args.export && args.save_md.is_none() {
                println!(
                    "{}",
                    "💡 Tip: Use --steps, --code, --doc, --save-md, or --export to unlock full project deliverables!".bright_yellow()
                );
                println!(
                    "   {} cargo run -- --level {} --domain {} --steps --code --doc\n",
                    "Example:".bright_black(),
                    project.difficulty.as_str().to_lowercase(),
                    project.domain.short_name()
                );
            }
        }
        Err(err) => {
            println!("{}", format!("❌ Error: {}", err).bold().bright_red());
            println!(
                "{}",
                "💡 Falling back to curated offline catalog...".bright_yellow()
            );
            if let Some(fallback) = database::pick_random(domain, difficulty) {
                ui::print_project_card(fallback);
            }
        }
    }
}

fn run_interactive_mode() {
    // Step 1: Select Generation Source
    let source_options = vec![
        "💾 Curated Catalog (50 hand-crafted offline blueprints - instant)",
        "🤖 AI Architect (Infinite custom blueprints via Gemini, OpenAI, or Ollama)",
        "🐙 GitHub Live Explorer (Discover real-world open source templates & projects)",
    ];

    let source_choice = Select::new("Choose Project Source:", source_options)
        .prompt()
        .unwrap_or("💾 Curated Catalog (50 hand-crafted offline blueprints - instant)");

    let is_ai = source_choice.contains("AI Architect");
    let is_github = source_choice.contains("GitHub Live");

    // Step 2: Select Domain
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

    // Step 3: Select Difficulty
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

    // Step 4: Optional topic prompt for AI/GitHub
    let mut custom_topic: Option<String> = None;
    if is_ai || is_github {
        let prompt_label = if is_ai {
            "Custom Topic or Idea for AI (Optional, press Enter to skip):"
        } else {
            "Custom Keyword or Tag for GitHub Search (Optional, press Enter to skip):"
        };

        if let Ok(topic_input) = Text::new(prompt_label).prompt() {
            let trimmed = topic_input.trim();
            if !trimmed.is_empty() {
                custom_topic = Some(trimmed.to_string());
            }
        }
    }

    // Step 5: Fetch or Generate Project
    let current_project = fetch_project_interactive(
        is_ai,
        is_github,
        selected_domain,
        selected_diff,
        custom_topic.as_deref(),
    );

    if current_project.is_none() {
        println!("{}", "❌ Unable to acquire project blueprint. Exiting.".bright_red());
        return;
    }

    let mut project = current_project.unwrap();
    ui::print_project_card(&project);

    // Step 6: Interactive Action Loop
    loop {
        let action_options = vec![
            "🗺️  Generate Step-by-Step Implementation Roadmap",
            "📦 Generate Single-File Starter Code",
            "📖 Generate Project Documentation & README",
            "📝 Save Project as Markdown (.md) File",
            "💾 Export Full Project Bundle to Disk",
            "🎲 Re-Roll from Curated Catalog",
            "🤖 Generate Infinite Idea with AI",
            "🐙 Discover Live Project on GitHub",
            "🚪 Exit",
        ];

        let action = match Select::new("What would you like to do next?", action_options).prompt() {
            Ok(ans) => ans,
            Err(_) => break,
        };

        if action.contains("Roadmap") {
            ui::print_steps(&project);
        } else if action.contains("Starter Code") {
            ui::print_code(&project);
        } else if action.contains("Documentation") {
            ui::print_documentation(&project);
        } else if action.contains("Markdown") {
            let default_name = format!("{}.md", project.id.replace('-', "_"));
            let filename = Text::new("Enter Markdown filename:")
                .with_default(&default_name)
                .prompt()
                .unwrap_or(default_name);
            save_markdown_file(&project, &filename);
        } else if action.contains("Export") {
            let default_dir = format!("./project_{}", project.id.replace('-', "_"));
            export_project(&project, &default_dir);
        } else if action.contains("Re-Roll") {
            ui::play_randomizer_animation();
            if let Some(next_p) = database::pick_random(selected_domain, selected_diff) {
                project = next_p.clone();
                ui::print_project_card(&project);
            }
        } else if action.contains("AI") {
            if let Some(next_p) = fetch_project_interactive(true, false, selected_domain, selected_diff, custom_topic.as_deref()) {
                project = next_p;
                ui::print_project_card(&project);
            }
        } else if action.contains("GitHub") {
            if let Some(next_p) = fetch_project_interactive(false, true, selected_domain, selected_diff, custom_topic.as_deref()) {
                project = next_p;
                ui::print_project_card(&project);
            }
        } else {
            println!("{}", "👋 Happy coding! Build something legendary.".bright_magenta().bold());
            break;
        }
    }
}

fn fetch_project_interactive(
    is_ai: bool,
    is_github: bool,
    domain: Option<Domain>,
    difficulty: Option<Difficulty>,
    topic: Option<&str>,
) -> Option<ProjectIdea> {
    if is_ai {
        ui::play_ai_animation("AI");
        match ai::generate_ai_project(None, domain, difficulty, topic, None) {
            Ok(p) => Some(p),
            Err(e) => {
                println!("{}", format!("⚠️  AI Generation Notice: {}", e).yellow());
                println!("{}", "🔄 Falling back to curated offline blueprints...".bright_cyan());
                database::pick_random(domain, difficulty).cloned()
            }
        }
    } else if is_github {
        ui::play_github_animation();
        match github::fetch_github_project(domain, difficulty, topic, None) {
            Ok(p) => Some(p),
            Err(e) => {
                println!("{}", format!("⚠️  GitHub Explorer Notice: {}", e).yellow());
                println!("{}", "🔄 Falling back to curated offline blueprints...".bright_cyan());
                database::pick_random(domain, difficulty).cloned()
            }
        }
    } else {
        ui::play_randomizer_animation();
        database::pick_random(domain, difficulty).cloned()
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

pub fn generate_markdown_content(p: &ProjectIdea) -> String {
    let mut md = String::new();
    let source_label = if p.id.starts_with("gh-") {
        "🐙 GitHub Live Discovery"
    } else if p.id.starts_with("ai-gen-") {
        "🤖 AI Architect Generated"
    } else {
        "💾 Curated Blueprint"
    };

    md.push_str(&format!("# {}\n\n", p.title));
    md.push_str(&format!(
        "> **Source**: {} | **Domain**: {} | **Difficulty**: {} | **Estimated Duration**: {}\n\n",
        source_label, p.domain, p.difficulty, p.duration
    ));

    md.push_str("## 📋 Description\n\n");
    md.push_str(&format!("{}\n\n", p.description));

    md.push_str("## 🛠️ Recommended Tech Stack\n\n");
    for tech in &p.technologies {
        md.push_str(&format!("- `{}`\n", tech));
    }
    md.push_str("\n");

    md.push_str("## ✅ Core Requirements\n\n");
    for req in &p.requirements {
        md.push_str(&format!("- [ ] {}\n", req));
    }
    md.push_str("\n");

    md.push_str("## 🗺️ Step-by-Step Implementation Roadmap\n\n");
    for (i, step) in p.steps.iter().enumerate() {
        md.push_str(&format!("{}. **Phase {}**: {}\n", i + 1, i + 1, step));
    }
    md.push_str("\n");

    md.push_str("## 📦 Starter Code\n\n");
    md.push_str(&format!("**File**: `{}` | **Language**: `{}`\n\n", p.starter_code_filename, p.starter_code_language));
    md.push_str(&format!(
        "```{}\n{}\n```\n\n",
        p.starter_code_language, p.starter_code
    ));

    md.push_str("## 📖 Documentation & Architecture\n\n");
    md.push_str(&format!("{}\n", p.documentation));

    md
}

fn save_markdown_file(p: &ProjectIdea, path_str: &str) {
    let target_path_str = if path_str.ends_with(".md") {
        path_str.to_string()
    } else {
        format!("{}.md", path_str)
    };
    let path = Path::new(&target_path_str);
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            let _ = fs::create_dir_all(parent);
        }
    }

    let content = generate_markdown_content(p);
    match fs::write(path, content) {
        Ok(_) => {
            println!();
            println!("{}", "🎉 PROJECT SAVED TO MARKDOWN SUCCESSFULLY!".bold().bright_green());
            println!("  📄 Markdown File: {}", path.display().to_string().bright_cyan().bold());
            println!();
        }
        Err(e) => {
            eprintln!("{}", format!("❌ Failed to write Markdown file: {}", e).bright_red());
        }
    }
}
