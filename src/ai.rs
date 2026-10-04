use crate::models::{Difficulty, Domain, ProjectIdea};
use serde_json::Value;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiProvider {
    Gemini,
    OpenAi,
    Ollama,
}

impl AiProvider {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "gemini" | "google" => Some(AiProvider::Gemini),
            "openai" | "chatgpt" | "gpt" => Some(AiProvider::OpenAi),
            "ollama" | "local" => Some(AiProvider::Ollama),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AiProvider::Gemini => "Google Gemini",
            AiProvider::OpenAi => "OpenAI GPT",
            AiProvider::Ollama => "Local Ollama",
        }
    }
}

pub fn generate_ai_project(
    provider: Option<AiProvider>,
    domain: Option<Domain>,
    difficulty: Option<Difficulty>,
    custom_topic: Option<&str>,
    api_key: Option<&str>,
) -> Result<ProjectIdea, String> {
    let resolved_provider = provider.unwrap_or_else(|| detect_provider(api_key));

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let prompt = build_system_prompt(domain, difficulty, custom_topic);

    match resolved_provider {
        AiProvider::Gemini => call_gemini(&client, &prompt, api_key),
        AiProvider::OpenAi => call_openai(&client, &prompt, api_key),
        AiProvider::Ollama => call_ollama(&client, &prompt),
    }
}

fn detect_provider(explicit_key: Option<&str>) -> AiProvider {
    if explicit_key.is_some() {
        return AiProvider::Gemini;
    }
    if std::env::var("GEMINI_API_KEY").is_ok() {
        return AiProvider::Gemini;
    }
    if std::env::var("OPENAI_API_KEY").is_ok() {
        return AiProvider::OpenAi;
    }
    // Check if Ollama is running locally
    if let Ok(client) = reqwest::blocking::Client::builder().timeout(Duration::from_millis(600)).build() {
        if client.get("http://localhost:11434/api/tags").send().is_ok() {
            return AiProvider::Ollama;
        }
    }
    // Default to Gemini (most common free tier API)
    AiProvider::Gemini
}

fn build_system_prompt(
    domain: Option<Domain>,
    difficulty: Option<Difficulty>,
    custom_topic: Option<&str>,
) -> String {
    let domain_str = domain
        .map(|d| d.as_str().to_string())
        .unwrap_or_else(|| "Any software engineering field (surprise me)".to_string());
    let diff_str = difficulty
        .map(|d| d.as_str().to_string())
        .unwrap_or_else(|| "Any difficulty".to_string());
    let topic_str = custom_topic
        .map(|t| format!("Specific inspiration/topic: {}", t))
        .unwrap_or_else(|| "Generate an exciting, modern, real-world project".to_string());

    format!(
        r#"You are an elite software architect and engineering mentor.
Design an innovative, highly engaging coding project blueprint based on:
- Domain: {domain}
- Difficulty: {difficulty}
- {topic}

Return ONLY valid JSON matching this exact schema:
{{
  "id": "slug-name-lowercase",
  "title": "Project Title",
  "domain": "WebDev | MobileApp | GameDev | AiMachineLearning | CliSystems | Cybersecurity",
  "difficulty": "Beginner | Intermediate | Advanced",
  "description": "2-3 sentences explaining what makes this project cool and what real-world problem it solves.",
  "requirements": [
    "Requirement 1",
    "Requirement 2",
    "Requirement 3",
    "Requirement 4",
    "Requirement 5"
  ],
  "technologies": ["Tech 1", "Tech 2", "Tech 3"],
  "duration": "e.g. 2 - 3 Days, or 1 - 2 Weeks",
  "steps": [
    "Phase 1: Setup and Architecture",
    "Phase 2: Core Logic and State",
    "Phase 3: Features & Integration",
    "Phase 4: Polish, Testing, and UI",
    "Phase 5: Deployment or Packaging"
  ],
  "starter_code_language": "e.g. rust, python, javascript, go, or html",
  "starter_code_filename": "e.g. main.rs, app.py, index.js, or main.go",
  "starter_code": "Complete, working, compilable/runnable single-file boilerplate with comments explaining next steps.",
  "documentation": "Markdown documentation containing # Overview, ## Architecture, ## Quick Start guide, and ## Next Steps."
}}

CRITICAL: Output ONLY raw JSON. Do NOT wrap in markdown backticks."#,
        domain = domain_str,
        difficulty = diff_str,
        topic = topic_str
    )
}

fn call_gemini(
    client: &reqwest::blocking::Client,
    prompt: &str,
    explicit_key: Option<&str>,
) -> Result<ProjectIdea, String> {
    let key = explicit_key
        .map(|s| s.to_string())
        .or_else(|| std::env::var("GEMINI_API_KEY").ok())
        .ok_or_else(|| {
            "GEMINI_API_KEY is not set. Export it in your shell: `export GEMINI_API_KEY=your_key` or use `--api-key <key>`.".to_string()
        })?;

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        key
    );

    let body = serde_json::json!({
        "contents": [{
            "parts": [{ "text": prompt }]
        }],
        "generationConfig": {
            "responseMimeType": "application/json",
            "temperature": 0.8
        }
    });

    let res = client
        .post(&url)
        .json(&body)
        .send()
        .map_err(|e| format!("Gemini API request failed: {}", e))?;

    if !res.status().is_success() {
        let err_text = res.text().unwrap_or_default();
        return Err(format!("Gemini API error: {}", err_text));
    }

    let val: Value = res
        .json()
        .map_err(|e| format!("Failed to parse Gemini response JSON: {}", e))?;

    let text = val["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or_else(|| "Gemini returned empty response".to_string())?;

    parse_clean_json_idea(text)
}

fn call_openai(
    client: &reqwest::blocking::Client,
    prompt: &str,
    explicit_key: Option<&str>,
) -> Result<ProjectIdea, String> {
    let key = explicit_key
        .map(|s| s.to_string())
        .or_else(|| std::env::var("OPENAI_API_KEY").ok())
        .ok_or_else(|| {
            "OPENAI_API_KEY is not set. Export it in your shell: `export OPENAI_API_KEY=your_key` or use `--api-key <key>`.".to_string()
        })?;

    let body = serde_json::json!({
        "model": "gpt-4o-mini",
        "messages": [
            { "role": "system", "content": "You are a software architect outputting JSON specifications for coding projects." },
            { "role": "user", "content": prompt }
        ],
        "response_format": { "type": "json_object" },
        "temperature": 0.8
    });

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", key))
        .json(&body)
        .send()
        .map_err(|e| format!("OpenAI request failed: {}", e))?;

    if !res.status().is_success() {
        let err_text = res.text().unwrap_or_default();
        return Err(format!("OpenAI API error: {}", err_text));
    }

    let val: Value = res
        .json()
        .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;

    let text = val["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| "OpenAI returned empty message".to_string())?;

    parse_clean_json_idea(text)
}

fn call_ollama(client: &reqwest::blocking::Client, prompt: &str) -> Result<ProjectIdea, String> {
    // Find model installed in Ollama
    let mut model_name = "llama3".to_string();
    if let Ok(res) = client.get("http://localhost:11434/api/tags").send() {
        if let Ok(val) = res.json::<Value>() {
            if let Some(models) = val["models"].as_array() {
                if let Some(first) = models.first() {
                    if let Some(name) = first["name"].as_str() {
                        model_name = name.to_string();
                    }
                }
            }
        }
    }

    let body = serde_json::json!({
        "model": model_name,
        "prompt": prompt,
        "stream": false,
        "format": "json"
    });

    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&body)
        .send()
        .map_err(|e| {
            format!(
                "Failed to connect to local Ollama (http://localhost:11434): {}. Is `ollama serve` running?",
                e
            )
        })?;

    if !res.status().is_success() {
        return Err(format!("Ollama returned error: {}", res.status()));
    }

    let val: Value = res
        .json()
        .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

    let text = val["response"]
        .as_str()
        .ok_or_else(|| "Ollama returned empty response".to_string())?;

    parse_clean_json_idea(text)
}

fn parse_clean_json_idea(raw: &str) -> Result<ProjectIdea, String> {
    // Strip markdown code fences if model enclosed it in ```json ... ```
    let clean = raw.trim();
    let json_str = if clean.starts_with("```") {
        let lines: Vec<&str> = clean.lines().collect();
        if lines.len() >= 2 {
            let start = 1;
            let end = if lines.last().map_or(false, |l| l.starts_with("```")) {
                lines.len() - 1
            } else {
                lines.len()
            };
            lines[start..end].join("\n")
        } else {
            clean.to_string()
        }
    } else {
        clean.to_string()
    };

    let mut idea = serde_json::from_str::<ProjectIdea>(&json_str).map_err(|e| {
        format!(
            "Failed to parse generated idea into project schema: {}.\nResponse snippet: {}",
            e,
            json_str.chars().take(200).collect::<String>()
        )
    })?;

    if !idea.id.starts_with("ai-gen-") {
        idea.id = format!("ai-gen-{}", idea.id);
    }

    Ok(idea)
}
