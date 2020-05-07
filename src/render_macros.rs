macro_rules! set_layout {
    ($factory:expr, $([$times:expr] $ty:ident $flags:expr),*) => {
        $factory.create_descriptor_set_layout(
            amethyst::renderer::util::set_layout_bindings(
                std::iter::empty()
                    $(.chain(std::iter::once((
                        $times as u32,
                        amethyst::renderer::rendy::hal::pso::DescriptorType::$ty,
                        $flags
                    ))))*
            )
        )?.into()
    }
}
