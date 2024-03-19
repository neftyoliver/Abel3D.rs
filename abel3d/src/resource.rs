
pub mod resource {
    use std::sync::Arc;
    use crate::citadel::render::{AbelResult, Renderer};

    pub const ABEL_IMAGE_SIZE_2: i32 = 0b0;
    pub const ABEL_IMAGE_SIZE_4: i32 = 0b01;
    pub const ABEL_IMAGE_SIZE_8: i32 = 0b001;
    pub const ABEL_IMAGE_SIZE_16: i32 = 0b0001;
    pub const ABEL_IMAGE_SIZE_32: i32 = 0b00001;
    pub const ABEL_IMAGE_SIZE_64: i32 = 0b000001;
    pub const ABEL_IMAGE_SIZE_128: i32 = 0b0000001;
    pub const ABEL_IMAGE_SIZE_265: i32 = 0b00000001;
    pub const ABEL_IMAGE_SIZE_512: i32 = 0b000000001;
    pub const ABEL_IMAGE_SIZE_1042: i32 = 0b000000001;
    pub const ABEL_IMAGE_SIZE_2048: i32 = 0b0000000001;
    pub const ABEL_IMAGE_SIZE_4096: i32 = 0b00000000001;

    pub trait Resource {
        fn get_arc() -> Arc<impl Resource>;
        fn get_size() -> u64; // 1 = 1byte

        fn upload(renderer: impl Renderer) -> AbelResult;
    }

    pub struct Dim3D64F {
        x: f64,
        y: f64,
        z: f64
    }
    pub struct PixelU8RGBA {
        r: u8,
        g: u8,
        b: u8,
        a: u8
    }

    pub struct ImageDim {
        x: i32,
        y: i32
    }

    pub struct Image {
        size: ImageDim,
        data: [PixelU8RGBA]

    }


}
