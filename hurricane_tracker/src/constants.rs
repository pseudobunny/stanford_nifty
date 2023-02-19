pub const LAT_BOUNDS: (f64, f64) = (0.0, 45.0);
pub const LON_BOUNDS: (f64, f64) = (-90.0, -17.66);
pub const IMG_WIDTH: u32 = 965;
pub const IMG_HEIGHT: u32 = 600;

pub const BASIN_IMAGE: &str = "./images/atlantic-basin.png";

// Derived constants
pub const LAT_CENTER: f64 = (LAT_BOUNDS.0 + LAT_BOUNDS.1) * 0.5;
pub const LON_CENTER: f64 = (LON_BOUNDS.0 + LON_BOUNDS.1) * 0.5;
pub const LAT_HEIGHT_HALF: f64 = (LAT_BOUNDS.1 - LAT_BOUNDS.0) * 0.5;
pub const LON_WIDTH_HALF: f64 = (LON_BOUNDS.1 - LON_BOUNDS.0) * 0.5;
pub const IMG_WIDTH_HALF: f64 = IMG_WIDTH as f64 * 0.5;
pub const IMG_HEIGHT_HALF: f64 = IMG_HEIGHT as f64 * 0.5;

// Color constants
pub const CATEGORY_COLOR: [&str; 6] = ["white", "purple", "green", "yellow", "orange", "red"];
