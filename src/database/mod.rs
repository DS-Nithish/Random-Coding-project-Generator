pub mod ai;
pub mod cli_systems;
pub mod cyber;
pub mod game;
pub mod mobile;
pub mod web;

use crate::models::{Difficulty, Domain, ProjectIdea};
use rand::seq::SliceRandom;
use std::sync::OnceLock;

static ALL_PROJECTS: OnceLock<Vec<ProjectIdea>> = OnceLock::new();

pub fn get_all() -> &'static [ProjectIdea] {
    ALL_PROJECTS.get_or_init(|| {
        let mut list = Vec::new();
        list.extend(web::get_projects());
        list.extend(mobile::get_projects());
        list.extend(game::get_projects());
        list.extend(ai::get_projects());
        list.extend(cli_systems::get_projects());
        list.extend(cyber::get_projects());
        list
    })
}

pub fn filter(domain: Option<Domain>, difficulty: Option<Difficulty>) -> Vec<&'static ProjectIdea> {
    get_all()
        .iter()
        .filter(|p| {
            if let Some(d) = domain {
                if p.domain != d {
                    return false;
                }
            }
            if let Some(diff) = difficulty {
                if p.difficulty != diff {
                    return false;
                }
            }
            true
        })
        .collect()
}

pub fn pick_random(
    domain: Option<Domain>,
    difficulty: Option<Difficulty>,
) -> Option<&'static ProjectIdea> {
    let pool = filter(domain, difficulty);
    if pool.is_empty() {
        None
    } else {
        let mut rng = rand::thread_rng();
        pool.choose(&mut rng).copied()
    }
}
