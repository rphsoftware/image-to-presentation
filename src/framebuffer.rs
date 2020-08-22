use std::collections::HashSet;

pub struct FrameBuffer {
    pub pixels: Vec<u32>,
    width: u32,
    height: u32,
    pub colors: HashSet<u32>
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> FrameBuffer {
        FrameBuffer{
            pixels: vec![0; (width * height) as usize],
            width,
            height,
            colors: HashSet::new()
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: u32) {
        self.pixels[((y * self.width) + x) as usize] = value;
        self.colors.insert(value);
    }

    pub fn get_styles(&mut self) -> String {
        let mut z = String::with_capacity(self.colors.len() * 128);
        for color in self.colors.iter() {
            z.push_str("<style:style style:name=\"col");
            z.push_str(&*format!("{:06x}", color));
            z.push_str("\" style:family=\"paragraph\"><loext:graphic-properties draw:fill-color=\"#");
            z.push_str(&*format!("{:06x}", color));
            z.push_str("\"/></style:style>");
        }
        return z;
    }

    pub fn get_shapes(&mut self) -> String {
        let mut z = String::with_capacity((self.width * self.height * 350) as usize);

        for x in 0..self.width {
            for y in 0..self.height {
                let pixval = self.pixels[((y * self.width) + x) as usize];
                z.push_str("<draw:custom-shape draw:style-name=\"outline\" draw:text-style-name=\"col");
                z.push_str(&*format!("{:06x}", pixval));
                z.push_str("\" draw:layer=\"layout\" svg:width=\"1cm\" svg:height=\"1cm\" svg:x=\"");
                z.push_str(&*format!("{}cm \" svg:y=\"{}cm\">", x, y));
                z.push_str("<draw:enhanced-geometry svg:viewBox=\"0 0 1 1\" draw:type=\"rectangle\" draw:enhanced-path=\"M 0 0 L 1 0 1 1 0 1 0 0 Z N\"/></draw:custom-shape>");
            }
        }

        return z;
    }
}