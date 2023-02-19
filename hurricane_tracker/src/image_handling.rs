use crate::constants::{BASIN_IMAGE, IMG_HEIGHT, IMG_WIDTH};
use anyhow::{Ok, Result};
use image::imageops;
use resvg::{tiny_skia, usvg};

pub fn apply_turtle_to_atlantic_basin(svg_path: &str, png_path: &str) -> Result<()> {
    // TODO: Need to wait until the svg is actually saved. This is currently not being able to read the svg
    // Because the file is not created by the time it gets here.
    let mut opt = usvg::Options::default();
    // Get file's absolute directory.
    opt.resources_dir = std::fs::canonicalize(svg_path)?
        .parent()
        .map(|p| p.to_path_buf());

    let svg_data = std::fs::read(svg_path)?;
    let tree = usvg::Tree::from_data(&svg_data, &opt)?;

    // This is to remove the auto-fill background turtle adds
    tree.root.descendants().nth(1).unwrap().detach();

    let mut pixmap = tiny_skia::Pixmap::new(IMG_WIDTH, IMG_HEIGHT).unwrap();
    resvg::render(
        &tree,
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    );

    pixmap.save_png(png_path)?;

    // Now to place it on the basin image
    let mut basin = image::open(BASIN_IMAGE)?;
    let hurricane_path = image::open(png_path)?;

    imageops::overlay(&mut basin, &hurricane_path, 0, 0);

    basin.save(png_path)?;

    Ok(())
}
