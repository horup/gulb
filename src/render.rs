use gl::*;
use std::os::raw::c_void;
use cgmath::{Vector3};

use super::primitive::*;

pub struct Render
{
    vertex_array:u32,
    vertex_buffer:u32,
    max_primitives:usize,
    num_primitives:usize,
    vertex_shader_id:u32,
    fragment_shader_id:u32,
    program_id:u32,
    primitives:Vec<Primitive>
}


impl Render
{
    pub fn new() -> Self
    {
        Render
        {
            vertex_array:0, 
            vertex_buffer:0, 
            max_primitives:0,
            num_primitives:0,
            vertex_shader_id:0,
            fragment_shader_id:0,
            program_id:0,
            primitives:Vec::new()
        }
    }

    pub fn set_primitive(&mut self, index:usize, primitive:Primitive)
    {
        let length = self.primitives.len();
        if index >= self.primitives.len()
        {
            let extend = index - length + 1;
            self.primitives.extend(vec![Primitive::new();extend]);
        }

        self.primitives[index] = primitive;
    }

    pub fn get_primitive(&mut self, index:usize) -> &mut Primitive
    {
        &mut self.primitives[index]
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
      /*  BindBuffer(ARRAY_BUFFER, self.vertex_buffer);

        let vertices = self.generate();

        let size = (std::mem::size_of::<Vector3<f32>>() * vertices.len()) as isize;
        let p = &vertices[0] as *const Vector3<f32> as *const c_void;
        BufferData(ARRAY_BUFFER, size, p, STATIC_DRAW);
        self.num_primitives = self.max_primitives;*/

        self.load_shaders();
    }

    unsafe fn update_buffers(&mut self)
    {
        BindBuffer(ARRAY_BUFFER, self.vertex_buffer);
        let num_primitives = self.primitives.len();
        let mut vertices:Vec<Vector3<f32>> = Vec::new();//;self.generate();
        for primitive in &self.primitives
        {
            let mut transformed = primitive.vertices.clone();
            for t in &mut transformed
            {
                *t = *t * primitive.scale;
                *t = *t + primitive.translate;
                //let test = *t + primitive.translate;
            }

            vertices.extend(&transformed);
        }

        let size = (std::mem::size_of::<Vector3<f32>>() * vertices.len()) as isize;
        let p = &vertices[0] as *const Vector3<f32> as *const c_void;

        if self.max_primitives < num_primitives
        {
            BufferData(ARRAY_BUFFER, size, p, DYNAMIC_DRAW);
            self.max_primitives = num_primitives;
        }
        else
        {
            BufferSubData(ARRAY_BUFFER, 0, size, p);
        }
    }

    pub unsafe fn draw(&mut self)
    {
        self.update_buffers();
        
        Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        UseProgram(self.program_id);
        EnableVertexAttribArray(0);
        BindBuffer(ARRAY_BUFFER, self.vertex_buffer);
        VertexAttribPointer(0, 3, FLOAT, FALSE, 0, 0 as *const c_void);
        DrawArrays(TRIANGLES, 0, self.primitives.len() as i32 * 3 * 2);
        DisableVertexAttribArray(0);
    }
}