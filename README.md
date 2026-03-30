# whoami

A portable profile format for AI agents. Define yourself once, use everywhere.

## The Problem

Every AI tool starts cold. You explain your preferences, your stack, your projects. Then you switch tools and repeat it. Claude Code, Cursor, ChatGPT—each one needs the same context.

## The Solution

`whoami.toml` is a standard format that AI agents can read to understand who you are, how you work, and what you're building.

```toml
version = "20260330"

[person]
name = "Jake Goldsborough"
roles = ["backend engineer", "infrastructure engineer"]

[communication]
style = "direct, no fluff, technical peer not cheerleader"
emoji_in_code = false

[technical.languages]
primary = ["rust", "typescript", "bash"]

[[projects.active]]
name = "skillz"
path = "~/dev/skillz"
description = "Claude Code skill package manager"
tech = ["rust", "clap"]
```

## Installation

Place your profile at:

```bash
~/.config/agent/whoami.toml
```

Or set a custom location:

```bash
export AGENT_WHOAMI=~/dotfiles/whoami.toml
```

## Usage

### For Users

1. Create your `whoami.toml` (see [examples/whoami.toml](examples/whoami.toml))
2. Place it at `~/.config/agent/whoami.toml`
3. Encrypt sensitive fields with SOPS (optional):
   ```bash
   sops -e -i ~/.config/agent/whoami.toml
   ```
4. AI tools that support whoami will automatically read it

### For Tool Developers

Read the [SPEC.md](SPEC.md) for integration guidelines.

Basic usage:

```rust
// Rust
let path = env::var("AGENT_WHOAMI")
    .unwrap_or_else(|_| format!("{}/.config/agent/whoami.toml", env::var("HOME").unwrap()));
let profile: Profile = toml::from_str(&fs::read_to_string(path)?)?;
```

```typescript
// TypeScript
const path = process.env.AGENT_WHOAMI ||
  `${process.env.HOME}/.config/agent/whoami.toml`;
const profile = TOML.parse(await fs.readFile(path, 'utf-8'));
```

## What Goes In It

- **Person**: Name, roles, pronouns
- **Communication**: Style, tone, preferences
- **Technical**: Languages, frameworks, tools
- **Preferences**: Code style, testing philosophy, architecture choices
- **Domains**: Expertise areas, learning goals
- **Projects**: Active projects with paths and tech stacks
- **Context**: Important paths (dotfiles, notes, projects index)
- **Boundaries**: Hard constraints and guiding principles
- **API Keys**: Encrypted credentials (use SOPS)

See [SPEC.md](SPEC.md) for complete field documentation.

## Privacy

Sensitive fields like email and API keys should be encrypted with [SOPS](https://github.com/mozilla/sops):

```bash
# Encrypt
sops -e -i whoami.toml

# Edit encrypted file
sops whoami.toml

# Decrypt for reading
sops -d whoami.toml
```

Create a public version for sharing:

```bash
cp whoami.toml whoami.pub.toml
# Remove [api_keys] and person.email
```

## Tools That Support whoami

- *(Coming soon)*

## Examples

- [Jake's whoami.toml](examples/whoami.toml) - Full example with all sections
- [Minimal whoami.toml](examples/minimal.toml) - Just the essentials

## Validator

Validate your profile:

```bash
cargo run --bin validate ~/.config/agent/whoami.toml
```

## Dotfiles Integration

`whoami.toml` works great in dotfiles:

1. Add to your dotfiles repo:
   ```bash
   mkdir -p ~/dotfiles/agent
   cp ~/.config/agent/whoami.toml ~/dotfiles/agent/
   ```

2. Symlink on new machines:
   ```bash
   ln -s ~/dotfiles/agent/whoami.toml ~/.config/agent/whoami.toml
   ```

3. Version and sync across machines

## Specification

See [SPEC.md](SPEC.md) for the complete format specification.

**Version:** 20260330 (Draft)

## Contributing

The spec is in early draft. Feedback welcome via issues or PRs.

## License

- Specification: CC0 1.0 (public domain)
- Code: MIT OR Apache-2.0
