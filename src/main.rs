mod ability;
mod attack;
mod field;
mod game;
mod gui;
mod object;

const CSS_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/resource/css/main.css");

const CSS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resource/css/main.css"
));
const FONT_DEJAVU_SANS_MONO: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resource/font/DejaVuSansMono.ttf"
));

fn main() {
    use crate::{
        field::{Block, FieldBuilder},
        game::{Game, GameConfig, GuiConfig},
        object::Object,
    };
    use azul::prelude::{css, App, AppConfig, FontSource, WindowCreateOptions};

    let creature1 = build_creature();
    let creature2 = build_creature();
    let creature3 = build_creature();
    let creature4 = build_creature();

    let field = FieldBuilder::default()
        .field(vec![Block::Empty(None); 16])
        .field_width(4)
        .field_height(4)
        .build()
        .unwrap();

    let mut app = App::new(
        Game {
            field: field,
            config: GameConfig {
                gui_config: GuiConfig {
                    field_view_grid_count: 30,
                    font_size: 20.0,
                },
            },
        },
        AppConfig::default(),
    )
    .unwrap();

    #[cfg(not(debug_assertions))]
    let window = {
        let css = css::from_str(CSS).unwrap();

        app.create_window(WindowCreateOptions::default(), css)
            .unwrap()
    };

    #[cfg(debug_assertions)]
    let window = {
        let css = css::hot_reload(CSS_PATH, std::time::Duration::from_millis(500));
        app.create_hot_reload_window(WindowCreateOptions::default(), css)
            .unwrap()
    };

    let dejavu_sans_mono_id = app.app_state.resources.add_css_font_id("DejaVuSansMono");

    app.app_state.resources.add_font_source(
        dejavu_sans_mono_id,
        FontSource::Embedded(FONT_DEJAVU_SANS_MONO),
    );

    app.run(window).unwrap();
}

fn build_creature() -> object::creature::Creature {
    use object::{
        creature::{CreatureBuilder, EnergyConsumptionBuilder, PartBuilder},
        StrengthBuilder,
    };

    let strength = StrengthBuilder::default()
        .strength_max(100.0)
        .strength_growing_ratio(0.01)
        .cure_rate(1.0)
        .slash_damage_scale(0.5)
        .shock_damage_scale(0.4)
        .build()
        .unwrap();

    let energy_consumption = EnergyConsumptionBuilder::default()
        .idle(1.0)
        .cure(1.0)
        .action(10.0)
        .build()
        .unwrap();

    let part = PartBuilder::default()
        .strength(strength)
        .size(10.0)
        .energy_consumption(energy_consumption)
        .build()
        .unwrap();

    CreatureBuilder::default()
        .body(part)
        .energy(1000.0)
        .build()
        .unwrap()
}
