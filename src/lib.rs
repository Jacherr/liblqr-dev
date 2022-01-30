#[allow(dead_code)]
#[allow(deref_nullptr)]
pub mod bindings;

#[cfg(test)]
mod tests {
    use std::{ptr, time::Instant};

    use image::{load_from_memory, DynamicImage, GenericImage, ImageBuffer, Rgba};

    use crate::bindings::{
        _LqrRetVal_LQR_OK, gint, lqr_carver_destroy, lqr_carver_flatten, lqr_carver_init,
        lqr_carver_new, lqr_carver_resize, lqr_carver_scan, lqr_carver_scan_by_row,
        lqr_carver_scan_line, lqr_carver_scan_reset, lqr_carver_set_preserve_input_image,
    };

    #[test]
    fn it_works() {
        let image = load_from_memory(include_bytes!("../test/otter.png"))
            .expect("Failed to read image")
            .to_rgba8();

        let start = Instant::now();

        let raw = image.as_raw().as_ptr();

        unsafe {
            let carver = lqr_carver_new(raw as *mut u8, image.width() as _, image.height() as _, 4);
            assert_eq!(lqr_carver_init(carver, 1, 1.0), _LqrRetVal_LQR_OK as _);
            assert_eq!(lqr_carver_resize(carver, 407, 407), _LqrRetVal_LQR_OK as _);
            assert_eq!(lqr_carver_resize(carver, 814, 814), _LqrRetVal_LQR_OK as _);
            lqr_carver_set_preserve_input_image(carver);
            lqr_carver_scan_reset(carver);
            let mut x: gint = 0;
            let mut y: gint = 0;
            let mut rgba: *mut u8 = 1 as *mut u8;
            //println!("a");
            let mut img = ImageBuffer::<Rgba<u8>, _>::new(814, 814);
            while lqr_carver_scan(carver, &mut x, &mut y, &mut rgba) != 0 {
                img.unsafe_put_pixel(
                    x as _,
                    y as _,
                    Rgba([*rgba, *rgba.add(1), *rgba.add(2), *rgba.add(3)]),
                )
            }
            println!("{:?}", start.elapsed().as_millis());
            img.save("./test/otter2.png").expect("Failed to save image");
            std::mem::forget(img);
            std::mem::forget(carver);
            std::mem::forget(image);
            lqr_carver_destroy(carver);
        }
    }
}
