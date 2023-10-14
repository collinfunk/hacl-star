module Hacl.Impl.PCurves.PointMul

open FStar.Mul
open FStar.HyperStack.All
open FStar.HyperStack
module ST = FStar.HyperStack.ST

open Lib.IntTypes
open Lib.Buffer

open Hacl.Impl.PCurves.Bignum
open Hacl.Impl.PCurves.Field
open Hacl.Impl.PCurves.Constants
open Hacl.Impl.PCurves.InvSqrt
open Hacl.Impl.PCurves.Point

module S = Spec.PCurves

include Hacl.Impl.PCurves.Group
include Hacl.Impl.PCurves.PrecompTable


#set-options "--z3rlimit 30 --fuel 0 --ifuel 0"
noextract inline_for_extraction
let point_mul_t {| cp:S.curve_params |} {| curve_constants |} {| bn_ops |} {| f:field_ops |}
              {| curve_inv_sqrt|} {| point_ops |} {| pt:precomp_tables |} =
  res:point -> scalar:felem -> p:point -> Stack unit
  (requires fun h ->
    live h p /\ live h res /\ live h scalar /\
    disjoint p res /\ disjoint scalar res /\ disjoint p scalar /\
    point_inv h p /\ as_nat h scalar < S.order)
  (ensures fun h0 _ h1 -> modifies (loc res) h0 h1 /\
    point_inv h1 res /\
    S.to_aff_point (from_mont_point (as_point_nat h1 res)) ==
    S.to_aff_point (S.point_mul (as_nat h0 scalar) (from_mont_point (as_point_nat h0 p))))

[@(strict_on_arguments [0;1;2;3;4;5;6])]
noextract inline_for_extraction
val point_mul_gen {| cp:S.curve_params |} {| curve_constants |} {| bn_ops |} {| f:field_ops |}
              {| curve_inv_sqrt|} {| point_ops |} {| pt:precomp_tables |}: point_mul_t


noextract inline_for_extraction
let point_mul_g_t {| cp:S.curve_params |} {| curve_constants |} {| bn_ops |} {| f:field_ops |}
                {| curve_inv_sqrt|} {| point_ops |} {| pt:precomp_tables |} =
  res:point -> scalar:felem -> Stack unit
  (requires fun h ->
    live h res /\ live h scalar /\ disjoint res scalar /\
    as_nat h scalar < S.order)
  (ensures fun h0 _ h1 -> modifies (loc res) h0 h1 /\
    point_inv h1 res /\
    S.to_aff_point (from_mont_point (as_point_nat h1 res)) ==
    S.to_aff_point (S.point_mul_g (as_nat h0 scalar)))

[@(strict_on_arguments [0;1;2;3;4;5;6])]
noextract inline_for_extraction
val point_mul_g_gen {| cp:S.curve_params |} {| curve_constants |} {| bn_ops |} {| f:field_ops |}
                {| curve_inv_sqrt|} {| point_ops |} {| pt:precomp_tables |}: point_mul_g_t

noextract inline_for_extraction
let point_mul_double_g_t {| cp:S.curve_params |} {| curve_constants |} {| bn_ops |} {| f:field_ops |}
                       {| curve_inv_sqrt|} {| point_ops |} {| pt:precomp_tables |} = 
  res:point -> scalar1:felem -> scalar2:felem -> p:point -> Stack unit
  (requires fun h ->
    live h res /\ live h scalar1 /\ live h scalar2 /\ live h p /\
    disjoint res scalar1 /\ disjoint res scalar2 /\ disjoint res p /\
    disjoint p scalar1 /\ disjoint p scalar2 /\
    point_inv h p /\ as_nat h scalar1 < S.order /\ as_nat h scalar2 < S.order)
  (ensures  fun h0 _ h1 -> modifies (loc res) h0 h1 /\
    point_inv h1 res /\
    S.to_aff_point (from_mont_point (as_point_nat h1 res)) ==
    S.to_aff_point (S.point_mul_double_g (as_nat h0 scalar1) (as_nat h0 scalar2)
      (from_mont_point (as_point_nat h0 p))))

[@(strict_on_arguments [0;1;2;3;4;5;6])]
noextract inline_for_extraction
val point_mul_double_g_gen {| cp:S.curve_params |} {| curve_constants |} {| bn_ops |} {| f:field_ops |}
                       {| curve_inv_sqrt|} {| point_ops |} {| pt:precomp_tables |}: point_mul_double_g_t


inline_for_extraction
class point_mul_ops {| cp:S.curve_params |} {| curve_constants |} {| bn_ops |} {| f:field_ops |}
                    {| curve_inv_sqrt|} {| point_ops |} {| pt:precomp_tables |} = {
      point_mul: point_mul_t;
      point_mul_g: point_mul_g_t;
      point_mul_double_g: point_mul_double_g_t
}
