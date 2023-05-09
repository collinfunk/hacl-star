module Spec.EC.Lemmas

open FStar.Mul
open Spec.EC

#set-options "--z3rlimit 50 --ifuel 0 --fuel 0"

let aff_point_at_inf_lemma (k:curve) (p:aff_point k) = ()

let aff_point_add_assoc_lemma (k:curve) (p q s:aff_point k) = admit()

let aff_point_add_comm_lemma (k:curve) (p q:aff_point k) = admit()


val lemma_eq_neg_value_prime: m:pos{m % 2 = 1} -> x:nat{x < m} ->
  Lemma (x = (-x) % m <==> (x = 0))
let lemma_eq_neg_value_prime m x = ()


let aff_point_negate_lemma (k:curve) (p:aff_point k) =
  let p_neg = aff_point_negate k p in
  let px, py = p_neg in
  let qx, qy = p in
  assert (px = qx /\ py = (-qy) % prime);
  let res = aff_point_add k p_neg p in

  if is_aff_point_at_inf k p_neg then ()
  else begin
    if is_aff_point_at_inf k p then ()
    else begin
    if p_neg = p then begin
      assert (py = qy);
      assert (qy = (-qy) % prime);
      lemma_eq_neg_value_prime prime qy end
    else () end
  end
