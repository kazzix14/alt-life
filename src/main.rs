use ezui::*;

fn main() {
    println!("Hello, world!");
}

mod object {
    use crate::attack::Attack;
    use crate::attack::ResistAttack;

    pub struct Strength {
        strength: f32,
        strength_max: f32,
        strength_growing_ratio: f32,
        cure_rate: f32,
        slash_damage_scale: f32,
        shock_damage_scale: f32,
    }

    impl Strength {
        pub fn take_damage(&mut self, attack: &Attack) {
            let attack = self.resist_attack(attack);
            let damage = attack.damage();
            self.strength -= damage;
        }
        pub fn cure(&mut self) -> Result<(), ()> {
            if self.strength < self.strength_max {
                self.strength += self.cure_rate;
                self.grow();
                Ok(())
            } else {
                Err(())
            }
        }
        fn grow(&mut self) {
            let damage = self.strength_max - self.strength;
            let addtional_strength = damage * self.strength_growing_ratio;
            self.strength_max += addtional_strength;
        }
    }

    impl ResistAttack for Strength {
        fn resist_attack(&self, Attack { slash, shock }: &Attack) -> Attack {
            let slash = slash * self.slash_damage_scale;
            let shock = shock * self.shock_damage_scale;
            Attack { slash, shock }
        }
    }

    mod part {
        use crate::ablity::Ability;
        use crate::object::Strength;

        struct EnergyConsumption {
            idle: f32,
            action: f32,
            cure: f32,
        }
        pub struct Part {
            parts_inside: Vec<Part>,
            parts_outside: Vec<Part>,
            ablities: Vec<Ability>,
            strength: Strength,
            size: f32,
            energy_consumption: EnergyConsumption,
        }

        impl Part {
            pub fn idle(&self, energy: f32) -> f32 {
                energy -= self.energy_consumption.idle;
                if self.strength.cure().is_ok() {
                    energy -= self.energy_consumption.cure;
                }
                energy
            }
        }
    }
}

mod attack {
    pub struct Attack {
        slash: f32,
        shock: f32,
    }

    impl Attack {
        fn damage(&self) -> f32 {
            self.slash + self.shock
        }
    }

    pub trait ResistAttack {
        fn resist_attack(&self, attack: &Attack) -> Attack;
    }
}

mod ablity {
    pub enum Ability {
        Walk,
    }
}
