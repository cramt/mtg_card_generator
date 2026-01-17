use crate::mana::{
    CastingManaCost, CastingManaCostProxy, LoyaltyCost, LoyaltyCostProxy, LoyaltyValue,
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
    pub text: String,
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
    pub text: String,
}

/// An adventure card's adventure half
#[derive(Facet, Debug, Clone)]
pub struct AdventureCard {
    /// The adventure spell name
    pub name: String,
    /// The adventure spell's mana cost
    #[facet(proxy = CastingManaCostProxy)]
    pub mana_cost: CastingManaCost,
    /// The adventure spell's type
    pub type_line: String,
    /// The adventure spell's rules text
    pub rules_text: String,
}

/// A loyalty ability on a planeswalker
#[derive(Facet, Debug, Clone)]
pub struct LoyaltyAbility {
    /// Cost (e.g., "+2", "-1", "0")
    #[facet(proxy = LoyaltyCostProxy)]
    pub cost: LoyaltyCost,
    /// Ability text
    pub text: String,
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
    #[facet(default)]
    pub text: Option<String>,
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
    #[facet(default)]
    pub rules_text: Option<String>,
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
    #[facet(default)]
    pub rules_text: Option<String>,
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

/// Variant-specific data for different card types
///
/// Each variant contains a `CardBase` with common fields like name, mana cost, type line, etc.
/// Use the `base()` method to access the common fields without pattern matching.
#[derive(Facet, Debug, Clone)]
#[facet(tag = "type")]
#[repr(C)]
pub enum Card {
    #[facet(rename = "normal")]
    Normal {
        #[facet(flatten)]
        base: CardBase,
    },
    #[facet(rename = "planeswalker")]
    Planeswalker {
        #[facet(flatten)]
        base: CardBase,
        loyalty: LoyaltyValue,
        loyalty_abilities: Vec<LoyaltyAbility>,
    },
    #[facet(rename = "saga")]
    Saga {
        #[facet(flatten)]
        base: CardBase,
        chapters: Vec<SagaChapter>,
    },
    #[facet(rename = "class")]
    Class {
        #[facet(flatten)]
        base: CardBase,
        levels: Vec<ClassLevel>,
    },
    #[facet(rename = "adventure")]
    Adventure {
        #[facet(flatten)]
        base: CardBase,
        adventure: AdventureCard,
    },
    #[facet(rename = "split")]
    Split {
        #[facet(flatten)]
        base: CardBase,
        faces: Vec<CardFace>,
        #[facet(default)]
        fuse: Option<bool>,
        #[facet(default)]
        aftermath: Option<bool>,
    },
    #[facet(rename = "flip")]
    Flip {
        #[facet(flatten)]
        base: CardBase,
        faces: Vec<CardFace>,
    },
    #[facet(rename = "transform")]
    Transform {
        #[facet(flatten)]
        base: CardBase,
        faces: Vec<CardFace>,
    },
    #[facet(rename = "modal_dfc")]
    ModalDfc {
        #[facet(flatten)]
        base: CardBase,
        faces: Vec<CardFace>,
    },
    #[facet(rename = "battle")]
    Battle {
        #[facet(flatten)]
        base: CardBase,
        defense: u32,
        backside_name: String,
        backside_type_line: String,
        backside_rules_text: String,
    },
    #[facet(rename = "meld")]
    Meld {
        #[facet(flatten)]
        base: CardBase,
        faces: Vec<CardFace>,
    },
    #[facet(rename = "leveler")]
    Leveler {
        #[facet(flatten)]
        base: CardBase,
        leveler_ranges: Vec<LevelerRange>,
    },
    #[facet(rename = "prototype")]
    Prototype {
        #[facet(flatten)]
        base: CardBase,
        prototype: CardFace,
    },
}

impl Card {
    /// Returns a reference to the common card base fields.
    ///
    /// This provides access to name, mana_cost, type_line, rules_text, flavor_text,
    /// power, toughness, and rarity without needing to pattern match on the variant.
    #[must_use]
    pub fn base(&self) -> &CardBase {
        match self {
            Card::Normal { base }
            | Card::Planeswalker { base, .. }
            | Card::Saga { base, .. }
            | Card::Class { base, .. }
            | Card::Adventure { base, .. }
            | Card::Split { base, .. }
            | Card::Flip { base, .. }
            | Card::Transform { base, .. }
            | Card::ModalDfc { base, .. }
            | Card::Battle { base, .. }
            | Card::Meld { base, .. }
            | Card::Leveler { base, .. }
            | Card::Prototype { base, .. } => base,
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
