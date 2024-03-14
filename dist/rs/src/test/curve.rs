#![allow(non_upper_case_globals)]
#![allow(dead_code)]

const scalar1: [u8; 32] = [
    0xa5, 0x46, 0xe3, 0x6b, 0xf0, 0x52, 0x7c, 0x9d,
    0x3b, 0x16, 0x15, 0x4b, 0x82, 0x46, 0x5e, 0xdd,
    0x62, 0x14, 0x4c, 0x0a, 0xc1, 0xfc, 0x5a, 0x18,
    0x50, 0x6a, 0x22, 0x44, 0xba, 0x44, 0x9a, 0xc4
];

const scalar2: [u8; 32] = [
    0x4b, 0x66, 0xe9, 0xd4, 0xd1, 0xb4, 0x67, 0x3c,
    0x5a, 0xd2, 0x26, 0x91, 0x95, 0x7d, 0x6a, 0xf5,
    0xc1, 0x1b, 0x64, 0x21, 0xe0, 0xea, 0x01, 0xd4,
    0x2c, 0xa4, 0x16, 0x9e, 0x79, 0x18, 0xba, 0x0d
];

const point1: [u8; 32] = [
    0xe6, 0xdb, 0x68, 0x67, 0x58, 0x30, 0x30, 0xdb,
    0x35, 0x94, 0xc1, 0xa4, 0x24, 0xb1, 0x5f, 0x7c,
    0x72, 0x66, 0x24, 0xec, 0x26, 0xb3, 0x35, 0x3b,
    0x10, 0xa9, 0x03, 0xa6, 0xd0, 0xab, 0x1c, 0x4c
];

const point2: [u8; 32] = [
    0xe5, 0x21, 0x0f, 0x12, 0x78, 0x68, 0x11, 0xd3,
    0xf4, 0xb7, 0x95, 0x9d, 0x05, 0x38, 0xae, 0x2c,
    0x31, 0xdb, 0xe7, 0x10, 0x6f, 0xc0, 0x3c, 0x3e,
    0xfc, 0x4c, 0xd5, 0x49, 0xc7, 0x15, 0xa4, 0x93
];

const expected1: [u8; 32] = [
    0xc3, 0xda, 0x55, 0x37, 0x9d, 0xe9, 0xc6, 0x90,
    0x8e, 0x94, 0xea, 0x4d, 0xf2, 0x8d, 0x08, 0x4f,
    0x32, 0xec, 0xcf, 0x03, 0x49, 0x1c, 0x71, 0xf7,
    0x54, 0xb4, 0x07, 0x55, 0x77, 0xa2, 0x85, 0x52
];

const expected2: [u8; 32] = [
    0x95, 0xcb, 0xde, 0x94, 0x76, 0xe8, 0x90, 0x7d,
    0x7a, 0xad, 0xe4, 0x5c, 0xb4, 0xb8, 0x73, 0xf8,
    0x8b, 0x59, 0x5a, 0x68, 0x79, 0x9f, 0xa1, 0x52,
    0xe6, 0xf8, 0xf7, 0x64, 0x7a, 0xac, 0x79, 0x57
];

#[test]
pub fn test_curve() {
  let mut res1 = [ 0u8; expected1.len() ];
  let mut res2 = [ 0u8; expected2.len() ];
  crate::hacl::curve25519_51::scalarmult(&mut res1, &mut scalar1, &mut point1);
  crate::hacl::curve25519_51::scalarmult(&mut res2, &mut scalar2, &mut point2);
  assert_eq!(res1, expected1);
  assert_eq!(res2, expected2);
}
