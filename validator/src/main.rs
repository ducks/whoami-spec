use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "whoami-validator")]
#[command(about = "Validate and create whoami.toml profile files")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to whoami.toml file to validate (if no subcommand)
    path: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new whoami.toml profile interactively
    Init {
        /// Output path (default: ~/.config/agent/whoami.toml)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
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

    match cli.command {
        Some(Commands::Init { output }) => init_profile(output),
        None => {
            let path = cli.path.context("Path to whoami.toml required (or use 'init' subcommand)")?;
            validate_profile(path)
        }
    }
}

fn init_profile(output: Option<PathBuf>) -> Result<()> {
    let theme = ColorfulTheme::default();

    let default_path = PathBuf::from(format!(
        "{}/.config/agent/whoami.toml",
        env::var("HOME").unwrap_or_else(|_| ".".to_string())
    ));

    let output_path = output.unwrap_or(default_path);

    println!("\nCreating whoami profile at: {}\n", output_path.display());

    // Person section
    let name: String = Input::with_theme(&theme)
        .with_prompt("Name")
        .interact_text()?;

    let roles: String = Input::with_theme(&theme)
        .with_prompt("Roles (comma-separated)")
        .with_initial_text("software engineer")
        .interact_text()?;
    let roles: Vec<String> = roles.split(',').map(|s| s.trim().to_string()).collect();

    let pronouns: String = Input::with_theme(&theme)
        .with_prompt("Pronouns (optional)")
        .allow_empty(true)
        .interact_text()?;

    // Communication section
    let style: String = Input::with_theme(&theme)
        .with_prompt("Communication style")
        .with_initial_text("clear and concise")
        .interact_text()?;

    let code_comments: String = Input::with_theme(&theme)
        .with_prompt("When to add code comments")
        .with_initial_text("for non-obvious logic")
        .interact_text()?;

    let emoji_in_code = Confirm::with_theme(&theme)
        .with_prompt("Use emoji in code?")
        .default(false)
        .interact()?;

    let emoji_in_commits = Confirm::with_theme(&theme)
        .with_prompt("Use emoji in commits?")
        .default(false)
        .interact()?;

    // Technical section
    let primary_languages: String = Input::with_theme(&theme)
        .with_prompt("Primary languages (comma-separated)")
        .with_initial_text("python, javascript")
        .interact_text()?;
    let primary_languages: Vec<String> = primary_languages.split(',').map(|s| s.trim().to_string()).collect();

    let editor: String = Input::with_theme(&theme)
        .with_prompt("Editor")
        .with_initial_text("vscode")
        .interact_text()?;

    let shell: String = Input::with_theme(&theme)
        .with_prompt("Shell")
        .with_initial_text("bash")
        .interact_text()?;

    // Projects
    let mut projects = Vec::new();
    if Confirm::with_theme(&theme)
        .with_prompt("Add active projects?")
        .default(false)
        .interact()?
    {
        loop {
            println!();
            let project_name: String = Input::with_theme(&theme)
                .with_prompt("  Project name")
                .interact_text()?;

            let project_path: String = Input::with_theme(&theme)
                .with_prompt("  Path")
                .allow_empty(true)
                .interact_text()?;

            let project_desc: String = Input::with_theme(&theme)
                .with_prompt("  Description")
                .allow_empty(true)
                .interact_text()?;

            let project_tech: String = Input::with_theme(&theme)
                .with_prompt("  Tech (comma-separated)")
                .allow_empty(true)
                .interact_text()?;
            let project_tech: Vec<String> = if project_tech.is_empty() {
                vec![]
            } else {
                project_tech.split(',').map(|s| s.trim().to_string()).collect()
            };

            projects.push((project_name, project_path, project_desc, project_tech));

            if !Confirm::with_theme(&theme)
                .with_prompt("  Add another project?")
                .default(false)
                .interact()?
            {
                break;
            }
        }
    }

    // Generate TOML
    let mut toml = String::new();
    toml.push_str("version = \"20260330\"\n\n");

    // Person
    toml.push_str("[person]\n");
    toml.push_str(&format!("name = \"{}\"\n", name));
    toml.push_str(&format!("roles = [{}]\n",
        roles.iter().map(|r| format!("\"{}\"", r)).collect::<Vec<_>>().join(", ")));
    if !pronouns.is_empty() {
        toml.push_str(&format!("pronouns = \"{}\"\n", pronouns));
    }
    toml.push('\n');

    // Communication
    toml.push_str("[communication]\n");
    toml.push_str(&format!("style = \"{}\"\n", style));
    toml.push_str(&format!("code_comments = \"{}\"\n", code_comments));
    toml.push_str(&format!("emoji_in_code = {}\n", emoji_in_code));
    toml.push_str(&format!("emoji_in_commits = {}\n", emoji_in_commits));
    toml.push('\n');

    // Technical
    toml.push_str("[technical.languages]\n");
    toml.push_str(&format!("primary = [{}]\n",
        primary_languages.iter().map(|l| format!("\"{}\"", l)).collect::<Vec<_>>().join(", ")));
    toml.push('\n');

    toml.push_str("[technical.tools]\n");
    toml.push_str(&format!("editor = \"{}\"\n", editor));
    toml.push_str(&format!("shell = \"{}\"\n", shell));
    toml.push('\n');

    // Projects
    if !projects.is_empty() {
        for (name, path, desc, tech) in projects {
            toml.push_str("[[projects.active]]\n");
            toml.push_str(&format!("name = \"{}\"\n", name));
            if !path.is_empty() {
                toml.push_str(&format!("path = \"{}\"\n", path));
            }
            if !desc.is_empty() {
                toml.push_str(&format!("description = \"{}\"\n", desc));
            }
            if !tech.is_empty() {
                toml.push_str(&format!("tech = [{}]\n",
                    tech.iter().map(|t| format!("\"{}\"", t)).collect::<Vec<_>>().join(", ")));
            }
            toml.push('\n');
        }
    }

    // Create parent directory if needed
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write file
    fs::write(&output_path, toml)?;

    println!("\n✓ Created {}", output_path.display());
    println!("  Run: whoami-validator {}", output_path.display());

    Ok(())
}

fn validate_profile(path: PathBuf) -> Result<()> {
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

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
