use image::{ColorType, codecs::jpeg::JpegEncoder};

use crate::{encoder::compression::*, tags::CompressionMethod};
use std::io::Write;

// TODO add quality, tile dimension, icc_profile as properties to Jpeg struct
/// The Jpeg compression algorithm used to compress image data in TIFF files.
#[derive(Debug, Clone)]
pub struct Jpeg{
    color_type: ColorType,
    icc_profile: Option<Vec<u8>>,
    width: u32,
    height: u32,
}

/// TODO: As of now, icc_profile is not used. For our testing purpose, we don't need icc_profile
/// Add icc_profile to encoded image 
impl Jpeg {
    /// Create a new jpeg compressor with a icc_profile.
    pub fn new(color_type: ColorType, icc_profile: Option<Vec<u8>>, width: u32, height: u32) -> Self {
        Self {
            color_type,
            icc_profile: icc_profile.clone(),
            width,
            height
        }
    }
}

impl Default for Jpeg {
    fn default() -> Self {
        Self {
            color_type: ColorType::Rgb8,
            icc_profile: None,
            width: 256,
            height: 256,
        }
    }
}


impl Compression for Jpeg {
    const COMPRESSION_METHOD: CompressionMethod = CompressionMethod::JPEG;

    fn get_algorithm(&self) -> Compressor {
        Compressor::Jpeg(self.clone())
    }
}

impl CompressionAlgorithm for Jpeg {
    fn write_to<W: Write>(&mut self, writer: &mut W, bytes: &[u8]) -> Result<u64, io::Error> {

        // validate whether we received right amount of bytes 
        let mut encoded_jpeg = Vec::new();
        let mut encoder = JpegEncoder::new_with_quality(&mut encoded_jpeg, 95);
        encoder.encode(&bytes, self.width, self.height, self.color_type).unwrap();

        writer.write_all(&encoded_jpeg)?;
        writer.flush()?;

        Ok(encoded_jpeg.len() as u64)        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    #[test]
    fn test_jpeg() {
        const EXPECTED_COMPRESSED_DATA: [u8; 629] = [255, 216, 255, 224, 0, 16,
        74, 70, 73, 70, 0, 1, 2, 0, 0, 1, 0, 1, 0, 0, 255, 192, 0, 17, 8, 0, 
        2, 0, 2, 3, 1, 17, 0, 2, 17, 1, 3, 17, 1, 255, 219, 0, 67, 0, 2, 1, 1,
        1, 1, 1, 2, 1, 1, 1, 2, 2, 2, 2, 2, 4, 3, 2, 2, 2, 2, 5, 4, 4, 3, 4, 6,
        5, 6, 6, 6, 5, 6, 6, 6, 7, 9, 8, 6, 7, 9, 7, 6, 6, 8, 11, 8, 9, 10, 10,
        10, 10, 10, 6, 8, 11, 12, 11, 10, 12, 9, 10, 10, 10, 255, 219, 0, 67, 1,
        2, 2, 2, 2, 2, 2, 5, 3, 3, 5, 10, 7, 6, 7, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 255, 196, 0, 31, 0, 0, 1, 5, 1, 1, 1, 1, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 255, 196,
        0, 181, 16, 0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4, 4, 0, 0, 1, 125, 1, 2, 3,
        0, 4, 17, 5, 18, 33, 49, 65, 6, 19, 81, 97, 7, 34, 113, 20, 50, 129,
        145, 161, 8, 35, 66, 177, 193, 21, 82, 209, 240, 36, 51, 98, 114, 130,
        9, 10, 22, 23, 24, 25, 26, 37, 38, 39, 40, 41, 42, 52, 53, 54, 55, 56,
        57, 58, 67, 68, 69, 70, 71, 72, 73, 74, 83, 84, 85, 86, 87, 88, 89, 90,
        99, 100, 101, 102, 103, 104, 105, 106, 115, 116, 117, 118, 119, 120,
        121, 122, 131, 132, 133, 134, 135, 136, 137, 138, 146, 147, 148, 149,
        150, 151, 152, 153, 154, 162, 163, 164, 165, 166, 167, 168, 169, 170,
        178, 179, 180, 181, 182, 183, 184, 185, 186, 194, 195, 196, 197, 198,
        199, 200, 201, 202, 210, 211, 212, 213, 214, 215, 216, 217, 218, 225,
        226, 227, 228, 229, 230, 231, 232, 233, 234, 241, 242, 243, 244, 245,
        246, 247, 248, 249, 250, 255, 196, 0, 31, 1, 0, 3, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 255,
        196, 0, 181, 17, 0, 2, 1, 2, 4, 4, 3, 4, 7, 5, 4, 4, 0, 1, 2, 119,
        0, 1, 2, 3, 17, 4, 5, 33, 49, 6, 18, 65, 81, 7, 97, 113, 19, 34, 50,
        129, 8, 20, 66, 145, 161, 177, 193, 9, 35, 51, 82, 240, 21, 98, 114,
        209, 10, 22, 36, 52, 225, 37, 241, 23, 24, 25, 26, 38, 39, 40, 41, 42,
        53, 54, 55, 56, 57, 58, 67, 68, 69, 70, 71, 72, 73, 74, 83, 84, 85, 86, 87,
        88, 89, 90, 99, 100, 101, 102, 103, 104, 105, 106, 115, 116, 117, 118, 119,
        120, 121, 122, 130, 131, 132, 133, 134, 135, 136, 137, 138, 146, 147, 148,
        149, 150, 151, 152, 153, 154, 162, 163, 164, 165, 166, 167, 168, 169, 170,
        178, 179, 180, 181, 182, 183, 184, 185, 186, 194, 195, 196, 197, 198, 199,
        200, 201, 202, 210, 211, 212, 213, 214, 215, 216, 217, 218, 226, 227, 228,
        229, 230, 231, 232, 233, 234, 242, 243, 244, 245, 246, 247, 248, 249, 250,
        255, 218, 0, 12, 3, 1, 0, 2, 17, 3, 17, 0, 63, 0, 253, 252, 160, 15, 255, 
        217];

        let width = 2;
        let height = 2;
        let mut pixels = vec![0; width * height * 3]; // RGB image
    
        // Generate some white pixel data
        for i in 0..pixels.len() {
            pixels[i] = 255;
        }

        let mut compressed_data = Vec::<u8>::new();
        let mut writer = Cursor::new(&mut compressed_data);

        let mut jpeg_compressor = Jpeg::new(ColorType::Rgb8, None, 2, 2);
        let bytes_written = jpeg_compressor.write_to(&mut writer, &pixels).unwrap();
        
        assert_eq!(bytes_written, 629);
        assert_eq!(EXPECTED_COMPRESSED_DATA, compressed_data.as_slice());
    }
}
