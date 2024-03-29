pub mod vulkan;

pub trait Renderer {
    fn create(name: &str) -> Self
    where
        Self: Sized;
    fn render(&self);
}

impl std::fmt::Debug for dyn Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Renderer")
            // .field("instance", &self.instance.type_id())
            .finish()
    }
}

pub struct VoidRenderer {}

impl Renderer for VoidRenderer {
    fn create(_name: &str) -> Self {
        VoidRenderer {}
    }

    // TODO -- for debug purposes, mostly
    // fn get_info(self) -> RendererInfo {}

    fn render(&self) {
        // do nothing
    }
}
