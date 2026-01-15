use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use bevy::math::prelude::*;
use bevy::mesh::prelude::*;
use bytemuck::cast_slice_mut;

use crate::terrain_processing::CELL_SIZE;

const CELL_SIZE_F: f32 = CELL_SIZE as f32;
const V_STRIDE: usize = CELL_SIZE + 1;
const NUM_V: usize = V_STRIDE * V_STRIDE;

#[derive(Debug)]
pub struct TerrainMesh {
    /// min corner coordinate
    position: UVec2,
    vertices: Vec<Vec3>,
    uvs: Vec<Vec2>,
    indices: Vec<u32>,
}

impl TerrainMesh {
    pub const FILE_EXT: &str = "tmesh";
    pub const FILE_SIG: &[u8; 16] = b"WEdit-TMesh     ";
    pub const FILE_VER: u32 = 1;

    pub fn position(&self) -> &UVec2 {
        &self.position
    }

    pub fn position_mut(&mut self) -> &mut UVec2 {
        &mut self.position
    }

    pub fn set_position(&mut self, position: UVec2) {
        self.position = position;
    }

    pub fn vertices(&self) -> &Vec<Vec3> {
        &self.vertices
    }

    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    }

    pub fn bevy_mesh(&self) -> Mesh {
        use bevy::asset::RenderAssetUsages;
        use bevy::mesh::Indices;
        use bevy::mesh::PrimitiveTopology;

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone());
        mesh.insert_indices(Indices::U32(self.indices.clone()));
        mesh
    }

    pub fn new(position: UVec2, f: &dyn Fn(UVec2) -> f32) -> Self {
        let mut vertices = vec![Vec3::ZERO; NUM_V];
        let mut uvs = vec![Vec2::ZERO; NUM_V];

        for (z, (v_row, uv_row)) in vertices
            .chunks_exact_mut(V_STRIDE)
            .zip(uvs.chunks_exact_mut(V_STRIDE))
            .enumerate()
        {
            let pos_z = z as u32 + position.y;
            let uv_v = z as f32 / CELL_SIZE_F;

            for (x, (vertex, uv)) in v_row.iter_mut().zip(uv_row.iter_mut()).enumerate() {
                let pos_x = x as u32 + position.x;
                let uv_u = x as f32 / CELL_SIZE_F;
                let h = f(uvec2(pos_x, pos_z));

                *vertex = Vec3::new(pos_x as f32, h, pos_z as f32);
                *uv = Vec2::new(uv_u, uv_v);
            }
        }

        let num_quads = CELL_SIZE * CELL_SIZE;
        let mut indices = Vec::with_capacity(num_quads * 6);

        for z in 0..CELL_SIZE {
            let row0 = z * V_STRIDE;
            let row1 = (z + 1) * V_STRIDE;

            for x in 0..CELL_SIZE {
                let a = (row0 + x) as u32;
                let b = (row0 + x + 1) as u32;
                let c = (row1 + x) as u32;
                let d = (row1 + x + 1) as u32;

                indices.push(a);
                indices.push(c);
                indices.push(b);

                indices.push(b);
                indices.push(c);
                indices.push(d);
            }
        }

        Self {
            position,
            uvs,
            vertices,
            indices,
        }
    }

    pub fn apply_height(&mut self, f: &dyn Fn(UVec2) -> f32) {
        for (i, vertex) in self.vertices.iter_mut().enumerate() {
            let mut position = self.position;
            position.x += (i % V_STRIDE) as u32;
            position.y += (i / V_STRIDE) as u32;
            vertex.y = f(position);
        }
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let mut file = BufWriter::new(File::create(path)?);

        file.write_all(Self::FILE_SIG)?;
        file.write_all(&Self::FILE_VER.to_le_bytes())?;
        file.write_all(&self.position.x.to_le_bytes())?;
        file.write_all(&self.position.y.to_le_bytes())?;
        file.write_all(bytemuck::cast_slice(&self.vertices))?;
        file.write_all(bytemuck::cast_slice(&self.uvs))?;
        file.write_all(bytemuck::cast_slice(&self.indices))?;

        Ok(())
    }

    pub fn load(path: &Path) -> std::io::Result<Self> {
        let mut file = BufReader::new(File::open(path)?);

        let mut buf = [0u8; 4];
        let mut sig_buf = [0u8; 16];

        file.read_exact(&mut sig_buf)?;
        if &sig_buf != Self::FILE_SIG {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid FILE_SIG",
            ));
        }
        file.read_exact(&mut buf)?;
        let ver = u32::from_le_bytes(buf);
        if ver != Self::FILE_VER {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid version: exp '{}', got '{ver}'", Self::FILE_VER),
            ));
        }

        file.read_exact(&mut buf)?;
        let pos_x = u32::from_le_bytes(buf);
        file.read_exact(&mut buf)?;
        let pos_y = u32::from_le_bytes(buf);

        const NUM_I: usize = CELL_SIZE * CELL_SIZE * 6; // num_quads * indices_in_quad

        let position = uvec2(pos_x, pos_y);
        let mut vertices = vec![Vec3::ZERO; NUM_V];
        let mut uvs = vec![Vec2::ZERO; NUM_V];
        let mut indices = vec![0; NUM_I];

        let vertices_bytes: &mut [u8] = cast_slice_mut(&mut vertices);
        let uvs_bytes: &mut [u8] = cast_slice_mut(&mut uvs);
        let indices_bytes: &mut [u8] = cast_slice_mut(&mut indices);
        file.read_exact(vertices_bytes)?;
        file.read_exact(uvs_bytes)?;
        file.read_exact(indices_bytes)?;

        Ok(Self {
            position,
            vertices,
            uvs,
            indices,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const fn dummy_height_fn(_: UVec2) -> f32 {
        0.0
    }

    #[test]
    fn test_vertices() {
        let cell = TerrainMesh::new(UVec2::default(), &dummy_height_fn);
        let vertices = cell.vertices();

        for z in 0..=CELL_SIZE {
            let row_off = z * (CELL_SIZE + 1);
            let zf = z as f32;
            for x in 0..=CELL_SIZE {
                let xf = x as f32;
                assert_eq!(vertices[row_off + x], vec3(xf, 0., zf),)
            }
        }
    }

    // #[test]
    // fn test_serialize_flat() {
    //     let cell = TerrainMesh::flat(vec2(69., 420.));
    // }
}
