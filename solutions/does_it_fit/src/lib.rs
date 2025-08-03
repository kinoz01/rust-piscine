mod areas_volumes;
pub use areas_volumes::*;
use crate::areas_volumes::{ self as av};
use GeometricalShapes::*;
use GeometricalVolumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize)
) -> bool {
    let rect_area = (x * y) as f64;

    let shape_area = match kind {
        Square => av::square_area(a) as f64,
        Circle => av::circle_area(a),
        Rectangle => av::rectangle_area(a, b) as f64,
        Triangle => av::triangle_area(a, b),
    };

    shape_area * (times as f64) <= rect_area
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize)
) -> bool {
    let box_vol = (x * y * z) as f64;

    let shape_vol = match kind {
        Cube => av::cube_volume(a) as f64,
        Sphere => av::sphere_volume(a),
        Cone => av::cone_volume(a, b),
        TriangularPyramid => av::triangular_pyramid_volume(a as f64, b),
        Parallelepiped => av::parallelepiped_volume(a, b, c) as f64,
    };

    shape_vol * (times as f64) <= box_vol
}
