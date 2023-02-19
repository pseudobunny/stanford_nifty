use anyhow::{anyhow, Result};
use turtle::Turtle;

mod constants;
mod hurricane_data;
mod image_handling;

use constants::{CATEGORY_COLOR, IMG_HEIGHT, IMG_WIDTH};
use image_handling::apply_turtle_to_atlantic_basin;

fn main() -> Result<()> {
    turtle::start();

    let args: Vec<String> = std::env::args().collect();
    let hurricane_name = &args[1];

    let mut turtle = Turtle::new();

    // TODO: We have multiple sets of data - name will probably come from the
    // commandline argument provided to choose the data
    let turtle_drawing = turtle.drawing_mut();
    turtle_drawing.set_title(&format!("Hurricane {hurricane_name}"));
    turtle_drawing.set_size((IMG_WIDTH, IMG_HEIGHT));

    let data_path = format!("./data/{hurricane_name}.csv");
    let hurricane_data_points = hurricane_data::read_csv_data_from_file(&data_path)?;

    if hurricane_data_points.is_empty() {
        return Err(anyhow!("No hurricane data points to display."));
    }

    // Set initial position
    turtle.pen_up();
    turtle.go_to(hurricane_data_points[0].translate_to_turtle_grid()?);
    turtle.pen_down();

    // Draw the rest of the path
    for point in hurricane_data_points.iter().skip(1) {
        let category = point.determine_category();

        turtle.set_pen_color(CATEGORY_COLOR[category]);
        turtle.set_pen_size(category as f64 + 1.0);

        turtle.go_to(point.translate_to_turtle_grid()?);
    }

    let svg_path = format!("./output_images/{hurricane_name}.svg");
    turtle.drawing().save_svg(&svg_path);

    let png_path = format!("./output_images/{hurricane_name}.png");
    apply_turtle_to_atlantic_basin(&svg_path, &png_path)?;

    Ok(())
}
