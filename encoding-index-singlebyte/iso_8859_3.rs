// AUTOGENERATED FROM index-iso-8859-3.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index index-iso-8859-3.txt see the Encoding Standard
// https://encoding.spec.whatwg.org/
//
// Identifier: af8f1e12df79b768322b5e83613698cdc619438270a2fc359554331c805054a3
// Date: 2014-12-19

static FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 294, 728, 163, 164, 65535, 292, 167, 168, 304, 350, 286,
    308, 173, 65535, 379, 176, 295, 178, 179, 180, 181, 293, 183, 184, 305,
    351, 287, 309, 189, 65535, 380, 192, 193, 194, 65535, 196, 266, 264, 199,
    200, 201, 202, 203, 204, 205, 206, 207, 65535, 209, 210, 211, 212, 288,
    214, 215, 284, 217, 218, 219, 220, 364, 348, 223, 224, 225, 226, 65535,
    228, 267, 265, 231, 232, 233, 234, 235, 236, 237, 238, 239, 65535, 241,
    242, 243, 244, 289, 246, 247, 285, 249, 250, 251, 252, 365, 349, 729,
];

/// Returns the index code point for pointer `code` in this index.
#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as usize]
}

static BACKWARD_TABLE_LOWER: &'static [u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137,
    138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152,
    153, 154, 155, 156, 157, 158, 159, 160, 0, 0, 163, 164, 0, 0, 167, 168, 0,
    0, 0, 0, 173, 0, 0, 176, 0, 178, 179, 180, 181, 0, 183, 184, 0, 0, 0, 0,
    189, 0, 0, 192, 193, 194, 0, 196, 0, 0, 199, 200, 201, 202, 203, 204, 205,
    206, 207, 0, 209, 210, 211, 212, 0, 214, 215, 0, 217, 218, 219, 220, 0, 0,
    223, 224, 225, 226, 0, 228, 0, 0, 231, 232, 233, 234, 235, 236, 237, 238,
    239, 0, 241, 242, 243, 244, 0, 246, 247, 0, 249, 250, 251, 252, 0, 0, 0,
    198, 230, 197, 229, 0, 0, 0, 0, 0, 0, 0, 0, 216, 248, 171, 187, 213, 245,
    0, 0, 166, 182, 161, 177, 169, 185, 0, 0, 172, 188, 0, 0, 0, 0, 0, 0, 222,
    254, 170, 186, 0, 0, 0, 0, 221, 253, 0, 0, 0, 0, 0, 175, 191, 0, 0, 0, 162,
    255, 0, 0, 0, 0, 0, 0,
];

static BACKWARD_TABLE_UPPER: &'static [u16] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 16, 24, 32, 40, 48, 56,
    64, 72, 80, 88, 96, 104, 112, 120, 128, 0, 136, 0, 144, 152, 0, 160, 0, 0,
    0, 0, 168, 0, 176, 0, 184, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 192,
];

/// Returns the index pointer for code point `code` in this index.
#[inline]
pub fn backward(code: u32) -> u8 {
    let offset = (code >> 3) as usize;
    let offset = if offset < 92 {BACKWARD_TABLE_UPPER[offset] as usize} else {0};
    BACKWARD_TABLE_LOWER[offset + ((code & 7) as usize)]
}

#[cfg(test)]
single_byte_tests!(
    mod = iso_8859_3
);
