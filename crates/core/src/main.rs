use wgpu::Backends::{METAL, VULKAN};

fn main() {
    let instances = wgpu::Instance::new(wpgu::InstanceDescriptor {
        backends: wgpu::util::backend_bits_from_env()
            .unwrap(wgpu::Backends::VULKAN | wgpu::Backends::Metal)
    })
}
