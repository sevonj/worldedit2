pub mod base_hmp;
pub mod heightmap;
pub mod terrain_cruncher;
mod terrain_mesh;

pub use terrain_mesh::TerrainMesh;

pub type GrayF32Image = image::ImageBuffer<image::Luma<f32>, Vec<f32>>;

// All dimensions are in metres
pub const WORLD_SIZE: usize = 2048;
pub const WORLD_HEIGHT: f32 = 512.;
pub const WORLD_HEIGHT_OFFSET: f32 = -WORLD_HEIGHT / 10.;
pub const CELL_SIZE: usize = 512;
pub const NUM_CELLS_ROW: usize = WORLD_SIZE / CELL_SIZE;
pub const NUM_CELLS: usize = NUM_CELLS_ROW * NUM_CELLS_ROW;

pub const CACHE_DIR: &str = "assets/cache/";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_size_is_multiple_of_cell_size() {
        assert_eq!(WORLD_SIZE % CELL_SIZE, 0);
    }
}
