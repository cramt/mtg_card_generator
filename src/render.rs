use crate::card::{Card, LoyaltyAbility};
use crate::mana::{ActionCost, CastingManaCost, CastingManaSymbol, LoyaltyValue, ManaSymbol};
use anyhow::Result;
use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;
use maud::{html, Markup};
use std::path::Path;

pub struct Renderer {
    browser: Browser,
}

impl Renderer {
    pub async fn new() -> Result<Self> {
        let mut config = BrowserConfig::builder().no_sandbox();

        if let Ok(path) = std::env::var("CHROME_PATH") {
            config = config.chrome_executable(path);
        }

        let (browser, mut handler) =
            Browser::launch(config.build().map_err(anyhow::Error::msg)?).await?;

        tokio::spawn(async move {
            while let Some(h) = handler.next().await {
                if let Err(e) = h {
                    eprintln!("Browser handler error: {}", e);
                    break;
                }
            }
        });

        Ok(Self { browser })
    }

    pub fn render_casting_symbol(symbol: CastingManaSymbol) -> Markup {
        let scryfall_symbol = match symbol {
            CastingManaSymbol::White => "W",
            CastingManaSymbol::Blue => "U",
            CastingManaSymbol::Black => "B",
            CastingManaSymbol::Red => "R",
            CastingManaSymbol::Green => "G",
            CastingManaSymbol::Colorless => "C",
            CastingManaSymbol::Generic(n) => return html! { span.mana-generic { (n) } },
            CastingManaSymbol::X => "X",
            CastingManaSymbol::Y => "Y",
            CastingManaSymbol::Z => "Z",
            CastingManaSymbol::Snow => "S",
            CastingManaSymbol::WhiteBlue => "WU",
            CastingManaSymbol::WhiteBlack => "WB",
            CastingManaSymbol::WhiteRed => "WR",
            CastingManaSymbol::WhiteGreen => "WG",
            CastingManaSymbol::BlueBlack => "UB",
            CastingManaSymbol::BlueRed => "UR",
            CastingManaSymbol::BlueGreen => "UG",
            CastingManaSymbol::BlackRed => "BR",
            CastingManaSymbol::BlackGreen => "BG",
            CastingManaSymbol::RedGreen => "RG",
            CastingManaSymbol::TwoWhite => "2W",
            CastingManaSymbol::TwoBlue => "2U",
            CastingManaSymbol::TwoBlack => "2B",
            CastingManaSymbol::TwoRed => "2R",
            CastingManaSymbol::TwoGreen => "2G",
            CastingManaSymbol::PhyrexianWhite => "WP",
            CastingManaSymbol::PhyrexianBlue => "UP",
            CastingManaSymbol::PhyrexianBlack => "BP",
            CastingManaSymbol::PhyrexianRed => "RP",
            CastingManaSymbol::PhyrexianGreen => "GP",
        };

        let url = format!(
            "https://svgs.scryfall.io/card-symbols/{}.svg",
            scryfall_symbol
        );
        html! {
            img.mana-symbol src=(url) alt=(scryfall_symbol);
        }
    }

    pub fn render_mana_symbol(symbol: ManaSymbol) -> Markup {
        match symbol {
            ManaSymbol::Casting(s) => Self::render_casting_symbol(s),
            ManaSymbol::Tap => {
                let url = "https://svgs.scryfall.io/card-symbols/T.svg";
                html! { img.mana-symbol src=(url) alt="T"; }
            }
            ManaSymbol::Untap => {
                let url = "https://svgs.scryfall.io/card-symbols/Q.svg";
                html! { img.mana-symbol src=(url) alt="Q"; }
            }
            ManaSymbol::Energy => {
                let url = "https://svgs.scryfall.io/card-symbols/E.svg";
                html! { img.mana-symbol src=(url) alt="E"; }
            }
            ManaSymbol::Chaos => {
                let url = "https://svgs.scryfall.io/card-symbols/CHAOS.svg";
                html! { img.mana-symbol src=(url) alt="CHAOS"; }
            }
        }
    }

    pub fn render_mana_cost(cost: &CastingManaCost) -> Markup {
        html! {
            div.mana-cost-container {
                @for symbol in &cost.symbols {
                    (Self::render_casting_symbol(*symbol))
                }
            }
        }
    }

    pub fn render_rules_text(text: &str) -> Markup {
        let mut parts = Vec::new();
        let mut last_end = 0;

        for (start, _) in text.match_indices('{') {
            if let Some(end) = text[start..].find('}') {
                let end = start + end;
                if last_end < start {
                    parts.push(html! { (text[last_end..start]) });
                }

                let symbol_str = &text[start..end + 1];
                if let Ok(cost) = ActionCost::parse(symbol_str) {
                    if let Some(symbol) = cost.symbols.first() {
                        parts.push(Self::render_mana_symbol(*symbol));
                    } else {
                        parts.push(html! { (symbol_str) });
                    }
                } else {
                    parts.push(html! { (symbol_str) });
                }
                last_end = end + 1;
            }
        }

        if last_end < text.len() {
            parts.push(html! { (text[last_end..]) });
        }

        html! {
            div.rules-text-inner {
                @for part in parts {
                    (part)
                }
            }
        }
    }

    pub async fn render_card(&self, card: &Card, output_path: &Path) -> Result<()> {
        let page = self.browser.new_page("about:blank").await?;

        let mana_cost = card.get_mana_cost();

        let card_body = match card {
            Card::Planeswalker {
                loyalty,
                loyalty_abilities,
                ..
            } => self.render_planeswalker(card, loyalty, loyalty_abilities),
            _ => self.render_normal_card(card, &mana_cost),
        };

        let html_content = html! {
            (maud::DOCTYPE)
            html {
                head {
                    style {
                        (include_str!("card.css"))
                    }
                }
                body {
                    (card_body)
                }
            }
        }
        .into_string();

        page.set_content(html_content).await?;

        // Wait for images to load
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        let element = page.find_element(".card").await?;
        let image_data = element
            .screenshot(chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat::Png)
            .await?;

        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(output_path, image_data).await?;

        page.close().await?;

        Ok(())
    }

    fn render_normal_card(&self, card: &Card, mana_cost: &CastingManaCost) -> Markup {
        let base = card.base();
        html! {
            div.card.normal {
                div.header {
                    div.name { (&base.name) }
                    div.mana_cost { (Self::render_mana_cost(mana_cost)) }
                }
                div.type_line { (&base.type_line) }
                div.rules_box {
                    div.rules_text {
                        (Self::render_rules_text(base.rules_text.as_deref().unwrap_or("")))
                    }
                    @if let Some(flavor) = &base.flavor_text {
                        div.flavor_text { (flavor) }
                    }
                }
                @if let (Some(p), Some(t)) = (&base.power, &base.toughness) {
                    div.pt {
                        (p) "/" (t)
                    }
                }
            }
        }
    }

    fn render_planeswalker(
        &self,
        card: &Card,
        loyalty: &LoyaltyValue,
        loyalty_abilities: &[LoyaltyAbility],
    ) -> Markup {
        let mana_cost = card.get_mana_cost();
        let base = card.base();

        html! {
            div.card.planeswalker {
                div.header {
                    div.name { (&base.name) }
                    div.mana_cost { (Self::render_mana_cost(&mana_cost)) }
                }
                div.type_line { (&base.type_line) }
                div.loyalty_box {
                    @for ability in loyalty_abilities {
                        div.loyalty_ability {
                            div.loyalty_cost { (ability.cost) }
                            div.loyalty_text { (Self::render_rules_text(&ability.text)) }
                        }
                    }
                }
                div.starting_loyalty { (loyalty) }
            }
        }
    }
}
