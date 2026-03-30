use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "whoami-validator")]
#[command(about = "Validate whoami.toml profile files")]
struct Cli {
    /// Path to whoami.toml file to validate
    path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct Profile {
    version: String,
    #[serde(default)]
    person: Option<Person>,
    #[serde(default)]
    communication: Option<Communication>,
    #[serde(default)]
    technical: Option<Technical>,
    #[serde(default)]
    preferences: Option<toml::Value>,
    #[serde(default)]
    domains: Option<toml::Value>,
    #[serde(default)]
    projects: Option<Projects>,
    #[serde(default)]
    context: Option<toml::Value>,
    #[serde(default)]
    boundaries: Option<toml::Value>,
    #[serde(default)]
    api_keys: Option<toml::Value>,
}

#[derive(Debug, Deserialize)]
struct Person {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    username: Option<String>,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    roles: Vec<String>,
    #[serde(default)]
    pronouns: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Communication {
    #[serde(default)]
    style: Option<String>,
    #[serde(default)]
    code_comments: Option<String>,
    #[serde(default)]
    emoji_in_code: Option<bool>,
    #[serde(default)]
    emoji_in_commits: Option<bool>,
    #[serde(default)]
    tone: Option<String>,
    #[serde(default)]
    explanations: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Technical {
    #[serde(default)]
    languages: Option<toml::Value>,
    #[serde(default)]
    frameworks: Option<toml::Value>,
    #[serde(default)]
    tools: Option<toml::Value>,
}

#[derive(Debug, Deserialize)]
struct Projects {
    #[serde(default)]
    active: Vec<Project>,
}

#[derive(Debug, Deserialize)]
struct Project {
    name: String,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    tech: Vec<String>,
    #[serde(default)]
    url: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let content = fs::read_to_string(&cli.path)
        .with_context(|| format!("Failed to read file: {}", cli.path.display()))?;

    let profile: Profile = toml::from_str(&content)
        .context("Failed to parse TOML")?;

    // Validate version
    if profile.version.is_empty() {
        anyhow::bail!("version field is required and cannot be empty");
    }

    // Check version format (should be YYYYMMDD)
    if profile.version.len() != 8 || !profile.version.chars().all(|c| c.is_ascii_digit()) {
        eprintln!("Warning: version should be in format YYYYMMDD (e.g., 20260330)");
    }

    // Validate projects have required fields
    if let Some(projects) = &profile.projects {
        for (i, project) in projects.active.iter().enumerate() {
            if project.name.is_empty() {
                anyhow::bail!("Project {} has empty name field", i);
            }
        }
    }

    println!("✓ Valid whoami.toml");
    println!("  Version: {}", profile.version);

    if let Some(person) = &profile.person {
        if let Some(name) = &person.name {
            println!("  Name: {}", name);
        }
        if !person.roles.is_empty() {
            println!("  Roles: {}", person.roles.join(", "));
        }
    }

    if let Some(projects) = &profile.projects {
        if !projects.active.is_empty() {
            println!("  Projects: {} active", projects.active.len());
        }
    }

    Ok(())
}
