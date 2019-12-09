use crate::object::Object;
use azul::prelude::{Dom, Layout};
use derive_builder::Builder;

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Empty(Option<Object>),
    Wall,
}

impl Iterator for Block {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        use Block::Empty;
        if let Empty(Some(object)) = self {
            object.next()
        } else {
            None
        }
    }
}

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct Field {
    field: Vec<Block>,
    field_width: u16,
    field_height: u16,
}

impl Field {
    pub fn get(&self, x: i16, y: i16) -> Option<Block> {
        if x < 0 || self.field_width as i16 <= x || y < 0 || self.field_height as i16 <= y {
            None
        } else {
            Some(self.field[y as usize * self.field_height as usize + x as usize].clone())
        }
    }
}

impl Iterator for Field {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        use crate::field::Block::Empty;
        self.field.iter_mut().for_each(|block| {
            block.next();
        });
        Some(())
    }
}
