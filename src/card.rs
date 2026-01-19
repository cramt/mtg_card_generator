use crate::mana::{
    CastingManaCost, CastingManaCostProxy, LoyaltyCost, LoyaltyCostProxy, LoyaltyValue, RulesText,
    RulesTextProxy,
};
use facet::Facet;

#[derive(Facet, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Rarity {
    #[facet(rename = "common")]
    Common,
    #[facet(rename = "uncommon")]
    Uncommon,
    #[facet(rename = "rare")]
    Rare,
    #[facet(rename = "mythic")]
    Mythic,
}

/// A single chapter in a saga
#[derive(Facet, Debug, Clone, PartialEq, Eq)]
pub struct SagaChapter {
    /// Chapter numbers this ability applies to
    pub chapters: Vec<u32>,
    /// The chapter text
    #[facet(proxy = RulesTextProxy)]
    pub text: RulesText,
}

/// A level in a class enchantment
#[derive(Facet, Debug, Clone)]
pub struct ClassLevel {
    /// The level number (1, 2, 3)
    pub level: u32,
    /// Cost to level up to this level (only for level 2+)
    #[facet(default, proxy = CastingManaCostProxy)]
    pub cost: Option<CastingManaCost>,
    /// Ability text for this level
    #[facet(proxy = RulesTextProxy)]
    pub text: RulesText,
}

/// An adventure spell (the left side of an adventure card)
#[derive(Facet, Debug, Clone)]
pub struct AdventureSpell {
    /// The adventure spell name
    pub name: String,
    /// The adventure spell's mana cost
    #[facet(proxy = CastingManaCostProxy)]
    pub mana_cost: CastingManaCost,
    /// The adventure spell's type
    pub type_line: String,
    /// The adventure spell's rules text
    #[facet(proxy = RulesTextProxy)]
    pub rules_text: RulesText,
}

/// A loyalty ability on a planeswalker
#[derive(Facet, Debug, Clone)]
pub struct LoyaltyAbility {
    /// Cost (e.g., "+2", "-1", "0")
    #[facet(proxy = LoyaltyCostProxy)]
    pub cost: LoyaltyCost,
    /// Ability text
    #[facet(proxy = RulesTextProxy)]
    pub text: RulesText,
}

/// A level range for leveler creatures
#[derive(Facet, Debug, Clone)]
pub struct LevelerRange {
    /// Level range (e.g., 0..3 means 0-3)
    pub range: Vec<Option<u32>>,
    /// Power for this range
    #[facet(default)]
    pub power: Option<String>,
    /// Toughness for this range
    #[facet(default)]
    pub toughness: Option<String>,
    /// Ability text for this range
    #[facet(default, proxy = RulesTextProxy)]
    pub text: Option<RulesText>,
}

/// A card face (for DFC, split, flip, etc.)
#[derive(Facet, Debug, Clone)]
pub struct CardFace {
    /// Face name
    #[facet(default)]
    pub name: Option<String>,
    /// Mana cost
    #[facet(default, proxy = CastingManaCostProxy)]
    pub mana_cost: Option<CastingManaCost>,
    /// Type line
    #[facet(default)]
    pub type_line: Option<String>,
    /// Rules text
    #[facet(default, proxy = RulesTextProxy)]
    pub rules_text: Option<RulesText>,
    /// Flavor text
    #[facet(default)]
    pub flavor_text: Option<String>,
    /// Power (for creatures)
    #[facet(default)]
    pub power: Option<String>,
    /// Toughness (for creatures)
    #[facet(default)]
    pub toughness: Option<String>,
    /// Color indicator (for colorless spells or multi-colored cards without mana cost)
    #[facet(default)]
    pub color_indicator: Option<Vec<String>>,
}

/// Common fields shared by all card types
#[derive(Facet, Debug, Clone)]
pub struct CardBase {
    /// Card name
    pub name: String,
    /// Mana cost
    #[facet(default, proxy = CastingManaCostProxy)]
    pub mana_cost: Option<CastingManaCost>,
    /// Type line
    pub type_line: String,
    /// Rules text (for static abilities, etc.)
    #[facet(default, proxy = RulesTextProxy)]
    pub rules_text: Option<RulesText>,
    /// Flavor text
    #[facet(default)]
    pub flavor_text: Option<String>,
    /// Power (for creatures)
    #[facet(default)]
    pub power: Option<String>,
    /// Toughness (for creatures)
    #[facet(default)]
    pub toughness: Option<String>,
    /// Card rarity
    pub rarity: Rarity,
}

// ============================================================================
// Card Type Structs
// ============================================================================

/// A standard card (creature, instant, sorcery, enchantment, artifact)
#[derive(Facet, Debug, Clone)]
pub struct NormalCard {
    #[facet(flatten)]
    pub base: CardBase,
}

/// A planeswalker card with loyalty abilities
#[derive(Facet, Debug, Clone)]
pub struct PlaneswalkerCard {
    #[facet(flatten)]
    pub base: CardBase,
    pub loyalty: LoyaltyValue,
    pub loyalty_abilities: Vec<LoyaltyAbility>,
}

/// A saga enchantment with chapter abilities
#[derive(Facet, Debug, Clone)]
pub struct SagaCard {
    #[facet(flatten)]
    pub base: CardBase,
    pub chapters: Vec<SagaChapter>,
}

/// A class enchantment with level-up abilities
#[derive(Facet, Debug, Clone)]
pub struct ClassCard {
    #[facet(flatten)]
    pub base: CardBase,
    pub levels: Vec<ClassLevel>,
}

/// An adventure card (creature with an adventure spell)
#[derive(Facet, Debug, Clone)]
pub struct AdventureCard {
    #[facet(flatten)]
    pub base: CardBase,
    pub adventure: AdventureSpell,
}

/// A split card (two spells side-by-side, like Fire // Ice)
#[derive(Facet, Debug, Clone)]
pub struct SplitCard {
    #[facet(flatten)]
    pub base: CardBase,
    pub faces: Vec<CardFace>,
    #[facet(default)]
    pub fuse: Option<bool>,
    #[facet(default)]
    pub aftermath: Option<bool>,
}

/// A flip card (Kamigawa-style, rotated bottom half)
#[derive(Facet, Debug, Clone)]
pub struct FlipCard {
    #[facet(flatten)]
    pub base: CardBase,
    pub faces: Vec<CardFace>,
}

/// A transform double-faced card (like Delver of Secrets)
#[derive(Facet, Debug, Clone)]
pub struct TransformCard {
    #[facet(flatten)]
    pub base: CardBase,
    pub faces: Vec<CardFace>,
}

/// A modal double-faced card (either side playable)
#[derive(Facet, Debug, Clone)]
pub struct ModalDfcCard {
    #[facet(flatten)]
    pub base: CardBase,
    pub faces: Vec<CardFace>,
}

/// A battle card with defense counter
#[derive(Facet, Debug, Clone)]
pub struct BattleCard {
    #[facet(flatten)]
    pub base: CardBase,
    pub defense: u32,
    pub backside_name: String,
    pub backside_type_line: String,
    #[facet(proxy = RulesTextProxy)]
    pub backside_rules_text: RulesText,
}

/// A meld card (two cards that combine into one)
#[derive(Facet, Debug, Clone)]
pub struct MeldCard {
    #[facet(flatten)]
    pub base: CardBase,
    pub faces: Vec<CardFace>,
}

/// A leveler creature (Rise of the Eldrazi style)
#[derive(Facet, Debug, Clone)]
pub struct LevelerCard {
    #[facet(flatten)]
    pub base: CardBase,
    pub leveler_ranges: Vec<LevelerRange>,
}

/// A prototype card (two casting costs/stats)
#[derive(Facet, Debug, Clone)]
pub struct PrototypeCard {
    #[facet(flatten)]
    pub base: CardBase,
    pub prototype: CardFace,
}

// ============================================================================
// Card Enum (for parsing)
// ============================================================================

/// A Magic: The Gathering card.
///
/// This enum represents all possible card layouts. Each variant wraps a
/// specific card type struct that contains the layout-specific data.
#[derive(Facet, Debug, Clone)]
#[facet(tag = "type")]
#[repr(C)]
pub enum Card {
    #[facet(rename = "normal")]
    Normal(#[facet(flatten)] NormalCard),

    #[facet(rename = "planeswalker")]
    Planeswalker(#[facet(flatten)] PlaneswalkerCard),

    #[facet(rename = "saga")]
    Saga(#[facet(flatten)] SagaCard),

    #[facet(rename = "class")]
    Class(#[facet(flatten)] ClassCard),

    #[facet(rename = "adventure")]
    Adventure(#[facet(flatten)] AdventureCard),

    #[facet(rename = "split")]
    Split(#[facet(flatten)] SplitCard),

    #[facet(rename = "flip")]
    Flip(#[facet(flatten)] FlipCard),

    #[facet(rename = "transform")]
    Transform(#[facet(flatten)] TransformCard),

    #[facet(rename = "modal_dfc")]
    ModalDfc(#[facet(flatten)] ModalDfcCard),

    #[facet(rename = "battle")]
    Battle(#[facet(flatten)] BattleCard),

    #[facet(rename = "meld")]
    Meld(#[facet(flatten)] MeldCard),

    #[facet(rename = "leveler")]
    Leveler(#[facet(flatten)] LevelerCard),

    #[facet(rename = "prototype")]
    Prototype(#[facet(flatten)] PrototypeCard),
}

impl Card {
    /// Returns a reference to the common card base fields.
    ///
    /// This provides access to name, mana_cost, type_line, rules_text, flavor_text,
    /// power, toughness, and rarity without needing to pattern match on the variant.
    #[must_use]
    pub fn base(&self) -> &CardBase {
        match self {
            Card::Normal(card) => &card.base,
            Card::Planeswalker(card) => &card.base,
            Card::Saga(card) => &card.base,
            Card::Class(card) => &card.base,
            Card::Adventure(card) => &card.base,
            Card::Split(card) => &card.base,
            Card::Flip(card) => &card.base,
            Card::Transform(card) => &card.base,
            Card::ModalDfc(card) => &card.base,
            Card::Battle(card) => &card.base,
            Card::Meld(card) => &card.base,
            Card::Leveler(card) => &card.base,
            Card::Prototype(card) => &card.base,
        }
    }

    /// Returns the card's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.base().name
    }

    /// Returns the card's rarity.
    #[must_use]
    pub fn rarity(&self) -> Rarity {
        self.base().rarity
    }
}
