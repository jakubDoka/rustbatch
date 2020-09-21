extern crate image;

use self::image::{DynamicImage, GenericImageView, GenericImage};
use crate::math::rect::Rect;

pub struct Sheet {
    pub pic: DynamicImage,
    pub regions: Vec<Rect>,
}

impl Sheet {
    pub fn new(img: &[DynamicImage]) -> Self {
        let mut len = 0u32;
        let mut height = 0;
        for img in img.iter() {
            len += img.width();
            let h = img.height();
            if h > height {
                height = h;
            }
        }

        let mut sheet = DynamicImage::new_rgba8(len, height);
        let mut offset = 0;
        let mut regions = Vec::new();
        for img in img {
            for (x, y, color) in img.pixels() {
                sheet.put_pixel(x + offset, y, color);
            }
            regions.push(Rect::wh(offset as f32, 0f32, img.width() as f32, img.height() as f32));
            offset += img.width();
        }

        Self{
            pic: sheet,
            regions,
        }
    }
}
