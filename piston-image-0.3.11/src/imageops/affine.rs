//! Functions for performing affine transformations.

use buffer::{ImageBuffer, Pixel};
use image::GenericImage;

/// Rotate an image 90 degrees clockwise.
// TODO: Is the 'static bound on `I` really required? Can we avoid it?
pub fn rotate90<I: GenericImage + 'static>(image:  &I)
    -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
    where I::Pixel: 'static,
          <I::Pixel as Pixel>::Subpixel: 'static {
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(height, width);

    for y in (0..height) {
        for x in (0..width) {
            let p = image.get_pixel(x, y);
            out.put_pixel(height - 1 - y, x, p);
        }
    }

    out
}

/// Rotate an image 180 degrees clockwise.
// TODO: Is the 'static bound on `I` really required? Can we avoid it?
pub fn rotate180<I: GenericImage + 'static>(image:  &I)
    -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
    where I::Pixel: 'static,
          <I::Pixel as Pixel>::Subpixel: 'static {
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);

    for y in (0..height) {
        for x in (0..width) {
            let p = image.get_pixel(x, y);
            out.put_pixel(width - 1 - x, height - 1 - y, p);
        }
    }

    out
}

/// Rotate an image 270 degrees clockwise.
// TODO: Is the 'static bound on `I` really required? Can we avoid it?
pub fn rotate270<I: GenericImage + 'static>(image:  &I)
    -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
    where I::Pixel: 'static,
          <I::Pixel as Pixel>::Subpixel: 'static {
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(height, width);

    for y in (0..height) {
        for x in (0..width) {
            let p = image.get_pixel(x, y);
            out.put_pixel(y, width - 1 - x, p);
        }
    }

    out
}

/// Flip an image horizontally
// TODO: Is the 'static bound on `I` really required? Can we avoid it?
pub fn flip_horizontal<I: GenericImage + 'static>(image:  &I)
    -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
    where I::Pixel: 'static,
          <I::Pixel as Pixel>::Subpixel: 'static {
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(height, width);

    for y in (0..height) {
        for x in (0..width) {
            let p = image.get_pixel(x, y);
            out.put_pixel(width - 1 - x, y, p);
        }
    }

    out
}

/// Flip an image vertically
// TODO: Is the 'static bound on `I` really required? Can we avoid it?
pub fn flip_vertical<I: GenericImage + 'static>(image:  &I)
    -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
    where I::Pixel: 'static,
          <I::Pixel as Pixel>::Subpixel: 'static {
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);

    for y in (0..height) {
        for x in (0..width) {
            let p = image.get_pixel(x, y);
            out.put_pixel(x, height - 1 - y, p);
        }
    }

    out
}
