use amethyst::{
    ecs::prelude::{Component, DenseVecStorage}
};

pub struct Block {
    pub color: (f32, f32, f32)
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}