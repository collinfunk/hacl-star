module Spec.ECC

open FStar.Mul

open Lib.ByteSequence
open Lib.IntTypes
open Lib.Sequence
open Lib.RawIntTypes

open Lib.LoopCombinators 

open FStar.Math.Lemmas
open FStar.Math.Lib

open Spec.ECC.Curves

#set-options "--fuel 0 --ifuel 0 --z3rlimit 100"

type coordSyst = 
  |Affine 
  |Jacobian


let invert_state_s (a: coordSyst): Lemma
  (requires True)
  (ensures (inversion coordSyst))
  [SMTPat (coordSyst) ]
  = allow_inversion (coordSyst)


let pointAtInfinity #c : point_nat_prime #c = (0, 0, 0)

let isPointAtInfinity #coordSyst (p:point_nat) =
  match coordSyst with 
  |Jacobian -> let (_, _, z) = p in z = 0
  |Affine -> let (x, y, _) = p in x = 0 && y = 0



noextract
let _point_double_nist #curve (p:point_nat_prime #curve) : point_nat_prime #curve =
  let prime = getPrime curve in 
  let x, y, z = p in
  let delta = z * z in 
  let gamma = y * y in 
  let beta = x * gamma in 
  let alpha = 3 * (x - delta) * (x + delta) in 
  let x3 = (alpha * alpha - 8 * beta) % prime in 
  let y3 = (alpha * (4 * beta - x3) - 8 * gamma * gamma) % prime in 
  let z3 = ((y + z) * (y + z) - delta - gamma) % prime in 
  (x3, y3, z3)


(* https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#doubling-dbl-1998-cmo-2 *)

noextract
let _point_double_general #curve (p: point_nat_prime #curve) : point_nat_prime #curve = 
  let prime = getPrime curve in 
  let x, y, z = p in
  let gamma = y * y in 
  let delta = z * z in 
  let beta = x * gamma in 
   
  let alpha = 3 * x * x + aCoordinate #curve * delta * delta in 
  
  let x3 = (alpha * alpha - 8 * beta) % prime in 
  let y3 = (alpha * (4 * beta - x3) - 8 * gamma * gamma) % prime in 
  let z3 = ((y + z) * (y + z) - delta - gamma) % prime in 
  (x3, y3, z3)


noextract
let _point_double_koblitz #curve (p: point_nat_prime #curve) : point_nat_prime #curve = 
  let prime = getPrime curve in 
  let x, y, z = p in
  let gamma = y * y in 
  let delta = z * z in 
  let beta = x * gamma in 
   
  let alpha = 3 * x * x in 
  
  let x3 = (alpha * alpha - 8 * beta) % prime in 
  let y3 = (alpha * (4 * beta - x3) - 8 * gamma * gamma) % prime in 
  let z3 = ((y + z) * (y + z) - delta - gamma) % prime in 
  (x3, y3, z3)


noextract
let _point_double #curve (p: point_nat_prime #curve) = 
  match curve with 
  |P256 -> _point_double_nist p 
  |P384 -> _point_double_nist p


let _point_inverse #curve (p:point_nat_prime #curve) : point_nat_prime #curve =
  let x, y, z = p in
  let minusY = (0 - y) % getPrime curve in 
  (x, minusY, z)



let _norm #curve (p:point_nat_prime #curve) : (point_nat_prime #curve) =
  let prime = getPrime curve in 
  let (x, y, z) = p in
  let z2 = z * z in
  let z2i = modp_inv2 #curve z2 in
  let z3 = z * z * z in
  let z3i = modp_inv2 #curve z3 in
  let x3 = (z2i * x) % prime in
  let y3 = (z3i * y) % prime in
  let z3 = if isPointAtInfinity #Jacobian p then 0 else 1 in
  (x3, y3, z3)


let scalar_bytes (#c: curve) = lbytes (v (getScalarLenBytes c))

let ith_bit (#c: curve) (k: scalar_bytes #c) (i:nat{i < v (getScalarLen c)}) : (r: uint64 {v r == 0 \/ v r == 1}) =
  let q = (v (getScalarLenBytes c) - 1) - i / 8 in 
  let r = size (i % 8) in
  logand_mask (index k q >>. r) (u8 1) 1;
  to_u64 ((index k q >>. r) &. u8 1)
  
let isPointOnCurve (#c: curve) (p: point_nat_prime #c) : bool = 
  let (x, y, z) = p in
  let prime = getPrime c in 
  (y * y) % prime = (x * x * x + aCoordinate #c  * x + bCoordinate #c) % prime


val toJacobianCoordinates: tuple2 nat nat -> r: tuple3 nat nat nat {~ (isPointAtInfinity #Jacobian r)}

let toJacobianCoordinates (r0, r1) =  (r0, r1, 1)


let pointEqual #curve (p: point_nat_prime #curve) (q: point_nat_prime #curve) = 
  
  let pAffineX, pAffineY, pAffineZ = _norm p in 
  let qAffineX, qAffineY, qAffineZ = _norm q in 

  (* if one point is at infinity, but not the second *)
  if pAffineZ <> qAffineZ then false else
  (* if two points are at infinity *)
  if isPointAtInfinity #Jacobian (pAffineX, pAffineY, pAffineZ) && isPointAtInfinity #Jacobian (qAffineX, qAffineY, qAffineZ) then true else
  (* if affine coordinates are equal *)
  if (pAffineX = qAffineX && pAffineY = qAffineY) then 
    true
  else false



noextract
let _point_add #curve  (p:point_nat_prime #curve) (q:point_nat_prime #curve) : point_nat_prime #curve =
  let prime = getPrime curve in 
  let (x1, y1, z1) = p in
  let (x2, y2, z2) = q in

  let z2z2 = z2 * z2 in
  let z1z1 = z1 * z1 in

  let u1 = x1 * z2z2 % prime in
  let u2 = x2 * z1z1 % prime in

  let s1 = y1 * z2 * z2z2 % prime in
  let s2 = y2 * z1 * z1z1 % prime in

  let h = (u2 - u1) % prime in
  let r = (s2 - s1) % prime in

  let rr = r * r in
  let hh = h * h in
  let hhh = h * h * h in

  let x3 = (rr - hhh - 2 * u1 * hh) % prime in
  let y3 = (r * (u1 * hh - x3) - s1 * hhh) % prime in
  let z3 = (h * z1 * z2) % prime in
  if isPointAtInfinity #Jacobian  q then
    (x1, y1, z1)
  else
    if isPointAtInfinity #Jacobian  p then
      (x2, y2, z2)
    else
      (x3, y3, z3)


noextract
let _point_add_mixed #curve (p:point_nat_prime #curve) (q:point_nat_prime #curve) : point_nat_prime #curve =
  let prime = getPrime curve in 
  let (x1, y1, z1) = p in
  let (x2, y2, z2) = q in

  let z2z2 = z2 * z2 in
  let z1z1 = z1 * z1 in

  let u1 = x1 * z2z2 % prime in
  let u2 = x2 * z1z1 % prime in

  let s1 = y1 * z2 * z2z2 % prime in
  let s2 = y2 * z1 * z1z1 % prime in

  let h = (u2 - u1) % prime in
  let r = (s2 - s1) % prime in

  let rr = r * r in
  let hh = h * h in
  let hhh = h * h * h in

  let x3 = (rr - hhh - 2 * u1 * hh) % prime in
  let y3 = (r * (u1 * hh - x3) - s1 * hhh) % prime in
  let z3 = (h * z1 * z2) % prime in
  if isPointAtInfinity #Affine q then
    (x1, y1, z1)
  else
    if isPointAtInfinity #Jacobian p then
      (x2, y2, z2)
    else
      (x3, y3, z3)



let pointAdd #curve (p:point_nat_prime #curve) (q:point_nat_prime #curve) : point_nat_prime #curve =
  if pointEqual p q then 
    _point_double p 
  else
    _point_add #curve p q


val _ml_step0: #c: curve -> r0: point_nat_prime #c -> r1: point_nat_prime #c -> tuple2 (point_nat_prime #c) (point_nat_prime #c) 

let _ml_step0 #c r0 r1 =
  let r0 = pointAdd #c  r1 r0 in
  let r1 = pointAdd #c r1 r1 in
  (r0, r1)


val _ml_step1: #c: curve -> r0: point_nat_prime #c -> r1: point_nat_prime #c -> tuple2 (point_nat_prime #c) (point_nat_prime #c)

let _ml_step1 #c r0 r1 =
  let r1 = pointAdd #c  r0 r1 in
  let r0 = pointAdd #c r0 r0 in
  (r0, r1)


val _ml_step: #c: curve -> k: scalar_bytes #c -> i: nat{i < v (getScalarLen c)} 
  -> r: tuple2 (point_nat_prime #c) (point_nat_prime #c)
  -> tuple2 (point_nat_prime #c) (point_nat_prime #c)

let _ml_step #c k i (p, q) =
  let bit = v (getScalarLen c) - 1 - i in
  let bit = ith_bit k bit in
  if uint_to_nat bit = 0 then
    _ml_step1 p q
  else
    _ml_step0 p q

(* P + R == P + Q <==> R == Q --> P + P == P + Q <==> P == Q *)
assume val curve_point_equality: #c: curve ->  p: point_nat_prime #c -> q: point_nat_prime #c -> Lemma 
  (pointEqual (pointAdd #c p q) (pointAdd #c p p) <==> pointEqual p q)

(* P + Q == Q + P *)
assume val curve_commutativity_lemma: #c: curve -> p: point_nat_prime #c -> q: point_nat_prime #c -> Lemma 
  (pointEqual (pointAdd #c p q) (pointAdd #c q p))

(* P == P' ==> P + Q == P' + Q *)
assume val curve_compatibility_with_translation_lemma: #c: curve -> p: point_nat_prime #c -> p1: point_nat_prime #c {pointEqual p p1}
  -> q: point_nat_prime #c -> 
  Lemma (pointEqual (pointAdd #c p q) (pointAdd #c p1 q))

(* order is the smallest number such that P * order == 0 *)

assume val curve_order_is_the_smallest: #c: curve ->  p: point_nat_prime #c -> Lemma
  (forall (n: nat {isPointAtInfinity #Jacobian (repeat n (fun x -> pointAdd #c p x) p)}). n >= getOrder #c)

(* order * P == 0 *)
assume val curve_multiplication_by_order: #c: curve -> i: int {(i + 1) % getOrder #c = 0} -> p: point_nat_prime -> 
  Lemma (repeat (i % getOrder #c) (fun x -> pointAdd #c p x) p == pointAtInfinity)

(* P + 0 == P *)
assume val curve_point_at_infinity_property: #c: curve -> q: point_nat_prime #c -> Lemma (pointEqual (pointAdd #c pointAtInfinity q) q)

assume val curve_associativity: #c: curve ->  p: point_nat_prime #c -> q: point_nat_prime #c -> r: point_nat_prime #c -> Lemma 
  (pointEqual (pointAdd #c (pointAdd #c p q) r) (pointAdd #c p (pointAdd #c q r)))


val point_mult: #c: curve -> i: int -> p: point_nat_prime #c -> point_nat_prime #c

let point_mult #c i p = repeat ((i - 1) % getOrder #c) (fun x -> pointAdd #c p x) p


val point_mult_0: #c: curve ->  p: point_nat_prime #c -> i: int {i % getOrder #c = 0} -> 
  Lemma (point_mult #c i p == pointAtInfinity)

let point_mult_0 #c p i = 
  small_mod 0 (getOrder #c);
  curve_multiplication_by_order #c (getOrder #c - 1) p;
  assert(repeat ((getOrder #c - 1) % getOrder #c) (fun x -> pointAdd #c p x) p == pointAtInfinity)


val point_mult_1: #c: curve ->  p: point_nat_prime #c ->  Lemma (point_mult #c 1 p == p)

let point_mult_1 #c p = eq_repeat0 (fun x -> pointAdd #c p x) p 


val point_mult_ext: #c: curve ->  i: int -> p: point_nat_prime #c -> Lemma (
  let f = (fun x -> pointAdd #c p x) in  
  pointEqual (f (point_mult #c i p)) (point_mult #c (i + 1) p))

let point_mult_ext #c i p = 
  let f = (fun x -> pointAdd #c p x) in 
  if i % getOrder #c = 0 then 
    begin
      point_mult_0 #c p 0;
      curve_point_at_infinity_property #c p;  
      curve_commutativity_lemma #c pointAtInfinity p; 
      point_mult_1 #c p
    end
  else 
    begin
      unfold_repeat (getOrder #c) f p ((i - 1) % getOrder #c);
      lemma_mod_add_distr 1 (i - 1) (getOrder #c)
    end


val lemma_scalar_reduce: #c: curve -> p: point_nat_prime #c -> pk: int ->
  Lemma (point_mult #c pk p == point_mult #c (pk % getOrder #c) p)

let lemma_scalar_reduce #c p pk = 
  let o = getOrder #c in 
  assert(point_mult #c pk p == repeat ((pk - 1) % getOrder #c) (fun x -> pointAdd #c p x) p); 
  assert(point_mult #c (pk % o) p == repeat (((pk % o) - 1) % o) (fun x -> pointAdd #c p x) p);
  lemma_mod_add_distr (-1) pk o; 
  lemma_mod_twice (pk - 1) (getOrder #c)


val point_mult_def: #c: curve ->  i: int -> p: point_nat_prime #c -> Lemma 
  (point_mult #c i p == repeat ((i - 1) % getOrder #c) (fun x -> pointAdd #c p x) p)

let point_mult_def #c i p = ()


val lemma_commutativity_extended: #c: curve ->  p0: point_nat_prime #c  -> a: point_nat_prime #c -> b: point_nat_prime #c -> 
  Lemma (
    let f = (fun x -> pointAdd #c p0 x) in 
    pointEqual (pointAdd #c (f a) b) (pointAdd #c (f b) a))

let lemma_commutativity_extended #c p0 a b = 
  let f = (fun x -> pointAdd #c p0 x) in 
  curve_commutativity_lemma #c  (f b) a;
  curve_commutativity_lemma #c  (pointAdd #c  a (pointAdd #c  p0 b)) (pointAdd #c  (pointAdd #c  p0 b) a);
  curve_commutativity_lemma #c  p0 a; 
  curve_compatibility_with_translation_lemma #c  (pointAdd #c  p0 a) (pointAdd #c  a p0) b;
  curve_associativity #c  a p0 b
  

val lemma_point_add_minus_plus_same_value: #c: curve -> p0: point_nat_prime #c -> pk: nat -> qk: nat -> i: nat -> Lemma ( 
  let p = pointAdd  #c  (point_mult #c  pk p0) (point_mult #c  qk p0) in 
  let p_i = pointAdd #c  (point_mult #c  (pk - i) p0) (point_mult #c  (qk + i) p0) in 
  pointEqual p p_i)

let rec lemma_point_add_minus_plus_same_value #curve  p0 pk qk i = 
   match i with 
  |0 -> ()
  | _ -> 
    let o = getOrder #curve in  
    let f = (fun x -> pointAdd #curve  p0 x) in 
    lemma_point_add_minus_plus_same_value #curve  p0 pk qk (i - 1);
    
    let p = pointAdd #curve  (point_mult #curve  pk p0) (point_mult #curve  qk p0) in 
  
    let a = (point_mult #curve  (pk - i + 1) p0) in 
    let b = (point_mult #curve  (qk + i - 1) p0) in 
    let c = (point_mult #curve  (pk - i) p0) in 
    let d = (point_mult #curve  (qk + i) p0) in 

    let p_i1 = pointAdd #curve   (point_mult #curve  (pk - i + 1) p0) (point_mult #curve  (qk + i - 1) p0) in 
    let p_i = pointAdd #curve   (point_mult #curve  (pk - i) p0) (point_mult #curve  (qk + i) p0) in  

    point_mult_def #curve  (pk - i + 1) p0;
    point_mult_def #curve  (pk - i) p0;
    unfold_repeat o f p0 ((pk - i - 1) % o);

    curve_point_at_infinity_property #curve  p0;
    curve_commutativity_lemma #curve  pointAtInfinity  p0;
 
    if ((pk - i - 1) % o + 1) < o then 
      begin
	small_mod (((pk - i - 1) % o + 1)) o;
	lemma_mod_add_distr 1 (pk - i - 1) o;
	assert(pointEqual a (f c))
      end 
    else begin
      curve_multiplication_by_order #curve  ((pk - i - 1) % o) p0;
      lemma_mod_add_distr 1 (pk - i - 1) o;
      eq_repeat0 f p0;
      assert(pointEqual a (f c))
    end;

    point_mult_def #curve  (qk + i - 1) p0;
    point_mult_def #curve  (qk + i) p0;
    unfold_repeat o f p0 ((qk + i - 1 - 1) % o);

    if ((qk + i - 1 - 1) % o + 1) < o then 
      begin 
	small_mod (((qk + i - 1 - 1) % o) + 1) o;
	lemma_mod_add_distr 1 (qk + i - 2) o;
	assert (pointEqual d (f b))
      end
    else begin 
      assert((qk + i - 1 - 1) % o + 1 = o);
      lemma_mod_add_distr 1 (qk + i - 1 - 1) o;
      curve_multiplication_by_order #curve  ((qk + i - 1 - 1) % o) p0;
      lemma_mod_twice (qk + i - 1 - 1) o;
      eq_repeat0 f p0;
      assert (pointEqual (repeat (((qk + i) - 1) % o) f p0) (f pointAtInfinity))
    end;

    curve_compatibility_with_translation_lemma #curve  a (f c) b;
    curve_compatibility_with_translation_lemma #curve  d (f b) c;
    curve_commutativity_lemma #curve  d c;

    lemma_commutativity_extended #curve  p0 c b;
    assert(pointEqual p_i1 p_i)


val lemmaApplPointDouble: #c: curve 
  -> p0: point_nat_prime #c 
  -> pk: int
  -> p: point_nat_prime #c {pointEqual p (point_mult #c  pk p0)} ->
  Lemma (pointEqual (pointAdd #c  p p) (point_mult #c  (2 * pk) p0))

let lemmaApplPointDouble #c  p0 pk p =  
  let o = getOrder #c in 

  let pk_p = point_mult  #c   pk p0 in 

  lemma_point_add_minus_plus_same_value  #c   p0 (pk % o) (pk % o) ((pk - 1) % o);
  lemma_scalar_reduce  #c   p0 pk;

  calc (==) {
    point_mult  #c   (pk % o - ((pk - 1) % o)) p0;
    (==) {lemma_scalar_reduce  #c   p0 (pk % o - ((pk - 1) % o))}
    point_mult  #c   ((pk % o - ((pk - 1) % o)) % o) p0;
    (==) {lemma_mod_add_distr (- ((pk - 1) % o)) pk o}
    point_mult  #c   ((pk - ((pk - 1) % o)) % o) p0;
    (==) {lemma_mod_sub_distr pk (pk - 1) o}
    point_mult  #c   ((pk - (pk - 1)) % o) p0;
    (==) {lemma_scalar_reduce  #c   p0 (pk - (pk - 1))}
    point_mult  #c   ((pk - (pk - 1))) p0;
  };

  calc (==) {
    point_mult  #c   (pk % o + ((pk - 1) % o)) p0;
    (==) {lemma_scalar_reduce  #c   p0 (pk % o + ((pk - 1) % o))}
    point_mult  #c   ((pk % o + ((pk - 1) % o)) % o) p0;
    (==) {lemma_mod_add_distr ((pk - 1) % o) pk o}
    point_mult  #c   ((pk + ((pk - 1) % o)) % o) p0;
    (==) {lemma_mod_add_distr pk (pk - 1) o}
    point_mult  #c   ((pk + pk - 1) % o) p0;
    (==) {lemma_scalar_reduce  #c   p0 (pk + pk - 1)}
    point_mult  #c   ((pk + pk - 1)) p0;
    
  };
    
  point_mult_1  #c   p0;
  point_mult_ext  #c   (2 * pk - 1) p0;

  curve_compatibility_with_translation_lemma #c  p pk_p pk_p


val lemmaApplPointAdd: #c: curve -> p0: point_nat_prime #c 
  -> pk: int
  -> p: point_nat_prime #c {pointEqual p (point_mult #c  pk p0)} 
  -> qk: int
  -> q: point_nat_prime #c {pointEqual q (point_mult #c  qk p0)} -> 
  Lemma (pointEqual (pointAdd #c p q) (point_mult #c (pk + qk) p0))

let lemmaApplPointAdd #c  p0 pk p qk q = 
  let pk_p = point_mult  #c  pk p0 in 
  let qk_p = point_mult  #c  qk p0 in 
  
  let o = getOrder #c in 

  lemma_point_add_minus_plus_same_value #c  p0 (qk % o) (pk % o) ((qk - 1) % o);
  lemma_scalar_reduce  #c  p0 qk;
  lemma_scalar_reduce  #c  p0 pk;

  assert(pointEqual (pointAdd #c (point_mult  #c  qk p0) (point_mult  #c  pk p0)) 
    (pointAdd #c  (point_mult  #c  ((qk % o) - ((qk - 1) % o)) p0) (point_mult  #c  (pk % o + ((qk - 1) % o)) p0)));

  calc (==) {
    point_mult  #c  ((qk % o) - ((qk - 1) % o)) p0;
    (==) {lemma_scalar_reduce   #c  p0 ((qk % o) - ((qk - 1) % o))}
    point_mult  #c  (((qk % o) - ((qk - 1) % o)) % o) p0;
    (==) {lemma_mod_add_distr (- ((qk - 1) % o)) qk o}
    point_mult  #c  ((qk - ((qk - 1) % o)) % o) p0;
    (==) {lemma_mod_sub_distr qk (qk - 1) o}
    point_mult  #c  ((qk - qk + 1) % o) p0;
    (==) {lemma_scalar_reduce  #c  p0 1}
    point_mult  #c  1 p0;
  };

  calc (==) {
    point_mult  #c  (pk % o + ((qk - 1) % o)) p0;
    (==) {lemma_scalar_reduce  #c  p0 (pk % o + ((qk - 1) % o))}
    point_mult  #c  ((pk % o + ((qk - 1) % o)) % o) p0;    
    (==) {lemma_mod_add_distr ((qk - 1) % o) pk o}
    point_mult  #c  ((pk + ((qk - 1) % o)) % o) p0;    
    (==) {lemma_mod_add_distr pk (qk - 1) o}
    point_mult  #c  ((pk + qk - 1) % o) p0;    
    (==) {lemma_scalar_reduce  #c  p0 (pk + qk - 1)}
    point_mult  #c  (pk + qk - 1) p0;  
  };

  point_mult_1  #c  p0;
  
  point_mult_ext  #c  (pk + qk - 1) p0; 
  curve_compatibility_with_translation_lemma  #c  p pk_p qk_p;
  curve_compatibility_with_translation_lemma  #c  q qk_p p;
   
  curve_commutativity_lemma  #c  pk_p qk_p;
  curve_commutativity_lemma  #c  p qk_p;
  curve_commutativity_lemma  #c  q p



val mlStep0AsPointAdd: #c: curve 
  -> p0: point_nat_prime #c 
  -> pk: nat
  -> p: point_nat_prime #c {pointEqual p (point_mult #c pk p0)}  
  -> qk: nat 
  -> q: point_nat_prime #c {pointEqual q (point_mult #c qk p0)} -> 
  Lemma
  (ensures (
    let p_i, q_i = _ml_step0 p q in 
    pointEqual p_i (point_mult #c  (pk + qk) p0) /\
    pointEqual q_i (point_mult #c (2 * qk) p0)))


let mlStep0AsPointAdd #c p0 p_k p q_k q = 
  curve_commutativity_lemma #c  p q; 
  lemmaApplPointAdd #c  p0 p_k p q_k q;
  lemmaApplPointDouble #c  p0 q_k q


val mlStep1AsPointAdd: #c: curve
  -> p0: point_nat_prime #c
  -> pk: nat
  -> p: point_nat_prime #c {pointEqual p (point_mult #c  pk p0)} 
  -> qk: nat 
  -> q: point_nat_prime #c {pointEqual q (point_mult #c  qk p0)} -> 
  Lemma
  (ensures (
    let p_i, q_i = _ml_step1 p q in 
    pointEqual q_i (point_mult #c  (pk + qk) p0) /\ 
    pointEqual p_i (point_mult #c  (2 * pk) p0)))
      
let mlStep1AsPointAdd #c p0 pk p qk q = 
  lemmaApplPointAdd #c  p0 pk p qk q;
  lemmaApplPointDouble #c  p0 pk p


val point_mult_0_lemma: #c: curve -> p: point_nat_prime #c ->  Lemma (point_mult #c  1 p == p)

let point_mult_0_lemma #c p = 
  Lib.LoopCombinators.eq_repeat0 (fun x -> pointAdd #c  p x) p 


val scalar_as_nat_: #c: curve -> scalar_bytes #c -> i: nat {i <= v (getScalarLen c)} -> nat

let rec scalar_as_nat_ #c s i = 
  if i = 0 then 0 else 
  let bit = ith_bit s (v (getScalarLen c) - i) in 
  scalar_as_nat_ #c s (i - 1) * 2 + uint_to_nat bit 

#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"

val scalar_as_nat_zero: #c: curve -> s: scalar_bytes #c -> Lemma (scalar_as_nat_ s 0 == 0)

let scalar_as_nat_zero #c s = ()


val scalar_as_nat_def: #c: curve -> s: scalar_bytes #c -> i: nat {i > 0 /\ i <= v (getScalarLen c)} -> Lemma (
  scalar_as_nat_ #c s i == 2 * scalar_as_nat_ #c s (i - 1) + uint_to_nat (ith_bit s (v (getScalarLen c) - i)))

let scalar_as_nat_def #c s i = ()


val scalar_as_nat: #c: curve -> scalar_bytes #c -> nat

let scalar_as_nat #c s = scalar_as_nat_ #c s (v (getScalarLen c))


val scalar_as_nat_upperbound: #c: curve -> s: scalar_bytes #c -> i: nat {i <= v (getScalarLen c)} -> Lemma
  (scalar_as_nat_ s i < pow2 i)

let rec scalar_as_nat_upperbound #c s i = 
  match i with 
  |0 -> scalar_as_nat_zero s
  |_ -> 
    scalar_as_nat_upperbound #c s (i - 1);
    scalar_as_nat_def s i;
    pow2_double_sum (i - 1)



val pred0: #c: curve -> x: tuple2 (point_nat_prime #c) (point_nat_prime #c) -> s: scalar_bytes 
  -> p: tuple2 (point_nat_prime #c) (point_nat_prime #c) -> i: nat {i < v (getScalarLen c)} ->
  Lemma (
    let f = _ml_step #c s in 
    let pred i (p_i, q_i) =  
      let p0, q0 = p in 
      let si = scalar_as_nat_ #c s i in 
      let si1 = scalar_as_nat_ #c s i + 1 in
      pointEqual p_i (point_mult #c si q0) /\ pointEqual q_i (point_mult #c si1 q0)  in 
    (pred i x ==> pred (i + 1) (f i x))) 

let pred0 #c (pi, qi) s (p0, q0) i = 
  let f = _ml_step #c s in 
  let si = scalar_as_nat_ #c s i in 
  let si1 = scalar_as_nat_ #c s i + 1 in  
  if pointEqual pi (point_mult #c si q0) && pointEqual qi (point_mult #c si1 q0) then 
    let pi1, qi1 = f i (pi, qi) in 
    mlStep0AsPointAdd q0 si pi si1 qi;
    mlStep1AsPointAdd q0 si pi si1 qi;
    scalar_as_nat_def s (i + 1)
  else ()

   
val lemma_predicate0:  #c: curve ->  s: scalar_bytes 
  -> p: tuple2 (point_nat_prime #c) (point_nat_prime #c) -> 
  Lemma (
    let f = _ml_step #c s in 
    let pred i r =  
      let p_i, q_i = r in 
      let p0, q0 = p in 
      let si = scalar_as_nat_ #c s i in 
      let si1 = scalar_as_nat_ #c s i + 1 in
      pointEqual p_i (point_mult #c  si q0) /\ pointEqual q_i (point_mult #c  si1 q0)  in 
    forall (i:nat{i < v (getScalarLen c)}) (x: (tuple2 (point_nat_prime #c) (point_nat_prime #c))). 
    pred i x ==> pred (i + 1) (f i x))

let lemma_predicate0 #c s p = 
  let pred = pred0 #c in
  Classical.forall_intro_4 pred


val montgomery_ladder_spec_left: #c: curve -> s: scalar_bytes #c 
  -> i: tuple2 (point_nat_prime #c) (point_nat_prime #c) {let i0, i1 = i in pointEqual i0 (point_mult #c  0 i1)} 
  -> r: tuple2 (point_nat_prime #c) (point_nat_prime #c) {
    let r0, r1 = r in let i0, i1 = i in
    r == repeati (v (getScalarLen c)) (_ml_step #c s) i /\
    pointEqual r0 (point_mult #c  (scalar_as_nat #c s) i1)}

let montgomery_ladder_spec_left #c s (p0, q0) = 
  let len : nat  = v (getScalarLen c) in 
  
  let pred i r = (
    let p_i, q_i = r in 
    let si = scalar_as_nat_ #c s i in 
    let si1 = scalar_as_nat_ #c s i + 1 in
    pointEqual p_i (point_mult #c  si q0) /\
    pointEqual q_i (point_mult #c  si1 q0)) in
    
  let f = _ml_step #c s in 

  lemma_predicate0 s (p0, q0);
  point_mult_0_lemma #c q0;
  repeati_inductive' #(tuple2 (point_nat_prime #c) (point_nat_prime #c)) len pred f (p0, q0)

#pop-options


val scalar_multiplication: #c: curve -> scalar_bytes #c -> 
  p: point_nat_prime #c {~ (isPointAtInfinity #Jacobian  p)} -> 
  point_nat_prime #c 

let scalar_multiplication #c k p =
  point_mult #c  (scalar_as_nat #c k) p

val secret_to_public: #c: curve -> scalar_bytes #c -> point_nat_prime #c

let secret_to_public #c k =
  point_mult #c  (scalar_as_nat #c k) (basePoint #c)

