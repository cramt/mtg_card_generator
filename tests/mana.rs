use mtg_gen::*;

#[test]
fn test_parse_single_colors() {
    assert_eq!(
        CastingManaCost::parse("{W}").unwrap().symbols,
        vec![CastingManaSymbol::White]
    );
    assert_eq!(
        CastingManaCost::parse("{U}").unwrap().symbols,
        vec![CastingManaSymbol::Blue]
    );
    assert_eq!(
        CastingManaCost::parse("{B}").unwrap().symbols,
        vec![CastingManaSymbol::Black]
    );
    assert_eq!(
        CastingManaCost::parse("{R}").unwrap().symbols,
        vec![CastingManaSymbol::Red]
    );
    assert_eq!(
        CastingManaCost::parse("{G}").unwrap().symbols,
        vec![CastingManaSymbol::Green]
    );
}

#[test]
fn test_parse_colorless_and_generic() {
    assert_eq!(
        CastingManaCost::parse("{C}").unwrap().symbols,
        vec![CastingManaSymbol::Colorless]
    );
    assert_eq!(
        CastingManaCost::parse("{0}").unwrap().symbols,
        vec![CastingManaSymbol::Generic(0)]
    );
    assert_eq!(
        CastingManaCost::parse("{1}").unwrap().symbols,
        vec![CastingManaSymbol::Generic(1)]
    );
    assert_eq!(
        CastingManaCost::parse("{5}").unwrap().symbols,
        vec![CastingManaSymbol::Generic(5)]
    );
    assert_eq!(
        CastingManaCost::parse("{20}").unwrap().symbols,
        vec![CastingManaSymbol::Generic(20)]
    );
}

#[test]
fn test_parse_variables() {
    assert_eq!(
        CastingManaCost::parse("{X}").unwrap().symbols,
        vec![CastingManaSymbol::X]
    );
    assert_eq!(
        CastingManaCost::parse("{Y}").unwrap().symbols,
        vec![CastingManaSymbol::Y]
    );
    assert_eq!(
        CastingManaCost::parse("{Z}").unwrap().symbols,
        vec![CastingManaSymbol::Z]
    );
}

#[test]
fn test_parse_snow() {
    assert_eq!(
        CastingManaCost::parse("{S}").unwrap().symbols,
        vec![CastingManaSymbol::Snow]
    );
}

#[test]
fn test_parse_hybrid_colors() {
    assert_eq!(
        CastingManaCost::parse("{W/U}").unwrap().symbols,
        vec![CastingManaSymbol::WhiteBlue]
    );
    assert_eq!(
        CastingManaCost::parse("{W/B}").unwrap().symbols,
        vec![CastingManaSymbol::WhiteBlack]
    );
    assert_eq!(
        CastingManaCost::parse("{W/R}").unwrap().symbols,
        vec![CastingManaSymbol::WhiteRed]
    );
    assert_eq!(
        CastingManaCost::parse("{W/G}").unwrap().symbols,
        vec![CastingManaSymbol::WhiteGreen]
    );
    assert_eq!(
        CastingManaCost::parse("{U/B}").unwrap().symbols,
        vec![CastingManaSymbol::BlueBlack]
    );
    assert_eq!(
        CastingManaCost::parse("{U/R}").unwrap().symbols,
        vec![CastingManaSymbol::BlueRed]
    );
    assert_eq!(
        CastingManaCost::parse("{U/G}").unwrap().symbols,
        vec![CastingManaSymbol::BlueGreen]
    );
    assert_eq!(
        CastingManaCost::parse("{B/R}").unwrap().symbols,
        vec![CastingManaSymbol::BlackRed]
    );
    assert_eq!(
        CastingManaCost::parse("{B/G}").unwrap().symbols,
        vec![CastingManaSymbol::BlackGreen]
    );
    assert_eq!(
        CastingManaCost::parse("{R/G}").unwrap().symbols,
        vec![CastingManaSymbol::RedGreen]
    );
}

#[test]
fn test_parse_twobrid() {
    assert_eq!(
        CastingManaCost::parse("{2/W}").unwrap().symbols,
        vec![CastingManaSymbol::TwoWhite]
    );
    assert_eq!(
        CastingManaCost::parse("{2/U}").unwrap().symbols,
        vec![CastingManaSymbol::TwoBlue]
    );
    assert_eq!(
        CastingManaCost::parse("{2/B}").unwrap().symbols,
        vec![CastingManaSymbol::TwoBlack]
    );
    assert_eq!(
        CastingManaCost::parse("{2/R}").unwrap().symbols,
        vec![CastingManaSymbol::TwoRed]
    );
    assert_eq!(
        CastingManaCost::parse("{2/G}").unwrap().symbols,
        vec![CastingManaSymbol::TwoGreen]
    );
}

#[test]
fn test_parse_phyrexian() {
    assert_eq!(
        CastingManaCost::parse("{W/P}").unwrap().symbols,
        vec![CastingManaSymbol::PhyrexianWhite]
    );
    assert_eq!(
        CastingManaCost::parse("{U/P}").unwrap().symbols,
        vec![CastingManaSymbol::PhyrexianBlue]
    );
    assert_eq!(
        CastingManaCost::parse("{B/P}").unwrap().symbols,
        vec![CastingManaSymbol::PhyrexianBlack]
    );
    assert_eq!(
        CastingManaCost::parse("{R/P}").unwrap().symbols,
        vec![CastingManaSymbol::PhyrexianRed]
    );
    assert_eq!(
        CastingManaCost::parse("{G/P}").unwrap().symbols,
        vec![CastingManaSymbol::PhyrexianGreen]
    );
}

#[test]
fn test_parse_action_symbols() {
    assert_eq!(
        ActionCost::parse("{T}").unwrap().symbols,
        vec![ManaSymbol::Tap]
    );
    assert_eq!(
        ActionCost::parse("{Q}").unwrap().symbols,
        vec![ManaSymbol::Untap]
    );
    assert_eq!(
        ActionCost::parse("{E}").unwrap().symbols,
        vec![ManaSymbol::Energy]
    );
    assert_eq!(
        ActionCost::parse("{CHAOS}").unwrap().symbols,
        vec![ManaSymbol::Chaos]
    );
    // ActionCost should also support casting symbols via composition
    assert_eq!(
        ActionCost::parse("{W}").unwrap().symbols,
        vec![ManaSymbol::Casting(CastingManaSymbol::White)]
    );
}

#[test]
fn test_parse_complex_cost() {
    // Lightning Bolt: {R}
    let cost = CastingManaCost::parse("{R}").unwrap();
    assert_eq!(cost.symbols.len(), 1);
    assert_eq!(cost.colored_count(), 1);

    // Counterspell: {U}{U}
    let cost = CastingManaCost::parse("{U}{U}").unwrap();
    assert_eq!(cost.symbols.len(), 2);
    assert_eq!(cost.colored_count(), 2);

    // Doom Whisperer: {1}{B}{B}
    let cost = CastingManaCost::parse("{1}{B}{B}").unwrap();
    assert_eq!(cost.symbols.len(), 3);
    assert_eq!(cost.generic_cost(), 1);
    assert_eq!(cost.colored_count(), 2);

    // Jace TMS: {2}{U}{U}
    let cost = CastingManaCost::parse("{2}{U}{U}").unwrap();
    assert_eq!(cost.symbols.len(), 3);
    assert_eq!(cost.generic_cost(), 2);
    assert_eq!(cost.colored_count(), 2);
}

#[test]
fn test_parse_with_whitespace() {
    // Should handle spaces
    let cost1 = CastingManaCost::parse("{2} {U} {U}").unwrap();
    let cost2 = CastingManaCost::parse("{2}{U}{U}").unwrap();
    assert_eq!(cost1.symbols, cost2.symbols);
}

#[test]
fn test_parse_variable_cost() {
    let cost = CastingManaCost::parse("{X}{U}{U}").unwrap();
    assert!(cost.has_variable());
    assert_eq!(cost.colored_count(), 2);

    let cost = CastingManaCost::parse("{Y}{1}{G}").unwrap();
    assert!(cost.has_variable());

    let cost = CastingManaCost::parse("{R}{R}").unwrap();
    assert!(!cost.has_variable());
}

#[test]
fn test_parse_empty_cost() {
    let cost = CastingManaCost::parse("").unwrap();
    assert_eq!(cost.symbols.len(), 0);
    assert_eq!(cost.generic_cost(), 0);
    assert!(!cost.has_variable());
}

#[test]
fn test_parse_big_generic() {
    let cost = CastingManaCost::parse("{15}").unwrap();
    assert_eq!(cost.generic_cost(), 15);
    assert_eq!(cost.symbols.len(), 1);
}

#[test]
fn test_display_casting_symbols() {
    let cost = CastingManaCost::parse("{W}").unwrap();
    assert_eq!(cost.to_string(), "{W}");

    let cost = CastingManaCost::parse("{2}{U}{U}").unwrap();
    assert_eq!(cost.to_string(), "{2}{U}{U}");
}

#[test]
fn test_display_action_cost() {
    let cost = ActionCost::parse("{T}{1}{U}").unwrap();
    assert_eq!(cost.to_string(), "{T}{1}{U}");
}

#[test]
fn test_casting_cost_cannot_have_tap() {
    let err = CastingManaCost::parse("{T}").unwrap_err();
    assert!(matches!(err, ManaCostParseError::UnknownSymbol { .. }));
}

#[test]
fn test_real_world_examples() {
    // Siege Rhino
    let cost = CastingManaCost::parse("{1}{W}{B}{G}").unwrap();
    assert_eq!(cost.symbols.len(), 4);
    assert_eq!(cost.colored_count(), 3);
    assert_eq!(cost.generic_cost(), 1);

    // Rakdos Cackler
    let cost = CastingManaCost::parse("{R/B}").unwrap();
    assert_eq!(cost.symbols.len(), 1);
    assert_eq!(cost.generic_cost(), 0);
}

#[test]
fn test_loyalty_cost_parsing() {
    use std::convert::TryFrom;
    assert_eq!(
        LoyaltyCost::try_from("+2".to_string()).unwrap(),
        LoyaltyCost::Plus(2)
    );
    assert_eq!(
        LoyaltyCost::try_from("-3".to_string()).unwrap(),
        LoyaltyCost::Minus(3)
    );
    assert_eq!(
        LoyaltyCost::try_from("0".to_string()).unwrap(),
        LoyaltyCost::Zero
    );
    assert_eq!(
        LoyaltyCost::try_from("+X".to_string()).unwrap(),
        LoyaltyCost::PlusX
    );
    assert_eq!(
        LoyaltyCost::try_from("-X".to_string()).unwrap(),
        LoyaltyCost::MinusX
    );
    assert_eq!(
        LoyaltyCost::try_from("2".to_string()).unwrap(),
        LoyaltyCost::Plus(2)
    );
}

#[test]
fn test_loyalty_cost_display() {
    assert_eq!(LoyaltyCost::Plus(2).to_string(), "+2");
    assert_eq!(LoyaltyCost::Minus(3).to_string(), "-3");
    assert_eq!(LoyaltyCost::Zero.to_string(), "0");
    assert_eq!(LoyaltyCost::PlusX.to_string(), "+X");
    assert_eq!(LoyaltyCost::MinusX.to_string(), "-X");
}
