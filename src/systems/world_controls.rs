use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::Camera;

#[derive(SystemDesc)]
pub struct WorldControls;

impl<'s> System<'s> for WorldControls {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, camera, input): Self::SystemData) {
        
    }

}