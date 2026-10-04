# ⚡ Random Coding Project Generator CLI

A high-performance, colorful terminal CLI application built in **Rust** that generates curated coding project ideas across various domains and skill levels. Designed according to the specifications in [implementation_plan.md](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/implementation_plan.md).

---
![alt text](<Screenshot 2026-10-01 at 11.16.10 PM.png>)

## 🌟 Key Features

- 🎨 **Vibrant Terminal Aesthetics**: ANSI 256-color gradient banners, difficulty badges, tech tags, and animated randomizer spinners.
- 🎯 **Domain & Level Filtering**:
  - **Domains**: Web Development (`web`), Mobile Apps (`mobile`), Game Development (`game`), AI & Machine Learning (`ai`), CLI & Systems (`cli`), Cybersecurity & Networking (`cyber`).
  - **Skill Levels**: Beginner, Intermediate, Advanced.
- 💾 **Curated Offline Catalog**: 60 meticulously detailed projects equipped with real-world requirements, suggested tech stacks, and estimated durations.
- 🤖 **AI Architect Generation**: Infinite, customized project blueprints generated on-the-fly with bespoke starter code and roadmaps via Google Gemini, OpenAI, or local Ollama.
- 🐙 **GitHub Live Explorer**: Discover real-world open-source templates, boilerplates, and trending projects directly from GitHub.
- 🛡️ **Offline Fallback Resilience**: Seamless fallback to offline curated blueprints whenever network or API keys are unavailable.
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

Launch the interactive prompt to choose your project source (Curated Catalog, AI Architect, or GitHub Live):

```bash
cargo run
```

Or explicitly:

```bash
cargo run -- --interactive
```

---

## 💻 CLI Flags & Options

You can specify direct filters, generation sources, and choose exactly which deliverables you want to see:

| Flag | Long Flag | Description |
|---|---|---|
| `-l` | `--level` | Filter by difficulty: `beginner`, `intermediate`, `advanced` |
| `-d` | `--domain` | Filter by domain: `web`, `mobile`, `game`, `ai`, `cli`, `cyber` |
| | `--ai` | Generate an infinite dynamic project blueprint using AI |
| | `--provider` | AI Provider: `gemini` (default), `openai`, or `ollama` |
| | `--api-key` | AI API Key (or set `GEMINI_API_KEY` / `OPENAI_API_KEY`) |
| `-t` | `--topic` | Custom topic/keywords for AI prompt or GitHub search |
| `-g` | `--github` | Discover live open-source project templates from GitHub |
| | `--github-token` | GitHub access token (optional, increases rate limit to 5000 req/hr) |
| | `--steps` | Display the step-by-step roadmap |
| | `--code` | Display the self-contained starter code |
| | `--doc` | Display the project documentation and README |
| `-e` | `--export` | Export project deliverables into a folder |
| `-o` | `--output` | Destination folder for export (default: `./generated_project`) |
| `-m` | `--save-md` | Save complete project blueprint as a standalone Markdown (`.md`) file |
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

### 5. Discover a Live Project from GitHub (Web / React / Templates)
```bash
cargo run -- --github --domain web
```

### 6. Search GitHub for a Specific Topic & Export
```bash
cargo run -- --github --topic "rust tui tool" --steps --export --output ./my_tui_app
```

### 7. Generate an Infinite AI Project Blueprint (Gemini, OpenAI, or Ollama)
```bash
# Using Gemini (with GEMINI_API_KEY environment variable or --api-key)
cargo run -- --ai --domain ai --level advanced --topic "local multimodal vector search" --steps --code

# Using local Ollama (100% free and private, no key required!)
cargo run -- --ai --provider ollama --topic "distributed key-value store in rust"
```

### 8. Save Project as a Standalone Markdown File
```bash
# Save to default filename (e.g. project_id.md)
cargo run -- -d web -l intermediate --save-md

# Save to a custom Markdown file path
cargo run -- -d cyber -l advanced --save-md ./security_audit_spec.md
```

---

## 📁 Architecture & File Layout

- [src/main.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/main.rs): CLI entrypoint, argument dispatch, interactive menu loop, and project file exporter.
- [src/ai.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/ai.rs): Dynamic AI project generator supporting Google Gemini, OpenAI, and local Ollama.
- [src/github.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/github.rs): Live GitHub repository and template discovery engine via GitHub REST API.
- [src/models.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/models.rs): Enums (`Domain`, `Difficulty`) and data models (`ProjectIdea`).
- [src/ui.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/ui.rs): ANSI color scheme, progress spinners, source badges, and card formatting.
- [src/cli.rs](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/cli.rs): Clap command-line parser definition.
- [src/database/](file:///Users/nithish/Development/Projects/Random-Coding-project-Generator/src/database): Curated offline blueprint catalog across Web, Mobile, Game, AI, CLI, and Cyber domains.
