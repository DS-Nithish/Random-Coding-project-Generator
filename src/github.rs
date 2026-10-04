use crate::models::{Difficulty, Domain, ProjectIdea};
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct GithubSearchResponse {
    items: Vec<GithubRepoItem>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GithubRepoItem {
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub stargazers_count: u64,
    pub language: Option<String>,
    pub topics: Option<Vec<String>>,
    pub default_branch: Option<String>,
}

/// Fetch project blueprints and starter templates from GitHub.
pub fn fetch_github_project(
    domain: Option<Domain>,
    difficulty: Option<Difficulty>,
    custom_query: Option<&str>,
    token: Option<&str>,
) -> Result<ProjectIdea, String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Random-Coding-Project-Generator/1.0")
        .timeout(Duration::from_secs(12))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    // Strategy 1: If domain matches and no custom query, we can try to fetch from florinpop17/app-ideas or GitHub search
    // Let's perform a smart GitHub search for curated templates / starter kits / projects
    let query_term = if let Some(q) = custom_query {
        q.to_string()
    } else {
        build_search_query(domain, difficulty)
    };

    let url = format!(
        "https://api.github.com/search/repositories?q={}&sort=stars&order=desc&per_page=30",
        urlencoding_encode(&query_term)
    );

    let mut request = client.get(&url);
    if let Some(t) = token {
        request = request.header("Authorization", format!("Bearer {}", t));
    } else if let Ok(t) = std::env::var("GITHUB_TOKEN") {
        request = request.header("Authorization", format!("Bearer {}", t));
    }

    let response = request
        .send()
        .map_err(|e| format!("Network error querying GitHub: {}. Check internet connection.", e))?;

    if response.status() == reqwest::StatusCode::FORBIDDEN {
        return Err(
            "GitHub API rate limit exceeded. Set GITHUB_TOKEN environment variable to get 5,000 requests/hour."
                .to_string(),
        );
    }

    if !response.status().is_success() {
        return Err(format!("GitHub API returned error: {}", response.status()));
    }

    let search_res: GithubSearchResponse = response
        .json()
        .map_err(|e| format!("Failed to parse GitHub response: {}", e))?;

    if search_res.items.is_empty() {
        return Err(format!("No GitHub repositories found matching query: '{}'", query_term));
    }

    // Pick a random repository from top results for variety
    let mut rng = rand::thread_rng();
    let sample_pool: Vec<&GithubRepoItem> = search_res.items.iter().take(15).collect();
    let repo = sample_pool
        .choose(&mut rng)
        .copied()
        .ok_or_else(|| "Failed to pick repository".to_string())?;

    // Try fetching the repository README for rich documentation
    let readme_content = fetch_readme(&client, &repo.full_name, repo.default_branch.as_deref());

    // Map GitHub repository to our ProjectIdea model
    let target_domain = domain.unwrap_or(infer_domain_from_repo(repo));
    let target_diff = difficulty.unwrap_or(infer_difficulty_from_stars(repo.stargazers_count));

    let main_lang = repo.language.clone().unwrap_or_else(|| "Markdown".to_string());
    let (_code_ext, _filename) = language_to_starter_filename(&main_lang, &repo.name);

    let starter_script = format!(
        "#!/usr/bin/env bash\n# =====================================================================\n# Project: {}\n# Repository: {}\n# Stars: ⭐ {}\n# =====================================================================\n\necho \"🚀 Setting up {}...\"\n\n# 1. Clone repository\ngit clone {}\ncd {}\n\necho \"✅ Repository cloned successfully! Follow instructions in README.md.\"\n",
        repo.name,
        repo.html_url,
        repo.stargazers_count,
        repo.name,
        repo.html_url,
        repo.name
    );

    let doc = if let Some(readme) = readme_content {
        // Truncate to first ~35 lines for clean terminal viewing if huge
        let lines: Vec<&str> = readme.lines().take(35).collect();
        format!(
            "# {} (GitHub Open Source Blueprint)\n\n🔗 Repository: {}\n⭐ Stars: {}\n\n{}\n\n*(Full README available at {})*",
            repo.name, repo.html_url, repo.stargazers_count, lines.join("\n"), repo.html_url
        )
    } else {
        format!(
            "# {}\n\n🔗 Repository: {}\n⭐ Stars: {}\n\n## Description\n{}\n",
            repo.name,
            repo.html_url,
            repo.stargazers_count,
            repo.description.as_deref().unwrap_or("No description provided.")
        )
    };

    let mut tech_list = Vec::new();
    if let Some(ref l) = repo.language {
        tech_list.push(l.clone());
    }
    if let Some(ref topics) = repo.topics {
        for t in topics.iter().take(5) {
            if !tech_list.contains(t) {
                tech_list.push(t.clone());
            }
        }
    }
    if tech_list.is_empty() {
        tech_list.push("Git".to_string());
        tech_list.push("Open Source".to_string());
    }

    let description = repo.description.clone().unwrap_or_else(|| {
        format!(
            "An open-source {} project template from GitHub with ⭐ {} stars.",
            main_lang, repo.stargazers_count
        )
    });

    let requirements = vec![
        format!("Explore the open source architecture of '{}'", repo.full_name),
        format!("Clone the repository from {}", repo.html_url),
        "Review repository dependencies and setup the local environment".to_string(),
        "Implement custom features or solve an open issue labeled 'good-first-issue'".to_string(),
        "Write tests and document your enhancements".to_string(),
    ];

    let steps = vec![
        format!("Phase 1: Clone and inspect '{}' structure and documentation", repo.name),
        format!("Phase 2: Install dependencies ({}) and run the project locally", tech_list.join(", ")),
        "Phase 3: Study the core modules and identify areas for enhancement or refactoring".to_string(),
        "Phase 4: Build your custom feature or fix open community issues".to_string(),
        "Phase 5: Package, run integration tests, and submit a PR or publish your fork".to_string(),
    ];

    Ok(ProjectIdea {
        id: format!("gh-{}", repo.name.to_lowercase().replace(' ', "-")),
        title: format!("{} (⭐ {} Stars)", repo.name, repo.stargazers_count),
        domain: target_domain,
        difficulty: target_diff,
        description,
        requirements,
        technologies: tech_list,
        duration: "1 - 3 Weeks".to_string(),
        steps,
        starter_code_language: "bash".to_string(),
        starter_code_filename: "setup.sh".to_string(),
        starter_code: starter_script,
        documentation: doc,
    })
}

fn fetch_readme(client: &reqwest::blocking::Client, full_name: &str, branch: Option<&str>) -> Option<String> {
    let branches = [branch.unwrap_or("main"), "master"];
    for b in &branches {
        let raw_url = format!("https://raw.githubusercontent.com/{}/{}/README.md", full_name, b);
        if let Ok(res) = client.get(&raw_url).send() {
            if res.status().is_success() {
                if let Ok(text) = res.text() {
                    return Some(text);
                }
            }
        }
    }
    None
}

fn build_search_query(domain: Option<Domain>, _diff: Option<Difficulty>) -> String {
    match domain {
        Some(Domain::WebDev) => "starter template web stars:>50 fork:false".to_string(),
        Some(Domain::MobileApp) => "starter template mobile stars:>30 fork:false".to_string(),
        Some(Domain::GameDev) => "starter template game stars:>30 fork:false".to_string(),
        Some(Domain::AiMachineLearning) => "starter template machine-learning stars:>50 fork:false".to_string(),
        Some(Domain::CliSystems) => "starter template cli stars:>50 fork:false".to_string(),
        Some(Domain::Cybersecurity) => "cybersecurity security tool stars:>30 fork:false".to_string(),
        None => "starter-kit template stars:>100 fork:false".to_string(),
    }
}

fn infer_domain_from_repo(repo: &GithubRepoItem) -> Domain {
    let text = format!(
        "{} {} {}",
        repo.name.to_lowercase(),
        repo.description.as_deref().unwrap_or("").to_lowercase(),
        repo.topics.as_ref().map(|t| t.join(" ")).unwrap_or_default().to_lowercase()
    );

    if text.contains("game") || text.contains("bevy") || text.contains("godot") || text.contains("unity") {
        Domain::GameDev
    } else if text.contains("mobile") || text.contains("flutter") || text.contains("ios") || text.contains("android") {
        Domain::MobileApp
    } else if text.contains("ai") || text.contains("ml") || text.contains("gpt") || text.contains("neural") || text.contains("llm") {
        Domain::AiMachineLearning
    } else if text.contains("cli") || text.contains("terminal") || text.contains("tui") || text.contains("system") {
        Domain::CliSystems
    } else if text.contains("security") || text.contains("cyber") || text.contains("crypto") || text.contains("exploit") {
        Domain::Cybersecurity
    } else {
        Domain::WebDev
    }
}

fn infer_difficulty_from_stars(stars: u64) -> Difficulty {
    if stars < 300 {
        Difficulty::Beginner
    } else if stars < 2000 {
        Difficulty::Intermediate
    } else {
        Difficulty::Advanced
    }
}

fn language_to_starter_filename(lang: &str, project_name: &str) -> (&'static str, String) {
    let clean_name = project_name.to_lowercase().replace('-', "_");
    match lang.to_lowercase().as_str() {
        "rust" => ("rs", format!("{}.rs", clean_name)),
        "python" => ("py", format!("{}.py", clean_name)),
        "javascript" => ("js", format!("{}.js", clean_name)),
        "typescript" => ("ts", format!("{}.ts", clean_name)),
        "go" => ("go", format!("{}.go", clean_name)),
        "c++" | "cpp" => ("cpp", format!("{}.cpp", clean_name)),
        "html" => ("html", "index.html".to_string()),
        _ => ("sh", "setup.sh".to_string()),
    }
}

fn urlencoding_encode(s: &str) -> String {
    let mut encoded = String::new();
    for b in s.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            b' ' => encoded.push('+'),
            _ => encoded.push_str(&format!("%{:02X}", b)),
        }
    }
    encoded
}
