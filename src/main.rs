mod color_utils;
mod framebuffer;

use std::fs::File;
use std::io::Write;
use png::ColorType::RGB;
use crate::framebuffer::FrameBuffer;

fn write_styles_xml(w: u32, h: u32) {
    let z = format!("{}{}{}{}{}",
            include_str!("binary/styles.xml"),
            w,
            include_str!("binary/styles-2.xml"),
            h,
            include_str!("binary/styles-3.xml")
    );

    let mut fd = File::create("styles.xml").expect("Failed to create styles.xml");
    fd.write(z.as_ref());
    fd.flush().expect("Failed to flush");
}

fn main() {
    let decoder = png::Decoder::new(File::open("in.png").unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();

    write_styles_xml(info.width, info.height);


    let mut fb = FrameBuffer::new(info.width, info.height);

    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).expect("Failed reading frame");
    let mut p = 4;
    if info.color_type == RGB {
        p = 3;
    }

    for x in 0..info.width {
        for y in 0..info.height {
            let index = (((y * (info.width)) * p) + (x * p)) as usize;
            fb.set_pixel(x, y, color_utils::join_colors(buf[index], buf[index+1], buf[index+2]));
        }
    }

    println!("{:?}", fb.colors);
    println!("{}", fb.get_styles());

    let mut fd = File::create("content.xml").expect("Failed to make content.xml");
    fd.write(include_str!("binary/content.xml").as_ref());
    fd.write(fb.get_styles().as_ref());
    fd.write(include_str!("binary/content-2.xml").as_ref());
    fd.write(fb.get_shapes().as_ref());
    fd.write(include_str!("binary/content-3.xml").as_ref());
}