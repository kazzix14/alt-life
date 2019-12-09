use crate::{ability::Ability, attack::Attack, object::Strength};

use std::collections::VecDeque;

use derive_builder::Builder;

pub type TargetPart = Vec<(PartSelector)>;

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Creature {
    #[builder(default = "VecDeque::new()")]
    operation: VecDeque<CreatureOperation>,
    energy: f32,
    body: Part,
}

impl Creature {
    pub fn take_damage(&mut self, attack: &Attack, target: &TargetPart) {
        let part = self.get(target);
        part.take_damage(attack)
    }

    fn get(&mut self, target: &TargetPart) -> &mut Part {
        let mut part = &mut self.body;
        for selector in target {
            part = part.get(selector);
        }
        part
    }

    fn idle(&mut self) {
        if let Some(energy_consumption) = self.body.next() {
            self.energy -= energy_consumption;
        } else {
            self.energy = 0.0;
        };
    }

    fn is_alive(&mut self) -> bool {
        0.0 < self.energy
    }
}

impl Iterator for Creature {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        use CreatureOperation::*;

        if self.is_alive() {
            match self.operation.pop_front() {
                Some(Idle) => self.idle(),
                None => self.idle(),
            }
            Some(())
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CreatureOperation {
    Idle,
}

#[derive(Clone, Copy, PartialEq, Debug, Builder)]
pub struct EnergyConsumption {
    idle: f32,
    cure: f32,
    action: f32,
}

impl EnergyConsumption {
    pub fn new(idle: f32, cure: f32, action: f32) -> Self {
        Self { idle, cure, action }
    }
}

#[derive(Clone, PartialEq, Debug, Builder)]
#[builder(pattern = "owned")]
pub struct Part {
    #[builder(default = "Vec::new()")]
    parts_inside: Vec<Part>,
    #[builder(default = "Vec::new()")]
    parts_outside: Vec<Part>,
    #[builder(default = "Vec::new()")]
    abilities: Vec<Ability>,
    strength: Strength,
    size: f32,
    energy_consumption: EnergyConsumption,
}

impl Part {
    fn take_damage(&mut self, attack: &Attack) {
        self.strength.take_damage(attack);
    }

    fn get(&mut self, selector: &PartSelector) -> &mut Part {
        use PartSelector::*;

        match *selector {
            This => self,
            Inside { index } => &mut self.parts_inside[index],
            Outside { index } => &mut self.parts_outside[index],
        }
    }
}

impl Iterator for Part {
    // energy consumption
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.strength.is_exhausted() {
            return None;
        }

        let mut energy_consumption = 0.0;
        energy_consumption += self.energy_consumption.idle;
        if self.strength.cure().is_ok() {
            energy_consumption += self.energy_consumption.cure;
        }

        self.parts_inside = self
            .parts_inside
            .iter()
            .cloned()
            .map(|mut part| (part.next(), part))
            .filter(|(ec, _)| ec.is_some())
            .inspect(|(ec, _)| energy_consumption += ec.unwrap())
            .unzip::<_, _, Vec<_>, _>()
            .1;

        self.parts_outside = self
            .parts_outside
            .iter()
            .cloned()
            .map(|mut part| (part.next(), part))
            .filter(|(ec, _)| ec.is_some())
            .inspect(|(ec, _)| energy_consumption += ec.unwrap())
            .unzip::<_, _, Vec<_>, _>()
            .1;

        Some(energy_consumption)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PartSelector {
    This,
    Inside { index: usize },
    Outside { index: usize },
}
