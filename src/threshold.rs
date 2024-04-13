use crate::centroid::find_star_centres_and_size;
use crate::StarDetect;

pub trait ThresholdingExtensions {
    fn optimize_threshold_for_star_count(&self, min_star_count: usize) -> u8;
    fn binarize(&mut self, threshold: u8);
}

impl ThresholdingExtensions for StarDetect {
    fn optimize_threshold_for_star_count(&self, min_star_count: usize) -> u8 {
        // Current star count
        let mut star_count = 0;

        // Starting threshold value
        let mut threshold = u8::MAX;

        // Iterate until you've found the best threshold
        while star_count < min_star_count {
            // Panic if we reach the 0 intensity value while iterating.
            // This means that there are lesser stars than we hoped for.
            if threshold == 0 {
                panic!("Maximum iteration count reached");
            }

            // Reduce threshold to 95% of its previous value.
            // Using this, we check finer and finer differences
            // in threshold for each iteration.
            threshold = (0.95 * threshold as f32) as u8;

            // Clone the source data since we need to modify it
            // without affecting original data.
            let mut source = self.clone();

            // Binarize the source data image using current threshold
            ThresholdingExtensions::binarize(&mut source, threshold);

            // Find the number of stars detected with the current threshold
            star_count = find_star_centres_and_size(&source.source).len();
        }

        threshold
    }

    fn binarize(&mut self, threshold: u8) {
        // Iterate over every pixel in source image
        for pixel in self.source.iter_mut() {
            if *pixel > threshold {
                // If pixel intensity is greater than threshold
                // set it to maximum intensity instead.
                *pixel = u8::MAX;
            } else {
                // Otherwise, set it to 0 intensity.
                *pixel = 0;
            }
        }
    }
}
