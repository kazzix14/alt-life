use std::ops::Add;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Ability {
    PhysicalAbility(PhysicalAbility),
    MentalAbility(MentalAbility),
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PhysicalAbility {
    Move { speed: f32, power: f32 },
}

impl Add for PhysicalAbility {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        use PhysicalAbility::*;

        const PANIC_STRING: &'static str = "can't add diffrent variant of 'PhysicalAbility'";

        match self {
            Move { speed, power } => {
                if let Move {
                    speed: speed_other,
                    power: power_other,
                } = other
                {
                    Move {
                        speed: speed.min(speed_other),
                        power: power + power_other,
                    }
                } else {
                    panic!(PANIC_STRING);
                }
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MentalAbility {}
