use crate::attack::Attack;
//use crate::gui::GuiOperation;
use crate::field::Field;
use crate::object::creature::Creature;
use crate::object::creature::CreatureBuilder;
use crate::object::creature::EnergyConsumption;
use crate::object::creature::PartBuilder;
use crate::object::creature::PartSelector;
use crate::object::StrengthBuilder;

pub struct Game {
    pub field: Field,
    pub config: GameConfig,
}

pub struct GameConfig {
    pub gui_config: GuiConfig,
}

pub struct GuiConfig {
    pub field_view_grid_count: u16,
    pub font_size: f32,
}

pub struct App;

impl App {
    pub fn new() -> App {
        let strength = StrengthBuilder::default()
            .strength_max(100.0)
            .strength_growing_ratio(0.01)
            .cure_rate(1.0)
            .slash_damage_scale(0.7)
            .shock_damage_scale(0.5)
            .build()
            .unwrap();

        let energy_consumption = EnergyConsumption::new(1.0, 1.0, 10.0);

        let part = PartBuilder::default()
            .strength(strength)
            .size(10.0)
            .energy_consumption(energy_consumption)
            .build()
            .unwrap();

        let mut creature = CreatureBuilder::default().body(part).build().unwrap();

        let target = vec![PartSelector::This];
        //println!("init: {:?}", creature);
        creature.take_damage(
            &Attack {
                slash: 10.0,
                shock: 5.0,
            },
            &target,
        );
        //println!("attacked: {:?}", creature);

        App
    }
}
