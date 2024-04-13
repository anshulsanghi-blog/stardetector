use image::Rgba;
use stardetector::StarDetect;

fn main() {
    // Load the image as mutable, you need mutability so that
    // you can draw on this image.
    let mut image = image::open("m42-star-detection.jpg").unwrap();

    // Create a new star detector instance, you clone the image
    // here because you need to also draw on the image for
    // visualisation purposes in this example.
    let mut star_detector = StarDetect::from(image.clone());

    // Run the star finder function with a minimum star count of
    // 500
    let stars = star_detector.find_stars(3500);

    // Iterate over all stars you've found
    for star in stars {
        // Draw a hollow circle on the image so that you
        // can see what the algorithm found
        imageproc::drawing::draw_hollow_circle_mut(
            &mut image,
            (star.coord().x as i32, star.coord().y as i32),
            // Extend the radius by 4px so that it's easier to see
            // in the visualisation.
            star.radius() as i32 + 4,
            // Draw the circle with pure green color
            Rgba([0, u8::MAX, 0, 1]),
        );
    }

    // Save the image with star positions annotated with
    // green circles.
    image.save("annotated.jpg").unwrap();
}
