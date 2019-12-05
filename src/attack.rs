#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Attack {
    pub slash: f32,
    pub shock: f32,
}

impl Attack {
    pub fn damage(&self) -> f32 {
        self.slash + self.shock
    }
}

pub trait ResistAttack {
    fn slash_damage_scale(&self) -> f32;
    fn shock_damage_scale(&self) -> f32;
    fn resist_attack(&self, Attack { slash, shock }: &Attack) -> Attack {
        let slash = slash * self.slash_damage_scale();
        let shock = shock * self.shock_damage_scale();
        Attack { slash, shock }
    }}
