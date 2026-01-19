#!/usr/bin/env bash
#
# fix-card.sh
#
# Runs Claude Code to iteratively fix a card's rendering until it looks correct.
#
# Usage:
#   ./scripts/fix-card.sh <card.yaml> [options]
#
# Arguments:
#   card.yaml        Path to the card YAML file to fix
#
# Options:
#   --model MODEL    Model to use (default: claude-sonnet-4-20250514)
#   --reference IMG  Path to a reference image to compare against
#   --output DIR     Output directory (default: output)
#
# Examples:
#   ./scripts/fix-card.sh tests/fixtures/normal_creature.yaml
#   ./scripts/fix-card.sh tests/fixtures/planeswalker.yaml --model opus
#   ./scripts/fix-card.sh my_card.yaml --reference reference.png
#

set -euo pipefail

# Defaults
MODEL="claude-sonnet-4-20250514"
REFERENCE=""
OUTPUT_DIR="output"
CARD_FILE=""

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --model)
            MODEL="$2"
            shift 2
            ;;
        --reference)
            REFERENCE="$2"
            shift 2
            ;;
        --output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --help|-h)
            head -24 "$0" | tail -n +2 | sed 's/^# \?//'
            exit 0
            ;;
        -*)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
        *)
            if [[ -z "$CARD_FILE" ]]; then
                CARD_FILE="$1"
            else
                echo "Error: Multiple card files specified"
                exit 1
            fi
            shift
            ;;
    esac
done

# Validate arguments
if [[ -z "$CARD_FILE" ]]; then
    echo "Error: No card file specified"
    echo "Usage: $0 <card.yaml> [options]"
    exit 1
fi

if [[ ! -f "$CARD_FILE" ]]; then
    echo "Error: Card file not found: $CARD_FILE"
    exit 1
fi

if [[ -n "$REFERENCE" && ! -f "$REFERENCE" ]]; then
    echo "Error: Reference image not found: $REFERENCE"
    exit 1
fi

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

# Extract card name from filename for display
CARD_NAME=$(basename "$CARD_FILE" .yaml | tr '_' ' ')

echo "=== MTG Card Fixer ==="
echo "Card: $CARD_NAME ($CARD_FILE)"
echo "Model: $MODEL"
echo "Output: $OUTPUT_DIR"
[[ -n "$REFERENCE" ]] && echo "Reference: $REFERENCE"
echo ""

# Build the prompt
REFERENCE_INSTRUCTION=""
if [[ -n "$REFERENCE" ]]; then
    REFERENCE_INSTRUCTION="
## Reference Image

A reference image is provided at: $REFERENCE

Read this image first to understand exactly what the card should look like. Your goal is to make the rendered output match this reference as closely as possible."
fi

PROMPT=$(cat <<EOF
Your task is to fix the rendering of the card defined in: $CARD_FILE

## Instructions

1. First, read the card YAML file to understand what card you're rendering.

2. Check if rendering is implemented in src/render.rs. If the relevant render function is still todo!(), implement it first.

3. Build and run the renderer:
   cargo run -- $CARD_FILE -o $OUTPUT_DIR/

4. Read the output PNG file to see the current result.
$REFERENCE_INSTRUCTION
5. Evaluate the rendered card:
   - Does it have the correct frame color for its mana cost?
   - Is the card name positioned correctly at the top?
   - Is the mana cost in the top right corner?
   - Is the type line correct and properly positioned?
   - Is the rules text readable and properly formatted?
   - Is flavor text in italics (if present)?
   - Is power/toughness shown (for creatures)?
   - Do mana symbols render correctly in rules text?

6. Identify specific visual issues and fix them one at a time in src/render.rs

7. After each fix, re-render and read the new PNG to evaluate progress.

8. Keep iterating until the card looks correct.

Work autonomously. Make commits after significant progress. Do not ask for permission.
EOF
)

# Run claude interactively with the prompt
# Using --dangerously-skip-permissions to allow autonomous file edits
exec claude --model "$MODEL" --dangerously-skip-permissions "$PROMPT"
