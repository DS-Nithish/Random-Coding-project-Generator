use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl Difficulty {
    #[allow(dead_code)]
    pub fn all() -> &'static [Difficulty] {
        &[Difficulty::Beginner, Difficulty::Intermediate, Difficulty::Advanced]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Difficulty::Beginner => "Beginner",
            Difficulty::Intermediate => "Intermediate",
            Difficulty::Advanced => "Advanced",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "beginner" | "easy" | "beg" => Some(Difficulty::Beginner),
            "intermediate" | "med" | "medium" | "inter" => Some(Difficulty::Intermediate),
            "advanced" | "hard" | "adv" => Some(Difficulty::Advanced),
            _ => None,
        }
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Domain {
    WebDev,
    MobileApp,
    GameDev,
    AiMachineLearning,
    CliSystems,
    Cybersecurity,
}

impl Domain {
    #[allow(dead_code)]
    pub fn all() -> &'static [Domain] {
        &[
            Domain::WebDev,
            Domain::MobileApp,
            Domain::GameDev,
            Domain::AiMachineLearning,
            Domain::CliSystems,
            Domain::Cybersecurity,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Domain::WebDev => "Web Development",
            Domain::MobileApp => "Mobile App Development",
            Domain::GameDev => "Game Development",
            Domain::AiMachineLearning => "AI & Machine Learning",
            Domain::CliSystems => "CLI & System Tools",
            Domain::Cybersecurity => "Cybersecurity & Networking",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Domain::WebDev => "web",
            Domain::MobileApp => "mobile",
            Domain::GameDev => "game",
            Domain::AiMachineLearning => "ai",
            Domain::CliSystems => "cli",
            Domain::Cybersecurity => "cyber",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "web" | "webdev" | "web development" => Some(Domain::WebDev),
            "mobile" | "app" | "mobile app" => Some(Domain::MobileApp),
            "game" | "gamedev" | "game development" => Some(Domain::GameDev),
            "ai" | "ml" | "ai/ml" | "machine learning" => Some(Domain::AiMachineLearning),
            "cli" | "systems" | "tools" | "system" => Some(Domain::CliSystems),
            "cyber" | "security" | "cybersecurity" | "network" => Some(Domain::Cybersecurity),
            _ => None,
        }
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectIdea {
    pub id: String,
    pub title: String,
    pub domain: Domain,
    pub difficulty: Difficulty,
    pub description: String,
    pub requirements: Vec<String>,
    pub technologies: Vec<String>,
    pub duration: String,
    pub steps: Vec<String>,
    pub starter_code_language: String,
    pub starter_code_filename: String,
    pub starter_code: String,
    pub documentation: String,
}
