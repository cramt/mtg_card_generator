use facet_yaml::from_str;
use mtg_gen::*;
use std::fs;

fn read_fixture(name: &str) -> String {
    let path = format!("tests/fixtures/{}.yaml", name);
    fs::read_to_string(path).expect("Failed to read fixture")
}

#[test]
fn test_parse_normal_creature() {
    let yaml = read_fixture("normal_creature");
    let card: Card = from_str(&yaml).expect("Failed to parse normal creature");

    if let Card::Normal(normal) = card {
        let base = &normal.base;
        assert_eq!(base.name, "Llanowar Elves");
        let mana = base
            .mana_cost
            .clone()
            .unwrap_or_else(|| CastingManaCost { symbols: vec![] });
        assert_eq!(mana.symbols.len(), 1);
        assert_eq!(mana.symbols[0], CastingManaSymbol::Green);
        assert_eq!(
            base.mana_cost.as_ref().map(|c| c.to_string()),
            Some("{G}".to_string())
        );
        assert_eq!(base.type_line, "Creature — Elf Druid");
        assert_eq!(base.rules_text, Some("{T}: Add {G}.".to_string()));
        assert_eq!(
            base.flavor_text,
            Some("One bone broken for every twig snapped underfoot.".to_string())
        );
        assert_eq!(base.power, Some("1".to_string()));
        assert_eq!(base.toughness, Some("1".to_string()));
        assert_eq!(base.rarity, Rarity::Common);
    } else {
        panic!("Expected Normal variant");
    }
}

#[test]
fn test_parse_planeswalker() {
    let yaml = read_fixture("planeswalker");
    let card: Card = from_str(&yaml).expect("Failed to parse planeswalker");

    if let Card::Planeswalker(pw) = card {
        assert_eq!(pw.base.name, "Jace, the Mind Sculptor");
        assert_eq!(pw.loyalty, LoyaltyValue::Numeric(3));
        assert_eq!(pw.loyalty_abilities.len(), 4);
        assert_eq!(pw.loyalty_abilities[0].cost, LoyaltyCost::Plus(2));
        assert_eq!(pw.loyalty_abilities[1].cost, LoyaltyCost::Zero);
        assert_eq!(pw.loyalty_abilities[2].cost, LoyaltyCost::Minus(1));
        assert_eq!(pw.loyalty_abilities[3].cost, LoyaltyCost::Minus(12));
    } else {
        panic!("Expected Planeswalker variant");
    }
}

#[test]
fn test_parse_saga() {
    let yaml = read_fixture("saga");
    let card: Card = from_str(&yaml).expect("Failed to parse saga");

    if let Card::Saga(saga) = card {
        assert_eq!(saga.base.name, "The Eldest Reborn");
        assert_eq!(saga.chapters.len(), 3);
        assert_eq!(saga.chapters[0].chapters, vec![1]);
        assert_eq!(saga.chapters[1].chapters, vec![2]);
        assert_eq!(saga.chapters[2].chapters, vec![3]);
    } else {
        panic!("Expected Saga variant");
    }
}

#[test]
fn test_parse_class() {
    let yaml = read_fixture("class");
    let card: Card = from_str(&yaml).expect("Failed to parse class");

    if let Card::Class(class) = card {
        assert_eq!(class.base.name, "Ranger Class");
        assert_eq!(class.levels.len(), 3);
        assert_eq!(class.levels[0].level, 1);
        assert_eq!(class.levels[0].cost, None);
        assert_eq!(class.levels[1].level, 2);
        assert_eq!(
            class.levels[1].cost.as_ref().map(|c| c.to_string()),
            Some("{1}{G}".to_string())
        );
    } else {
        panic!("Expected Class variant");
    }
}

#[test]
fn test_parse_adventure() {
    let yaml = read_fixture("adventure");
    let card: Card = from_str(&yaml).expect("Failed to parse adventure");

    if let Card::Adventure(adv_card) = card {
        assert_eq!(adv_card.base.name, "Bonecrusher Giant");
        assert_eq!(adv_card.adventure.name, "Stomp");
        assert_eq!(adv_card.adventure.mana_cost.to_string(), "{1}{R}");
        assert_eq!(adv_card.adventure.type_line, "Instant — Adventure");
    } else {
        panic!("Expected Adventure variant");
    }
}

#[test]
fn test_parse_split() {
    let yaml = read_fixture("split");
    let card: Card = from_str(&yaml).expect("Failed to parse split");

    if let Card::Split(split) = card {
        assert_eq!(split.base.name, "Fire // Ice");
        assert_eq!(split.faces.len(), 2);
        assert_eq!(split.faces[0].name, Some("Fire".to_string()));
        assert_eq!(
            split.faces[0].mana_cost.as_ref().map(|c| c.to_string()),
            Some("{1}{R}".to_string())
        );
        assert_eq!(split.faces[1].name, Some("Ice".to_string()));
        assert_eq!(
            split.faces[1].mana_cost.as_ref().map(|c| c.to_string()),
            Some("{1}{U}".to_string())
        );
    } else {
        panic!("Expected Split variant");
    }
}

#[test]
fn test_parse_transform_dfc() {
    let yaml = read_fixture("transform");
    let card: Card = from_str(&yaml).expect("Failed to parse transform DFC");

    if let Card::Transform(transform) = card {
        assert_eq!(transform.base.name, "Delver of Secrets");
        assert_eq!(transform.faces.len(), 2);
        assert_eq!(
            transform.faces[0].name,
            Some("Delver of Secrets".to_string())
        );
        assert_eq!(
            transform.faces[1].name,
            Some("Insectile Aberration".to_string())
        );
        assert_eq!(
            transform.faces[1].color_indicator,
            Some(vec!["blue".to_string()])
        );
    } else {
        panic!("Expected Transform variant");
    }
}

#[test]
fn test_parse_modal_dfc() {
    let yaml = read_fixture("modal_dfc");
    let card: Card = from_str(&yaml).expect("Failed to parse modal DFC");

    if let Card::ModalDfc(mdfc) = card {
        assert_eq!(mdfc.base.name, "Emeria's Call");
        assert_eq!(mdfc.faces.len(), 2);
        assert_eq!(mdfc.faces[0].name, Some("Emeria's Call".to_string()));
        assert_eq!(
            mdfc.faces[1].name,
            Some("Emeria, Shattered Skyclave".to_string())
        );
    } else {
        panic!("Expected ModalDfc variant");
    }
}

#[test]
fn test_parse_battle() {
    let yaml = read_fixture("battle");
    let card: Card = from_str(&yaml).expect("Failed to parse battle");

    if let Card::Battle(battle) = card {
        assert_eq!(battle.base.name, "Invasion of Gobakhan");
        assert_eq!(battle.defense, 3);
    } else {
        panic!("Expected Battle variant");
    }
}

#[test]
fn test_parse_flip() {
    let yaml = read_fixture("flip");
    let card: Card = from_str(&yaml).expect("Failed to parse flip");

    if let Card::Flip(flip) = card {
        assert_eq!(flip.base.name, "Akki Lavarunner");
        assert_eq!(flip.faces.len(), 2);
    } else {
        panic!("Expected Flip variant");
    }
}

#[test]
fn test_parse_leveler() {
    let yaml = read_fixture("leveler");
    let card: Card = from_str(&yaml).expect("Failed to parse leveler");

    if let Card::Leveler(leveler) = card {
        assert_eq!(leveler.base.name, "Kargan Dragonlord");
        assert_eq!(leveler.leveler_ranges.len(), 3);
        assert_eq!(leveler.leveler_ranges[0].power, Some("2".to_string()));
        assert_eq!(leveler.leveler_ranges[1].text, Some("Flying".to_string()));
        assert!(leveler.leveler_ranges[2].text.is_some());
    } else {
        panic!("Expected Leveler variant");
    }
}

#[test]
fn test_parse_prototype() {
    let yaml = read_fixture("prototype");
    let card: Card = from_str(&yaml).expect("Failed to parse prototype");

    if let Card::Prototype(proto_card) = card {
        assert_eq!(proto_card.base.name, "Phyrexian Fleshgorger");
        assert_eq!(
            proto_card
                .prototype
                .mana_cost
                .as_ref()
                .map(|c| c.to_string()),
            Some("{1}{B}{B}".to_string())
        );
        assert_eq!(proto_card.prototype.power, Some("3".to_string()));
        assert_eq!(proto_card.prototype.toughness, Some("3".to_string()));
    } else {
        panic!("Expected Prototype variant");
    }
}

#[test]
fn test_parse_saga_with_combined_chapters() {
    let yaml = read_fixture("saga");
    let _card: Card = from_str(&yaml).expect("Failed to parse saga");
}
