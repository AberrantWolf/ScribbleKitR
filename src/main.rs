use scribblekit::{
    app::App,
    render::{vulkan, Renderer},
};

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let mut app = App::new("ScribbleKit", 720, 480);

    let renderer = Box::new(vulkan::VulkanRenderer::create("ScribbleKit"));
    app.set_renderer(renderer);

    Ok(app.run()?)
}
