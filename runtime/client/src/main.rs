use rustaria::api::Api;
use rustaria_graphics::{RenderBackend, RenderHandler};
use rustaria_util::info;
use rustaria_wgpu::WgpuBackend;

fn main() {
    rustaria_util::initialize().unwrap();

    let mut api = Api::new();
    info!("reload");
    api.reload().unwrap();

    //let mut render = RenderHandler::<WgpuBackend>::new();
    //while render.alive() {
    //    render.poll(|event| {
    //    });
//
    //    render.draw();
    //}
}
