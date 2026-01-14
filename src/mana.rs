use facet::Facet;
use std::convert::Infallible;
use std::fmt;

/// Represents a single mana symbol valid for casting costs
#[derive(Facet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CastingManaSymbol {
    // Basic colors
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,

    // Generic/numeric
    Generic(u32),

    // Variable
    X,
    Y,
    Z,

    // Snow
    Snow,

    // Hybrid (two options)
    WhiteBlue,
    WhiteBlack,
    WhiteRed,
    WhiteGreen,
    BlueBlack,
    BlueRed,
    BlueGreen,
    BlackRed,
    BlackGreen,
    RedGreen,

    // Twobrid (generic or color)
    TwoWhite,
    TwoBlue,
    TwoBlack,
    TwoRed,
    TwoGreen,

    // Phyrexian (color or life)
    PhyrexianWhite,
    PhyrexianBlue,
    PhyrexianBlack,
    PhyrexianRed,
    PhyrexianGreen,
}

/// Represents any symbol that can appear in a cost (casting or action)
#[derive(Facet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ManaSymbol {
    Casting(CastingManaSymbol),
    // Special (Not part of casting costs usually)
    Tap,
    Untap,
    Energy,
    Chaos,
}

impl From<CastingManaSymbol> for ManaSymbol {
    fn from(s: CastingManaSymbol) -> Self {
        ManaSymbol::Casting(s)
    }
}

impl fmt::Display for CastingManaSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CastingManaSymbol::White => write!(f, "{{W}}"),
            CastingManaSymbol::Blue => write!(f, "{{U}}"),
            CastingManaSymbol::Black => write!(f, "{{B}}"),
            CastingManaSymbol::Red => write!(f, "{{R}}"),
            CastingManaSymbol::Green => write!(f, "{{G}}"),
            CastingManaSymbol::Colorless => write!(f, "{{C}}"),
            CastingManaSymbol::Generic(n) => write!(f, "{{{}}}", n),
            CastingManaSymbol::X => write!(f, "{{X}}"),
            CastingManaSymbol::Y => write!(f, "{{Y}}"),
            CastingManaSymbol::Z => write!(f, "{{Z}}"),
            CastingManaSymbol::Snow => write!(f, "{{S}}"),
            CastingManaSymbol::WhiteBlue => write!(f, "{{W/U}}"),
            CastingManaSymbol::WhiteBlack => write!(f, "{{W/B}}"),
            CastingManaSymbol::WhiteRed => write!(f, "{{W/R}}"),
            CastingManaSymbol::WhiteGreen => write!(f, "{{W/G}}"),
            CastingManaSymbol::BlueBlack => write!(f, "{{U/B}}"),
            CastingManaSymbol::BlueRed => write!(f, "{{U/R}}"),
            CastingManaSymbol::BlueGreen => write!(f, "{{U/G}}"),
            CastingManaSymbol::BlackRed => write!(f, "{{B/R}}"),
            CastingManaSymbol::BlackGreen => write!(f, "{{B/G}}"),
            CastingManaSymbol::RedGreen => write!(f, "{{R/G}}"),
            CastingManaSymbol::TwoWhite => write!(f, "{{2/W}}"),
            CastingManaSymbol::TwoBlue => write!(f, "{{2/U}}"),
            CastingManaSymbol::TwoBlack => write!(f, "{{2/B}}"),
            CastingManaSymbol::TwoRed => write!(f, "{{2/R}}"),
            CastingManaSymbol::TwoGreen => write!(f, "{{2/G}}"),
            CastingManaSymbol::PhyrexianWhite => write!(f, "{{W/P}}"),
            CastingManaSymbol::PhyrexianBlue => write!(f, "{{U/P}}"),
            CastingManaSymbol::PhyrexianBlack => write!(f, "{{B/P}}"),
            CastingManaSymbol::PhyrexianRed => write!(f, "{{R/P}}"),
            CastingManaSymbol::PhyrexianGreen => write!(f, "{{G/P}}"),
        }
    }
}

impl fmt::Display for ManaSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ManaSymbol::Casting(s) => write!(f, "{}", s),
            ManaSymbol::Tap => write!(f, "{{T}}"),
            ManaSymbol::Untap => write!(f, "{{Q}}"),
            ManaSymbol::Energy => write!(f, "{{E}}"),
            ManaSymbol::Chaos => write!(f, "{{CHAOS}}"),
        }
    }
}

/// A mana cost specifically for casting a spell (corner symbols)
#[derive(Facet, Debug, Clone, PartialEq, Eq)]
#[facet(proxy = CastingManaCostProxy)]
pub struct CastingManaCost {
    pub symbols: Vec<CastingManaSymbol>,
}

#[derive(Facet)]
#[facet(transparent)]
pub struct CastingManaCostProxy(pub String);

impl TryFrom<CastingManaCostProxy> for CastingManaCost {
    type Error = String;
    fn try_from(proxy: CastingManaCostProxy) -> Result<Self, Self::Error> {
        Self::parse(&proxy.0).map_err(|e| e.to_string())
    }
}

impl TryFrom<&CastingManaCost> for CastingManaCostProxy {
    type Error = Infallible;
    fn try_from(v: &CastingManaCost) -> Result<Self, Self::Error> {
        Ok(CastingManaCostProxy(v.to_string()))
    }
}

impl TryFrom<CastingManaCostProxy> for Option<CastingManaCost> {
    type Error = String;
    fn try_from(proxy: CastingManaCostProxy) -> Result<Self, Self::Error> {
        if proxy.0.is_empty() {
            Ok(None)
        } else {
            Ok(Some(CastingManaCost::try_from(proxy)?))
        }
    }
}

impl TryFrom<&Option<CastingManaCost>> for CastingManaCostProxy {
    type Error = Infallible;
    fn try_from(v: &Option<CastingManaCost>) -> Result<Self, Self::Error> {
        match v {
            Some(v) => Ok(CastingManaCostProxy(v.to_string())),
            None => Ok(CastingManaCostProxy("".to_string())),
        }
    }
}

/// A general cost that can include actions like tap/untap (rules text)
#[derive(Facet, Debug, Clone, PartialEq, Eq)]
#[facet(proxy = ActionCostProxy)]
pub struct ActionCost {
    pub symbols: Vec<ManaSymbol>,
}

#[derive(Facet)]
#[facet(transparent)]
pub struct ActionCostProxy(pub String);

impl TryFrom<ActionCostProxy> for ActionCost {
    type Error = String;
    fn try_from(proxy: ActionCostProxy) -> Result<Self, Self::Error> {
        Self::parse(&proxy.0).map_err(|e| e.to_string())
    }
}

impl TryFrom<&ActionCost> for ActionCostProxy {
    type Error = Infallible;
    fn try_from(v: &ActionCost) -> Result<Self, Self::Error> {
        Ok(ActionCostProxy(v.to_string()))
    }
}

/// Represents a loyalty cost for planeswalker abilities
#[derive(Facet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[facet(proxy = LoyaltyCostProxy)]
pub enum LoyaltyCost {
    Plus(u8),
    Minus(u8),
    Zero,
    PlusX,
    MinusX,
}

#[derive(Facet)]
#[facet(transparent)]
pub struct LoyaltyCostProxy(pub String);

impl TryFrom<LoyaltyCostProxy> for LoyaltyCost {
    type Error = String;
    fn try_from(proxy: LoyaltyCostProxy) -> Result<Self, Self::Error> {
        let v = proxy.0.trim().to_uppercase();
        if v == "0" {
            Ok(LoyaltyCost::Zero)
        } else if v == "+X" {
            Ok(LoyaltyCost::PlusX)
        } else if v == "-X" {
            Ok(LoyaltyCost::MinusX)
        } else if v.starts_with('+') {
            let n = v[1..]
                .parse::<u8>()
                .map_err(|_| format!("Invalid plus loyalty cost: {}", v))?;
            Ok(LoyaltyCost::Plus(n))
        } else if v.starts_with('-') {
            let n = v[1..]
                .parse::<u8>()
                .map_err(|_| format!("Invalid minus loyalty cost: {}", v))?;
            Ok(LoyaltyCost::Minus(n))
        } else if let Ok(n) = v.parse::<u8>() {
            if n == 0 {
                Ok(LoyaltyCost::Zero)
            } else {
                Ok(LoyaltyCost::Plus(n))
            }
        } else {
            Err(format!("Unknown loyalty cost: {}", v))
        }
    }
}

impl TryFrom<&LoyaltyCost> for LoyaltyCostProxy {
    type Error = Infallible;
    fn try_from(v: &LoyaltyCost) -> Result<Self, Self::Error> {
        Ok(LoyaltyCostProxy(v.to_string()))
    }
}

impl fmt::Display for LoyaltyCost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoyaltyCost::Plus(n) => write!(f, "+{}", n),
            LoyaltyCost::Minus(n) => write!(f, "-{}", n),
            LoyaltyCost::Zero => write!(f, "0"),
            LoyaltyCost::PlusX => write!(f, "+X"),
            LoyaltyCost::MinusX => write!(f, "-X"),
        }
    }
}

/// Represents a starting loyalty value
#[derive(Facet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[facet(proxy = LoyaltyValueProxy)]
pub enum LoyaltyValue {
    Numeric(u32),
    X,
}

#[derive(Facet)]
#[facet(transparent)]
pub struct LoyaltyValueProxy(pub String);

impl TryFrom<LoyaltyValueProxy> for LoyaltyValue {
    type Error = String;
    fn try_from(proxy: LoyaltyValueProxy) -> Result<Self, Self::Error> {
        let v = proxy.0.trim().to_uppercase();
        if v == "X" {
            Ok(LoyaltyValue::X)
        } else {
            let n = v
                .parse::<u32>()
                .map_err(|_| format!("Invalid loyalty value: {}", v))?;
            Ok(LoyaltyValue::Numeric(n))
        }
    }
}

impl TryFrom<&LoyaltyValue> for LoyaltyValueProxy {
    type Error = Infallible;
    fn try_from(v: &LoyaltyValue) -> Result<Self, Self::Error> {
        Ok(LoyaltyValueProxy(v.to_string()))
    }
}

impl TryFrom<LoyaltyValueProxy> for Option<LoyaltyValue> {
    type Error = String;
    fn try_from(proxy: LoyaltyValueProxy) -> Result<Self, Self::Error> {
        if proxy.0.is_empty() {
            Ok(None)
        } else {
            Ok(Some(LoyaltyValue::try_from(proxy)?))
        }
    }
}

impl TryFrom<&Option<LoyaltyValue>> for LoyaltyValueProxy {
    type Error = Infallible;
    fn try_from(v: &Option<LoyaltyValue>) -> Result<Self, Self::Error> {
        match v {
            Some(v) => Ok(LoyaltyValueProxy(v.to_string())),
            None => Ok(LoyaltyValueProxy("".to_string())),
        }
    }
}

impl TryFrom<String> for CastingManaCost {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value).map_err(|e| e.to_string())
    }
}

impl TryFrom<String> for ActionCost {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value).map_err(|e| e.to_string())
    }
}

impl TryFrom<String> for LoyaltyCost {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let v = value.trim().to_uppercase();
        if v == "0" {
            Ok(LoyaltyCost::Zero)
        } else if v == "+X" {
            Ok(LoyaltyCost::PlusX)
        } else if v == "-X" {
            Ok(LoyaltyCost::MinusX)
        } else if v.starts_with('+') {
            let n = v[1..]
                .parse::<u8>()
                .map_err(|_| format!("Invalid plus loyalty cost: {}", v))?;
            Ok(LoyaltyCost::Plus(n))
        } else if v.starts_with('-') {
            let n = v[1..]
                .parse::<u8>()
                .map_err(|_| format!("Invalid minus loyalty cost: {}", v))?;
            Ok(LoyaltyCost::Minus(n))
        } else if let Ok(n) = v.parse::<u8>() {
            if n == 0 {
                Ok(LoyaltyCost::Zero)
            } else {
                Ok(LoyaltyCost::Plus(n))
            }
        } else {
            Err(format!("Unknown loyalty cost: {}", v))
        }
    }
}

impl TryFrom<String> for LoyaltyValue {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let v = value.trim().to_uppercase();
        if v == "X" {
            Ok(LoyaltyValue::X)
        } else {
            let n = v
                .parse::<u32>()
                .map_err(|_| format!("Invalid loyalty value: {}", v))?;
            Ok(LoyaltyValue::Numeric(n))
        }
    }
}

impl fmt::Display for LoyaltyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoyaltyValue::Numeric(n) => write!(f, "{}", n),
            LoyaltyValue::X => write!(f, "X"),
        }
    }
}

impl CastingManaCost {
    pub fn parse(input: &str) -> Result<Self, ManaCostParseError> {
        let mut symbols = Vec::new();
        let bytes = input.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i] == b'{' {
                let start = i + 1;
                let end = bytes[start..]
                    .iter()
                    .position(|&b| b == b'}')
                    .ok_or(ManaCostParseError::UnclosedBrace { position: i })?;
                let content = std::str::from_utf8(&bytes[start..start + end])
                    .map_err(|_| ManaCostParseError::InvalidUtf8)?;

                let symbol = Self::parse_symbol(content)?;
                symbols.push(symbol);
                i = start + end + 1;
            } else if bytes[i].is_ascii_whitespace() {
                i += 1;
            } else {
                return Err(ManaCostParseError::UnexpectedCharacter {
                    character: bytes[i] as char,
                    position: i,
                });
            }
        }

        Ok(CastingManaCost { symbols })
    }

    pub fn parse_symbol(content: &str) -> Result<CastingManaSymbol, ManaCostParseError> {
        match content {
            "W" => Ok(CastingManaSymbol::White),
            "U" => Ok(CastingManaSymbol::Blue),
            "B" => Ok(CastingManaSymbol::Black),
            "R" => Ok(CastingManaSymbol::Red),
            "G" => Ok(CastingManaSymbol::Green),
            "C" => Ok(CastingManaSymbol::Colorless),
            "X" => Ok(CastingManaSymbol::X),
            "Y" => Ok(CastingManaSymbol::Y),
            "Z" => Ok(CastingManaSymbol::Z),
            "S" => Ok(CastingManaSymbol::Snow),
            // Hybrid colors (both orderings for compatibility)
            "W/U" | "U/W" => Ok(CastingManaSymbol::WhiteBlue),
            "W/B" | "B/W" => Ok(CastingManaSymbol::WhiteBlack),
            "W/R" | "R/W" => Ok(CastingManaSymbol::WhiteRed),
            "W/G" | "G/W" => Ok(CastingManaSymbol::WhiteGreen),
            "U/B" | "B/U" => Ok(CastingManaSymbol::BlueBlack),
            "U/R" | "R/U" => Ok(CastingManaSymbol::BlueRed),
            "U/G" | "G/U" => Ok(CastingManaSymbol::BlueGreen),
            "B/R" | "R/B" => Ok(CastingManaSymbol::BlackRed),
            "B/G" | "G/B" => Ok(CastingManaSymbol::BlackGreen),
            "R/G" | "G/R" => Ok(CastingManaSymbol::RedGreen),
            // Twobrid
            "2/W" => Ok(CastingManaSymbol::TwoWhite),
            "2/U" => Ok(CastingManaSymbol::TwoBlue),
            "2/B" => Ok(CastingManaSymbol::TwoBlack),
            "2/R" => Ok(CastingManaSymbol::TwoRed),
            "2/G" => Ok(CastingManaSymbol::TwoGreen),
            // Phyrexian
            "W/P" => Ok(CastingManaSymbol::PhyrexianWhite),
            "U/P" => Ok(CastingManaSymbol::PhyrexianBlue),
            "B/P" => Ok(CastingManaSymbol::PhyrexianBlack),
            "R/P" => Ok(CastingManaSymbol::PhyrexianRed),
            "G/P" => Ok(CastingManaSymbol::PhyrexianGreen),
            // Generic numbers
            s => {
                if let Ok(num) = s.parse::<u32>() {
                    Ok(CastingManaSymbol::Generic(num))
                } else {
                    Err(ManaCostParseError::UnknownSymbol {
                        symbol: s.to_string(),
                    })
                }
            }
        }
    }

    /// Get the total generic mana cost (including all numeric symbols)
    pub fn generic_cost(&self) -> u32 {
        self.symbols
            .iter()
            .filter_map(|s| match s {
                CastingManaSymbol::Generic(n) => Some(*n),
                _ => None,
            })
            .sum()
    }

    /// Check if this mana cost contains a variable component (X, Y, or Z)
    pub fn has_variable(&self) -> bool {
        self.symbols.iter().any(|s| {
            matches!(
                s,
                CastingManaSymbol::X | CastingManaSymbol::Y | CastingManaSymbol::Z
            )
        })
    }

    /// Count how many colored mana symbols are in this cost
    pub fn colored_count(&self) -> u32 {
        self.symbols
            .iter()
            .filter(|s| {
                matches!(
                    s,
                    CastingManaSymbol::White
                        | CastingManaSymbol::Blue
                        | CastingManaSymbol::Black
                        | CastingManaSymbol::Red
                        | CastingManaSymbol::Green
                )
            })
            .count() as u32
    }
}

impl ActionCost {
    pub fn parse(input: &str) -> Result<Self, ManaCostParseError> {
        let mut symbols = Vec::new();
        let bytes = input.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i] == b'{' {
                let start = i + 1;
                let end = bytes[start..]
                    .iter()
                    .position(|&b| b == b'}')
                    .ok_or(ManaCostParseError::UnclosedBrace { position: i })?;
                let content = std::str::from_utf8(&bytes[start..start + end])
                    .map_err(|_| ManaCostParseError::InvalidUtf8)?;

                let symbol = Self::parse_symbol(content)?;
                symbols.push(symbol);
                i = start + end + 1;
            } else if bytes[i].is_ascii_whitespace() {
                i += 1;
            } else {
                return Err(ManaCostParseError::UnexpectedCharacter {
                    character: bytes[i] as char,
                    position: i,
                });
            }
        }

        Ok(ActionCost { symbols })
    }

    fn parse_symbol(content: &str) -> Result<ManaSymbol, ManaCostParseError> {
        match content {
            "T" => Ok(ManaSymbol::Tap),
            "Q" => Ok(ManaSymbol::Untap),
            "E" => Ok(ManaSymbol::Energy),
            "CHAOS" => Ok(ManaSymbol::Chaos),
            // Fallback to mana symbol parsing if it matches
            s => {
                if let Ok(mana) = CastingManaCost::parse_symbol(s) {
                    Ok(ManaSymbol::Casting(mana))
                } else {
                    Err(ManaCostParseError::UnknownSymbol {
                        symbol: s.to_string(),
                    })
                }
            }
        }
    }
}

impl fmt::Display for CastingManaCost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for symbol in &self.symbols {
            write!(f, "{}", symbol)?;
        }
        Ok(())
    }
}

impl fmt::Display for ActionCost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for symbol in &self.symbols {
            write!(f, "{}", symbol)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManaCostParseError {
    UnclosedBrace { position: usize },
    UnexpectedCharacter { character: char, position: usize },
    UnknownSymbol { symbol: String },
    InvalidUtf8,
}

impl fmt::Display for ManaCostParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ManaCostParseError::UnclosedBrace { position } => {
                write!(f, "Unclosed brace at position {}", position)
            }
            ManaCostParseError::UnexpectedCharacter {
                character,
                position,
            } => {
                write!(
                    f,
                    "Unexpected character '{}' at position {}",
                    character, position
                )
            }
            ManaCostParseError::UnknownSymbol { symbol } => {
                write!(f, "Unknown mana symbol: {}", symbol)
            }
            ManaCostParseError::InvalidUtf8 => {
                write!(f, "Invalid UTF-8 in mana cost")
            }
        }
    }
}

impl std::error::Error for ManaCostParseError {}

/// Implement From<&str> for convenience
impl From<&str> for CastingManaCost {
    fn from(s: &str) -> Self {
        CastingManaCost::parse(s).unwrap_or_else(|_| CastingManaCost { symbols: vec![] })
    }
}

impl From<&str> for ActionCost {
    fn from(s: &str) -> Self {
        ActionCost::parse(s).unwrap_or_else(|_| ActionCost { symbols: vec![] })
    }
}
