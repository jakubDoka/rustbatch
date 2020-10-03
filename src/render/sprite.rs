use crate::math::mat::Mat;
use crate::math::rect::Rect;
use crate::math::vect::Vect;
use crate::render::buffer;
use crate::render::batch::{Target, VertexData};
use crate::math::rgba::RGBA;
use crate::render::particle::system::Particle;
use crate::Batch;

/// Sprite is used for drawing images to batch
/// It does not contain any heavy image data.
/// if you check size of sprite its not horribly big and copying it is only right approach
/// its a trade of - copying is slower in order to make drawing faster
pub struct Sprite {
    loc_verts: [Vect; Sprite::VERTEX_COUNT],
    verts: [Vect; Sprite::VERTEX_COUNT],
    radius: f32,
    trig_data: [f32; Sprite::VERTEX_COUNT * Sprite::VERTEX_SIZE],
    buff: [f32; Sprite::VERTEX_SIZE],
}



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
    pub const PATTERN: [u32; 6] = [0, 1, 3, 1, 2, 3];
    pub const VERTEX_COUNT: usize = 4;
    pub const VERTEX_SIZE: usize = 8;
    pub const COLOR_OFFSET: usize = 4;

    /// new returns new sprite. rect is a texture region of texture in sprite-sheet
    /// so if you are not using sprite-sheet just pass a bounding rectangle of whole texture
    #[inline]
    pub fn new(region: Rect) -> Sprite {
        Sprite {
            loc_verts: region.loc_verts(),
            verts: region.verts(),
            trig_data: [0f32; Self::VERTEX_SIZE * Self::VERTEX_COUNT],
            buff: [0f32; Self::VERTEX_SIZE],
            radius: region.radius(),
        }
    }

    #[inline]
    fn update_trig_data(&mut self, mat: &Mat, color: &RGBA) {
        self.buff[Self::COLOR_OFFSET..Self::VERTEX_SIZE].copy_from_slice(color);

        let mut projected;
        for i in 0..Self::VERTEX_COUNT {
            projected = mat.prj(self.loc_verts[i]);
            self.buff[0] = projected.x;
            self.buff[1] = projected.y;
            self.buff[2] = self.verts[i].x;
            self.buff[3] = self.verts[i].y;

            let idx = i * Self::VERTEX_SIZE;
            self.trig_data[idx..idx + Self::VERTEX_SIZE]
                .copy_from_slice(&self.buff);
        }
    }

    /// draw_with_matrix "draws" texture to to batch transformed by given matrix and multiplied
    /// by given color
    #[inline]
    pub fn draw_with_matrix<T: Target>(&mut self, target: &mut T, mat: &Mat, color: &RGBA) {
        self.update_trig_data(mat, color);
        target.append(&self.trig_data, &Self::PATTERN, buffer::DEFAULT_VERTEX_SIZE, None, None, &None);
    }

    /// draw "draws" texture to to batch
    #[inline]
    pub fn draw<T: Target>(&mut self, target: &mut T, position: Vect, scale: Vect, rotation: f32, color: &RGBA) {
        self.draw_with_matrix(target, &Mat::new(position, scale, rotation), color);
    }
}

impl Particle for Sprite {
    fn draw(&mut self, target: &mut VertexData, position: Vect, rotation: f32, scale: f32, color: &RGBA) {
        self.draw_with_matrix(target, &Mat::new(position, Vect::new(scale, scale), rotation), color);
    }
}