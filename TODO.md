# TODO - MTG Card Generator

This file tracks all pending work for the project. Items are organized by priority and category.

## High Priority - Core Rendering

### Browser Integration
- [x] Implement `render_card()` main entry point in src/render.rs
  - Create new browser page
  - Load HTML content
  - Capture screenshot with proper dimensions
  - Save to output path
  - Handle DPI scaling (300/600 DPI)

### Normal Card Rendering
- [x] Implement `render_normal_card()` in src/render.rs
  - Create HTML structure for card frame
  - Add CSS styling for card frames
  - Implement frame color derivation from mana costs
  - Add power/toughness box for creatures
  - Handle rules text and flavor text layout
  - Test with tests/fixtures/normal_creature.yaml

### Output Management
- [x] Implement directory structure mirroring
- [ ] Implement file naming logic
  - Normal cards: `{name}.png`
  - Double-faced cards: `{name}_front.png`, `{name}_back.png`
  - Split cards: `{left_name}_{right_name}.png`
- [x] Implement name sanitization (lowercase, underscores, remove special chars)
- [x] Create output directories if they don't exist

## Medium Priority - Additional Card Layouts

### Planeswalker Cards (NEXT PRIORITY)
- [ ] Implement `render_planeswalker()` in src/render.rs (currently `todo!()`)
  - Loyalty counter display
  - Loyalty ability layout with costs
  - Proper spacing for 3-5 abilities
  - Test with tests/fixtures/planeswalker.yaml

### Saga Cards
- [ ] Implement saga renderer
  - Chapter markers (I, II, III, etc.)
  - Vertical art layout
  - Combined chapter support (I-II)
  - Test with tests/fixtures/saga.yaml

### Class Cards
- [x] Implement class renderer
  - Level 1, 2, 3 sections
  - Level-up costs
  - Horizontal dividers
  - Test with tests/fixtures/class.yaml

### Adventure Cards
- [ ] Implement adventure renderer
  - Split frame (creature + adventure)
  - Adventure spell in corner/side
  - Test with tests/fixtures/adventure.yaml

### Split Cards
- [ ] Implement split renderer
  - Side-by-side layout
  - Rotated text for each half
  - Fuse indicator (if applicable)
  - Aftermath indicator (if applicable)
  - Test with tests/fixtures/split.yaml

### Double-Faced Cards
- [ ] Implement transform renderer
  - Generate two separate images
  - Front face rendering
  - Back face rendering
  - Color indicator support
  - Test with tests/fixtures/transform.yaml

- [ ] Implement modal DFC renderer
  - Similar to transform but different indicator
  - Test with tests/fixtures/modal_dfc.yaml

### Flip Cards
- [ ] Implement flip renderer
  - Top half normal
  - Bottom half rotated 180°
  - Test with tests/fixtures/flip.yaml

### Battle Cards
- [ ] Implement battle renderer
  - Defense counter
  - Landscape orientation
  - Backside rendering (separate image)
  - Test with tests/fixtures/battle.yaml

### Leveler Cards
- [ ] Implement leveler renderer
  - Level bars (0-3, 4-7, 8+)
  - Power/toughness changes per level
  - Ability text per level
  - Test with tests/fixtures/leveler.yaml

### Prototype Cards
- [ ] Implement prototype renderer
  - Dual mana cost display
  - Dual power/toughness display
  - Prototype indicator
  - Test with tests/fixtures/prototype.yaml

### Meld Cards
- [ ] Implement meld renderer
  - Front face rendering
  - Meld result rendering (separate image)
  - Meld indicator
  - Note: No test fixture yet - create one first

## Low Priority - Validation & Polish

### Validation System
- [ ] Add warning system for missing fields
  - Creature without power/toughness
  - Planeswalker without loyalty
  - Planeswalker without loyalty_abilities
  - Saga without chapters
  - Battle without defense
- [ ] Implement type-specific validation
- [ ] Add validation tests

### CLI Improvements
- [ ] Add progress bar for batch processing
- [ ] Improve error messages with context
- [ ] Add verbose/quiet modes
- [ ] Add dry-run mode

### CSS & Styling
- [ ] Create comprehensive CSS for all card frames
- [ ] Add proper font loading (MTG fonts)
- [ ] Implement proper text sizing and wrapping
- [ ] Add reminder text styling (italics, smaller)
- [ ] Handle long card names (text scaling)

### Frame Colors
- [ ] Implement frame color derivation logic
  - Single color: Mono-colored frame
  - Two colors: Gold frame with gradient
  - Three+ colors: Gold frame
  - Colorless: Artifact/colorless frame
  - Land: Land frame (no mana cost)

## Future Enhancements

### Art Handling
- [ ] Design art system (relative paths, asset folders)
- [ ] Implement art loading and placement
- [ ] Add art cropping/scaling logic
- [ ] Support different art layouts per card type

### Advanced Features
- [ ] Set symbol rendering
- [ ] Watermark support
- [ ] Custom frame colors (override auto-detection)
- [ ] Foil/premium treatments
- [ ] Collector number and set info
- [ ] Artist credit

### Testing
- [ ] Add rendering tests (visual regression?)
- [ ] Add integration tests for full pipeline
- [ ] Test all 12 fixture files end-to-end
- [ ] Add mana parsing tests review

### Documentation
- [ ] Add examples directory with sample outputs
- [ ] Create user guide
- [ ] Document CSS customization
- [ ] Add troubleshooting guide

## Technical Debt & Improvements

### Type Safety
- [ ] Consider removing power/toughness from non-creature Card variants
  - Would require more complex enum structure
  - Evaluate if worth the added type safety
  - See AGENTS.md "Known Issues" #6

### Performance
- [ ] Profile rendering performance
- [ ] Consider caching browser instances
- [ ] Optimize HTML/CSS generation
- [ ] Batch processing optimizations

### Code Quality
- [ ] Add clippy lints
- [ ] Add rustfmt configuration
- [ ] Review and document all public APIs
- [ ] Add more inline documentation

## Completed ✅

- [x] YAML parsing infrastructure
- [x] All 13 card layout type definitions
- [x] Mana cost parsing (CastingManaCost, ActionCost)
- [x] Loyalty cost parsing
- [x] CLI framework with facet-args
- [x] File/directory input handling
- [x] Recursive YAML file discovery
- [x] All parsing tests (12 fixtures)
- [x] Mana symbol rendering (Scryfall SVG URLs)
- [x] Rules text parsing with inline symbols
- [x] Browser initialization with chromiumoxide
- [x] Chrome path configuration via CHROME_PATH
- [x] SPEC.md documentation
- [x] AGENTS.md for AI development
- [x] Test fixtures for all card types
