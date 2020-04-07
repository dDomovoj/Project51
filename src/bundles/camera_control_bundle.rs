// use std::marker::PhantomData;

// use super::*;
use amethyst::{
    // core::SystemDesc,
    // derive::SystemDesc,
    // ecs::{System, SystemData, World},
    shrev::{EventChannel, ReaderId},
    controls::{
        HideCursor, WindowFocus,
        CursorHideSystemDesc, MouseFocusUpdateSystemDesc, FreeRotationSystemDesc,
    },
    core::{
        bundle::SystemBundle,
        math::{one, convert, Vector3, Unit},
        // shrev::{EventChannel, ReaderId},
        SystemDesc,
        timing::Time,
        transform::Transform,
    },
    derive::{SystemDesc},
    // derive_new::new,
    // ecs::prelude::{Component, DenseVecStorage}
    ecs::{
        prelude::{Join, DispatcherBuilder, World, Component, NullStorage},
        System, SystemData, Read, ReadStorage, WriteStorage,// WorldExt,
    },
    error::Error,
    input::{BindingTypes, InputHandler, get_input_axis_simple},
    // prelude::*,
    winit::{DeviceEvent, Event, Window, WindowEvent},
};
use serde::{Deserialize, Serialize};

// use derive_new::new;
// use winit::{DeviceEvent, Event, Window, WindowEvent};

// #[cfg(feature = "profiler")]
// use thread_profiler::profile_scope;

// use amethyst_core::{
//     ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, Write, WriteStorage},
//     math::{convert, Unit, Vector3},
//     shrev::{EventChannel, ReaderId},
//     timing::Time,
//     transform::Transform,
// };
// use amethyst_derive::SystemDesc;
// use amethyst_input::{get_input_axis_simple, BindingTypes, InputHandler};

// use crate::{
//     components::{ArcBallControlTag, FlyControlTag},
//     resources::{HideCursor, WindowFocus},
// };

/// Add this to a camera if you want it to be a fly camera.
/// You need to add the FlyControlBundle or the required systems for it to work.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct MouseControlTag;

impl Component for MouseControlTag {
    type Storage = NullStorage<MouseControlTag>;
}

/// The bundle that creates a flying movement system.
///
/// Note: Will not actually create a moving entity. It will only register the needed resources and
/// systems.
///
/// You might want to add `"fly_movement"` and `"free_rotation"` as dependencies of the
/// `TransformSystem` in order to apply changes made by these systems in the same frame.
/// Adding this bundle will grab the mouse, hide it and keep it centered.
///
/// # Type parameters
///
/// * `T`: This are the keys the `InputHandler` is using for axes and actions. Often, this is a `StringBindings`.
///
/// # Systems
///
/// This bundle adds the following systems:
///
/// * `FlyMovementSystem`
/// * `FreeRotationSystem`
/// * `MouseFocusUpdateSystem`
/// * `CursorHideSystem`
#[derive(Debug)]
pub struct CameraControlBundle <T: BindingTypes> {
    sensitivity_x: f32,
    sensitivity_y: f32,
    speed: f32,
    side_input_axis: Option<T::Axis>,
    up_input_axis: Option<T::Axis>,
    forward_input_axis: Option<T::Axis>,
}

impl<T: BindingTypes> CameraControlBundle<T> {

    /// Builds a new fly control bundle using the provided axes as controls.
    pub fn new() -> Self {
        CameraControlBundle {
            sensitivity_x: 1.0,
            sensitivity_y: 1.0,
            speed: one(),
            side_input_axis: None,
            up_input_axis: None,
            forward_input_axis: None,
        }
    }

    /// Alters the mouse sensitivy on this `FlyControlBundle`
    pub fn with_sensitivity(mut self, x: f32, y: f32) -> Self {
        self.sensitivity_x = x;
        self.sensitivity_y = y;
        self
    }

    /// Alters the speed on this `FlyControlBundle`.
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn with_side_input_axis(mut self, side_input_axis: Option<T::Axis>) -> Self {
        self.side_input_axis = side_input_axis;
        self
    }

    pub fn with_forward_input_axis(mut self, forward_input_axis: Option<T::Axis>) -> Self {
        self.forward_input_axis = forward_input_axis;
        self
    }

    pub fn with_up_input_axis(mut self, up_input_axis: Option<T::Axis>) -> Self {
        self.up_input_axis = up_input_axis;
        self
    }

}

impl<'a, 'b, T: BindingTypes> SystemBundle<'a, 'b> for CameraControlBundle<T> {

    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>,) -> Result<(), Error> {
        builder.add(CreativeMovementSystemDesc::<T>::new(
                self.speed,
                self.side_input_axis,
                self.up_input_axis,
                self.forward_input_axis,
            ).build(world), 
            "creative_movement", 
            &[]
        );
        builder.add(
            MouseRotationSystemDesc::new(self.sensitivity_x, self.sensitivity_y).build(world),
            "mouse_rotation",
            &[],
        );
        builder.add(
            MouseFocusUpdateSystemDesc::default().build(world),
            "mouse_focus",
            &["mouse_rotation"],
        );
        builder.add(
            CursorHideSystemDesc::default().build(world),
            "cursor_hide",
            &["mouse_focus"],
        );
        Ok(())
    }
}

/// Creative Fly

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct CreativeMovementControlTag;

impl Component for CreativeMovementControlTag {
    type Storage = NullStorage<CreativeMovementControlTag>;
}

/// The system that manages the fly movement.
///
/// # Type parameters
///
/// * `T`: This are the keys the `InputHandler` is using for axes and actions. Often, this is a `StringBindings`.
#[derive(Debug, SystemDesc)]
#[system_desc(name(CreativeMovementSystemDesc))]
pub struct CreativeMovementSystem<T>
where
    T: BindingTypes,
{
    /// The movement speed of the movement in units per second.
    speed: f32,
    /// The name of the input axis to locally move in the x coordinates.
    side_input_axis: Option<T::Axis>,
    /// The name of the input axis to locally move in the y coordinates.
    up_input_axis: Option<T::Axis>,
    /// The name of the input axis to locally move in the z coordinates.
    forward_input_axis: Option<T::Axis>,
}

impl<T: BindingTypes> CreativeMovementSystem<T> {
    /// Builds a new `CreativeMovementSystem` using the provided speeds and axis controls.
    fn new(
        speed: f32,
        side_input_axis: Option<T::Axis>,
        up_input_axis: Option<T::Axis>,
        forward_input_axis: Option<T::Axis>,
    ) -> Self {
        CreativeMovementSystem {
            speed,
            side_input_axis,
            up_input_axis,
            forward_input_axis,
        }
    }
}

impl<'a, T: BindingTypes> System<'a> for CreativeMovementSystem<T> {

    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler<T>>,
        ReadStorage<'a, CreativeMovementControlTag>,
    );

    fn run(&mut self, (time, mut transform, input, tag): Self::SystemData) {
        // #[cfg(feature = "profiler")]
        // profile_scope!("fly_movement_system");

        let x = get_input_axis_simple(&self.side_input_axis, &input);
        let y = get_input_axis_simple(&self.up_input_axis, &input);
        let z = get_input_axis_simple(&self.forward_input_axis, &input);

        if let Some(dir) = Unit::try_new(Vector3::new(x, y, z), convert(1.0e-6)) {
            for (transform, _) in (&mut transform, &tag).join() {
                let delta_sec = time.delta_seconds();
                transform.append_translation_along(dir, delta_sec * self.speed);
            }
        }
    }

}

/// Rotation

#[derive(Debug)]
pub struct MouseRotationSystem {
    sensitivity_x: f32,
    sensitivity_y: f32,
    // #[system_desc(event_channel_reader)]
    event_reader: ReaderId<Event>,
}

#[derive(Debug)]
pub struct MouseRotationSystemDesc {
    sensitivity_x: f32,
    sensitivity_y: f32,
}

impl MouseRotationSystemDesc {

    fn new(sensitivity_x: f32, sensitivity_y: f32) -> Self {
        MouseRotationSystemDesc { sensitivity_x, sensitivity_y }
    }

}

impl<'a, 'b> SystemDesc<'a, 'b, MouseRotationSystem> for MouseRotationSystemDesc {

    fn build(self, world: &mut World) -> MouseRotationSystem {
        <MouseRotationSystem as System<'_>>::SystemData::setup(world);

        let reader_id = world
            .fetch_mut::<EventChannel<Event>>()
            .register_reader();

        MouseRotationSystem::new(self.sensitivity_x, self.sensitivity_y, reader_id)
    }

}

impl MouseRotationSystem {

    pub fn new(sensitivity_x: f32, sensitivity_y: f32, event_reader: ReaderId<Event>) -> Self {
        MouseRotationSystem {
            sensitivity_x,
            sensitivity_y,
            event_reader
        }
    }

}

impl<'a> System<'a> for MouseRotationSystem {

    type SystemData = (
        Read<'a, EventChannel<Event>>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, MouseControlTag>,
        Read<'a, WindowFocus>,
        Read<'a, HideCursor>,
    );

    fn run(&mut self, (events, mut transform, tag, focus, hide): Self::SystemData) {
        let focused = focus.is_focused;
        for event in events.read(&mut self.event_reader) {
            if focused && hide.hide {
                if let Event::DeviceEvent { ref event, .. } = *event {
                    if let DeviceEvent::MouseMotion { delta: (x, y) } = *event {
                        for (transform, _) in (&mut transform, &tag).join() {
                            transform.append_rotation_x_axis(
                                (-(y as f32) * self.sensitivity_y).to_radians(),
                            );
                            transform.prepend_rotation_y_axis(
                                (-(x as f32) * self.sensitivity_x).to_radians(),
                            );
                        }
                    }
                }
            }
        }
    }

}