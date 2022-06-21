// pub(crate) const VERTICES : &[Vertex; 3] =  &[
//     Vertex { position: [0.0, 0.5, 0.0], colour: [1.0, 0.0, 0.0] },
//     Vertex { position: [-0.5, -0.5, 0.0], colour: [0.0, 1.0, 0.0] },
//     Vertex { position: [0.5, -0.5, 0.0], colour: [0.0, 0.0, 1.0] },
// ];

use wgpu::{Buffer, Device, util::DeviceExt};

// pub(crate) const SHAPE_1 : VertexShape = VertexShape {
//     vertices: &[
//         Vertex { position: [-0.0868241, 0.49240386, 0.0], colour: [0.5, 0.0, 0.5] }, // A
//         Vertex { position: [-0.49513406, 0.06958647, 0.0], colour: [0.5, 0.0, 0.5] }, // B
//         Vertex { position: [-0.21918549, -0.44939706, 0.0], colour: [0.5, 0.0, 0.5] }, // C
//         Vertex { position: [0.35966998, -0.3473291, 0.0], colour: [0.5, 0.0, 0.5] }, // D
//         Vertex { position: [0.44147372, 0.2347359, 0.0], colour: [0.5, 0.0, 0.5] }, // E
//     ],
//     indices: &[
//         0, 1, 4,
//         1, 2, 4,
//         2, 3, 4,
//     ],
// };


pub(crate) struct VertexShape {
    pub(crate) indices: &'static [u16],
    pub(crate) vertex_buffer: Buffer,
    pub(crate) index_buffer: Buffer,
}

impl VertexShape {
    pub(crate) fn new(id: u32, device: &Device, vertices: &'static [Vertex], indices: &'static [u16]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(format!("Vertex Buffer {}", id).as_str()),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(format!("Indices Buffer {}", id).as_str()),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        Self {
            indices,
            vertex_buffer,
            index_buffer,
        }
    } 
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) tex_coords: [f32; 2],
}
impl Vertex {
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2, // NEW!
                },
            ]
        }
    }
}