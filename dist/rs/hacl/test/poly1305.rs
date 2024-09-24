#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(const_item_mutation)]

const input1: [u8; 34] = [
  0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x70,
  0x68, 0x69, 0x63, 0x20, 0x46, 0x6f, 0x72, 0x75, 0x6d, 0x20,
  0x52, 0x65, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x20, 0x47,
  0x72, 0x6f, 0x75, 0x70
];

const key1: [u8; 32] = [
  0x85, 0xd6, 0xbe, 0x78, 0x57, 0x55, 0x6d, 0x33, 0x7f, 0x44,
  0x52, 0xfe, 0x42, 0xd5, 0x06, 0xa8, 0x01, 0x03, 0x80, 0x8a,
  0xfb, 0x0d, 0xb2, 0xfd, 0x4a, 0xbf, 0xf6, 0xaf, 0x41, 0x49,
  0xf5, 0x1b 
];

const tag1: [u8; 16] = [
  0xa8, 0x06, 0x1d, 0xc1, 0x30, 0x51, 0x36, 0xc6, 0xc2, 0x2b,
  0x8b, 0xaf, 0x0c, 0x01, 0x27, 0xa9 
];

const input2: [u8; 16] = [
  0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
  0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
];

const key2: [u8; 32] = [
  0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 
];

const tag2: [u8; 16] = [
  0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00
];

const input3: [u8; 528] = [
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x17, 0x03, 0x03, 0x02, 0x00,
  0x00, 0x00, 0x00, 0x06, 0xdb, 0x1f, 0x1f, 0x36, 0x8d, 0x69, 0x6a, 0x81, 0x0a,
  0x34, 0x9c, 0x0c, 0x71, 0x4c, 0x9a, 0x5e, 0x78, 0x50, 0xc2, 0x40, 0x7d, 0x72,
  0x1a, 0xcd, 0xed, 0x95, 0xe0, 0x18, 0xd7, 0xa8, 0x52, 0x66, 0xa6, 0xe1, 0x28,
  0x9c, 0xdb, 0x4a, 0xeb, 0x18, 0xda, 0x5a, 0xc8, 0xa2, 0xb0, 0x02, 0x6d, 0x24,
  0xa5, 0x9a, 0xd4, 0x85, 0x22, 0x7f, 0x3e, 0xae, 0xdb, 0xb2, 0xe7, 0xe3, 0x5e,
  0x1c, 0x66, 0xcd, 0x60, 0xf9, 0xab, 0xf7, 0x16, 0xdc, 0xc9, 0xac, 0x42, 0x68,
  0x2d, 0xd7, 0xda, 0xb2, 0x87, 0xa7, 0x02, 0x4c, 0x4e, 0xef, 0xc3, 0x21, 0xcc,
  0x05, 0x74, 0xe1, 0x67, 0x93, 0xe3, 0x7c, 0xec, 0x03, 0xc5, 0xbd, 0xa4, 0x2b,
  0x54, 0xc1, 0x14, 0xa8, 0x0b, 0x57, 0xaf, 0x26, 0x41, 0x6c, 0x7b, 0xe7, 0x42,
  0x00, 0x5e, 0x20, 0x85, 0x5c, 0x73, 0xe2, 0x1d, 0xc8, 0xe2, 0xed, 0xc9, 0xd4,
  0x35, 0xcb, 0x6f, 0x60, 0x59, 0x28, 0x00, 0x11, 0xc2, 0x70, 0xb7, 0x15, 0x70,
  0x05, 0x1c, 0x1c, 0x9b, 0x30, 0x52, 0x12, 0x66, 0x20, 0xbc, 0x1e, 0x27, 0x30,
  0xfa, 0x06, 0x6c, 0x7a, 0x50, 0x9d, 0x53, 0xc6, 0x0e, 0x5a, 0xe1, 0xb4, 0x0a,
  0xa6, 0xe3, 0x9e, 0x49, 0x66, 0x92, 0x28, 0xc9, 0x0e, 0xec, 0xb4, 0xa5, 0x0d,
  0xb3, 0x2a, 0x50, 0xbc, 0x49, 0xe9, 0x0b, 0x4f, 0x4b, 0x35, 0x9a, 0x1d, 0xfd,
  0x11, 0x74, 0x9c, 0xd3, 0x86, 0x7f, 0xcf, 0x2f, 0xb7, 0xbb, 0x6c, 0xd4, 0x73,
  0x8f, 0x6a, 0x4a, 0xd6, 0xf7, 0xca, 0x50, 0x58, 0xf7, 0x61, 0x88, 0x45, 0xaf,
  0x9f, 0x02, 0x0f, 0x6c, 0x3b, 0x96, 0x7b, 0x8f, 0x4c, 0xd4, 0xa9, 0x1e, 0x28,
  0x13, 0xb5, 0x07, 0xae, 0x66, 0xf2, 0xd3, 0x5c, 0x18, 0x28, 0x4f, 0x72, 0x92,
  0x18, 0x60, 0x62, 0xe1, 0x0f, 0xd5, 0x51, 0x0d, 0x18, 0x77, 0x53, 0x51, 0xef,
  0x33, 0x4e, 0x76, 0x34, 0xab, 0x47, 0x43, 0xf5, 0xb6, 0x8f, 0x49, 0xad, 0xca,
  0xb3, 0x84, 0xd3, 0xfd, 0x75, 0xf7, 0x39, 0x0f, 0x40, 0x06, 0xef, 0x2a, 0x29,
  0x5c, 0x8c, 0x7a, 0x07, 0x6a, 0xd5, 0x45, 0x46, 0xcd, 0x25, 0xd2, 0x10, 0x7f,
  0xbe, 0x14, 0x36, 0xc8, 0x40, 0x92, 0x4a, 0xae, 0xbe, 0x5b, 0x37, 0x08, 0x93,
  0xcd, 0x63, 0xd1, 0x32, 0x5b, 0x86, 0x16, 0xfc, 0x48, 0x10, 0x88, 0x6b, 0xc1,
  0x52, 0xc5, 0x32, 0x21, 0xb6, 0xdf, 0x37, 0x31, 0x19, 0x39, 0x32, 0x55, 0xee,
  0x72, 0xbc, 0xaa, 0x88, 0x01, 0x74, 0xf1, 0x71, 0x7f, 0x91, 0x84, 0xfa, 0x91,
  0x64, 0x6f, 0x17, 0xa2, 0x4a, 0xc5, 0x5d, 0x16, 0xbf, 0xdd, 0xca, 0x95, 0x81,
  0xa9, 0x2e, 0xda, 0x47, 0x92, 0x01, 0xf0, 0xed, 0xbf, 0x63, 0x36, 0x00, 0xd6,
  0x06, 0x6d, 0x1a, 0xb3, 0x6d, 0x5d, 0x24, 0x15, 0xd7, 0x13, 0x51, 0xbb, 0xcd,
  0x60, 0x8a, 0x25, 0x10, 0x8d, 0x25, 0x64, 0x19, 0x92, 0xc1, 0xf2, 0x6c, 0x53,
  0x1c, 0xf9, 0xf9, 0x02, 0x03, 0xbc, 0x4c, 0xc1, 0x9f, 0x59, 0x27, 0xd8, 0x34,
  0xb0, 0xa4, 0x71, 0x16, 0xd3, 0x88, 0x4b, 0xbb, 0x16, 0x4b, 0x8e, 0xc8, 0x83,
  0xd1, 0xac, 0x83, 0x2e, 0x56, 0xb3, 0x91, 0x8a, 0x98, 0x60, 0x1a, 0x08, 0xd1,
  0x71, 0x88, 0x15, 0x41, 0xd5, 0x94, 0xdb, 0x39, 0x9c, 0x6a, 0xe6, 0x15, 0x12,
  0x21, 0x74, 0x5a, 0xec, 0x81, 0x4c, 0x45, 0xb0, 0xb0, 0x5b, 0x56, 0x54, 0x36,
  0xfd, 0x6f, 0x13, 0x7a, 0xa1, 0x0a, 0x0c, 0x0b, 0x64, 0x37, 0x61, 0xdb, 0xd6,
  0xf9, 0xa9, 0xdc, 0xb9, 0x9b, 0x1a, 0x6e, 0x69, 0x08, 0x54, 0xce, 0x07, 0x69,
  0xcd, 0xe3, 0x97, 0x61, 0xd8, 0x2f, 0xcd, 0xec, 0x15, 0xf0, 0xd9, 0x2d, 0x7d,
  0x8e, 0x94, 0xad, 0xe8, 0xeb, 0x83, 0xfb, 0xe0
];

const key3: [u8; 32] = [
  0x99, 0xe5, 0x82, 0x2d, 0xd4, 0x17, 0x3c, 0x99, 0x5e, 0x3d, 0xae,
  0x0d, 0xde, 0xfb, 0x97, 0x74, 0x3f, 0xde, 0x3b, 0x08, 0x01, 0x34,
  0xb3, 0x9f, 0x76, 0xe9, 0xbf, 0x8d, 0x0e, 0x88, 0xd5, 0x46 
];
  
const tag3: [u8; 16] = [
  0x26, 0x37, 0x40, 0x8f, 0xe1, 0x30, 0x86, 0xea, 0x73, 0xf9,
  0x71, 0xe3, 0x42, 0x5e, 0x28, 0x20 
];

#[test]
pub fn test_poly1305() {
  let mut tag = [ 0u8; 16];
  crate::mac_poly1305::mac(&mut tag, &mut input1, input1.len() as u32, &mut key1);
  assert_eq!(tag, tag1); 
  crate::mac_poly1305::mac(&mut tag, &mut input2, input2.len() as u32, &mut key2);
  assert_eq!(tag, tag2); 
  crate::mac_poly1305::mac(&mut tag, &mut input3, input3.len() as u32, &mut key3);
  assert_eq!(tag, tag3); 
}
