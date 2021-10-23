//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_thumbnail() {
    let generated_thumbnail =
        thumbo_core::thumbnail(sample_image(), thumbo_core::ImageFormat::Png, 10, 10);
    assert_eq!(generated_thumbnail, expected_thumbnail());
}

fn sample_image() -> Vec<u8> {
    vec![
        137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 20, 0, 0, 0, 20, 8,
        6, 0, 0, 0, 141, 137, 29, 13, 0, 0, 4, 9, 73, 68, 65, 84, 120, 156, 117, 84, 91, 104, 156,
        69, 20, 62, 103, 46, 187, 155, 38, 134, 94, 18, 80, 43, 104, 49, 168, 77, 75, 90, 17, 77,
        140, 86, 176, 49, 9, 54, 152, 96, 74, 240, 210, 74, 125, 104, 154, 226, 37, 182, 96, 11,
        86, 145, 86, 65, 68, 173, 216, 23, 5, 31, 84, 170, 129, 8, 125, 168, 151, 135, 88, 16, 21,
        53, 197, 135, 62, 73, 68, 164, 180, 98, 139, 4, 109, 75, 218, 36, 187, 249, 255, 127, 230,
        28, 191, 127, 163, 9, 210, 246, 99, 207, 158, 217, 57, 231, 124, 243, 205, 153, 153, 229,
        3, 253, 183, 211, 149, 192, 74, 245, 68, 186, 76, 178, 208, 41, 34, 169, 183, 70, 217, 218,
        90, 99, 237, 23, 108, 120, 26, 25, 151, 144, 118, 25, 248, 157, 45, 119, 193, 45, 130, 137,
        110, 214, 40, 231, 178, 217, 202, 233, 52, 200, 47, 89, 212, 118, 67, 52, 149, 145, 49, 98,
        77, 61, 51, 255, 80, 112, 166, 185, 224, 205, 170, 98, 209, 55, 50, 211, 41, 148, 45, 128,
        95, 233, 91, 15, 55, 15, 4, 75, 38, 100, 175, 133, 36, 236, 140, 206, 213, 128, 132, 132,
        248, 156, 18, 191, 173, 68, 203, 145, 178, 77, 149, 26, 13, 86, 101, 210, 74, 193, 208,
        187, 214, 242, 75, 88, 100, 14, 177, 42, 120, 127, 239, 58, 184, 121, 112, 146, 254, 29,
        131, 44, 73, 173, 43, 5, 227, 62, 0, 209, 65, 76, 159, 12, 196, 31, 129, 120, 10, 227, 97,
        216, 109, 32, 219, 3, 206, 109, 142, 181, 226, 73, 166, 138, 158, 87, 98, 190, 10, 126, 97,
        83, 11, 97, 197, 235, 209, 171, 55, 89, 116, 83, 106, 114, 50, 251, 8, 66, 159, 35, 126,
        85, 8, 81, 95, 36, 30, 45, 176, 150, 177, 171, 49, 180, 97, 55, 166, 255, 226, 93, 29, 107,
        32, 159, 26, 177, 210, 68, 102, 124, 99, 102, 93, 6, 53, 23, 81, 64, 80, 8, 3, 20, 25, 139,
        168, 96, 250, 40, 252, 62, 68, 123, 172, 202, 104, 41, 36, 127, 70, 162, 22, 100, 157, 231,
        221, 221, 45, 222, 197, 112, 74, 132, 110, 72, 216, 81, 240, 160, 6, 65, 78, 150, 35, 39,
        172, 142, 241, 169, 2, 19, 121, 20, 248, 20, 182, 21, 7, 54, 89, 138, 201, 138, 32, 242,
        59, 14, 170, 137, 159, 237, 88, 187, 218, 144, 190, 7, 22, 40, 116, 51, 193, 21, 47, 32,
        125, 8, 182, 74, 216, 80, 36, 243, 22, 10, 143, 193, 88, 200, 108, 135, 31, 200, 9, 113,
        92, 147, 168, 27, 54, 42, 77, 216, 242, 29, 150, 100, 169, 83, 121, 134, 135, 55, 174, 233,
        194, 233, 126, 150, 9, 79, 71, 231, 199, 197, 216, 253, 32, 26, 9, 100, 155, 97, 132, 241,
        135, 80, 120, 28, 68, 184, 154, 188, 25, 11, 117, 129, 40, 87, 249, 58, 252, 152, 215, 176,
        215, 197, 164, 205, 73, 44, 89, 9, 189, 60, 220, 177, 118, 72, 85, 15, 101, 108, 207, 42,
        155, 38, 20, 82, 197, 213, 76, 100, 236, 155, 161, 14, 205, 207, 13, 229, 255, 110, 25,
        172, 249, 175, 31, 49, 220, 0, 195, 47, 50, 117, 97, 230, 15, 47, 233, 242, 162, 198, 93,
        188, 115, 99, 203, 189, 208, 241, 101, 74, 230, 34, 106, 110, 68, 2, 205, 20, 234, 23, 9,
        153, 143, 65, 207, 132, 18, 53, 192, 111, 65, 14, 46, 5, 70, 164, 3, 72, 61, 98, 53, 238,
        45, 134, 185, 151, 11, 113, 46, 65, 247, 123, 249, 169, 206, 245, 131, 38, 134, 247, 19,
        50, 34, 236, 186, 177, 197, 100, 214, 213, 30, 70, 63, 111, 202, 9, 81, 250, 34, 10, 143,
        130, 240, 52, 198, 159, 96, 220, 15, 210, 156, 240, 27, 248, 3, 94, 178, 17, 23, 211, 149,
        80, 72, 53, 28, 183, 243, 214, 206, 214, 82, 173, 36, 35, 164, 177, 127, 214, 213, 81, 106,
        139, 148, 247, 46, 224, 64, 170, 91, 173, 146, 86, 209, 10, 203, 183, 153, 31, 210, 66, 4,
        10, 201, 103, 115, 228, 41, 142, 66, 250, 147, 60, 212, 125, 167, 247, 33, 249, 41, 144,
        174, 155, 197, 203, 171, 20, 234, 64, 102, 41, 87, 55, 79, 8, 29, 243, 200, 9, 39, 97, 39,
        96, 13, 48, 100, 128, 84, 2, 21, 98, 42, 181, 148, 158, 40, 26, 185, 155, 31, 237, 186, 39,
        47, 41, 213, 199, 217, 51, 101, 117, 13, 101, 95, 119, 68, 172, 251, 26, 100, 231, 81, 147,
        95, 151, 255, 48, 3, 139, 176, 2, 20, 111, 134, 119, 170, 252, 144, 9, 201, 64, 81, 195,
        164, 53, 213, 254, 167, 60, 216, 221, 10, 66, 93, 65, 34, 143, 39, 194, 251, 202, 236, 175,
        117, 214, 62, 55, 107, 74, 29, 145, 109, 31, 146, 46, 131, 18, 125, 133, 239, 239, 57, 75,
        95, 173, 215, 185, 179, 69, 150, 55, 188, 161, 143, 17, 154, 226, 193, 158, 118, 248, 121,
        224, 130, 126, 151, 70, 186, 14, 5, 191, 93, 176, 215, 108, 8, 198, 31, 34, 230, 49, 132,
        78, 194, 176, 17, 106, 34, 213, 30, 82, 217, 225, 99, 122, 28, 29, 191, 5, 111, 249, 140,
        115, 252, 0, 98, 85, 240, 142, 7, 219, 224, 22, 193, 170, 7, 51, 209, 246, 25, 113, 109,
        101, 46, 224, 254, 89, 82, 99, 33, 8, 175, 27, 239, 211, 224, 34, 150, 52, 208, 18, 77,
        199, 139, 150, 198, 141, 53, 123, 80, 182, 0, 126, 250, 225, 251, 224, 254, 7, 48, 168, 4,
        209, 111, 67, 212, 95, 67, 38, 143, 149, 201, 93, 2, 153, 150, 56, 46, 195, 159, 193, 97,
        231, 237, 106, 239, 237, 253, 248, 231, 198, 74, 213, 190, 46, 128, 159, 127, 162, 27, 238,
        170, 88, 10, 187, 21, 92, 120, 166, 56, 111, 200, 179, 214, 252, 140, 185, 105, 216, 21,
        241, 15, 105, 7, 201, 25, 58, 13, 132, 193, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
    ]
}

fn expected_thumbnail() -> Vec<u8> {
    vec![
        137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 10, 0, 0, 0, 10, 8,
        6, 0, 0, 0, 141, 50, 207, 189, 0, 0, 1, 100, 73, 68, 65, 84, 120, 156, 37, 144, 205, 43,
        69, 97, 16, 198, 103, 222, 143, 115, 78, 156, 43, 107, 91, 43, 223, 138, 194, 66, 201, 37,
        178, 177, 144, 20, 133, 44, 44, 40, 81, 148, 162, 36, 27, 254, 1, 59, 11, 69, 148, 178, 83,
        202, 74, 162, 236, 124, 172, 100, 45, 155, 187, 209, 189, 220, 123, 206, 121, 223, 25, 115,
        121, 106, 86, 243, 123, 102, 230, 25, 60, 152, 234, 134, 63, 49, 231, 92, 37, 93, 206, 50,
        223, 13, 74, 161, 182, 230, 49, 8, 204, 161, 86, 88, 146, 46, 224, 238, 120, 39, 32, 81,
        191, 79, 179, 227, 68, 217, 23, 7, 234, 213, 3, 86, 141, 29, 22, 185, 37, 50, 48, 43, 190,
        123, 220, 24, 105, 139, 21, 209, 107, 69, 7, 5, 1, 46, 184, 10, 253, 235, 147, 1, 114, 1,
        185, 117, 3, 212, 138, 107, 67, 173, 235, 158, 96, 162, 98, 163, 58, 2, 213, 236, 17, 175,
        100, 81, 36, 96, 94, 1, 109, 89, 239, 198, 12, 103, 151, 184, 146, 111, 57, 75, 193, 148,
        51, 19, 188, 149, 85, 180, 239, 192, 220, 18, 66, 136, 12, 77, 10, 120, 48, 164, 100, 181,
        54, 251, 214, 184, 148, 111, 63, 201, 24, 63, 18, 19, 21, 75, 38, 222, 147, 27, 183, 101,
        125, 131, 76, 92, 18, 112, 50, 244, 201, 78, 236, 74, 79, 184, 56, 220, 181, 224, 152, 143,
        190, 108, 61, 164, 104, 192, 131, 18, 6, 6, 0, 224, 70, 64, 27, 248, 4, 114, 84, 158, 199,
        185, 145, 190, 80, 187, 244, 177, 104, 226, 167, 68, 135, 167, 50, 237, 78, 160, 68, 106,
        152, 153, 102, 98, 247, 211, 22, 42, 234, 197, 197, 177, 190, 234, 43, 58, 189, 167, 131,
        2, 212, 54, 102, 218, 94, 131, 196, 213, 236, 71, 235, 168, 252, 94, 99, 120, 19, 17, 158,
        113, 117, 42, 47, 230, 170, 24, 61, 241, 116, 234, 160, 135, 136, 49, 212, 240, 160, 173,
        58, 71, 144, 88, 162, 95, 144, 171, 152, 109, 25, 139, 176, 105, 0, 0, 0, 0, 73, 69, 78,
        68, 174, 66, 96, 130,
    ]
}
