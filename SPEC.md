# whoami Specification

**Version:** 20260330
**Status:** Draft

## Overview

`whoami.toml` is a standard format for defining a person's identity, preferences, and context for AI agents. It provides a portable, version-controlled profile that AI tools can read to understand who they're interacting with.

## Purpose

Every AI interaction starts cold. You explain your preferences, your stack, your projects. Then you switch tools and repeat it. `whoami.toml` solves this by providing a single source of truth about you that AI agents can reference.

## Location

Tools MUST check for `whoami.toml` in this order:

1. `$AGENT_WHOAMI` environment variable (full path)
2. `~/.config/agent/whoami.toml`
3. `$XDG_CONFIG_HOME/agent/whoami.toml`

## Format

The file MUST be valid TOML. The top-level structure consists of these sections:

- `[person]` - Identity and basic information (REQUIRED)
- `[communication]` - Communication style and preferences (OPTIONAL)
- `[technical]` - Technical skills and tools (OPTIONAL)
- `[preferences]` - Coding and workflow preferences (OPTIONAL)
- `[domains]` - Expertise and learning areas (OPTIONAL)
- `[projects]` - Active projects and context (OPTIONAL)
- `[context]` - Important paths and resources (OPTIONAL)
- `[boundaries]` - Hard constraints and principles (OPTIONAL)
- `[api_keys]` - API credentials (OPTIONAL, SOPS-encrypted recommended)

## Required Fields

Only one field is REQUIRED:

```toml
version = "20260330"
```

The version uses date-based format: `YYYYMMDD` for the specification release date.

All other sections and fields are OPTIONAL. Tools MUST handle missing sections gracefully.

## Section Specifications

### `[person]` (RECOMMENDED)

Basic identity information.

```toml
[person]
name = "string"           # RECOMMENDED
username = "string"       # OPTIONAL
email = "string"          # OPTIONAL (consider SOPS encryption)
roles = ["string", ...]   # OPTIONAL
pronouns = "string"       # OPTIONAL
```

### `[communication]` (OPTIONAL)

How you prefer to communicate and receive responses.

```toml
[communication]
style = "string"                    # OPTIONAL - Overall communication style
code_comments = "string"            # OPTIONAL - When to add comments
emoji_in_code = boolean             # OPTIONAL - Use emoji in code
emoji_in_commits = boolean          # OPTIONAL - Use emoji in commits
tone = "string"                     # OPTIONAL - Desired tone
explanations = "string"             # OPTIONAL - How to explain things
```

### `[technical]` (OPTIONAL)

Technical skills organized by category.

```toml
[technical.languages]
primary = ["string", ...]    # OPTIONAL
secondary = ["string", ...]  # OPTIONAL
learning = ["string", ...]   # OPTIONAL

[technical.frameworks]
# Any keys allowed, values are arrays of strings
rust = ["string", ...]
typescript = ["string", ...]

[technical.tools]
editor = "string"            # OPTIONAL
shell = "string"             # OPTIONAL
vcs = "string"               # OPTIONAL
package_managers = ["string", ...]  # OPTIONAL
deployment = ["string", ...]        # OPTIONAL
```

### `[preferences]` (OPTIONAL)

Coding and workflow preferences.

```toml
[preferences.code]
tabs_or_spaces = "string"        # OPTIONAL - "tabs" or "spaces"
line_length = integer            # OPTIONAL
commit_style = "string"          # OPTIONAL
testing_philosophy = "string"    # OPTIONAL
error_handling = "string"        # OPTIONAL

[preferences.docs]
format = "string"                # OPTIONAL
location = "string"              # OPTIONAL
structure = "string"             # OPTIONAL

[preferences.architecture]
deployment = "string"            # OPTIONAL
dependencies = "string"          # OPTIONAL
complexity = "string"            # OPTIONAL
databases = "string"             # OPTIONAL
```

### `[domains]` (OPTIONAL)

Areas of expertise and learning.

```toml
[domains.expertise]
areas = ["string", ...]      # OPTIONAL
specific = ["string", ...]   # OPTIONAL

[domains.learning]
current = ["string", ...]    # OPTIONAL
interested = ["string", ...]  # OPTIONAL
```

### `[projects]` (OPTIONAL)

Active projects. Each project is a table with these fields:

```toml
[[projects.active]]
name = "string"              # REQUIRED if project defined
path = "string"              # OPTIONAL
description = "string"       # OPTIONAL
tech = ["string", ...]       # OPTIONAL
url = "string"               # OPTIONAL
```

### `[context]` (OPTIONAL)

Important paths and resources.

```toml
[context]
dotfiles = "string"              # OPTIONAL
notes = "string"                 # OPTIONAL
projects_index = "string"        # OPTIONAL
daily_notes_format = "string"    # OPTIONAL - Date format template
```

### `[boundaries]` (OPTIONAL)

Hard constraints and principles.

```toml
[boundaries]
no = ["string", ...]    # OPTIONAL - Things to avoid
yes = ["string", ...]   # OPTIONAL - Guiding principles
```

### `[api_keys]` (OPTIONAL)

API credentials. SHOULD be encrypted with SOPS or similar.

```toml
[api_keys]
# Any keys allowed, values are strings
openai = "string or ENC[...]"
anthropic = "string or ENC[...]"
github = "string or ENC[...]"
```

## Privacy and Security

### Sensitive Data

The following fields commonly contain sensitive data and SHOULD be encrypted:

- `person.email`
- `api_keys.*` (all keys)
- Any custom fields containing tokens, passwords, or personal information

### SOPS Integration

When using SOPS:

1. Encrypt specific fields in-place: `sops -e -i whoami.toml`
2. Decrypt when reading: `sops -d whoami.toml`
3. Tools MUST support reading SOPS-encrypted TOML files

Example encrypted field:

```toml
email = "ENC[AES256_GCM,data:xQx...,iv:...,tag:...,type:str]"
```

### Shareable Profile

Users MAY create a public version by removing sensitive fields:

```bash
cp whoami.toml whoami.pub.toml
# Remove [api_keys] section and person.email
```

## Version Evolution

The `version` field uses date-based versioning in the format `YYYYMMDD`.

Each version represents a snapshot of the specification at that date. Tools MUST ignore unknown fields and sections to support forward compatibility.

Versions are additive—new versions add optional fields but don't remove or break existing ones.

## Tool Integration

### Reading the Profile

Tools SHOULD:

1. Check for `whoami.toml` at standard locations
2. Parse as TOML
3. Handle SOPS-encrypted fields if present
4. Gracefully handle missing sections or fields
5. Validate the `version` field is present

### Using the Profile

Tools MAY:

- Load profile at startup
- Reference profile fields in prompts to AI models
- Suggest updates when detecting changed preferences
- Cache parsed profile for performance

Tools MUST NOT:

- Modify `whoami.toml` without explicit user permission
- Share profile contents with external services without consent
- Assume all sections or fields are present

## Example

See [examples/whoami.toml](examples/whoami.toml) for a complete example.

## Validator

A reference validator tool is available at [validator/](validator/).

## License

This specification is released under CC0 1.0 Universal (public domain).

## Contributing

This specification is open for community input. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
