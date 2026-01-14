# MTG Card Generator - Specification

A CLI tool for generating Magic: The Gathering card images from YAML definitions.

## Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│ YAML Files  │────▶│  Rust CLI   │────▶│  HTML/CSS   │────▶│  Chromium   │────▶ PNG
│ (card data) │     │  (parsing)  │     │  (Maud)     │     │ (screenshot)│
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
```

## Technology Stack

| Component          | Choice            | Rationale                                      |
| ------------------ | ----------------- | ---------------------------------------------- |
| CLI framework      | `clap`            | Industry standard, derive macros               |
| YAML parsing       | `serde_yaml`      | Type-safe deserialization                      |
| HTML templating    | `maud`            | Compile-time, type-safe, lightweight           |
| Browser automation | `chromiumoxide`   | Async, well-maintained, just needs Chrome      |
| Async runtime      | `tokio`           | Required by chromiumoxide                      |

## CLI Interface

```bash
# Process all YAML files in a folder
mtg-gen ./cards/

# Process a single card
mtg-gen ./cards/bolt.yaml

# Custom output directory
mtg-gen ./cards/ -o ./output/

# Specify resolution (default: 750x1050 at 300 DPI)
mtg-gen ./cards/ --dpi 600
```

### Output Behavior

- Output directory structure mirrors input directory structure
- For double-faced cards, outputs `{name}_front.png` and `{name}_back.png`
- On error: continue processing remaining cards, report all errors at the end

## Card Layouts Supported

| Layout      | Description                                    |
| ----------- | ---------------------------------------------- |
| `normal`    | Standard single-face card (default)            |
| `planeswalker` | Loyalty abilities, starting loyalty         |
| `saga`      | Chapter abilities, vertical art                |
| `class`     | Level-up enchantments                          |
| `adventure` | Two spells on one card                         |
| `split`     | Two cards side-by-side (Fire // Ice)           |
| `flip`      | Kamigawa-style, rotated bottom half            |
| `transform` | Two separate faces (DFC)                       |
| `modal_dfc` | Two faces, either side playable                |
| `battle`    | Defense, landscape orientation                 |
| `meld`      | Two cards that combine into one                |
| `leveler`   | Level up creatures (ROE style)                 |
| `prototype` | Two casting costs/stats                        |

## YAML Schema

### Common Fields

All cards share these fields:

```yaml
name: "Card Name"                    # Required
mana_cost: "{1}{W}{U}"               # Optional (lands have none)
type_line: "Creature — Human Wizard" # Required
rules_text: "Card rules here."       # Optional
flavor_text: "Flavor text here."     # Optional
rarity: common | uncommon | rare | mythic  # Required
type: normal                       # Optional, defaults to "normal"
```

### Frame Color Derivation

Frame colors are automatically derived from `mana_cost`:

- Single color: Mono-colored frame
- Two colors: Gold frame with appropriate gradient
- Three+ colors: Gold frame
- Colorless with no colors: Artifact/colorless frame
- No mana cost + Land type: Land frame

### Creature Cards

```yaml
name: "Llanowar Elves"
mana_cost: "{G}"
type_line: "Creature — Elf Druid"
rules_text: "{T}: Add {G}."
flavor_text: "One bone broken for every twig snapped underfoot."
power: 1
toughness: 1
rarity: common
```

### Planeswalker Cards

```yaml
name: "Jace, the Mind Sculptor"
mana_cost: "{2}{U}{U}"
type_line: "Legendary Planeswalker — Jace"
loyalty: 3
loyalty_abilities:
  - cost: "+2"
    text: "Look at the top card of target player's library. You may put that card on the bottom of that player's library."
  - cost: "0"
    text: "Draw three cards, then put two cards from your hand on top of your library in any order."
  - cost: "-1"
    text: "Return target creature to its owner's hand."
  - cost: "-12"
    text: "Exile all cards from target player's library, then that player shuffles their hand into their library."
rarity: mythic
```

### Saga Cards

Chapters can span multiple chapter numbers for shared abilities:

```yaml
name: "The Eldest Reborn"
mana_cost: "{4}{B}"
type_line: "Enchantment — Saga"
type: saga
chapters:
  - chapters: [1]
    text: "Each opponent sacrifices a creature or planeswalker."
  - chapters: [2]
    text: "Each opponent discards a card."
  - chapters: [3]
    text: "Put target creature or planeswalker card from a graveyard onto the battlefield under your control."
rarity: uncommon

# Combined chapters example
name: "The Modern Age"
mana_cost: "{1}{U}"
type_line: "Enchantment — Saga"
type: saga
chapters:
  - chapters: [1, 2]
    text: "Draw a card."
  - chapters: [3]
    text: "Exile this Saga, then return it to the battlefield transformed under your control."
rarity: common
```

### Class Cards

```yaml
name: "Ranger Class"
mana_cost: "{1}{G}"
type_line: "Enchantment — Class"
type: class
levels:
  - level: 1
    text: "When Ranger Class enters the battlefield, create a 2/2 green Wolf creature token."
  - level: 2
    cost: "{1}{G}"
    text: "Whenever you attack, put a +1/+1 counter on target attacking creature."
  - level: 3
    cost: "{1}{G}"
    text: "You may look at the top card of your library any time. You may cast creature spells from the top of your library."
rarity: rare
```

### Split Cards

```yaml
name: "Fire // Ice"
type: split
fuse: false        # Optional, default false
aftermath: false   # Optional, default false
faces:
  - name: "Fire"
    mana_cost: "{1}{R}"
    type_line: "Instant"
    rules_text: "Fire deals 2 damage divided as you choose among one or two targets."
  - name: "Ice"
    mana_cost: "{1}{U}"
    type_line: "Instant"
    rules_text: "Tap target permanent.\nDraw a card."
rarity: uncommon
```

### Transform / Double-Faced Cards

```yaml
name: "Delver of Secrets"
type: transform
faces:
  - name: "Delver of Secrets"
    mana_cost: "{U}"
    type_line: "Creature — Human Wizard"
    rules_text: "At the beginning of your upkeep, look at the top card of your library. You may reveal that card. If an instant or sorcery card is revealed this way, transform Delver of Secrets."
    power: 1
    toughness: 1
  - name: "Insectile Aberration"
    color_indicator: [blue]
    type_line: "Creature — Human Insect"
    rules_text: "Flying"
    power: 3
    toughness: 2
rarity: common
```

### Modal Double-Faced Cards

```yaml
name: "Emeria's Call"
type: modal_dfc
faces:
  - name: "Emeria's Call"
    mana_cost: "{4}{W}{W}{W}"
    type_line: "Sorcery"
    rules_text: "Create two 4/4 white Angel Warrior creature tokens with flying. Non-Angel creatures you control gain indestructible until your next turn."
  - name: "Emeria, Shattered Skyclave"
    type_line: "Land"
    rules_text: "As Emeria, Shattered Skyclave enters the battlefield, you may pay 3 life. If you don't, it enters the battlefield tapped.\n{T}: Add {W}."
rarity: mythic
```

### Battle Cards

```yaml
name: "Invasion of Gobakhan"
mana_cost: "{1}{W}"
type_line: "Battle — Siege"
type: battle
defense: 3
rules_text: "When Invasion of Gobakhan enters the battlefield, look at target opponent's hand. You may exile a nonland card from it. For as long as that card remains exiled, its owner may play it. A spell cast this way costs {2} more to cast."
faces:
  - # Front face uses top-level fields
  - name: "Lightshield Array"
    type_line: "Enchantment"
    rules_text: "At the beginning of your end step, put a +1/+1 counter on each creature that attacked this turn.\nSacrifice Lightshield Array: Creatures you control gain hexproof and indestructible until end of turn."
rarity: rare
```

### Flip Cards (Kamigawa)

```yaml
name: "Akki Lavarunner"
type: flip
faces:
  - name: "Akki Lavarunner"
    mana_cost: "{3}{R}"
    type_line: "Creature — Goblin Warrior"
    rules_text: "Haste\nWhenever Akki Lavarunner deals damage to an opponent, flip it."
    power: 1
    toughness: 1
  - name: "Tok-Tok, Volcano Born"
    type_line: "Legendary Creature — Goblin Shaman"
    rules_text: "Protection from red\nIf a red source would deal damage to a player, it deals that much damage plus 1 to that player instead."
    power: 2
    toughness: 2
rarity: rare
```

### Adventure Cards

```yaml
name: "Bonecrusher Giant"
type: adventure
adventure:
  name: "Stomp"
  mana_cost: "{1}{R}"
  type_line: "Instant — Adventure"
  rules_text: "Damage can't be prevented this turn. Stomp deals 2 damage to any target."
# Main card fields
mana_cost: "{2}{R}"
type_line: "Creature — Giant"
rules_text: "Whenever Bonecrusher Giant becomes the target of a spell, Bonecrusher Giant deals 2 damage to that spell's controller."
power: 4
toughness: 3
rarity: rare
```

### Leveler Cards

```yaml
name: "Kargan Dragonlord"
mana_cost: "{R}{R}"
type_line: "Creature — Human Warrior"
type: leveler
rules_text: "Level up {R}"
levels:
  - range: [0, 3]
    power: 2
    toughness: 2
  - range: [4, 7]
    power: 4
    toughness: 4
    text: "Flying"
  - range: [8, null]  # null means 8+
    power: 8
    toughness: 8
    text: "Flying, trample\n{R}: Kargan Dragonlord gets +1/+0 until end of turn."
rarity: mythic
```

### Prototype Cards

```yaml
name: "Phyrexian Fleshgorger"
mana_cost: "{7}"
type_line: "Artifact Creature — Phyrexian Wurm"
type: prototype
prototype:
  mana_cost: "{1}{B}{B}"
  power: 3
  toughness: 3
rules_text: "Prototype {1}{B}{B} — 3/3\nMenace, lifelink\nWard—Pay life equal to Phyrexian Fleshgorger's power."
power: 7
toughness: 5
rarity: mythic
```

## Mana Symbol Syntax

Uses standard Scryfall notation:

| Symbol                  | Meaning              |
| ----------------------- | -------------------- |
| `{W}` `{U}` `{B}` `{R}` `{G}` | Basic mana      |
| `{C}`                   | Colorless            |
| `{0}` - `{20}`          | Generic mana         |
| `{X}` `{Y}` `{Z}`       | Variable             |
| `{S}`                   | Snow                 |
| `{W/U}` `{G/U}` etc.    | Hybrid               |
| `{2/W}` `{2/U}` etc.    | Twobrid              |
| `{W/P}` `{U/P}` etc.    | Phyrexian            |
| `{T}` `{Q}`             | Tap / Untap          |
| `{E}`                   | Energy               |
| `{CHAOS}`               | Planar chaos         |

## Output

### Resolution

Default output is 750x1050 pixels (300 DPI for standard 2.5" x 3.5" card).

Available DPI options:
- 300 DPI: 750 x 1050 px (default)
- 600 DPI: 1500 x 2100 px

### File Naming

- Normal cards: `{name}.png`
- Double-faced cards: `{name}_front.png`, `{name}_back.png`
- Split cards: `{left_name}_{right_name}.png`

Names are sanitized: lowercase, spaces replaced with underscores, special characters removed.

## Validation

The CLI will warn (but not fail) on:

- Creature without power/toughness
- Planeswalker without loyalty
- Planeswalker without loyalty_abilities
- Saga without chapters
- Battle without defense
- Missing required fields

## Future Considerations

- Art handling (relative paths, asset folders)
- Custom frame colors (override auto-detection)
- Set symbol rendering
- Watermarks
- Foil/premium treatments
- Batch processing with progress bar
