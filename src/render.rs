//! Card rendering module
//!
//! This module handles rendering MTG cards to HTML and capturing them as PNG images.
//!
//! # Asset Repository
//!
//! The `mtgrender/` directory contains high-quality MTG card assets that should be used
//! for professional-looking renders:
//!
//! - **Card Frames**: `mtgrender/client/src/assets/img/frames/` (W.png, U.png, B.png, etc.)
//! - **Mana Symbols**: `mtgrender/client/src/assets/img/symbols/` (SVG files)
//! - **Fonts**: `mtgrender/client/src/assets/fonts/` (Beleren, MPlantin, Matrix)
//! - **Text Boxes**: `mtgrender/client/src/assets/img/boxes/` (color-specific text boxes)
//! - **P/T Boxes**: `mtgrender/client/src/assets/img/pt_boxes/`
//! - **Legendary Crowns**: `mtgrender/client/src/assets/img/legendary_crowns/`
//!
//! See AGENTS.md for complete asset documentation and usage guidelines.
//!
//! # Current Implementation Status
//!
//! - ✅ Mana symbol rendering (using Scryfall CDN URLs)
//! - ✅ Frame color derivation from mana costs
//! - ⚠️  Frame rendering (currently CSS gradients, should use real frame images)
//! - ⚠️  Font loading (currently generic fonts, should use MTG fonts)
//! - ❌ Planeswalker rendering (marked todo!())
//! - ❌ Saga, Adventure, Transform, and other special layouts

use crate::card::{Card, ClassLevel, LoyaltyAbility};
use crate::mana::{ActionCost, CastingManaCost, CastingManaSymbol, LoyaltyValue, ManaSymbol};
use anyhow::Result;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::page::ScreenshotParams;
use chromiumoxide_cdp::cdp::browser_protocol::emulation::SetDeviceMetricsOverrideParams;
use chromiumoxide_cdp::cdp::browser_protocol::page::CaptureScreenshotFormat;
use futures::StreamExt;
use maud::{Markup, html};
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

    /// Derive frame color from mana cost
    fn derive_frame_color(mana_cost: &Option<CastingManaCost>) -> &'static str {
        let Some(cost) = mana_cost else {
            return "land"; // No mana cost = land
        };

        let mut has_white = false;
        let mut has_blue = false;
        let mut has_black = false;
        let mut has_red = false;
        let mut has_green = false;
        let mut has_colorless = false;

        for symbol in &cost.symbols {
            match symbol {
                CastingManaSymbol::White
                | CastingManaSymbol::WhiteBlue
                | CastingManaSymbol::WhiteBlack
                | CastingManaSymbol::WhiteRed
                | CastingManaSymbol::WhiteGreen
                | CastingManaSymbol::TwoWhite
                | CastingManaSymbol::PhyrexianWhite => has_white = true,
                CastingManaSymbol::Blue
                | CastingManaSymbol::BlueBlack
                | CastingManaSymbol::BlueRed
                | CastingManaSymbol::BlueGreen
                | CastingManaSymbol::TwoBlue
                | CastingManaSymbol::PhyrexianBlue => has_blue = true,
                CastingManaSymbol::Black
                | CastingManaSymbol::BlackRed
                | CastingManaSymbol::BlackGreen
                | CastingManaSymbol::TwoBlack
                | CastingManaSymbol::PhyrexianBlack => has_black = true,
                CastingManaSymbol::Red
                | CastingManaSymbol::RedGreen
                | CastingManaSymbol::TwoRed
                | CastingManaSymbol::PhyrexianRed => has_red = true,
                CastingManaSymbol::Green
                | CastingManaSymbol::TwoGreen
                | CastingManaSymbol::PhyrexianGreen => has_green = true,
                CastingManaSymbol::Colorless => has_colorless = true,
                _ => {}
            }
        }

        let color_count = [has_white, has_blue, has_black, has_red, has_green]
            .iter()
            .filter(|&&x| x)
            .count();

        match color_count {
            0 => {
                if has_colorless {
                    "colorless"
                } else {
                    "artifact" // Generic mana only
                }
            }
            1 => {
                if has_white {
                    "white"
                } else if has_blue {
                    "blue"
                } else if has_black {
                    "black"
                } else if has_red {
                    "red"
                } else {
                    "green"
                }
            }
            _ => "gold", // Multicolor
        }
    }

    /// Generate CSS for card styling
    fn generate_css() -> Markup {
        html! {
            style {
                r#"
                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                }

                body {
                    font-family: 'Beleren', 'Plantin MT Pro', serif;
                    background: transparent;
                }

                .card {
                    width: 744px;
                    height: 1040px;
                    border-radius: 24px;
                    overflow: hidden;
                    position: relative;
                    border: 2px solid #000;
                }

                .card-inner {
                    width: 100%;
                    height: 100%;
                    padding: 24px;
                    display: flex;
                    flex-direction: column;
                }

                /* Frame colors */
                .frame-white { background: linear-gradient(135deg, #f0f0e0 0%, #e8e8d8 100%); }
                .frame-blue { background: linear-gradient(135deg, #0e68ab 0%, #0a4d7d 100%); }
                .frame-black { background: linear-gradient(135deg, #150b00 0%, #2b1810 100%); }
                .frame-red { background: linear-gradient(135deg, #d3202a 0%, #a01f23 100%); }
                .frame-green { background: linear-gradient(135deg, #00733e 0%, #005a31 100%); }
                .frame-gold { background: linear-gradient(135deg, #e0c96b 0%, #c8a858 100%); }
                .frame-artifact { background: linear-gradient(135deg, #bcc0c3 0%, #9ca3a8 100%); }
                .frame-colorless { background: linear-gradient(135deg, #ccc2c0 0%, #b8aeac 100%); }
                .frame-land { background: linear-gradient(135deg, #a6927a 0%, #8b7a65 100%); }

                /* Header section */
                .card-header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 12px 16px;
                    background: rgba(0, 0, 0, 0.6);
                    border-radius: 12px;
                    margin-bottom: 16px;
                }

                .card-name {
                    font-size: 28px;
                    font-weight: bold;
                    color: #fff;
                    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.8);
                }

                .mana-cost-container {
                    display: flex;
                    gap: 4px;
                    align-items: center;
                }

                .mana-symbol {
                    width: 24px;
                    height: 24px;
                    display: inline-block;
                    vertical-align: middle;
                }

                .mana-generic {
                    display: inline-flex;
                    align-items: center;
                    justify-content: center;
                    width: 24px;
                    height: 24px;
                    border-radius: 50%;
                    background: #ccc;
                    color: #000;
                    font-weight: bold;
                    font-size: 14px;
                }

                /* Art box */
                .art-box {
                    width: 100%;
                    height: 420px;
                    background: linear-gradient(135deg, #2a2a2a 0%, #1a1a1a 100%);
                    border-radius: 12px;
                    margin-bottom: 16px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: #666;
                    font-size: 18px;
                    border: 2px solid rgba(0, 0, 0, 0.4);
                }

                /* Type line */
                .type-line {
                    padding: 10px 16px;
                    background: rgba(0, 0, 0, 0.6);
                    border-radius: 8px;
                    margin-bottom: 12px;
                }

                .type-text {
                    font-size: 20px;
                    font-weight: bold;
                    color: #fff;
                    text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.8);
                }

                /* Text box */
                .text-box {
                    flex: 1;
                    padding: 16px;
                    background: rgba(255, 255, 255, 0.9);
                    border-radius: 8px;
                    margin-bottom: 12px;
                    overflow: hidden;
                }

                .rules-text {
                    font-size: 16px;
                    line-height: 1.4;
                    color: #000;
                    margin-bottom: 12px;
                }

                .rules-text-inner {
                    display: flex;
                    flex-wrap: wrap;
                    align-items: center;
                    gap: 2px;
                }

                .rules-text .mana-symbol {
                    width: 16px;
                    height: 16px;
                }

                .flavor-text {
                    font-size: 14px;
                    font-style: italic;
                    color: #333;
                    line-height: 1.3;
                    border-top: 1px solid #ccc;
                    padding-top: 8px;
                    margin-top: 8px;
                }

                /* Power/Toughness box */
                .pt-box {
                    position: absolute;
                    bottom: 32px;
                    right: 32px;
                    width: 80px;
                    height: 60px;
                    background: rgba(0, 0, 0, 0.7);
                    border: 3px solid rgba(255, 255, 255, 0.3);
                    border-radius: 8px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }

                .pt-text {
                    font-size: 32px;
                    font-weight: bold;
                    color: #fff;
                    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.9);
                }

                /* Rarity indicator */
                .rarity-indicator {
                    position: absolute;
                    bottom: 32px;
                    left: 50%;
                    transform: translateX(-50%);
                    width: 20px;
                    height: 20px;
                    border-radius: 50%;
                }

                .rarity-common { background: #1a1a1a; }
                .rarity-uncommon { background: #707070; }
                .rarity-rare { background: #a58e4a; }
                .rarity-mythic { background: #bf4427; }

                /* Class card styles */
                .class-text-box {
                    flex: 1;
                    display: flex;
                    flex-direction: column;
                    gap: 0;
                    background: rgba(255, 255, 255, 0.9);
                    border-radius: 8px;
                    margin-bottom: 12px;
                    overflow: hidden;
                }

                .class-level {
                    padding: 12px 16px;
                    border-bottom: 2px solid rgba(0, 0, 0, 0.2);
                }

                .class-level:last-child {
                    border-bottom: none;
                }

                .class-level-header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 8px;
                }

                .class-level-indicator {
                    font-size: 14px;
                    font-weight: bold;
                    color: #333;
                    background: rgba(0, 0, 0, 0.1);
                    padding: 4px 10px;
                    border-radius: 4px;
                }

                .class-level-cost {
                    display: flex;
                    align-items: center;
                    gap: 4px;
                    font-size: 14px;
                    color: #333;
                }

                .class-level-cost .mana-symbol {
                    width: 18px;
                    height: 18px;
                }

                .class-level-text {
                    font-size: 14px;
                    line-height: 1.4;
                    color: #000;
                }

                .class-level-text .rules-text-inner {
                    display: inline;
                }

                .class-level-text .mana-symbol {
                    width: 14px;
                    height: 14px;
                }
                "#
            }
        }
    }

    fn render_normal_card(&self, base: &crate::card::CardBase) -> Markup {
        let frame_color = Self::derive_frame_color(&base.mana_cost);
        let frame_class = format!("frame-{}", frame_color);

        let rarity_class = match base.rarity {
            crate::card::Rarity::Common => "rarity-common",
            crate::card::Rarity::Uncommon => "rarity-uncommon",
            crate::card::Rarity::Rare => "rarity-rare",
            crate::card::Rarity::Mythic => "rarity-mythic",
        };

        html! {
            html {
                head {
                    meta charset="utf-8";
                    (Self::generate_css())
                }
                body {
                    div class=(format!("card {}", frame_class)) {
                        div.card-inner {
                            // Header with name and mana cost
                            div.card-header {
                                div.card-name { (base.name) }
                                @if let Some(ref cost) = base.mana_cost {
                                    (Self::render_mana_cost(cost))
                                }
                            }

                            // Art box (placeholder for now)
                            div.art-box {
                                "[Art]"
                            }

                            // Type line
                            div.type-line {
                                div.type-text { (base.type_line) }
                            }

                            // Text box
                            div.text-box {
                                @if let Some(ref rules) = base.rules_text {
                                    div.rules-text {
                                        (Self::render_rules_text(rules))
                                    }
                                }
                                @if let Some(ref flavor) = base.flavor_text {
                                    div.flavor-text {
                                        (flavor)
                                    }
                                }
                            }

                            // Power/Toughness box (if creature)
                            @if let (Some(power), Some(toughness)) = (&base.power, &base.toughness) {
                                div.pt-box {
                                    div.pt-text { (power) "/" (toughness) }
                                }
                            }

                            // Rarity indicator
                            div.rarity-indicator class=(rarity_class) {}
                        }
                    }
                }
            }
        }
    }

    pub async fn render_card(&self, card: &Card, output_path: &Path) -> Result<()> {
        // Generate HTML based on card type
        let html = match card {
            Card::Normal { base } => self.render_normal_card(base),
            Card::Planeswalker {
                base,
                loyalty,
                loyalty_abilities,
            } => self.render_planeswalker(base, loyalty, loyalty_abilities),
            Card::Class { base, levels } => self.render_class(base, levels),
            _ => {
                anyhow::bail!("Card type not yet implemented for rendering");
            }
        };

        // Create a new page
        let page = self.browser.new_page("about:blank").await?;

        // Set device metrics for proper card dimensions (744x1040 at 4x scale = 300 DPI)
        let metrics = SetDeviceMetricsOverrideParams::builder()
            .width(744)
            .height(1040)
            .device_scale_factor(4.0)
            .mobile(false)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build device metrics: {}", e))?;

        page.execute(metrics).await?;

        // Load the HTML content
        let html_string = html.into_string();
        page.set_content(&html_string).await?;

        // Wait a bit for content to render
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Take screenshot with high DPI
        let screenshot_params = ScreenshotParams::builder()
            .format(CaptureScreenshotFormat::Png)
            .full_page(false)
            .omit_background(false)
            .build();

        page.save_screenshot(screenshot_params, output_path).await?;

        Ok(())
    }

    fn render_planeswalker(
        &self,
        _base: &crate::card::CardBase,
        _loyalty: &LoyaltyValue,
        _loyalty_abilities: &[LoyaltyAbility],
    ) -> Markup {
        todo!()
    }

    fn render_class(&self, base: &crate::card::CardBase, levels: &[ClassLevel]) -> Markup {
        let frame_color = Self::derive_frame_color(&base.mana_cost);
        let frame_class = format!("frame-{}", frame_color);

        let rarity_class = match base.rarity {
            crate::card::Rarity::Common => "rarity-common",
            crate::card::Rarity::Uncommon => "rarity-uncommon",
            crate::card::Rarity::Rare => "rarity-rare",
            crate::card::Rarity::Mythic => "rarity-mythic",
        };

        html! {
            html {
                head {
                    meta charset="utf-8";
                    (Self::generate_css())
                }
                body {
                    div class=(format!("card {}", frame_class)) {
                        div.card-inner {
                            // Header with name and mana cost
                            div.card-header {
                                div.card-name { (base.name) }
                                @if let Some(ref cost) = base.mana_cost {
                                    (Self::render_mana_cost(cost))
                                }
                            }

                            // Art box (placeholder for now)
                            div.art-box {
                                "[Art]"
                            }

                            // Type line
                            div.type-line {
                                div.type-text { (base.type_line) }
                            }

                            // Class levels text box
                            div.class-text-box {
                                @for level in levels {
                                    div.class-level {
                                        div.class-level-header {
                                            @if level.level == 1 {
                                                // Level 1 has no indicator, just the text
                                                span.class-level-indicator { "(Level 1)" }
                                            } @else {
                                                span.class-level-indicator {
                                                    (format!("Level {}", level.level))
                                                }
                                                @if let Some(ref cost) = level.cost {
                                                    div.class-level-cost {
                                                        (Self::render_mana_cost(cost))
                                                    }
                                                }
                                            }
                                        }
                                        div.class-level-text {
                                            (Self::render_rules_text(&level.text))
                                        }
                                    }
                                }
                            }

                            // Rarity indicator
                            div.rarity-indicator class=(rarity_class) {}
                        }
                    }
                }
            }
        }
    }
}
