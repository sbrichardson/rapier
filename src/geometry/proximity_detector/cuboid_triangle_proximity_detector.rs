use crate::geometry::proximity_detector::PrimitiveProximityDetectionContext;
use crate::geometry::{sat, Cuboid, Proximity, Shape, Triangle};
use crate::math::Isometry;

pub fn detect_proximity_cuboid_triangle(
    ctxt: &mut PrimitiveProximityDetectionContext,
) -> Proximity {
    if let (Shape::Cuboid(cube1), Shape::Triangle(triangle2)) = (ctxt.shape1, ctxt.shape2) {
        detect_proximity(
            ctxt.prediction_distance,
            cube1,
            ctxt.position1,
            triangle2,
            ctxt.position2,
        )
    } else if let (Shape::Triangle(triangle1), Shape::Cuboid(cube2)) = (ctxt.shape1, ctxt.shape2) {
        detect_proximity(
            ctxt.prediction_distance,
            cube2,
            ctxt.position2,
            triangle1,
            ctxt.position1,
        )
    } else {
        panic!("Invalid shape types")
    }
}

pub fn detect_proximity<'a>(
    prediction_distance: f32,
    cube1: &'a Cuboid,
    pos1: &'a Isometry<f32>,
    triangle2: &'a Triangle,
    pos2: &'a Isometry<f32>,
) -> Proximity {
    let pos12 = pos1.inverse() * pos2;
    let pos21 = pos12.inverse();

    /*
     *
     * Point-Face cases.
     *
     */
    let sep1 =
        sat::cube_support_map_find_local_separating_normal_oneway(cube1, triangle2, &pos12).0;
    if sep1 > prediction_distance {
        return Proximity::Disjoint;
    }

    let sep2 = sat::triangle_cuboid_find_local_separating_normal_oneway(triangle2, cube1, &pos21).0;
    if sep2 > prediction_distance {
        return Proximity::Disjoint;
    }

    /*
     *
     * Edge-Edge cases.
     *
     */
    #[cfg(feature = "dim2")]
    let sep3 = -f32::MAX; // This case does not exist in 2D.
    #[cfg(feature = "dim3")]
    let sep3 =
        sat::cube_triangle_find_local_separating_edge_twoway(cube1, triangle2, &pos12, &pos21).0;
    if sep3 > prediction_distance {
        return Proximity::Disjoint;
    }

    if sep2 > sep1 && sep2 > sep3 {
        if sep2 > 0.0 {
            Proximity::WithinMargin
        } else {
            Proximity::Intersecting
        }
    } else if sep3 > sep1 {
        if sep3 > 0.0 {
            Proximity::WithinMargin
        } else {
            Proximity::Intersecting
        }
    } else {
        if sep1 > 0.0 {
            Proximity::WithinMargin
        } else {
            Proximity::Intersecting
        }
    }
}
