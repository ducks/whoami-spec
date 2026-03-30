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
    /// Show your whoami profile in a readable format
    Show {
        /// Path to whoami.toml (default: $AGENT_WHOAMI or ~/.config/agent/whoami.toml)
        path: Option<PathBuf>,
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
        Some(Commands::Show { path }) => show_profile(path),
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

fn show_profile(path: Option<PathBuf>) -> Result<()> {
    // Resolve path: CLI arg -> $AGENT_WHOAMI -> default
    let profile_path = if let Some(p) = path {
        p
    } else if let Ok(env_path) = env::var("AGENT_WHOAMI") {
        PathBuf::from(env_path)
    } else {
        PathBuf::from(format!(
            "{}/.config/agent/whoami.toml",
            env::var("HOME").unwrap_or_else(|_| ".".to_string())
        ))
    };

    let content = fs::read_to_string(&profile_path)
        .with_context(|| format!("Failed to read file: {}", profile_path.display()))?;

    let profile: Profile = toml::from_str(&content)
        .context("Failed to parse TOML")?;

    println!("\n{}\n", "=".repeat(60));
    println!("  whoami profile: {}", profile_path.display());
    println!("{}\n", "=".repeat(60));

    // Person section
    if let Some(person) = &profile.person {
        println!("[Person]");
        if let Some(name) = &person.name {
            println!("  Name: {}", name);
        }
        if let Some(username) = &person.username {
            println!("  Username: {}", username);
        }
        if let Some(email) = &person.email {
            println!("  Email: {}", email);
        }
        if !person.roles.is_empty() {
            println!("  Roles: {}", person.roles.join(", "));
        }
        if let Some(pronouns) = &person.pronouns {
            println!("  Pronouns: {}", pronouns);
        }
        println!();
    }

    // Communication section
    if let Some(comm) = &profile.communication {
        println!("[Communication]");
        if let Some(style) = &comm.style {
            println!("  Style: {}", style);
        }
        if let Some(code_comments) = &comm.code_comments {
            println!("  Code comments: {}", code_comments);
        }
        if let Some(emoji_code) = comm.emoji_in_code {
            println!("  Emoji in code: {}", emoji_code);
        }
        if let Some(emoji_commits) = comm.emoji_in_commits {
            println!("  Emoji in commits: {}", emoji_commits);
        }
        if let Some(tone) = &comm.tone {
            println!("  Tone: {}", tone);
        }
        if let Some(explanations) = &comm.explanations {
            println!("  Explanations: {}", explanations);
        }
        println!();
    }

    // Technical section
    if let Some(tech) = &profile.technical {
        println!("[Technical]");

        if let Some(languages) = &tech.languages {
            if let Some(table) = languages.as_table() {
                for (key, value) in table {
                    if let Some(arr) = value.as_array() {
                        let items: Vec<String> = arr.iter()
                            .filter_map(|v| v.as_str())
                            .map(|s| s.to_string())
                            .collect();
                        println!("  Languages ({}): {}", key, items.join(", "));
                    }
                }
            }
        }

        if let Some(frameworks) = &tech.frameworks {
            if let Some(table) = frameworks.as_table() {
                for (key, value) in table {
                    if let Some(arr) = value.as_array() {
                        let items: Vec<String> = arr.iter()
                            .filter_map(|v| v.as_str())
                            .map(|s| s.to_string())
                            .collect();
                        println!("  Frameworks ({}): {}", key, items.join(", "));
                    }
                }
            }
        }

        if let Some(tools) = &tech.tools {
            if let Some(table) = tools.as_table() {
                for (key, value) in table {
                    if let Some(s) = value.as_str() {
                        println!("  {}: {}", key, s);
                    } else if let Some(arr) = value.as_array() {
                        let items: Vec<String> = arr.iter()
                            .filter_map(|v| v.as_str())
                            .map(|s| s.to_string())
                            .collect();
                        println!("  {}: {}", key, items.join(", "));
                    }
                }
            }
        }
        println!();
    }

    // Projects section
    if let Some(projects) = &profile.projects {
        if !projects.active.is_empty() {
            println!("[Projects] ({} active)", projects.active.len());
            for project in &projects.active {
                println!("  • {}", project.name);
                if let Some(path) = &project.path {
                    println!("    Path: {}", path);
                }
                if let Some(desc) = &project.description {
                    println!("    Description: {}", desc);
                }
                if !project.tech.is_empty() {
                    println!("    Tech: {}", project.tech.join(", "));
                }
                if let Some(url) = &project.url {
                    println!("    URL: {}", url);
                }
            }
            println!();
        }
    }

    // Preferences section
    if let Some(preferences) = &profile.preferences {
        if let Some(table) = preferences.as_table() {
            if !table.is_empty() {
                println!("[Preferences]");
                print_toml_table(table, 1);
                println!();
            }
        }
    }

    // Domains section
    if let Some(domains) = &profile.domains {
        if let Some(table) = domains.as_table() {
            if !table.is_empty() {
                println!("[Domains]");
                print_toml_table(table, 1);
                println!();
            }
        }
    }

    // Context section
    if let Some(context) = &profile.context {
        if let Some(table) = context.as_table() {
            if !table.is_empty() {
                println!("[Context]");
                print_toml_table(table, 1);
                println!();
            }
        }
    }

    // Boundaries section
    if let Some(boundaries) = &profile.boundaries {
        if let Some(table) = boundaries.as_table() {
            if !table.is_empty() {
                println!("[Boundaries]");
                print_toml_table(table, 1);
                println!();
            }
        }
    }

    // API Keys section
    if let Some(api_keys) = &profile.api_keys {
        if let Some(table) = api_keys.as_table() {
            if !table.is_empty() {
                println!("[API Keys]");
                for (key, value) in table {
                    if let Some(s) = value.as_str() {
                        // Hide actual key values
                        if s.starts_with("ENC[") {
                            println!("  {}: [encrypted]", key);
                        } else if s.len() > 10 {
                            println!("  {}: {}...{}", key, &s[..4], &s[s.len()-4..]);
                        } else {
                            println!("  {}: [hidden]", key);
                        }
                    }
                }
                println!();
            }
        }
    }

    println!("{}", "=".repeat(60));

    Ok(())
}

fn print_toml_table(table: &toml::map::Map<String, toml::Value>, indent: usize) {
    let prefix = "  ".repeat(indent);
    for (key, value) in table {
        match value {
            toml::Value::String(s) => println!("{}{}: {}", prefix, key, s),
            toml::Value::Integer(i) => println!("{}{}: {}", prefix, key, i),
            toml::Value::Float(f) => println!("{}{}: {}", prefix, key, f),
            toml::Value::Boolean(b) => println!("{}{}: {}", prefix, key, b),
            toml::Value::Array(arr) => {
                let items: Vec<String> = arr.iter()
                    .map(|v| match v {
                        toml::Value::String(s) => s.clone(),
                        _ => format!("{:?}", v),
                    })
                    .collect();
                println!("{}{}: {}", prefix, key, items.join(", "));
            },
            toml::Value::Table(t) => {
                println!("{}{}:", prefix, key);
                print_toml_table(t, indent + 1);
            },
            _ => println!("{}{}: {:?}", prefix, key, value),
        }
    }
}
