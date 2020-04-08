use amethyst::{
    core::ecs::{DispatcherBuilder, World},
    error::Error,
    renderer::{
        bundle::{RenderOrder, RenderPlan, RenderPlugin, Target},
        pass::DrawDebugLinesDesc,
        rendy::{factory::Factory, graph::render::RenderGroupDesc},
        types::Backend,
    },
};

#[derive(Default, Debug)]
pub struct RenderDebugLines {}

impl<B: Backend> RenderPlugin<B> for RenderDebugLines {
    fn on_build<'a, 'b>(
        &mut self, _: &mut World, _builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn on_plan(
        &mut self, plan: &mut RenderPlan<B>, _factory: &mut Factory<B>, _world: &World,
    ) -> Result<(), Error> {
        plan.extend_target(Target::Main, |ctx| {
            ctx.add(
                RenderOrder::Transparent,
                DrawDebugLinesDesc::new().builder(),
            )?;
            Ok(())
        });
        Ok(())
    }
}
