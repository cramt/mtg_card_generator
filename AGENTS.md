# AGENTS.md - MTG Card Generator Project Context

## Project Overview

**mtg-gen** is a CLI tool for generating Magic: The Gathering card images from YAML definitions. The project is in early development with core parsing infrastructure complete but rendering functionality still to be implemented.

**This project is AI-driven**: AI agents are expected to work autonomously on features, make commits, and push changes without human intervention for routine development tasks.

## Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│ YAML Files  │────▶│  Rust CLI   │────▶│  HTML/CSS   │────▶│  Chromium   │────▶ PNG
│ (card data) │     │  (parsing)  │     │  (Maud)     │     │ (screenshot)│
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
```

## Technology Stack

| Component          | Technology        | Status      | Notes                                    |
| ------------------ | ----------------- | ----------- | ---------------------------------------- |
| CLI framework      | facet-args        | ✅ Complete | Reflection-based CLI parsing             |
| YAML parsing       | facet-yaml        | ✅ Complete | Type-safe deserialization via facet      |
| Data structures    | facet             | ✅ Complete | Reflection-based serialization framework |
| HTML templating    | maud              | 🚧 Partial  | Compile-time HTML generation             |
| Browser automation | chromiumoxide     | 🚧 Partial  | Async Chrome control for screenshots     |
| Async runtime      | tokio             | ✅ Complete | Required by chromiumoxide                |
| Error handling     | thiserror, anyhow | ✅ Complete | Structured error types                   |

## Project Structure

```
mtg_card_generator/
├── src/
│   ├── main.rs         # CLI entry point, file processing
│   ├── lib.rs          # Module exports
│   ├── card.rs         # Card type definitions (447 lines)
│   ├── mana.rs         # Mana symbol parsing & types (624 lines)
│   └── render.rs       # HTML rendering & browser control (167 lines)
├── tests/
│   ├── parsing.rs      # Card parsing tests (all passing)
│   ├── mana.rs         # Mana parsing tests
│   └── fixtures/       # 12 YAML test files covering all card types
├── output/             # Default output directory for generated images
├── SPEC.md             # Comprehensive specification (379 lines)
├── Cargo.toml          # Dependencies and metadata
└── flake.nix           # Nix development environment
```

## Current Implementation Status

### ✅ Completed Components

1. **YAML Parsing Infrastructure**
   - All 13 card layout types fully defined as Rust enums
   - Comprehensive test coverage with fixtures for each type
   - Mana cost parsing with full symbol support
   - Loyalty cost parsing for planeswalkers
   - All parsing tests passing

2. **Card Type Definitions** (src/card.rs)
   - `Card` enum with 13 variants representing different MTG layouts
   - Supporting types: `SagaChapter`, `ClassLevel`, `AdventureCard`, `LoyaltyAbility`, `LevelerRange`, `CardFace`
   - `Rarity` enum: Common, Uncommon, Rare, Mythic

3. **Mana System** (src/mana.rs)
   - `CastingManaSymbol`: 53 variants covering all MTG mana symbols
   - `ManaSymbol`: Extends casting symbols with Tap, Untap, Energy, Chaos
   - `CastingManaCost`: Parses strings like "{2}{W}{U}" into symbol vectors
   - `ActionCost`: For activated abilities with tap/untap symbols
   - `LoyaltyCost`: Plus/Minus/Zero/X variants for planeswalkers
   - `LoyaltyValue`: Numeric or X starting loyalty
   - Full Display trait implementations for round-trip conversion

4. **CLI Framework** (src/main.rs)
   - Argument parsing with facet-args
   - File/directory input handling
   - Recursive YAML file discovery
   - Error reporting for failed cards

### 🚧 In Progress / TODO

1. **Rendering System** (src/render.rs)
   - ✅ Mana symbol rendering (Scryfall SVG URLs)
   - ✅ Rules text parsing with inline symbols
   - ❌ `render_card()` - Main rendering entry point (marked `todo!()`)
   - ❌ `render_normal_card()` - Normal card layout (marked `todo!()`)
   - ❌ `render_planeswalker()` - Planeswalker layout (marked `todo!()`)
   - ❌ Remaining 11 card layout renderers
   - ❌ CSS styling for card frames
   - ❌ Frame color derivation from mana costs
   - ❌ Screenshot capture and PNG output

2. **Browser Integration**
   - ✅ Browser initialization with chromiumoxide
   - ✅ Chrome path configuration via `CHROME_PATH` env var
   - ❌ Page creation and HTML loading
   - ❌ Screenshot capture with proper dimensions
   - ❌ DPI scaling (300/600 DPI support)

3. **Output Management**
   - ❌ Directory structure mirroring
   - ❌ File naming (normal vs double-faced cards)
   - ❌ Name sanitization

## Supported Card Layouts

All 13 layouts are parsed but not yet rendered:

| Layout      | Parsing | Rendering | Notes                                    |
| ----------- | ------- | --------- | ---------------------------------------- |
| normal      | ✅      | ❌        | Standard single-face card                |
| planeswalker| ✅      | ❌        | Loyalty abilities, starting loyalty      |
| saga        | ✅      | ❌        | Chapter abilities, vertical art          |
| class       | ✅      | ❌        | Level-up enchantments                    |
| adventure   | ✅      | ❌        | Two spells on one card                   |
| split       | ✅      | ❌        | Two cards side-by-side (Fire // Ice)     |
| flip        | ✅      | ❌        | Kamigawa-style, rotated bottom half      |
| transform   | ✅      | ❌        | Two separate faces (DFC)                 |
| modal_dfc   | ✅      | ❌        | Two faces, either side playable          |
| battle      | ✅      | ❌        | Defense, landscape orientation           |
| meld        | ✅      | ❌        | Two cards that combine into one          |
| leveler     | ✅      | ❌        | Level up creatures (ROE style)           |
| prototype   | ✅      | ❌        | Two casting costs/stats                  |

## Core Design Principles

### Make Impossible States Unrepresentable

**This is the foundational principle of this codebase.** Use Rust's type system to make invalid states impossible to construct.

**Examples in this codebase:**
- ✅ **Card layouts as enum variants**: A normal card can't accidentally have `loyalty_abilities`, and a planeswalker can't have `chapters`. Each variant only has the fields that make sense for that layout.
- ✅ **Separate mana types**: `CastingManaCost` (for card corners) vs `ActionCost` (for abilities with tap symbols). You can't put a tap symbol in a casting cost.
- ✅ **LoyaltyCost enum**: Can only be Plus(n), Minus(n), Zero, PlusX, or MinusX - not arbitrary strings that need validation.
- ✅ **Rarity enum**: Only Common, Uncommon, Rare, Mythic - not strings that could be misspelled.

**When adding new features, always ask:**
1. Can this field be optional when it should always be present for this card type?
2. Can this value be invalid? (Use enums instead of strings/numbers)
3. Can these two fields be in an inconsistent state? (Group them in a struct or enum)
4. Would pattern matching force me to handle cases that can't exist? (Use more specific types)

**Anti-patterns to avoid:**
- ❌ Single `Card` struct with `Option<Vec<LoyaltyAbility>>` - allows normal cards to have loyalty abilities
- ❌ Loyalty costs as strings like `"+2"` - allows invalid values like `"potato"` or `"++5"`
- ❌ Mana costs as raw strings - allows malformed costs like `"{W{U}"`
- ❌ Card types as strings - allows typos like `"planeswalker"` vs `"planeswalker"`

**When reviewing code or implementing features:**
- Prefer enums over strings for constrained values
- Prefer separate types over `Option<T>` when the presence is determined by context
- Prefer newtype wrappers with validation over raw primitives
- Use Rust's type system to encode business rules, not runtime checks

## Key Design Decisions

1. **Facet Framework**: Uses `facet` crate for reflection-based serialization instead of standard `serde`. This provides:
   - Automatic derive macros for YAML parsing
   - Tagged enum support via `#[facet(tag = "type")]`
   - Custom proxy types for complex parsing (mana costs, loyalty)

2. **Mana Cost Parsing**: Custom parser in mana.rs handles MTG-specific syntax:
   - Brace-delimited symbols: `{W}`, `{2}`, `{W/U}`
   - Hybrid, Phyrexian, and Twobrid mana
   - Separate types for casting costs vs action costs
   - **Follows "impossible states" principle**: `CastingManaSymbol` can't represent tap/untap symbols

3. **Card Variants**: Uses Rust enums with named fields rather than a single struct with optional fields:
   - **Follows "impossible states" principle**: Type safety ensures you can't have saga chapters on a normal card
   - Pattern matching: Exhaustive handling of all card types
   - Tradeoff: Some field duplication across variants, but much safer

4. **Rendering Strategy**: Maud for compile-time HTML generation + Chromium for screenshot:
   - Type-safe HTML templates
   - No runtime template parsing
   - High-quality output via browser rendering
   - Requires Chrome/Chromium installation

## Test Coverage

All parsing tests passing (tests/parsing.rs):
- ✅ Normal creature
- ✅ Planeswalker
- ✅ Saga (including combined chapters)
- ✅ Class
- ✅ Adventure
- ✅ Split
- ✅ Transform DFC
- ✅ Modal DFC
- ✅ Battle
- ✅ Flip
- ✅ Leveler
- ✅ Prototype

Mana parsing tests (tests/mana.rs) - not yet reviewed but likely comprehensive.

## Development Environment

- **Language**: Rust (edition 2024)
- **Build System**: Cargo
- **Nix Flake**: Available for reproducible dev environment
- **Git**: Repository at github.com/cramt/mtg_card_generator

## Common Workflows

### Adding a New Card Layout
1. Add variant to `Card` enum in src/card.rs
2. Create supporting types if needed (like `SagaChapter`)
3. Add test fixture in tests/fixtures/
4. Add parsing test in tests/parsing.rs
5. Implement renderer in src/render.rs
6. Update SPEC.md with YAML schema example

### Testing Card Parsing
```bash
cargo test test_parse_<layout_name>
```

### Running the CLI (when rendering is complete)
```bash
cargo run -- ./tests/fixtures/
cargo run -- ./tests/fixtures/normal_creature.yaml -o ./output/
```

## Known Issues & Limitations

1. **Rendering Not Implemented**: Core functionality (HTML generation, screenshot capture) still marked as `todo!()`
2. **No Art Handling**: Specification mentions future art support but not yet designed
3. **No Frame Color Logic**: Auto-detection from mana costs not implemented
4. **No Validation Warnings**: SPEC.md mentions warnings for missing fields, not implemented
5. **Battle Card Schema Mismatch**: Battle uses separate backside fields instead of faces array (see src/card.rs:356-370)
6. **Potential Type Safety Improvement**: Some Card variants (like Planeswalker, Saga) have `power`/`toughness` fields even though these card types can't be creatures. This is a minor violation of "make impossible states unrepresentable" but is acceptable for now since it simplifies the enum structure. Consider refactoring if this becomes problematic.

## Task Management

All project tasks are tracked in **TODO.md**. This file contains:
- High priority core rendering tasks
- Medium priority card layout implementations
- Low priority validation and polish items
- Future enhancements
- Technical debt tracking

Refer to TODO.md for the complete task list and implementation order.

## References

- **Scryfall API**: Used for mana symbol SVGs (https://svgs.scryfall.io/card-symbols/)
- **MTG Comprehensive Rules**: For card layout specifications
- **Facet Documentation**: For understanding the serialization framework
- **Maud Documentation**: For HTML templating syntax

## Git Workflow for AI Agents

### CRITICAL: GPG Signing Must Be Disabled

**IMPORTANT**: This repository has GPG signing enabled globally, but AI agents cannot sign commits. You MUST disable GPG signing for commits in this repository.

Before making any commits, run:
```bash
git config --local commit.gpgsign false
```

This sets the local repository config to disable signing, overriding the global setting.

### CRITICAL: Do NOT Push to Remote

**IMPORTANT**: This repository does not have a GitHub remote configured yet. DO NOT attempt to push commits.

- ✅ **DO**: Create commits locally after completing work
- ❌ **DO NOT**: Run `git push` commands
- ❌ **DO NOT**: Attempt to create pull requests

The human will handle pushing commits and setting up the remote repository when ready.

### Commit Guidelines

1. **Autonomous Commits**: You are expected to commit your work independently without asking for permission
2. **Commit Frequency**: Commit after completing each logical unit of work (e.g., implementing one card renderer, fixing a bug, adding tests)
3. **Commit Messages**: Write clear, descriptive commit messages following this format:
   - First line: Brief summary (50 chars or less) in imperative mood
   - Blank line
   - Detailed explanation if needed (wrap at 72 chars)
   - Examples:
     - `Implement normal card HTML renderer`
     - `Add CSS styling for card frames`
     - `Fix mana symbol alignment in rules text`

4. **Branch Strategy**: 
   - Work directly on `master` (current branch)
   - Do not create feature branches until remote repository is set up

### Example Workflow

```bash
# Before starting work
git config --local commit.gpgsign false

# After implementing a feature
git add src/render.rs
git commit -m "Implement normal card renderer with frame colors"

# DO NOT push - no remote repository yet
# git push origin master  # ❌ DO NOT RUN THIS
```

## Development Guidelines for Autonomous Work

### When to Work Independently

You should work autonomously on:
- Implementing card layout renderers (one layout at a time)
- Adding CSS styling for card frames
- Fixing bugs in parsing or rendering
- Adding new test cases
- Improving error messages
- Refactoring code for clarity
- Updating documentation

### When to Ask for Guidance

Ask the human for guidance on:
- Major architectural changes
- Adding new dependencies
- Changing the YAML schema (breaking changes)
- Design decisions for visual appearance (colors, fonts, layouts)
- Performance optimization strategies

### Testing Requirements

Before committing:
1. Run `cargo test` to ensure all tests pass
2. If you added new functionality, add corresponding tests
3. If you modified card parsing, verify with test fixtures
4. If you implemented rendering, test with actual YAML files

### Feature Implementation Strategy

When implementing a new card renderer:
1. Start with the simplest layout (normal cards)
2. Create HTML structure using Maud
3. Add CSS styling
4. Test with fixture files
5. Commit (do not push)
6. Move to next layout in complexity order

Recommended implementation order:
1. Normal cards (creatures, instants, sorceries, enchantments)
2. Planeswalkers (loyalty abilities)
3. Sagas (chapter markers)
4. Class cards (level-up)
5. Adventure cards (split frame)
6. Split cards (side-by-side)
7. Transform/Modal DFC (two separate images)
8. Flip cards (rotated bottom)
9. Battle cards (defense counter)
10. Leveler cards (level bars)
11. Prototype cards (dual stats)
12. Meld cards (combination)

## Notes for AI Agents

- The project uses `facet` instead of `serde` - don't suggest serde-based solutions
- All mana cost parsing should go through the types in mana.rs, not ad-hoc string parsing
- When adding new card fields, remember to update both the Card enum variant AND the test fixtures
- The `#[facet(proxy = ...)]` attribute is used for custom parsing - see CastingManaCostProxy for examples
- Browser automation is async - all rendering functions should be async and use tokio
- Test fixtures are the source of truth for YAML schema - keep them in sync with SPEC.md
- **NEVER run git commands with GPG signing enabled** - always disable it first with `git config --local commit.gpgsign false`

## OpenCode-Specific Features

### Restarting OpenCode
If you need OpenCode to reload configuration (e.g., after editing `opencode.json`, `.opencode/agent/*.md`, or `flake.nix`), ask the user to restart OpenCode. Changes to these files are not picked up automatically.

### Card Evaluation Agent
A custom `card-eval` subagent is configured in `.opencode/agent/card-eval.md` that uses Gemini vision to evaluate rendered card images. Invoke it via the Task tool:
```
Task(subagent_type="card-eval", prompt="Evaluate ./output/card.png")
```

### Available Custom Agents
- `card-eval`: Evaluates rendered MTG card images for visual correctness using vision capabilities
