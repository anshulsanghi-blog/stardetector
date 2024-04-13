mod centroid;
pub mod threshold;

use crate::centroid::{find_star_centres_and_size, StarCenter};
use crate::threshold::ThresholdingExtensions;
use image::{DynamicImage, GrayImage};
use image_dwt::kernels::LinearInterpolationKernel;
use image_dwt::recompose::{OutputLayer, RecomposableWaveletLayers};
use image_dwt::transform::ATrousTransform;

#[derive(Clone)]
pub struct StarDetect {
    source: GrayImage,
}

impl From<DynamicImage> for StarDetect {
    fn from(source: DynamicImage) -> Self {
        Self {
            source: source.to_luma8(),
        }
    }
}

impl StarDetect {
    fn extract_small_scale_structures(&mut self) {
        let (width, height) = self.source.dimensions();

        // Decompose the image into 8 layers
        let filtered_image = ATrousTransform::new(
            &DynamicImage::ImageLuma8(self.source.clone()),
            8,
            LinearInterpolationKernel,
        )
        // Filter out the residue image and keep the rest
        .filter(|item| item.pixel_scale.is_some())
        // Recompose the first 3 layers into a grayscale image.
        .recompose_into_image(width as usize, height as usize, OutputLayer::Grayscale);

        // Update the source image that we will work with
        // going forward.
        self.source = filtered_image.to_luma8();
    }

    fn apply_noise_reduction(&mut self) {
        self.source = imageproc::filter::bilateral_filter(&self.source, 10, 10., 3.);
    }

    pub fn find_stars(&mut self, min_stars: usize) -> Vec<StarCenter> {
        self.extract_small_scale_structures();
        self.apply_noise_reduction();

        let threshold = self.optimize_threshold_for_star_count(min_stars);
        self.binarize(threshold);

        find_star_centres_and_size(&self.source)
    }
}
