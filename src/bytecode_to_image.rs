use crate::opcode::{OpCode, OpCodeInfo};
use image::{ImageBuffer, Rgb};
use std::str::FromStr;
use tracing::debug;

/// Transform one bytecode to image
pub fn transform_bytecode_to_image(
    bytecode: &str,
    size: u32,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, hex::FromHexError> {
    // remove the prefix '0x' (if present)
    let bytecode = if bytecode.starts_with("0x") || bytecode.starts_with("0X") {
        &bytecode[2..]
    } else {
        bytecode
    };

    // we assume here that the output images are square, but we retain the length and width,
    // as we may use non-square images in the future.
    let (width, height) = (size, size);

    let mut img = ImageBuffer::new(width, height);
    let mut i = 0;
    // map byte arrays to RGB colors
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let index = (y * width + x) as usize;
        if index < bytecode.len() / 2 {
            let opcode = map_to_opcode(&bytecode[i..i + 2]);
            let opcode_info: OpCodeInfo = opcode.into();
            *pixel = Rgb(opcode_info.rgb_color);
        } else {
            // if the byte array is insufficient, fill in the default color (black)
            *pixel = Rgb([0, 0, 0]);
        }
        i = i + 2;
    }

    debug!("Succeed to transform bytecode to image, size: {}", size);

    Ok(img)
}

/// Convert hexadecimal string to OpCode
fn map_to_opcode(num: &str) -> OpCode {
    // Convert all bytecode to uppercase
    let num = num.to_uppercase();
    let opcode = OpCode::from_str(num.as_str()).unwrap();

    return opcode.into();
}

/// Based on the length of the bytecode, select the side length of a square of appropriate size.
/// This square serves as the template for the final image.
pub fn cal_appropriate_size(x: usize) -> u32 {
    let temp = x as f64;
    let sqrt_value = temp.sqrt();
    let result = sqrt_value.ceil() as u32;
    result
}
