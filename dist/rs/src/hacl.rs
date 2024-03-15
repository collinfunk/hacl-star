pub mod inttypes_intrinsics;
pub mod inttypes_intrinsics_128;
pub mod streaming_types;
pub mod spec;
pub mod impl_blake2_constants;
pub mod hash_blake2b;
pub mod hash_blake2s;
// pub mod hash_blake2b_simd256;
// pub mod hash_blake2s_simd128;
pub mod hash_base;
pub mod hash_sha1;
pub mod hash_sha2;
// pub mod hmac; // TODO: review splitting pattern to make sure it borrow-checks
// pub mod hmac_blake2s_128;
// pub mod hmac_blake2b_256;
pub mod hash_sha3;
pub mod hash_md5;
pub mod sha2_types;
pub mod chacha20;
pub mod salsa20;
pub mod bignum_base;
pub mod bignum;
// pub mod curve25519_64;
pub mod bignum25519_51;
pub mod curve25519_51;
pub mod mac_poly1305;
pub mod aead_chacha20poly1305;
// pub mod mac_poly1305_simd128;
// pub mod chacha20_vec128;
// pub mod aead_chacha20poly1305_simd128;
// pub mod mac_poly1305_simd256;
// pub mod chacha20_vec256;
// pub mod aead_chacha20poly1305_simd256;
pub mod ed25519_precomptable;
 pub mod ed25519; // TODO: doesn't borrow-check, too many operations in place
// pub mod nacl; // TODO: ownership issue, need to insert a copy
pub mod p256_precomptable;
// pub mod p256; // TODO: numerous in-place functions, need to reproduce the patterns of ed25519 and debug
pub mod bignum_k256;
pub mod k256_precomptable;
// pub mod k256_ecdsa; // TODO: same as p256
// pub mod frodo_kem; // TODO: missing randomness implementation
pub mod hpke_interface_hacl_impl_hpke_hacl_meta_hpke;
pub mod rsapss;
pub mod impl_ffdhe_constants;
pub mod ffdhe;
// pub mod frodo640; // TODO: needs frodo_kem, above
// pub mod hkdf; // TODO: needs hmac, above
// pub mod hpke_curve51_cp128_sha512;
// pub mod genericfield32; // TODO: in-place APIs
// pub mod sha2_vec256;
// pub mod ec_k256;
pub mod bignum4096;
pub mod chacha20_vec32;
pub mod bignum4096_32;
// pub mod hmac_drbg; // TODO: needs HMAC, above
// pub mod hpke_curve64_cp128_sha512;
// pub mod hpke_p256_cp128_sha256;
// pub mod hpke_curve51_cp256_sha512;
// pub mod frodo976;
// pub mod hkdf_blake2s_128;
// pub mod genericfield64;
// pub mod frodo1344;
// pub mod hpke_curve64_cp256_sha512;
pub mod bignum32;
// pub mod hpke_curve51_cp128_sha256;
// pub mod hpke_curve64_cp128_sha256;
pub mod bignum256_32;
// pub mod sha2_vec128;
// pub mod hpke_curve51_cp32_sha256; // TODO: needs HKDF, above
// pub mod hpke_curve64_cp256_sha256;
// pub mod hpke_curve51_cp32_sha512;
// pub mod hpke_p256_cp256_sha256;
// pub mod hpke_p256_cp32_sha256;
pub mod bignum64;
// pub mod frodo64;
// pub mod hkdf_blake2b_256;
// pub mod hpke_curve64_cp32_sha256;
// pub mod hpke_curve64_cp32_sha512;
// pub mod ec_ed25519;
// pub mod hpke_curve51_cp256_sha256;
pub mod bignum256;
