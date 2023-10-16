module Hacl.Impl.PCurves.PrecompTable.P384

open FStar.HyperStack
open FStar.HyperStack.ST
open FStar.Mul

open Lib.IntTypes
open Lib.Buffer

module ST = FStar.HyperStack.ST
module LSeq = Lib.Sequence

module LE = Lib.Exponentiation
module SE = Spec.Exponentiation
module SPT = Hacl.Spec.PrecompBaseTable
module SPT256 = Hacl.Spec.PrecompBaseTable256
module SPTK = Hacl.Spec.PCurves.PrecompTable

module S = Spec.PCurves
module SL = Spec.PCurves.Lemmas

open Hacl.Impl.PCurves.Point
include Hacl.Impl.PCurves.Group

module PP = Hacl.Impl.PCurves.PrecompPoints.P384
open Hacl.Impl.PCurves.PrecompTable
open Spec.P384

#set-options "--z3rlimit 100 --fuel 0 --ifuel 0"

//----------------


///  window size = 4; precomputed table = [[0]G, [1]G, ..., [15]G]
 
inline_for_extraction noextract
let p384_basepoint_table_list_w4: x:list uint64{FStar.List.Tot.length x = 288} =
  normalize_term (SPT.precomp_base_table_list mk_pcurve_precomp_base_table p384_basepoint 15)
 

inline_for_extraction noextract
let p384_basepoint_table_lseq_w4 : LSeq.lseq uint64 (288) =
  normalize_term_spec (SPT.precomp_base_table_list mk_pcurve_precomp_base_table S.base_point 15);
  Seq.seq_of_list p384_basepoint_table_list_w4

noextract
val p384_basepoint_table_lemma_w4: unit ->
  Lemma ((forall (i:nat{i < 16}). precomp_table_acc_inv g_aff 16
         p384_basepoint_table_lseq_w4 i))

noextract
let p384_basepoint_table_lemma_w4 () =
  normalize_term_spec (SPT.precomp_base_table_list mk_pcurve_precomp_base_table S.base_point 15);
  SPT.precomp_base_table_lemma #_ #_ #(3ul*.6ul) mk_pcurve_precomp_base_table S.base_point 16 p384_basepoint_table_lseq_w4

val p384_basepoint_table_w4:
  x:glbuffer uint64 288ul{witnessed x p384_basepoint_table_lseq_w4 /\ recallable x}

#push-options "--fuel 2 --ifuel 2"
let p384_basepoint_table_w4:
  x:glbuffer uint64 288ul{witnessed x p384_basepoint_table_lseq_w4 /\ recallable x} =
  createL_global p384_basepoint_table_list_w4
#pop-options

inline_for_extraction noextract
let p384_precomp_basepoint_table_w4 : precomp_table_w4 g_aff = {
  table_lseq_w4 = p384_basepoint_table_lseq_w4;
  table_lemma_w4 = p384_basepoint_table_lemma_w4;
  table_w4 = p384_basepoint_table_w4
}


///  window size = 4; precomputed table = [[0]([pow2 64]G), [1]([pow2 64]G), ..., [15]([pow2 64]G)]

inline_for_extraction noextract
let p384_g_pow2_64_table_list_w4: x:list uint64{FStar.List.Tot.length x = 288} =
  normalize_term (SPT.precomp_base_table_list mk_pcurve_precomp_base_table PP.proj_g_pow2_64 15)

inline_for_extraction noextract
let p384_g_pow2_64_table_lseq_w4 : LSeq.lseq uint64 288 =
  normalize_term_spec (SPT.precomp_base_table_list mk_pcurve_precomp_base_table PP.proj_g_pow2_64 15);
  Seq.seq_of_list p384_g_pow2_64_table_list_w4

noextract
val p384_g_pow2_64_table_lemma_w4: unit ->
  Lemma (forall (i:nat{i < 16}). precomp_table_acc_inv g_pow2_64 16 p384_g_pow2_64_table_lseq_w4 i)

noextract
let p384_g_pow2_64_table_lemma_w4 () =
  normalize_term_spec (SPT.precomp_base_table_list mk_pcurve_precomp_base_table PP.proj_g_pow2_64 15);
  SPT.precomp_base_table_lemma mk_pcurve_precomp_base_table
    PP.proj_g_pow2_64 16 p384_g_pow2_64_table_lseq_w4;
  PP.proj_g_pow2_64_lemma ()


val p384_g_pow2_64_table_w4 :
  x:glbuffer uint64 288ul{witnessed x p384_g_pow2_64_table_lseq_w4 /\ recallable x}

#push-options "--fuel 2 --ifuel 2"
let p384_g_pow2_64_table_w4 :
  x:glbuffer uint64 288ul{witnessed x p384_g_pow2_64_table_lseq_w4 /\ recallable x} =
  createL_global p384_g_pow2_64_table_list_w4
#pop-options

inline_for_extraction noextract
let p384_precomp_g_pow2_64_table_w4 : precomp_table_w4 g_pow2_64 = {
  table_lseq_w4 = p384_g_pow2_64_table_lseq_w4;
  table_lemma_w4 = p384_g_pow2_64_table_lemma_w4;
  table_w4 = p384_g_pow2_64_table_w4
}



///  window size = 4; precomputed table = [[0]([pow2 128]G), [1]([pow2 128]G),...,[15]([pow2 128]G)]

inline_for_extraction noextract
let p384_g_pow2_128_table_list_w4: x:list uint64{FStar.List.Tot.length x = 288} =
  normalize_term (SPT.precomp_base_table_list mk_pcurve_precomp_base_table PP.proj_g_pow2_128 15)

inline_for_extraction noextract
let p384_g_pow2_128_table_lseq_w4 : LSeq.lseq uint64 288 =
  normalize_term_spec (SPT.precomp_base_table_list mk_pcurve_precomp_base_table PP.proj_g_pow2_128 15);
  Seq.seq_of_list p384_g_pow2_128_table_list_w4

noextract
val p384_g_pow2_128_table_lemma_w4: unit ->
  Lemma (forall (i:nat{i < 16}). precomp_table_acc_inv g_pow2_128 16 p384_g_pow2_128_table_lseq_w4 i)

noextract
let p384_g_pow2_128_table_lemma_w4 () =
  normalize_term_spec (SPT.precomp_base_table_list mk_pcurve_precomp_base_table PP.proj_g_pow2_128 15);
  SPT.precomp_base_table_lemma mk_pcurve_precomp_base_table
    PP.proj_g_pow2_128 16 p384_g_pow2_128_table_lseq_w4;
  PP.proj_g_pow2_128_lemma ()


val p384_g_pow2_128_table_w4 :
  x:glbuffer uint64 288ul{witnessed x p384_g_pow2_128_table_lseq_w4 /\ recallable x}

#push-options "--fuel 2 --ifuel 2"
let p384_g_pow2_128_table_w4 :
  x:glbuffer uint64 288ul{witnessed x p384_g_pow2_128_table_lseq_w4 /\ recallable x} =
  createL_global p384_g_pow2_128_table_list_w4
#pop-options

inline_for_extraction noextract
let p384_precomp_g_pow2_128_table_w4 : precomp_table_w4 g_pow2_128 = {
  table_lseq_w4 = p384_g_pow2_128_table_lseq_w4;
  table_lemma_w4 = p384_g_pow2_128_table_lemma_w4;
  table_w4 = p384_g_pow2_128_table_w4
}


///  window size = 4; precomputed table = [[0]([pow2 192]G), [1]([pow2 192]G),...,[15]([pow2 192]G)]

inline_for_extraction noextract
let p384_g_pow2_192_table_list_w4: x:list uint64{FStar.List.Tot.length x = 288} =
  normalize_term (SPT.precomp_base_table_list mk_pcurve_precomp_base_table PP.proj_g_pow2_192 15)
  
inline_for_extraction noextract
let p384_g_pow2_192_table_lseq_w4 : LSeq.lseq uint64 288 =
  normalize_term_spec (SPT.precomp_base_table_list mk_pcurve_precomp_base_table PP.proj_g_pow2_192 15);
  Seq.seq_of_list p384_g_pow2_192_table_list_w4

noextract
val p384_g_pow2_192_table_lemma_w4: unit ->
  Lemma (forall (i:nat{i < 16}). precomp_table_acc_inv g_pow2_192 16 p384_g_pow2_192_table_lseq_w4 i)

noextract
let p384_g_pow2_192_table_lemma_w4 () =
  normalize_term_spec (SPT.precomp_base_table_list mk_pcurve_precomp_base_table PP.proj_g_pow2_192 15);
  SPT.precomp_base_table_lemma mk_pcurve_precomp_base_table
    PP.proj_g_pow2_192 16 p384_g_pow2_192_table_lseq_w4;
  PP.proj_g_pow2_192_lemma ()


val p384_g_pow2_192_table_w4 :
  x:glbuffer uint64 288ul{witnessed x p384_g_pow2_192_table_lseq_w4 /\ recallable x}

#push-options "--fuel 2 --ifuel 2"
let p384_g_pow2_192_table_w4 :
  x:glbuffer uint64 288ul{witnessed x p384_g_pow2_192_table_lseq_w4 /\ recallable x} =
  createL_global p384_g_pow2_192_table_list_w4
#pop-options

inline_for_extraction noextract
let p384_precomp_g_pow2_192_table_w4 : precomp_table_w4 g_pow2_192 = {
  table_lseq_w4 = p384_g_pow2_192_table_lseq_w4;
  table_lemma_w4 = p384_g_pow2_192_table_lemma_w4;
  table_w4 = p384_g_pow2_192_table_w4
}


///  window size = 5; precomputed table = [[0]G, [1]G, ..., [31]G]

inline_for_extraction noextract
let p384_basepoint_table_list_w5 :
    x:list uint64{FStar.List.Tot.length x = 576} =
  normalize_term (SPT.precomp_base_table_list mk_pcurve_precomp_base_table p384_basepoint 31)

inline_for_extraction noextract
let p384_basepoint_table_lseq_w5 : LSeq.lseq uint64 576 =
  normalize_term_spec (SPT.precomp_base_table_list mk_pcurve_precomp_base_table S.base_point 31);
  Seq.seq_of_list p384_basepoint_table_list_w5

noextract
val p384_basepoint_table_lemma_w5: unit ->
  Lemma ((forall (i:nat{i < 32}). precomp_table_acc_inv g_aff 32
         p384_basepoint_table_lseq_w5 i))

let p384_basepoint_table_lemma_w5 () =
  normalize_term_spec (SPT.precomp_base_table_list mk_pcurve_precomp_base_table S.base_point 31);
  SPT.precomp_base_table_lemma mk_pcurve_precomp_base_table S.base_point 32 p384_basepoint_table_lseq_w5

val p384_basepoint_table_w5:
  x:glbuffer uint64 576ul{witnessed x p384_basepoint_table_lseq_w5 /\ recallable x}

#push-options "--fuel 2 --ifuel 2"
let p384_basepoint_table_w5:
  x:glbuffer uint64 576ul{witnessed x p384_basepoint_table_lseq_w5 /\ recallable x} =
  createL_global p384_basepoint_table_list_w5
#pop-options

inline_for_extraction noextract
let p384_precomp_basepoint_table_w5 : precomp_table_w5 g_aff = {
  table_lseq_w5 = p384_basepoint_table_lseq_w5;
  table_lemma_w5 = p384_basepoint_table_lemma_w5;
  table_w5 = p384_basepoint_table_w5
}

open Hacl.Impl.PCurves.Constants.P384
open Hacl.Impl.PCurves.Bignum.P384
open Hacl.Impl.PCurves.Field.P384
open Hacl.Impl.PCurves.Finv.P384
open Hacl.Impl.PCurves.Qinv.P384
open Hacl.Impl.PCurves.Group.P384

[@CInline]
val precomp_get_consttime: precomp_get_consttime_t
let precomp_get_consttime = precomp_get_consttime_gen

///////////////////////////////////

inline_for_extraction noextract
instance p384_precomp_tables: precomp_tables = {
  mk_proj_g_pow2_64 = PP.g_pow2_64_table;
  mk_proj_g_pow2_128 = PP.g_pow2_128_table;
  mk_proj_g_pow2_192 = PP.g_pow2_192_table;
  basepoint_w4 = p384_precomp_basepoint_table_w4;
  g_pow2_64_w4 = p384_precomp_g_pow2_64_table_w4;
  g_pow2_128_w4 = p384_precomp_g_pow2_128_table_w4;
  g_pow2_192_w4 = p384_precomp_g_pow2_192_table_w4;
  basepoint_w5 = p384_precomp_basepoint_table_w5;
  precomp_get_consttime
}



