pub mod vulkan;

pub trait Renderer {
    /// Create an instance of the renderer using the given name.
    fn create(name: &str) -> Self
    where
        Self: Sized;

    /// Execute the rendering process.
    /// NOTE: This is likely to get split into separate stages, especially as I figure out how to
    /// execute command buffer builds asynchronously and such.
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
