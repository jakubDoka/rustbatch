use crate::math::mat::Mat;
use crate::math::rect::Rect;
use crate::math::vect::Vect;
use crate::render::buffer;
use crate::render::batch::Batch;
use crate::math::rgba::RGBA;

pub struct Sprite {
    loc_verts: [Vect; 4],
    verts: [Vect; 4],
    radius: f32,
    trig_data: Vec<f32>,
    buff: [f32; buffer::DATA_SIZE],
}

pub const PATTERN: [u32; 6] = [0, 1, 3, 1, 2, 3];
pub const VERTEX_COUNT: usize = 4;

impl Clone for Sprite {
    fn clone(&self) -> Self {
       Sprite{
           loc_verts: self.loc_verts.clone(),
           verts: self.verts.clone(),
           radius: self.radius.clone(),
           trig_data: self.trig_data.clone(),
           buff: self.buff.clone(),
       }
    }
}

impl Sprite {
    #[inline]
    pub fn new(region: Rect) -> Sprite {
        Sprite {
            loc_verts: region.loc_verts(),
            verts: region.verts(),
            trig_data: Vec::with_capacity(buffer::DATA_SIZE * VERTEX_COUNT),
            buff: [0f32; buffer::DATA_SIZE],
            radius: region.radius(),
        }
    }

    #[inline]
    fn update_trig_data(&mut self, mat: &Mat, color: &RGBA) {
        self.buff[buffer::COLOR.offset..buffer::DATA_SIZE].copy_from_slice(color);

        let mut projected;
        for i in 0..VERTEX_COUNT {
            projected = mat.prj(self.loc_verts[i]);
            self.buff[0] = projected.x;
            self.buff[1] = projected.y;
            self.buff[2] = self.verts[i].x;
            self.buff[3] = self.verts[i].y;

            self.trig_data.extend(&self.buff);
        }
    }

    #[inline]
    pub fn draw(&mut self, batch: &mut Batch, mat: &Mat, color: &RGBA) {
        self.update_trig_data(mat, color);
        batch.append(&mut self.trig_data, &mut Vec::from(PATTERN))
    }
}