use cgmath::{Vector3};

#[derive(Clone, Copy)]
pub struct Primitive
{
    pub visible:bool,
    pub translate:Vector3<f32>,
    pub scale:f32,


    pub vertices:[Vector3<f32>;6]
}

impl Primitive
{
    fn generate() -> [Vector3<f32>;6]
    {
        let mut vertices:[Vector3<f32>;6] = [Vector3::new(0.0, 0.0, 0.0);6];
        let k = 0.5;
        let x = 0.0;//((i % 255) as i32 - 128) as f32 / 255.0;
        let y = 0.0;//(i / 255) as f32 / 255.0;
        let offset:Vector3<f32> = Vector3::new(x, y, 0.0);

        let p1 = Vector3::new(-k, -k, 0.0) + offset;
        let p2 = Vector3::new( k, -k, 0.0) + offset;
        let p3 = Vector3::new(-k,  k, 0.0) + offset;
        let p4 = Vector3::new( k,  k, 0.0) + offset;

        vertices[0] = p1;
        vertices[1] = p2;
        vertices[2] = p3;

        vertices[3] = p2;
        vertices[4] = p4;
        vertices[5] = p3;

        vertices
    }

    pub fn new() -> Self
    {
        Primitive
        {
            visible:false,
            translate:Vector3::new(0.0, 0.0, 0.0),
            scale:1.0,
            vertices:Primitive::generate()
        }
    }
}