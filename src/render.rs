use gl::*;
use std::os::raw::c_void;
use cgmath::{Vector3};
pub struct Render
{
    pub vertex_array:u32,
    pub vertex_buffer:u32,
    pub max_primitives:usize,
    pub primitives:usize,
    pub vertex_shader_id:u32,
    pub fragment_shader_id:u32,
    pub program_id:u32
}

impl Render
{
    pub fn new() -> Self
    {
        Render
        {
            vertex_array:0, 
            vertex_buffer:0, 
            max_primitives:1,
            primitives:0,
            vertex_shader_id:0,
            fragment_shader_id:0,
            program_id:0
        }
    }


    pub unsafe fn load_shaders(&mut self)
    {
        {
            let shader_source = include_bytes!("../shaders/vertex.glsl");
            let source_length = [shader_source.len() as i32];
            let shader_id = CreateShader(VERTEX_SHADER);
            let ptr1 = &shader_source[0] as *const u8;
            let ptr2 = &ptr1 as *const *const u8;

            ShaderSource(shader_id, 1, ptr2 as *const *const i8, &source_length[0] as *const i32);
            CompileShader(shader_id);
            self.vertex_shader_id = shader_id;
        }

        {
            let shader_source = include_bytes!("../shaders/fragment.glsl");
            let source_length = [shader_source.len() as i32];
            let shader_id = CreateShader(FRAGMENT_SHADER);
            let ptr1 = &shader_source[0] as *const u8;
            let ptr2 = &ptr1 as *const *const u8;

            ShaderSource(shader_id, 1, ptr2 as *const *const i8, &source_length[0] as *const i32);
            CompileShader(shader_id);
            self.fragment_shader_id = shader_id;
        }

        self.program_id = CreateProgram();
        AttachShader(self.program_id, self.vertex_shader_id);
        AttachShader(self.program_id, self.fragment_shader_id);
        LinkProgram(self.program_id);
    }

    pub unsafe fn init(&mut self)
    {
        GenVertexArrays(1, &mut self.vertex_array);
        BindVertexArray(self.vertex_array);
        GenBuffers(1, &mut self.vertex_buffer);
        BindBuffer(ARRAY_BUFFER, self.vertex_buffer);

        let vertices = self.generate();

        let size = (std::mem::size_of::<Vector3<f32>>() * vertices.len()) as isize;
        let p = &vertices[0] as *const Vector3<f32> as *const c_void;
        BufferData(ARRAY_BUFFER, size, p, STATIC_DRAW);
        self.primitives = self.max_primitives;

        self.load_shaders();
    }
    
    fn generate(&self) -> Vec<Vector3<f32>>
    {
        let max = self.max_primitives;

        let mut vertices:Vec<Vector3<f32>> = Vec::with_capacity(max * 6);
        for i in 0..max
        {
            let k = 0.01;
            let x = 0.0;//((i % 255) as i32 - 128) as f32 / 255.0;
            let y = 0.0;//(i / 255) as f32 / 255.0;
            let offset:Vector3<f32> = Vector3::new(x, y, 0.0);

            let p1 = Vector3::new(-k, -k, 0.0) + offset;
            let p2 = Vector3::new( k, -k, 0.0) + offset;
            let p3 = Vector3::new(-k,  k, 0.0) + offset;
            let p4 = Vector3::new( k,  k, 0.0) + offset;

            vertices.push(p1);
            vertices.push(p2);
            vertices.push(p3);

            vertices.push(p2);
            vertices.push(p4);
            vertices.push(p3);
            
        }

        vertices
    }

    pub unsafe fn draw(&mut self)
    {
        Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        UseProgram(self.program_id);
        EnableVertexAttribArray(0);
        BindBuffer(ARRAY_BUFFER, self.vertex_buffer);
        VertexAttribPointer(0, 3, FLOAT, FALSE, 0, 0 as *const c_void);
        DrawArrays(TRIANGLES, 0, self.primitives as i32 * 3 * 2);
        DisableVertexAttribArray(0);
    }
}