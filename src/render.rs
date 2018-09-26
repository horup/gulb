use gl::*;
use std::os::raw::c_void;
pub struct Render
{
    pub vertex_array:u32,
    pub vertex_buffer:u32,

}

impl Render
{
    pub fn new() -> Self
    {
        Render {vertex_array:0, vertex_buffer:0}
    }

    pub unsafe fn init(&mut self)
    {
        GenVertexArrays(1, &mut self.vertex_array);
        BindVertexArray(self.vertex_array);

        let coords:[f32;9] = [-1.0, -1.0, 0.0,
                               1.0, -1.0, 0.0,
                               0.0,  1.0, 0.0 ]; 

        GenBuffers(1, &mut self.vertex_buffer);
        
        BindBuffer(ARRAY_BUFFER, self.vertex_buffer);

        let size = (std::mem::size_of::<u32>() * coords.len()) as isize;
        let p = &coords[0] as *const f32 as *const c_void;
        BufferData(ARRAY_BUFFER, size, p, STATIC_DRAW);

    }

    pub unsafe fn draw(&mut self)
    {
        EnableVertexAttribArray(0);
        BindBuffer(ARRAY_BUFFER, self.vertex_buffer);
        VertexAttribPointer(0, 3, FLOAT, FALSE, 0, 0 as *const c_void);
        
        DrawArrays(TRIANGLES, 0, 3);
        DisableVertexAttribArray(0);
    }
}