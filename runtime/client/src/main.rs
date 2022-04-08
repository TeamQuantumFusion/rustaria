use rustaria_graphics::{RenderBackend, RenderHandler};
use rustaria_wgpu::WgpuBackend;

fn main() {
    let mut render = RenderHandler::<WgpuBackend>::new();
    while render.alive() {
        render.poll(|event| {
            println!("{:?}", event);
        });

        render.draw();
    }
}
