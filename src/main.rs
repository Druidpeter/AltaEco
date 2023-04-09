#![allow(dead_code)]

// use tinybmp::{RawBmp, Bpp, Header, RawPixel, RowOrder};

use embedded_graphics::prelude::*;
use tinybmp::{RawBmp, Bpp, Header, RawPixel, RowOrder};

#[derive(Clone)]
struct CellInfo{
	elevation: u32,
    slope: u32,
    orient: u32,
}

// Here's how this work: *some function* will access the supplied
// bitmap data and take samples from the data.  Once that's done, the
// samples will be used to create generator functions which will take
// x, y, t, and seed coordinates as paramters, and which will return
// semi-random values weighted heavily by the generator function in
// question.

// NOTE: The seed coordinates are needed because we run multiple
// simulation steps over specific time steps.  Sometimes, when
// performing the simulation for a specific cell, we need to (re-)
// generate values for other cells, and it's important that the
// randomly generated value remains consistent within the same step.

// These functions are then used by structs that calculate various
// properties for each cell in the AltaEco data. Each calculated
// property is THEN finally used to modulate values back inside the
// terrain data.

struct GenData{
    id: i32 // Just placeholder variable. Will be deleted.
}

struct AltaEco{
    longitude: u32,
    lattitude: u32,
	sea_level: u32,
	terrain: Vec<CellInfo>,
    // gen_data: GenData,
}

fn init_cell() -> CellInfo{
    CellInfo{
		elevation: 0,
        slope: 0,
        orient: 0,
    }
}

fn generate_terrain(long: i32, lat: i32, sea_l: u32) -> AltaEco{
    AltaEco{
        longitude: long,
        lattitude: lat,
		sea_level: sea_l,
        terrain: vec![init_cell(); (long * lat) as usize]
    }
}

fn desc_terrain(terrain: &AltaEco){
    let mut counter = 0;
    let offset = terrain.lattitude;

    while counter*offset < terrain.lattitude * terrain.longitude{
        let slice = &(terrain.terrain[(counter * offset) as usize ..(counter * offset + offset) as usize]);

        println!("Data for cells at lattitude: {}", (counter * offset));

        for i in slice.iter(){
            println!("Cell Data: ");
            println!("  Slope: {}", i.slope);
            println!("  Orientation: {}", i.orient);
        }

        counter += 1;
    }
}

// This is going to be the method that calculates rainfall and other attributes
// and calculates soil erosion directly.

fn handle_precipitation(terrain: &mut AltaEco){
        //
}

struct Fds{
	data_fd: String,
	eco_data_fd: String,
}

/// Loads initial elevation data into AltaEco object from bmp
/// file.

fn populate_elevation(terrain: Box<AltaEco>, fd: String, row_length: usize) -> Box<AltaEco>{
	let bmp = RawBmp::from_slice(include_bytes!(fd)).expect("Failed to parse bmp image.");

	// Default row order is from the bottom up. That's important.
	let mut row_count = bmp.header.image_size / row_length;

	let pixels: Vec<RawPixel> = bmp.pixels().collect();

	let mut x_pos: f64 = 0.0;
	let mut y_pos: f64 = 0.0;

	let mut index = 0;
	let mut cell_data: &mut CellInfo;
	
	for pixelData in pixels{
        // Calculate percent position in image and terrain data.
        x_pos = pixelData.position.x / row_length;
        y_pos = pixelData.position.y / row_count;

		index = (terrain.longitude * x_pos + (terrain.lattitude * y_pos * terrain.longitude)) as usize;
		cell_data = &mut terrain.terrain[index];
		
		if cell_data.elevation == 0 {
			cell_data.elevation = pixelData.color;
		} else {
			cell_data.elevation = (cell_data.elevation + pixelData.color) / 2;
		}
	}

	return terrain;
}

fn main(){
    println!("Alta Eco v0.1a");

    let map_width: u32 = 60;
    let map_height: u32 = 40;
	let sea_level: u32 = 0;

    let mut terrain: Box<AltaEco> = Box::new(generate_terrain(map_width, map_height, sea_level));
    desc_terrain(&terrain);

	let fds = Fds{
		data_fd: String::from("data.bmp"),
		eco_data_fd: String::from("eco_data.bmp")
	};

	terrain = populate_elevation(terrain, fds.data_fd);
}
