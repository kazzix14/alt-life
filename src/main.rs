use ezui::*;

fn main() {
    println!("Hello, world!");
}

mod part {
    use crate::ablity::Ability;
    pub struct Part {
        parts_inside: Vec<Part>,
        parts_outside: Vec<Part>,
        ablities: Vec<Ability>,
        strength: Strength,
        parameter: PartParameter,
    }

    pub struct Strength {
        strength: f32,
        strength_max: f32,
        strength_growing_scale: f32,
        cure_rate: f32,
    }

    pub struct PartParameter {
        size: f32,
        energy_efficiency: f32,
    }

    struct AttackResistance {
        slash: f32,
        shock: f32,
    }
}

mod object {
    pub trait Breakable
}

mod attack {
    pub struct Attack {
        slash: f32,
        shock: f32,
    }

    pub trait CanBeAttacked {
        fn attacked(&mut self, attack: &Attack);
        fn slash_reduction_ratio(&self) -> f32;
        fn shock_reduction_ratio(&self) -> f32;
        fn resist(&self, Attack { slash, shock }: &Attack) -> Attack {
            let slash = slash * self.slash_reduction_ratio();
            let shock = shock * self.shock_reduction_ratio();
            Attack { slash, shock }
        }
    }
}

mod ablity {
    pub enum Ability {
        Walk,
    }
}
