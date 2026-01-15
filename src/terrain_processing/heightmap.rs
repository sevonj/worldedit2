use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use image::DynamicImage;
use image::GenericImageView;
use image::GrayImage;
use image::ImageBuffer;
use image::Luma;

/// Single channel f32 image for heightmaps
pub type GrayF32Image = image::ImageBuffer<image::Luma<f32>, Vec<f32>>;

pub const FILE_EXT: &str = "hmp";
pub const FILE_SIG: &[u8; 16] = b"WEdit-hmp_f32   ";
pub const FILE_VER: u32 = 0;

pub fn from_dynamic_image(img: DynamicImage) -> GrayF32Image {
    let (w, h) = img.dimensions();
    match img {
        DynamicImage::ImageRgb32F(rgb) => {
            ImageBuffer::from_fn(w, h, |x, y| Luma([rgb.get_pixel(x, y)[0]]))
        }
        DynamicImage::ImageRgba32F(rgba) => {
            ImageBuffer::from_fn(w, h, |x, y| Luma([rgba.get_pixel(x, y)[0]]))
        }
        _ => {
            println!("split_base_hmp() WARNING: Image isn't 32bit scalar. Quality will suffer.");
            let trash = img.to_luma32f();
            ImageBuffer::from_fn(w, h, |x, y| Luma([trash.get_pixel(x, y)[0]]))
        }
    }
}

pub fn save(path: &Path, map: &GrayF32Image) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);

    let (w, h) = map.dimensions();

    file.write_all(FILE_SIG)?;
    file.write_all(&FILE_VER.to_le_bytes())?;
    file.write_all(&w.to_le_bytes())?;
    file.write_all(&h.to_le_bytes())?;

    let bytes = bytemuck::cast_slice(map.as_raw());
    file.write_all(bytes)?;

    Ok(())
}

/// for preview purposes only
pub fn save_png(path: &Path, map: &GrayF32Image) -> Result<(), image::ImageError> {
    let width = map.width();
    let height = map.height();

    let mut img: GrayImage = GrayImage::new(width, height);
    for (x, y, pixel) in map.enumerate_pixels() {
        let val = (pixel[0].clamp(0.0, 1.0) * 255.0).round() as u8;
        img.put_pixel(x, y, Luma([val]));
    }
    img.save(path)
}

pub fn load(path: &Path) -> std::io::Result<GrayF32Image> {
    let mut file = BufReader::new(File::open(path)?);

    let mut buf = [0u8; 4];
    let mut sig_buf = [0u8; 16];

    file.read_exact(&mut sig_buf)?;
    if &sig_buf != FILE_SIG {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "invalid FILE_SIG",
        ));
    }
    file.read_exact(&mut buf)?;
    let ver = u32::from_le_bytes(buf);
    if ver != FILE_VER {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("invalid version: exp '{FILE_VER}', got '{ver}'"),
        ));
    }

    file.read_exact(&mut buf)?;
    let width = u32::from_le_bytes(buf);
    file.read_exact(&mut buf)?;
    let height = u32::from_le_bytes(buf);

    let pixel_count = width as usize * height as usize;
    let mut pixels = vec![0f32; pixel_count];
    let bytes = bytemuck::cast_slice_mut(&mut pixels);
    file.read_exact(bytes)?;

    GrayF32Image::from_raw(width, height, pixels).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "failed to construct GrayF32Image",
        )
    })
}
