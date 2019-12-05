#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ability {
    PhysicalAbility(PhysicalAbility),
    MentalAbility(MentalAbility),
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PhysicalAbility {
    Walk,
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MentalAbility {}
