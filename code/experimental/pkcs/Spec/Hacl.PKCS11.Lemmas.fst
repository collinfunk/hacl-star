module Hacl.PKCS11.Lemmas

open FStar.Seq


val lemmaFindLExistIfSome: #a: Type -> f: (a -> bool) -> s: seq a -> Lemma
  (requires (Some? (find_l f s)))
  (ensures (exists (a: nat {a < Seq.length s}). f (index s a)))
  (decreases (Seq.length s))


let rec lemmaFindLExistIfSome #a f s = 
  if length s = 0 then ()
  else if length s = 1 then () 
  else
    if f (head s) then ()
    else
    lemmaFindLExistIfSome f (tail s)


assume val lemmaIfExistAndNotFirst: #a: Type -> f: (a -> bool) -> s: seq a -> 
  Lemma 
    (requires (exists (a: nat {a < Seq.length s}). f (index s a) /\ not (f (head s))))
    (ensures (exists (a: nat {a < Seq.length (tail s)}). f (index (tail s) a)))


val lemmaFindLExistIfSomeOp: #a: Type -> f: (a -> bool) -> s: seq a -> 
  Lemma
    (requires (exists (a: nat {a < Seq.length s}). f (index s a)))
    (ensures (Some? (find_l f s)))
    (decreases (Seq.length s))


let rec lemmaFindLExistIfSomeOp #a f s  = 
  if Seq.length s = 0 then ()
  else
    if f (head s) then ()
    else
      begin
	lemmaIfExistAndNotFirst f s;
	lemmaFindLExistIfSomeOp #a f (tail s)
    end
