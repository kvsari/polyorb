//! Data for cube.

/*
fn vertexize(

const pub fn create() -> (Vec<
 */

pub struct CubeScene;

impl super::Scene for CubeScene {
    fn init(desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> Self {
        CubeScene
    }
    // resize(&mut self, desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device);
    fn update(&mut self, event: wgpu::winit::WindowEvent) { }
    fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device) { }
}
