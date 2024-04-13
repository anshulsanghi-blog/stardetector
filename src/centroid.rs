use geo::{Centroid, Coord, EuclideanDistance, LineString};
use image::GrayImage;
use imageproc::contours::Contour;
use imageproc::point::Point;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct StarCenter {
    coord: Point<u32>,
    radius: u32,
}

impl StarCenter {
    pub fn coord(&self) -> &Point<u32> {
        &self.coord
    }
    pub fn radius(&self) -> u32 {
        self.radius
    }
}

pub(crate) fn construct_closed_polygon(contour: &Contour<u32>) -> LineString<f32> {
    // Create a new line string that connects all points
    // in the contour. This can create either an open
    // or a closed shape.
    let mut line_string = LineString::from_iter(contour.points.iter().map(|point| Coord {
        x: point.x as f32,
        y: point.y as f32,
    }));

    // If it is an open shape, close the shape to create a
    // polygon. This does nothing otherwise.
    line_string.close();

    line_string
}

pub(crate) fn filter_map_contour_to_star_centers(contour: &Contour<u32>) -> Option<StarCenter> {
    // If there are no points in the contour
    // it is not a star.
    if contour.points.is_empty() {
        return None;
    }

    if contour.points.len() == 1 {
        // If there's only 1 point in the contour
        // consider it to be the center of the star
        // of size 1px.
        let center = contour.points.first().unwrap();
        let radius = 1_u32;

        return Some(StarCenter {
            coord: *center,
            radius,
        });
    }

    // Otherwise, construct a polygon around the star based on
    // contour information.
    let polygon = construct_closed_polygon(contour);

    // Find the centre of gravity of this polygon (centroid)
    let center = polygon.centroid().unwrap();

    // Find the radius of the star based on maximum distance between
    // the centroid and any of the points in contour.
    let radius = polygon.points().fold(0., |distance, point| {
        point.euclidean_distance(&center).max(distance)
    });

    // If the radius is less than 1p or more than 24px
    // we reject it as a non-star.
    if !(1. ..=24.).contains(&radius) {
        return None;
    }

    // Construct star center based on previously computed information
    Some(StarCenter {
        coord: Point {
            x: center.x() as u32,
            y: center.y() as u32,
        },
        radius: radius as u32,
    })
}

pub(crate) fn find_star_centres_and_size(image: &GrayImage) -> Vec<StarCenter> {
    // Compute the contours in source image
    let contours = imageproc::contours::find_contours::<u32>(image);

    contours
        .iter()
        // Iterate over all contours and create a list
        // of star center and size data.
        .filter_map(filter_map_contour_to_star_centers)
        .collect()
}
