use std::path::PathBuf;

use image::DynamicImage;
use image::ImageReader;
use image::imageops;

use crate::terrain_processing::GrayF32Image;

use crate::terrain_processing::CACHE_DIR;
use crate::terrain_processing::CELL_SIZE;
use crate::terrain_processing::NUM_CELLS;
use crate::terrain_processing::NUM_CELLS_ROW;
use crate::terrain_processing::WORLD_SIZE;
use crate::terrain_processing::heightmap;

pub fn split_base_hmp(hmp_path: PathBuf) -> Result<(), &'static str> {
    if std::fs::exists(CACHE_DIR).unwrap() {
        std::fs::remove_dir_all(CACHE_DIR).unwrap();
    }
    std::fs::create_dir_all(CACHE_DIR).unwrap();

    let img: DynamicImage = ImageReader::open(hmp_path).unwrap().decode().unwrap();
    let mut world_buf = heightmap::from_dynamic_image(img);

    if world_buf.width() as usize != WORLD_SIZE || world_buf.height() as usize != WORLD_SIZE {
        println!(
            "split_base_hmp() WARNING: Image doesn't match WORLD_SIZE. Resizing from '{}x{}' to '{WORLD_SIZE}x{WORLD_SIZE}'",
            world_buf.width(),
            world_buf.height()
        );
        heightmap::save_png(
            &PathBuf::from(CACHE_DIR).join("bigmap_unscaled.png"),
            &world_buf,
        )
        .unwrap();
        world_buf = imageops::resize(
            &world_buf,
            WORLD_SIZE as u32,
            WORLD_SIZE as u32,
            imageops::FilterType::Triangle,
        );
    }
    heightmap::save_png(&PathBuf::from(CACHE_DIR).join("bigmap.png"), &world_buf).unwrap();

    assert_eq!(world_buf.width(), WORLD_SIZE as u32);
    assert_eq!(world_buf.height(), WORLD_SIZE as u32);

    let temp_dir_path = PathBuf::from(CACHE_DIR);
    for i in 0..NUM_CELLS {
        let x = ((i % NUM_CELLS_ROW) * CELL_SIZE) as u32;
        let y = ((i / NUM_CELLS_ROW) * CELL_SIZE) as u32;

        println!("crop cell at x='{x:04}' y='{y:04}'");

        let cell_map: GrayF32Image =
            imageops::crop_imm(&world_buf, x, y, CELL_SIZE as u32, CELL_SIZE as u32).to_image();

        assert_eq!(cell_map.width(), CELL_SIZE as u32);
        assert_eq!(cell_map.height(), CELL_SIZE as u32);

        let hmp_path = temp_dir_path
            .join(format!("cell_{i:03}"))
            .with_extension(heightmap::FILE_EXT);
        let png_path = hmp_path.with_extension("png");

        heightmap::save(&hmp_path, &cell_map).unwrap();
        heightmap::save_png(&png_path, &cell_map).unwrap();
    }

    Ok(())
}
