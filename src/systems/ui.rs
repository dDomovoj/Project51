use amethyst::{
    core::{
        timing::Time,
    },
    ecs::{
        prelude::{Entity, Read, System, WriteStorage},
    },
    // ui::{DrawUiDesc, UiBundle, UiCreator, UiFinder, UiGlyphsSystemDesc, UiText},
    ui::{UiFinder, UiText},
    utils::{
        fps_counter::{FpsCounter},
    },
};

#[derive(Default)]
pub struct UISystem {
    fps_display: Option<Entity>,
}

impl<'a> System<'a> for UISystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, UiText>,
        Read<'a, FpsCounter>,
        UiFinder<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (time, mut ui_text, fps_counter, finder) = data;

        if self.fps_display.is_none() {
            if let Some(fps_entity) = finder.find("FPS") {
                self.fps_display = Some(fps_entity);
            }
        }
        if let Some(fps_entity) = self.fps_display {
            if let Some(fps_display) = ui_text.get_mut(fps_entity) {
                if time.frame_number() % 20 == 0 {
                    let fps = fps_counter.sampled_fps();
                    fps_display.text = format!("FPS: {:.*}", 2, fps);
                }
            }
        }
    }
}
