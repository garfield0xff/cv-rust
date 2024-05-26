pub struct KeyPoint {
    pub point: (f32, f32)
}



/// Image opening and processing/manipulation
pub mod image {
    /// Re-export of [`image`] to open and save images
    #[cfg(feature = "image")]
    #[allow(clippy::module_inception)]
    pub mod image {
        pub use image::*;
    }

    /// Re-export of [`imageproc`] crate for image manipulation routines
    #[cfg(feature = "imageproc")]
    pub mod imageproc {
        pub use imageproc::*;
    }

    /// Re-export of [`ndarray-vision`] for image manipulation routines
    #[cfg(feature = "ndarray-vision")]
    pub mod ndarray_vision {
        pub use ndarray_vision::*;
    }
}