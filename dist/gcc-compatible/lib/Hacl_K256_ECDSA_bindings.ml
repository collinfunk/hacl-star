open Ctypes
module Bindings(F:Cstubs.FOREIGN) =
  struct
    open F
    let hacl_Impl_K256_Point_make_point_at_inf =
      foreign "Hacl_Impl_K256_Point_make_point_at_inf"
        ((ptr uint64_t) @-> (returning void))
    let hacl_Impl_K256_Point_point_negate =
      foreign "Hacl_Impl_K256_Point_point_negate"
        ((ptr uint64_t) @-> ((ptr uint64_t) @-> (returning void)))
    let hacl_Impl_K256_Point_point_store =
      foreign "Hacl_Impl_K256_Point_point_store"
        (ocaml_bytes @-> ((ptr uint64_t) @-> (returning void)))
    let hacl_Impl_K256_Point_aff_point_load_vartime =
      foreign "Hacl_Impl_K256_Point_aff_point_load_vartime"
        ((ptr uint64_t) @-> (ocaml_bytes @-> (returning bool)))
    let hacl_Impl_K256_PointDouble_point_double =
      foreign "Hacl_Impl_K256_PointDouble_point_double"
        ((ptr uint64_t) @-> ((ptr uint64_t) @-> (returning void)))
    let hacl_Impl_K256_PointAdd_point_add =
      foreign "Hacl_Impl_K256_PointAdd_point_add"
        ((ptr uint64_t) @->
           ((ptr uint64_t) @-> ((ptr uint64_t) @-> (returning void))))
    let hacl_Impl_K256_PointMul_point_mul =
      foreign "Hacl_Impl_K256_PointMul_point_mul"
        ((ptr uint64_t) @->
           ((ptr uint64_t) @-> ((ptr uint64_t) @-> (returning void))))
    let hacl_K256_ECDSA_ecdsa_sign_hashed_msg =
      foreign "Hacl_K256_ECDSA_ecdsa_sign_hashed_msg"
        (ocaml_bytes @->
           (ocaml_bytes @->
              (ocaml_bytes @-> (ocaml_bytes @-> (returning bool)))))
    let hacl_K256_ECDSA_ecdsa_sign_sha256 =
      foreign "Hacl_K256_ECDSA_ecdsa_sign_sha256"
        (ocaml_bytes @->
           (uint32_t @->
              (ocaml_bytes @->
                 (ocaml_bytes @-> (ocaml_bytes @-> (returning bool))))))
    let hacl_K256_ECDSA_ecdsa_verify_hashed_msg =
      foreign "Hacl_K256_ECDSA_ecdsa_verify_hashed_msg"
        (ocaml_bytes @-> (ocaml_bytes @-> (ocaml_bytes @-> (returning bool))))
    let hacl_K256_ECDSA_ecdsa_verify_sha256 =
      foreign "Hacl_K256_ECDSA_ecdsa_verify_sha256"
        (uint32_t @->
           (ocaml_bytes @->
              (ocaml_bytes @-> (ocaml_bytes @-> (returning bool)))))
    let hacl_K256_ECDSA_secp256k1_ecdsa_signature_normalize =
      foreign "Hacl_K256_ECDSA_secp256k1_ecdsa_signature_normalize"
        (ocaml_bytes @-> (returning bool))
    let hacl_K256_ECDSA_secp256k1_ecdsa_is_signature_normalized =
      foreign "Hacl_K256_ECDSA_secp256k1_ecdsa_is_signature_normalized"
        (ocaml_bytes @-> (returning bool))
    let hacl_K256_ECDSA_secp256k1_ecdsa_sign_hashed_msg =
      foreign "Hacl_K256_ECDSA_secp256k1_ecdsa_sign_hashed_msg"
        (ocaml_bytes @->
           (ocaml_bytes @->
              (ocaml_bytes @-> (ocaml_bytes @-> (returning bool)))))
    let hacl_K256_ECDSA_secp256k1_ecdsa_sign_sha256 =
      foreign "Hacl_K256_ECDSA_secp256k1_ecdsa_sign_sha256"
        (ocaml_bytes @->
           (uint32_t @->
              (ocaml_bytes @->
                 (ocaml_bytes @-> (ocaml_bytes @-> (returning bool))))))
    let hacl_K256_ECDSA_secp256k1_ecdsa_verify_hashed_msg =
      foreign "Hacl_K256_ECDSA_secp256k1_ecdsa_verify_hashed_msg"
        (ocaml_bytes @-> (ocaml_bytes @-> (ocaml_bytes @-> (returning bool))))
    let hacl_K256_ECDSA_secp256k1_ecdsa_verify_sha256 =
      foreign "Hacl_K256_ECDSA_secp256k1_ecdsa_verify_sha256"
        (uint32_t @->
           (ocaml_bytes @->
              (ocaml_bytes @-> (ocaml_bytes @-> (returning bool)))))
    let hacl_K256_ECDSA_public_key_uncompressed_to_raw =
      foreign "Hacl_K256_ECDSA_public_key_uncompressed_to_raw"
        (ocaml_bytes @-> (ocaml_bytes @-> (returning bool)))
    let hacl_K256_ECDSA_public_key_uncompressed_from_raw =
      foreign "Hacl_K256_ECDSA_public_key_uncompressed_from_raw"
        (ocaml_bytes @-> (ocaml_bytes @-> (returning void)))
    let hacl_K256_ECDSA_public_key_compressed_to_raw =
      foreign "Hacl_K256_ECDSA_public_key_compressed_to_raw"
        (ocaml_bytes @-> (ocaml_bytes @-> (returning bool)))
    let hacl_K256_ECDSA_public_key_compressed_from_raw =
      foreign "Hacl_K256_ECDSA_public_key_compressed_from_raw"
        (ocaml_bytes @-> (ocaml_bytes @-> (returning void)))
    let hacl_K256_ECDSA_is_public_key_valid =
      foreign "Hacl_K256_ECDSA_is_public_key_valid"
        (ocaml_bytes @-> (returning bool))
    let hacl_K256_ECDSA_is_private_key_valid =
      foreign "Hacl_K256_ECDSA_is_private_key_valid"
        (ocaml_bytes @-> (returning bool))
    let hacl_K256_ECDSA_secret_to_public =
      foreign "Hacl_K256_ECDSA_secret_to_public"
        (ocaml_bytes @-> (ocaml_bytes @-> (returning bool)))
    let hacl_K256_ECDSA_ecdh =
      foreign "Hacl_K256_ECDSA_ecdh"
        (ocaml_bytes @-> (ocaml_bytes @-> (ocaml_bytes @-> (returning bool))))
  end