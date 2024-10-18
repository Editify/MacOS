use wgpu::Backends;

fn main() {
    let instances = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: Backends::METAL | Backends::VULKAN,
        dx12_shader_compiler: wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default(),
        gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        flags: wgpu::InstanceFlags::default(),
    });

    println!("Instance {:?}", instances);
}
