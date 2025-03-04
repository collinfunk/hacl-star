module Hacl.Streaming.Blake2.Types

module Common = Hacl.Streaming.Blake2.Common
module Core = Hacl.Impl.Blake2.Core
module G = FStar.Ghost
module Spec = Spec.Blake2

// This file is carefully engineered to be early on in the dependency graph, as to generate clean
// monomorphizations of these types, with the proper names and attributes.
//
// There is a really unpleasant difficulty here.
// 1. krml eliminates unused definitions early on, so we absolutely have to make sure every
//    declaration in this file is referenced by another file, otherwise, our efforts will be in
//    vain: essentially, our intention (by "claiming" a type abbreviation here with a suitable name
//    and suitable flags) is that it *should* create a dependency from e.g.
//    Hacl.Streaming.Blake2b_32 to this file, knowing that post-monomorphization the latter will
//    reference the type declared in the former, BUT because monomorphization happens after unused
//    declaration elimination, Hacl.Streaming.Blake2b_32 MUST refer to the actual names in this file
// 2. some of these declarations need to remain private (the optional_ one) because otherwise they
//    would refer to an incomplete struct type
// 3. this file does not have an fsti because then Hacl.Streaming.Blake2b_32 would need to friend it
//    and in turn would need an interface itself! arggghh!!
// 4. because of what I think is a bug in krml (or is it?), the pairs of vec* are inserted here
//    because they appear in the vectorized states below, but they end up with the wrong visibility by
//    default... should monomorphic instances be generated as private?

[@ CAbstractStruct ]
let block_state_blake2b_32 (kk: G.erased (Common.index Spec.Blake2B)) =
  Common.s Spec.Blake2B kk Core.M32

[@ CAbstractStruct ]
let block_state_blake2s_32 (kk: G.erased (Common.index Spec.Blake2S)) =
  Common.s Spec.Blake2S kk Core.M32


// These now in separate headers to avoid a regular algorithm (e.g. SHA1) including a header that contains references to vec128.
include Hacl.Streaming.Blake2.Types.Simd128
include Hacl.Streaming.Blake2.Types.Simd256
