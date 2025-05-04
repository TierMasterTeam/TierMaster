use image::{DynamicImage, GenericImageView};
use image::imageops::FilterType;

pub trait ImageHelper {
    /// Crop the image to a centered 1:1 square.
    /// Returns the cropped image.
    fn crop_to_square(&self) -> Self;

    /// Resize the image to fit within `size`, maintaining the aspect ratio.
    /// Returns the resized image
    fn resize_to_fit(&self, size: u32) -> Self;
}

impl ImageHelper for DynamicImage {
    fn crop_to_square(&self) -> DynamicImage {
        let (width, height) = self.dimensions();

        let crop_size = width.min(height);
        let crop_x = (width - crop_size) / 2;
        let crop_y = (height - crop_size) / 2;

        self.crop_imm(crop_x, crop_y, crop_size, crop_size)
    }
    
    fn resize_to_fit(&self, size: u32) -> DynamicImage {
        let (width, height) = self.dimensions();
        
        if width > size || height > size {
            return if width < height {
                self.resize(size, u32::MAX, FilterType::Lanczos3)
            } else {
                self.resize(u32::MAX, size, FilterType::Lanczos3)
            }
        }
       self.to_owned()
    }
}