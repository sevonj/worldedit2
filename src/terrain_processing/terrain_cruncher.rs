use std::path::PathBuf;

use bevy::math::prelude::*;

use crate::terrain_processing::CACHE_DIR;
use crate::terrain_processing::CELL_SIZE;
use crate::terrain_processing::GrayF32Image;
use crate::terrain_processing::NUM_CELLS;
use crate::terrain_processing::NUM_CELLS_ROW;
use crate::terrain_processing::TerrainMesh;
use crate::terrain_processing::WORLD_SIZE;
use crate::terrain_processing::heightmap;

pub fn crunch_terrain() {
    for i in 0..NUM_CELLS {
        let cell_name = format!("cell_{i:03}");
        let hmp_path = PathBuf::from(CACHE_DIR)
            .join(&cell_name)
            .with_extension("hmp");

        let cell_hmp: GrayF32Image = heightmap::load(&hmp_path).unwrap();

        let position = vec2(
            (i % NUM_CELLS_ROW) as f32 * CELL_SIZE as f32 - WORLD_SIZE as f32 / 2.0,
            (i / NUM_CELLS_ROW) as f32 * CELL_SIZE as f32 - WORLD_SIZE as f32 / 2.0,
        );

        let cell = match TerrainMesh::new(position, cell_hmp) {
            Ok(cell) => cell,
            Err(e) => {
                println!("{e}");
                return;
            }
        };

        let tmesh_path = PathBuf::from(CACHE_DIR)
            .join(&cell_name)
            .with_extension(TerrainMesh::FILE_EXT);

        cell.save(&tmesh_path).unwrap();
    }

    for i in 0..NUM_CELLS {
        let cell_name = format!("cell_{i:03}");
        let cell_path = PathBuf::from(CACHE_DIR)
            .join(&cell_name)
            .with_extension(TerrainMesh::FILE_EXT);
        let mut cell = TerrainMesh::load(&cell_path).unwrap();

        let x = i % NUM_CELLS_ROW;
        let y = i / NUM_CELLS_ROW;

        if x == NUM_CELLS_ROW - 1 && y == NUM_CELLS_ROW - 1 {
            continue;
        }

        if y < NUM_CELLS_ROW - 1 {
            let south_idx = i + NUM_CELLS_ROW;
            let south_cell_path = PathBuf::from(CACHE_DIR)
                .join(format!("cell_{south_idx:03}"))
                .with_extension(TerrainMesh::FILE_EXT);
            let south_cell = TerrainMesh::load(&south_cell_path).unwrap();
            cell.fix_south_seam(south_cell);
        }
        if x < NUM_CELLS_ROW - 1 {
            let east_idx = i + 1;
            let east_cell_path = PathBuf::from(CACHE_DIR)
                .join(format!("cell_{east_idx:03}"))
                .with_extension(TerrainMesh::FILE_EXT);
            let east_cell = TerrainMesh::load(&east_cell_path).unwrap();
            cell.fix_east_seam(east_cell);
        }
        if x < NUM_CELLS_ROW - 1 && y < NUM_CELLS_ROW - 1 {
            let southeast_idx = i + NUM_CELLS_ROW + 1;
            let southeast_cell_path = PathBuf::from(CACHE_DIR)
                .join(format!("cell_{southeast_idx:03}"))
                .with_extension(TerrainMesh::FILE_EXT);
            let southeast_cell = TerrainMesh::load(&southeast_cell_path).unwrap();
            cell.fix_southeast_seam(southeast_cell);
        }

        cell.save(&cell_path).unwrap();
    }
}
