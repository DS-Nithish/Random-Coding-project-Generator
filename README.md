# ⚡ Random Coding Project Generator CLI

A high-performance, colorful terminal CLI application built in **Rust** that generates curated coding project ideas across various domains and skill levels. Designed according to the specifications in [implementation_plan.md](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/implementation_plan.md).

---
![alt text](<Screenshot 2026-10-01 at 11.16.10 PM.png>)

## 🌟 Key Features

- 🎨 **Vibrant Terminal Aesthetics**: ANSI 256-color gradient banners, difficulty badges, tech tags, and animated randomizer spinners.
- 🎯 **Domain & Level Filtering**:
  - **Domains**: Web Development (`web`), Mobile Apps (`mobile`), Game Development (`game`), AI & Machine Learning (`ai`), CLI & Systems (`cli`), Cybersecurity & Networking (`cyber`).
  - **Skill Levels**: Beginner, Intermediate, Advanced.
- 📚 **Extensive Project Catalog**: 60 meticulously detailed projects equipped with real-world requirements, suggested tech stacks, and estimated durations.
- 🗺️ **Step-by-Step Roadmaps (On-Demand)**: Clear phase-by-phase implementation milestones for each project idea.
- 📦 **Self-Contained Starter Code (On-Demand)**: Ready-to-run, single-file templates in Python, Rust, JavaScript, HTML5/CSS, Go, Swift, and C.
- 📖 **Comprehensive Documentation (On-Demand)**: Ready-to-use READMEs, architecture diagrams, and deployment guidance.
- 💾 **One-Click Project Export**: Automatically scaffold a new project directory with `README.md`, starter code, and structured `project_spec.json`.
- 🕹️ **Dual Mode Operation**:
  - **Interactive Wizard**: Step-by-step interactive menus with arrow-key navigation.
  - **CLI Flags Mode**: Direct command-line automation for scripts and quick lookups.

---

## 🚀 Quick Start

### 1. Build and Run Interactively

Launch the interactive prompt to explore ideas with keyboard navigation:

```bash
cargo run
```

Or explicitly:

```bash
cargo run -- --interactive
```

---

## 💻 CLI Flags & Options

You can specify direct filters and choose exactly which deliverables you want to see:

| Flag | Long Flag | Description |
|---|---|---|
| `-l` | `--level` | Filter by difficulty: `beginner`, `intermediate`, `advanced` |
| `-d` | `--domain` | Filter by domain: `web`, `mobile`, `game`, `ai`, `cli`, `cyber` |
| | `--steps` | Display the step-by-step roadmap |
| | `--code` | Display the self-contained starter code |
| | `--doc` | Display the project documentation and README |
| `-e` | `--export` | Export project deliverables into a folder |
| `-o` | `--output` | Destination folder for export (default: `./generated_project`) |
| | `--list` | List all 60 projects in the database |
| `-h` | `--help` | Show help and options |

---

## 📖 Usage Examples

### 1. List All Available Projects
```bash
cargo run -- --list
```

### 2. Generate a Random AI Project with Full Deliverables
```bash
cargo run -- -d ai -l advanced --steps --code --doc
```

### 3. Generate a Beginner Web Project and Export to Disk
```bash
cargo run -- -d web -l beginner --export --output ./my_new_web_project
```

### 4. Surprise Me (Random Game Development Project)
```bash
cargo run -- -d game
```

---

## 📁 Architecture & File Layout

- [src/main.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/main.rs): CLI entrypoint, argument dispatch, interactive menu loop, and project file exporter.
- [src/models.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/models.rs): Enums (`Domain`, `Difficulty`) and data models (`ProjectIdea`).
- [src/ui.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/ui.rs): ANSI color scheme, progress spinner, badges, and card formatting.
- [src/cli.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/cli.rs): Clap command-line parser definition.
- [src/database/](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/database):
  - [web.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/database/web.rs): Web development projects.
  - [mobile.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/database/mobile.rs): Mobile app development projects.
  - [game.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/database/game.rs): Game development projects.
  - [ai.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/database/ai.rs): AI & Machine Learning projects.
  - [cli_systems.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/database/cli_systems.rs): CLI & System Tools projects.
  - [cyber.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/database/cyber.rs): Cybersecurity & Networking projects.
