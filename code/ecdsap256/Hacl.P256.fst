module Hacl.P256

open FStar.HyperStack.All
open FStar.HyperStack
module ST = FStar.HyperStack.ST

open Lib.IntTypes
open Lib.Buffer
open Lib.ByteSequence

open Spec.DH
open Spec.ECDSAP256.Definition

open Lib.IntTypes
open Lib.Buffer
open Lib.ByteSequence

open FStar.Mul

open Spec.P256
open Spec.P256.Lemmas
open Spec.P256.Definitions

open Spec.ECDSA
open Hacl.Impl.P256.Compression
open Spec.Hash.Definitions

open Hacl.Impl.ECDSA.P256.Signature.Agile
open Hacl.Impl.ECDSA.P256.Verification.Agile

(*)
let ecdsa_sign_p256_sha2 result mLen m privKey k = 
  ecdsa_signature (Hash SHA2_256) result mLen m privKey k

let ecdsa_sign_p256_sha384 result mLen m privKey k = 
  ecdsa_signature (Hash SHA2_384) result mLen m privKey k

let ecdsa_sign_p256_sha512 result mLen m privKey k = 
  ecdsa_signature (Hash SHA2_512) result mLen m privKey k


 let ecdsa_sign_p256_without_hash result mLen m privKey k = 
  ecdsa_signature NoHash result mLen m privKey k
 

let ecdsa_verif_p256_sha2 mLen m pubKey r s = 
  ecdsa_verification (Hash SHA2_256) pubKey r s mLen m

let ecdsa_verif_p256_sha384 mLen m pubKey r s = 
  ecdsa_verification (Hash SHA2_384) pubKey r s mLen m

let ecdsa_verif_p256_sha512 mLen m pubKey r s = 
  ecdsa_verification (Hash SHA2_512) pubKey r s mLen m

let ecdsa_verif_without_hash mLen m pubKey r s  =
   ecdsa_verification NoHash pubKey r s mLen m

 
let verify_q pubKey = 
    Hacl.Impl.P256.Signature.Common.verifyQ pubKey


let decompression_not_compressed_form b result = 
  Hacl.Impl.P256.Compression.decompressionNotCompressedForm b result

let decompression_compressed_form b result = 
  Hacl.Impl.P256.Compression.decompressionCompressedForm b result



let compression_not_compressed_form b result = 
  Hacl.Impl.P256.Compression.compressionNotCompressedForm b result

let compression_compressed_form b result = 
  Hacl.Impl.P256.Compression.compressionCompressedForm b result
*) 

let ecp256dh_i_ladder result scalar = Hacl.Impl.P256.DH.ecp256dh_i Ladder result scalar

let ecp256dh_i_radix4 result scalar = Hacl.Impl.P256.DH.ecp256dh_i Radix4 result scalar

let ecp256dh_i_cmb result scalar = Hacl.Impl.P256.DH.ecp256dh_i Comb result scalar




let ecp256dh_r_ladder result pubKey scalar = Hacl.Impl.P256.DH.ecp256dh_r Ladder result pubKey scalar

let ecp256dh_r_radix4 result pubKey scalar = Hacl.Impl.P256.DH.ecp256dh_r Radix4 result pubKey scalar

(* 
let is_more_than_zero_less_than_order x =
  Hacl.Impl.P256.Signature.Common.isMoreThanZeroLessThanOrder x

*)

let scalar_rwnaf out scalar = Hacl.Impl.ScalarMultiplication.WNAF.scalar_rwnaf out scalar

let scalar_bit  b c = Hacl.Impl.ScalarMultiplication.WNAF.scalar_bit b c