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
        let mut config = BrowserConfig::builder()
            .no_sandbox()
            .arg("--disable-web-security")
            .arg("--allow-file-access-from-files")
            .arg("--disable-features=IsolateOrigins,site-per-process")
            .arg("--disable-blink-features=AutomationControlled");

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

    /// Generate CSS for card styling with real MTG assets
    fn generate_css() -> Markup {
        // Get absolute path to mtgrender assets
        let assets_base = std::env::current_dir()
            .unwrap_or_default()
            .join("mtgrender/client/src/assets");

        html! {
            style {
                r#"
                /* Load real MTG fonts */
                @font-face {
                    font-family: 'Beleren';
                    src: url('file://"# (assets_base.join("fonts/beleren-bold_P1.01.ttf").display()) r#"') format('truetype');
                    font-weight: bold;
                }
                @font-face {
                    font-family: 'Beleren Small Caps';
                    src: url('file://"# (assets_base.join("fonts/belerensmallcaps-bold.ttf").display()) r#"') format('truetype');
                    font-weight: bold;
                }
                @font-face {
                    font-family: 'MPlantin';
                    src: url('file://"# (assets_base.join("fonts/mplantin.ttf").display()) r#"') format('truetype');
                    font-weight: normal;
                }
                @font-face {
                    font-family: 'MPlantin';
                    src: url('file://"# (assets_base.join("fonts/MPlantin-Italic.ttf").display()) r#"') format('truetype');
                    font-style: italic;
                }
                @font-face {
                    font-family: 'Matrix';
                    src: url('file://"# (assets_base.join("fonts/MatrixBold.ttf").display()) r#"') format('truetype');
                    font-weight: bold;
                }

                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                }

                body {
                    font-family: 'MPlantin', serif;
                    background: transparent;
                }

                .card {
                    width: 744px;
                    height: 1040px;
                    border-radius: 37px;
                    overflow: hidden;
                    position: relative;
                    background-size: cover;
                    background-position: center;
                }

                .card-inner {
                    width: 100%;
                    height: 100%;
                    padding: 0;
                    display: flex;
                    flex-direction: column;
                    position: relative;
                }

                /* Frame backgrounds using real assets - use bg/ for ornate textured borders */
                .frame-white { background-image: url('file://"# (assets_base.join("img/bg/W.png").display()) r#"'); }
                .frame-blue { background-image: url('file://"# (assets_base.join("img/bg/U.png").display()) r#"'); }
                .frame-black { background-image: url('file://"# (assets_base.join("img/bg/B.png").display()) r#"'); }
                .frame-red { background-image: url('file://"# (assets_base.join("img/bg/R.png").display()) r#"'); }
                .frame-green { background-image: url('file://"# (assets_base.join("img/bg/G.png").display()) r#"'); }
                .frame-gold { background-image: url('file://"# (assets_base.join("img/bg/Gold.png").display()) r#"'); }
                .frame-artifact { background-image: url('file://"# (assets_base.join("img/bg/Artifact.png").display()) r#"'); }
                .frame-colorless { background-image: url('file://"# (assets_base.join("img/bg/Colourless.png").display()) r#"'); }
                .frame-land { background-image: url('file://"# (assets_base.join("img/bg/Land.png").display()) r#"'); }

                /* Text box backgrounds */
                .text-box-white { background-image: url('file://"# (assets_base.join("img/boxes/W.png").display()) r#"'); }
                .text-box-blue { background-image: url('file://"# (assets_base.join("img/boxes/U.png").display()) r#"'); }
                .text-box-black { background-image: url('file://"# (assets_base.join("img/boxes/B.png").display()) r#"'); }
                .text-box-red { background-image: url('file://"# (assets_base.join("img/boxes/R.png").display()) r#"'); }
                .text-box-green { background-image: url('file://"# (assets_base.join("img/boxes/G.png").display()) r#"'); }
                .text-box-gold { background-image: url('file://"# (assets_base.join("img/boxes/Gold.png").display()) r#"'); }
                .text-box-artifact { background-image: url('file://"# (assets_base.join("img/boxes/Artifact.png").display()) r#"'); }
                .text-box-colorless { background-image: url('file://"# (assets_base.join("img/boxes/Colourless.png").display()) r#"'); }
                .text-box-land { background-image: url('file://"# (assets_base.join("img/boxes/Land.png").display()) r#"'); }

                /* P/T box backgrounds */
                .pt-box-white { background-image: url('file://"# (assets_base.join("img/pt_boxes/W.png").display()) r#"'); }
                .pt-box-blue { background-image: url('file://"# (assets_base.join("img/pt_boxes/U.png").display()) r#"'); }
                .pt-box-black { background-image: url('file://"# (assets_base.join("img/pt_boxes/B.png").display()) r#"'); }
                .pt-box-red { background-image: url('file://"# (assets_base.join("img/pt_boxes/R.png").display()) r#"'); }
                .pt-box-green { background-image: url('file://"# (assets_base.join("img/pt_boxes/G.png").display()) r#"'); }
                .pt-box-gold { background-image: url('file://"# (assets_base.join("img/pt_boxes/Gold.png").display()) r#"'); }
                .pt-box-artifact { background-image: url('file://"# (assets_base.join("img/pt_boxes/Artifact.png").display()) r#"'); }
                .pt-box-colorless { background-image: url('file://"# (assets_base.join("img/pt_boxes/Colourless.png").display()) r#"'); }
                .pt-box-land { background-image: url('file://"# (assets_base.join("img/pt_boxes/Land.png").display()) r#"'); }

                /* Header section */
                .card-header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 4px 12px;
                    margin-bottom: 0;
                    position: absolute;
                    top: 36px;
                    left: 36px;
                    width: 672px; /* 744 - 36*2 */
                    height: 38px;
                    z-index: 10;
                }

                .card-name {
                    font-size: 32px;
                    font-weight: bold;
                    color: #000;
                    font-family: 'Beleren', serif;
                    letter-spacing: 0.5px;
                }

                .mana-cost-container {
                    display: flex;
                    gap: 5px;
                    align-items: center;
                }

                .mana-symbol {
                    width: 26px;
                    height: 26px;
                    display: inline-block;
                    vertical-align: middle;
                    box-shadow: -2px 2px 0px rgba(0,0,0,0.4);
                    border-radius: 13px;
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
                    position: absolute;
                    top: 74px;
                    left: 36px;
                    width: 672px;
                    height: 356px;
                    background: linear-gradient(135deg, #2a2a2a 0%, #1a1a1a 100%);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: #666;
                    font-size: 18px;
                    border: 1px solid #000;
                    z-index: 5;
                }

                /* Type line */
                .type-line {
                    position: absolute;
                    top: 436px;
                    left: 36px;
                    width: 672px;
                    height: 38px;
                    display: flex;
                    align-items: center;
                    padding-left: 12px;
                    z-index: 10;
                }

                .type-text {
                    font-size: 28px;
                    font-weight: bold;
                    color: #000;
                    font-family: 'Beleren Small Caps', serif;
                    letter-spacing: 0.5px;
                }

                /* Text box */
                .text-box {
                    position: absolute;
                    top: 480px;
                    left: 36px;
                    width: 672px;
                    height: 420px;
                    padding: 24px 32px;
                    background-size: 100% 100%;
                    z-index: 5;
                    font-family: 'MPlantin', serif;
                    display: flex;
                    flex-direction: column;
                    justify-content: flex-start;
                    gap: 12px;
                }

                .rules-text {
                    font-size: 26px;
                    line-height: 1.3;
                    color: #000;
                    margin-bottom: 12px;
                }
                
                .rules-text-inner {
                    display: inline;
                }

                .rules-text .mana-symbol {
                    width: 22px;
                    height: 22px;
                    vertical-align: text-bottom;
                }

                .flavor-text {
                    font-size: 24px;
                    font-style: italic;
                    color: #000;
                    line-height: 1.2;
                    padding-top: 8px;
                    margin-top: 8px;
                }

                /* Power/Toughness box */
                .pt-box {
                    position: absolute;
                    bottom: 26px;
                    right: 26px;
                    width: 90px;
                    height: 64px;
                    background-size: contain;
                    background-repeat: no-repeat;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    z-index: 20;
                }

                .pt-text {
                    font-size: 36px;
                    font-weight: bold;
                    color: #000;
                    font-family: 'Matrix', serif;
                    padding-top: 6px;
                    padding-left: 6px;
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

                /* Planeswalker styles */
                .planeswalker-text-box {
                    position: absolute;
                    top: 480px;
                    left: 36px;
                    width: 672px;
                    height: 420px;
                    display: flex;
                    flex-direction: column;
                    gap: 8px;
                    padding: 16px 24px;
                    z-index: 5;
                }

                .loyalty-ability {
                    display: flex;
                    gap: 12px;
                    padding: 8px 12px;
                    background: rgba(255, 255, 255, 0.85);
                    border-radius: 6px;
                    border: 1px solid rgba(0, 0, 0, 0.2);
                    align-items: flex-start;
                }

                .loyalty-cost {
                    flex-shrink: 0;
                    width: 48px;
                    height: 48px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-size: 28px;
                    font-weight: bold;
                    font-family: 'Beleren', serif;
                    border-radius: 50%;
                    color: #fff;
                    text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.8);
                }

                .loyalty-cost-plus {
                    background: linear-gradient(135deg, #4a90e2 0%, #357abd 100%);
                    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
                }

                .loyalty-cost-minus {
                    background: linear-gradient(135deg, #e24a4a 0%, #bd3535 100%);
                    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
                }

                .loyalty-cost-zero {
                    background: linear-gradient(135deg, #888 0%, #666 100%);
                    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
                }

                .loyalty-ability-text {
                    flex: 1;
                    font-size: 22px;
                    line-height: 1.3;
                    color: #000;
                    font-family: 'MPlantin', serif;
                    padding-top: 4px;
                }

                .loyalty-ability-text .mana-symbol {
                    width: 20px;
                    height: 20px;
                }

                .loyalty-counter {
                    position: absolute;
                    bottom: 32px;
                    right: 36px;
                    width: 80px;
                    height: 80px;
                    background: linear-gradient(135deg, #f4f4f4 0%, #d4d4d4 100%);
                    border: 4px solid #000;
                    border-radius: 50%;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-size: 48px;
                    font-weight: bold;
                    font-family: 'Beleren', serif;
                    color: #000;
                    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.4);
                    z-index: 20;
                }

                /* Saga styles */
                .saga-text-box {
                    position: absolute;
                    top: 480px;
                    left: 36px;
                    width: 672px;
                    height: 420px;
                    display: flex;
                    flex-direction: column;
                    gap: 12px;
                    padding: 20px 28px;
                    z-index: 5;
                }

                .saga-chapter {
                    display: flex;
                    gap: 16px;
                    padding: 10px 14px;
                    background: rgba(255, 255, 255, 0.85);
                    border-radius: 6px;
                    border-left: 4px solid rgba(0, 0, 0, 0.3);
                    align-items: flex-start;
                }

                .saga-chapter-number {
                    flex-shrink: 0;
                    width: 40px;
                    height: 40px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-size: 24px;
                    font-weight: bold;
                    font-family: 'Beleren', serif;
                    color: #fff;
                    background: linear-gradient(135deg, #2a2a2a 0%, #1a1a1a 100%);
                    border-radius: 50%;
                    border: 2px solid #000;
                    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
                }

                .saga-chapter-text {
                    flex: 1;
                    font-size: 22px;
                    line-height: 1.3;
                    color: #000;
                    font-family: 'MPlantin', serif;
                    padding-top: 6px;
                }

                .saga-chapter-text .mana-symbol {
                    width: 20px;
                    height: 20px;
                }

                /* Adventure card styles */
                .adventure-card {
                    display: flex;
                    flex-direction: row;
                }

                .adventure-left {
                    width: 200px;
                    height: 100%;
                    display: flex;
                    flex-direction: column;
                    padding: 20px 16px;
                    border-right: 2px solid rgba(0, 0, 0, 0.3);
                    background: rgba(0, 0, 0, 0.05);
                }

                .adventure-name {
                    font-size: 20px;
                    font-weight: bold;
                    font-family: 'Beleren', serif;
                    color: #000;
                    margin-bottom: 8px;
                    writing-mode: vertical-rl;
                    text-orientation: mixed;
                    transform: rotate(180deg);
                    flex: 1;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }

                .adventure-cost {
                    writing-mode: vertical-rl;
                    transform: rotate(180deg);
                    display: flex;
                    gap: 4px;
                    margin-bottom: 12px;
                }

                .adventure-type {
                    font-size: 14px;
                    font-family: 'Beleren Small Caps', serif;
                    color: #000;
                    writing-mode: vertical-rl;
                    text-orientation: mixed;
                    transform: rotate(180deg);
                    margin-bottom: 12px;
                }

                .adventure-text {
                    font-size: 14px;
                    line-height: 1.2;
                    font-family: 'MPlantin', serif;
                    color: #000;
                    writing-mode: vertical-rl;
                    text-orientation: mixed;
                    transform: rotate(180deg);
                    flex: 2;
                }

                .adventure-right {
                    flex: 1;
                    display: flex;
                    flex-direction: column;
                    position: relative;
                }

                /* Split card styles */
                .split-card {
                    display: flex;
                    flex-direction: row;
                    transform: rotate(-90deg);
                    transform-origin: center center;
                    width: 1040px;
                    height: 744px;
                    position: absolute;
                    top: 148px;
                    left: -148px;
                }

                .split-half {
                    flex: 1;
                    display: flex;
                    flex-direction: column;
                    position: relative;
                    border-right: 2px solid rgba(0, 0, 0, 0.5);
                }

                .split-half:last-child {
                    border-right: none;
                }

                .split-header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 4px 12px;
                    margin: 36px 36px 0 36px;
                    height: 38px;
                }

                .split-name {
                    font-size: 28px;
                    font-weight: bold;
                    color: #000;
                    font-family: 'Beleren', serif;
                }

                .split-art {
                    margin: 8px 36px;
                    height: 280px;
                    background: linear-gradient(135deg, #2a2a2a 0%, #1a1a1a 100%);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: #666;
                    font-size: 16px;
                    border: 1px solid #000;
                }

                .split-type {
                    margin: 0 36px;
                    padding: 4px 12px;
                    height: 32px;
                    display: flex;
                    align-items: center;
                }

                .split-type-text {
                    font-size: 24px;
                    font-weight: bold;
                    color: #000;
                    font-family: 'Beleren Small Caps', serif;
                }

                .split-text-box {
                    margin: 8px 36px 36px 36px;
                    flex: 1;
                    padding: 16px 20px;
                    background-size: 100% 100%;
                }

                .split-rules {
                    font-size: 22px;
                    line-height: 1.3;
                    color: #000;
                }

                /* Battle card styles */
                .defense-counter {
                    position: absolute;
                    bottom: 32px;
                    right: 36px;
                    width: 80px;
                    height: 80px;
                    background: linear-gradient(135deg, #e8e8e8 0%, #c8c8c8 100%);
                    border: 4px solid #000;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-size: 48px;
                    font-weight: bold;
                    font-family: 'Beleren', serif;
                    color: #000;
                    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.4);
                    z-index: 20;
                    clip-path: polygon(50% 0%, 100% 25%, 100% 75%, 50% 100%, 0% 75%, 0% 25%);
                }
                "#
            }
        }
    }

    fn render_normal_card(&self, base: &crate::card::CardBase) -> Markup {
        let frame_color = Self::derive_frame_color(&base.mana_cost);
        let frame_class = format!("frame-{}", frame_color);
        let text_box_class = format!("text-box-{}", frame_color);
        let pt_box_class = format!("pt-box-{}", frame_color);

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
                            div class=(format!("text-box {}", text_box_class)) {
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
                                div class=(format!("pt-box {}", pt_box_class)) {
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
            Card::Saga { base, chapters } => self.render_saga(base, chapters),
            Card::Adventure { base, adventure } => self.render_adventure(base, adventure),
            Card::Split {
                base,
                faces,
                fuse,
                aftermath,
            } => self.render_split(base, faces, fuse, aftermath),
            Card::Transform { base, faces } => self.render_dfc_face(base, faces, "Transform"),
            Card::ModalDfc { base, faces } => self.render_dfc_face(base, faces, "Modal DFC"),
            Card::Flip { base, faces } => self.render_flip(base, faces),
            Card::Battle {
                base,
                defense,
                backside_name: _,
                backside_type_line: _,
                backside_rules_text: _,
            } => self.render_battle(base, *defense),
            Card::Leveler {
                base,
                leveler_ranges,
            } => self.render_leveler(base, leveler_ranges),
            Card::Prototype { base, prototype } => self.render_prototype(base, prototype),
            Card::Meld { base, faces } => self.render_dfc_face(base, faces, "Meld"),
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

        // Save HTML to temporary file and navigate to it
        // (set_content doesn't provide a base URL for external resources)
        let html_string = html.into_string();
        let temp_html = std::env::temp_dir().join(format!("mtg_card_{}.html", std::process::id()));
        std::fs::write(&temp_html, &html_string)?;
        eprintln!("Debug: HTML saved to {}", temp_html.display());

        let file_url = format!("file://{}", temp_html.display());
        page.goto(&file_url).await?;

        // Wait for page to fully load including external resources
        page.wait_for_navigation().await?;

        // Additional wait to ensure SVGs are rendered
        tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;

        // Keep temp file for debugging
        // let _ = std::fs::remove_file(&temp_html);

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
        base: &crate::card::CardBase,
        loyalty: &LoyaltyValue,
        loyalty_abilities: &[LoyaltyAbility],
    ) -> Markup {
        let frame_color = Self::derive_frame_color(&base.mana_cost);
        let frame_class = format!("frame-{}", frame_color);

        let rarity_class = match base.rarity {
            crate::card::Rarity::Common => "rarity-common",
            crate::card::Rarity::Uncommon => "rarity-uncommon",
            crate::card::Rarity::Rare => "rarity-rare",
            crate::card::Rarity::Mythic => "rarity-mythic",
        };

        // Format loyalty value
        let loyalty_text = match loyalty {
            LoyaltyValue::Numeric(n) => n.to_string(),
            LoyaltyValue::X => "X".to_string(),
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

                            // Planeswalker abilities
                            div.planeswalker-text-box {
                                @for ability in loyalty_abilities {
                                    div.loyalty-ability {
                                        @let (cost_class, cost_text) = match &ability.cost {
                                            crate::mana::LoyaltyCost::Plus(n) => ("loyalty-cost-plus", format!("+{}", n)),
                                            crate::mana::LoyaltyCost::Minus(n) => ("loyalty-cost-minus", format!("-{}", n)),
                                            crate::mana::LoyaltyCost::Zero => ("loyalty-cost-zero", "0".to_string()),
                                            crate::mana::LoyaltyCost::PlusX => ("loyalty-cost-plus", "+X".to_string()),
                                            crate::mana::LoyaltyCost::MinusX => ("loyalty-cost-minus", "-X".to_string()),
                                        };
                                        div class=(format!("loyalty-cost {}", cost_class)) {
                                            (cost_text)
                                        }
                                        div.loyalty-ability-text {
                                            (Self::render_rules_text(&ability.text))
                                        }
                                    }
                                }
                            }

                            // Loyalty counter
                            div.loyalty-counter {
                                (loyalty_text)
                            }

                            // Rarity indicator
                            div.rarity-indicator class=(rarity_class) {}
                        }
                    }
                }
            }
        }
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

    fn render_saga(
        &self,
        base: &crate::card::CardBase,
        chapters: &[crate::card::SagaChapter],
    ) -> Markup {
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

                            // Saga chapters
                            div.saga-text-box {
                                @for chapter in chapters {
                                    div.saga-chapter {
                                        div.saga-chapter-number {
                                            @if chapter.chapters.len() == 1 {
                                                (format!("{}", chapter.chapters[0]))
                                            } @else {
                                                // For combined chapters like "I-II", show range
                                                (format!("{}-{}",
                                                    chapter.chapters.first().unwrap_or(&1),
                                                    chapter.chapters.last().unwrap_or(&1)))
                                            }
                                        }
                                        div.saga-chapter-text {
                                            (Self::render_rules_text(&chapter.text))
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

    fn render_adventure(
        &self,
        base: &crate::card::CardBase,
        adventure: &crate::card::AdventureCard,
    ) -> Markup {
        let frame_color = Self::derive_frame_color(&base.mana_cost);
        let frame_class = format!("frame-{}", frame_color);
        let text_box_class = format!("text-box-{}", frame_color);
        let pt_box_class = format!("pt-box-{}", frame_color);

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
                        div.adventure-card {
                            // Left side - Adventure spell
                            div.adventure-left {
                                div.adventure-cost {
                                    (Self::render_mana_cost(&adventure.mana_cost))
                                }
                                div.adventure-name {
                                    (adventure.name)
                                }
                                div.adventure-type {
                                    (adventure.type_line)
                                }
                                div.adventure-text {
                                    (adventure.rules_text)
                                }
                            }

                            // Right side - Main creature card
                            div.adventure-right {
                                // Header with name and mana cost
                                div.card-header {
                                    div.card-name { (base.name) }
                                    @if let Some(ref cost) = base.mana_cost {
                                        (Self::render_mana_cost(cost))
                                    }
                                }

                                // Art box
                                div.art-box {
                                    "[Art]"
                                }

                                // Type line
                                div.type-line {
                                    div.type-text { (base.type_line) }
                                }

                                // Text box
                                div class=(format!("text-box {}", text_box_class)) {
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

                                // Power/Toughness box
                                @if let (Some(power), Some(toughness)) = (&base.power, &base.toughness) {
                                    div class=(format!("pt-box {}", pt_box_class)) {
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
    }

    fn render_split(
        &self,
        base: &crate::card::CardBase,
        faces: &[crate::card::CardFace],
        _fuse: &Option<bool>,
        _aftermath: &Option<bool>,
    ) -> Markup {
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
                    div.card {
                        div.split-card {
                            @for face in faces {
                                @let frame_color = Self::derive_frame_color(&face.mana_cost);
                                @let frame_class = format!("frame-{}", frame_color);
                                @let text_box_class = format!("text-box-{}", frame_color);

                                div class=(format!("split-half {}", frame_class)) {
                                    div.split-header {
                                        div.split-name {
                                            @if let Some(ref name) = face.name {
                                                (name)
                                            }
                                        }
                                        @if let Some(ref cost) = face.mana_cost {
                                            (Self::render_mana_cost(cost))
                                        }
                                    }

                                    div.split-art {
                                        "[Art]"
                                    }

                                    div.split-type {
                                        div.split-type-text {
                                            @if let Some(ref type_line) = face.type_line {
                                                (type_line)
                                            }
                                        }
                                    }

                                    div class=(format!("split-text-box {}", text_box_class)) {
                                        @if let Some(ref rules) = face.rules_text {
                                            div.split-rules {
                                                (Self::render_rules_text(rules))
                                            }
                                        }
                                    }
                                }
                            }

                            // Rarity indicator
                            div.rarity-indicator class=(rarity_class) style="position: absolute; bottom: 32px; left: 50%; transform: translateX(-50%);" {}
                        }
                    }
                }
            }
        }
    }

    /// Render a double-faced card (Transform or Modal DFC) - renders front face only
    fn render_dfc_face(
        &self,
        base: &crate::card::CardBase,
        faces: &[crate::card::CardFace],
        _card_type: &str,
    ) -> Markup {
        // For now, render the front face as a normal card
        // TODO: Generate both faces as separate images
        if let Some(front_face) = faces.first() {
            let frame_color = Self::derive_frame_color(&front_face.mana_cost);
            let frame_class = format!("frame-{}", frame_color);
            let text_box_class = format!("text-box-{}", frame_color);
            let pt_box_class = format!("pt-box-{}", frame_color);

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
                                    div.card-name {
                                        @if let Some(ref name) = front_face.name {
                                            (name)
                                        }
                                    }
                                    @if let Some(ref cost) = front_face.mana_cost {
                                        (Self::render_mana_cost(cost))
                                    }
                                }

                                // Art box
                                div.art-box {
                                    "[Art]"
                                }

                                // Type line
                                div.type-line {
                                    div.type-text {
                                        @if let Some(ref type_line) = front_face.type_line {
                                            (type_line)
                                        }
                                    }
                                }

                                // Text box
                                div class=(format!("text-box {}", text_box_class)) {
                                    @if let Some(ref rules) = front_face.rules_text {
                                        div.rules-text {
                                            (Self::render_rules_text(rules))
                                        }
                                    }
                                    @if let Some(ref flavor) = front_face.flavor_text {
                                        div.flavor-text {
                                            (flavor)
                                        }
                                    }
                                }

                                // Power/Toughness box
                                @if let (Some(power), Some(toughness)) = (&front_face.power, &front_face.toughness) {
                                    div class=(format!("pt-box {}", pt_box_class)) {
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
        } else {
            html! { html { body { "Error: No faces found" } } }
        }
    }

    fn render_flip(&self, base: &crate::card::CardBase, faces: &[crate::card::CardFace]) -> Markup {
        // Flip cards show the top half normally and the bottom half upside down
        // For now, just render the front face
        // TODO: Implement proper flip card layout
        if let Some(front_face) = faces.first() {
            let frame_color = Self::derive_frame_color(&front_face.mana_cost);
            let frame_class = format!("frame-{}", frame_color);
            let text_box_class = format!("text-box-{}", frame_color);
            let pt_box_class = format!("pt-box-{}", frame_color);

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
                                    div.card-name {
                                        @if let Some(ref name) = front_face.name {
                                            (name)
                                        }
                                    }
                                    @if let Some(ref cost) = front_face.mana_cost {
                                        (Self::render_mana_cost(cost))
                                    }
                                }

                                // Art box
                                div.art-box {
                                    "[Art]"
                                }

                                // Type line
                                div.type-line {
                                    div.type-text {
                                        @if let Some(ref type_line) = front_face.type_line {
                                            (type_line)
                                        }
                                    }
                                }

                                // Text box
                                div class=(format!("text-box {}", text_box_class)) {
                                    @if let Some(ref rules) = front_face.rules_text {
                                        div.rules-text {
                                            (Self::render_rules_text(rules))
                                        }
                                    }
                                    @if let Some(ref flavor) = front_face.flavor_text {
                                        div.flavor-text {
                                            (flavor)
                                        }
                                    }
                                }

                                // Power/Toughness box
                                @if let (Some(power), Some(toughness)) = (&front_face.power, &front_face.toughness) {
                                    div class=(format!("pt-box {}", pt_box_class)) {
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
        } else {
            html! { html { body { "Error: No faces found" } } }
        }
    }

    fn render_battle(&self, base: &crate::card::CardBase, defense: u32) -> Markup {
        let frame_color = Self::derive_frame_color(&base.mana_cost);
        let frame_class = format!("frame-{}", frame_color);
        let text_box_class = format!("text-box-{}", frame_color);

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

                            // Art box
                            div.art-box {
                                "[Art]"
                            }

                            // Type line
                            div.type-line {
                                div.type-text { (base.type_line) }
                            }

                            // Text box
                            div class=(format!("text-box {}", text_box_class)) {
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

                            // Defense counter (hexagonal shape)
                            div.defense-counter {
                                (defense)
                            }

                            // Rarity indicator
                            div.rarity-indicator class=(rarity_class) {}
                        }
                    }
                }
            }
        }
    }

    fn render_leveler(
        &self,
        base: &crate::card::CardBase,
        _leveler_ranges: &[crate::card::LevelerRange],
    ) -> Markup {
        // Leveler cards have a complex layout with level bars
        // For now, render as a normal card
        // TODO: Implement proper leveler layout
        self.render_normal_card(base)
    }

    fn render_prototype(
        &self,
        base: &crate::card::CardBase,
        _prototype: &crate::card::CardFace,
    ) -> Markup {
        // Prototype cards show two sets of stats
        // For now, render as a normal card showing the main stats
        // TODO: Implement proper prototype layout with both stat sets
        self.render_normal_card(base)
    }
}
