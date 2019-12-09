pub mod creature;

use crate::attack::Attack;
use crate::attack::ResistAttack;
use crate::object::creature::Creature;
use derive_builder::Builder;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Creature(Creature),
}

impl Iterator for Object {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        use Object::*;
        match self {
            Creature(creature) => creature.next(),
        };
        Some(())
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Builder)]
pub struct Strength {
    #[builder(default = "self.strength_max.unwrap()", setter(skip))]
    strength: f32,
    strength_max: f32,
    strength_growing_ratio: f32,
    cure_rate: f32,
    slash_damage_scale: f32,
    shock_damage_scale: f32,
}

impl Strength {
    fn is_exhausted(&self) -> bool {
        self.strength <= 0.0
    }

    fn take_damage(&mut self, attack: &Attack) {
        let attack = self.resist_attack(attack);
        let damage = attack.damage();
        self.strength -= damage;
    }

    fn cure(&mut self) -> Result<(), ()> {
        if self.is_exhausted() {
            return Err(());
        }

        if self.strength_max <= self.strength {
            return Err(());
        }

        self.strength += self.cure_rate;
        self.grow();
        Ok(())
    }
    fn grow(&mut self) {
        let damage = self.strength_max - self.strength;
        let addtional_strength = damage * self.strength_growing_ratio;
        self.strength_max += addtional_strength;
    }
}

impl ResistAttack for Strength {
    fn slash_damage_scale(&self) -> f32 {
        self.slash_damage_scale
    }
    fn shock_damage_scale(&self) -> f32 {
        self.shock_damage_scale
    }
}
