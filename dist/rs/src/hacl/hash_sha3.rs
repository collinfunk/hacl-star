pub fn update_multi_sha3(
  a: crate::spec::hash::definitions::hash_alg,
  s: &mut [u64],
  blocks: &mut [u8],
  n_blocks: u32
) ->
  ()
for i in 0u32..i
{
  let block: (&mut [u8], &mut [u8]) =
    blocks.split_at_mut(
      (i.wrapping_mul(crate::hacl::hash::sha3::block_len(a)) as usize).wrapping_add(0usize)
    );
  crate::hacl::impl::sha3::absorb_inner(crate::hacl::hash::sha3::block_len(a), block.1, s)
}

fn op_Bang_Star__Hacl_Streaming_Functor_state_s Spec_Hash_Definitions_hash_alg *  uint64_t* ()(
  p: &mut [crate::hacl::streaming::keccak::state]
) ->
  crate::hacl::streaming::keccak::state
{ p[0u32 as usize] }

fn
op_Star_Equals__Hacl_Streaming_Functor_state_s Spec_Hash_Definitions_hash_alg *  uint64_t* ()(
  p: &mut [crate::hacl::streaming::keccak::state],
  v: crate::hacl::streaming::keccak::state
) ->
  ()
{ p[0u32 as usize] = v }

pub fn block_len(s: &mut [crate::hacl::streaming::keccak::state]) -> u32
{
  let a1: crate::spec::hash::definitions::hash_alg = crate::hacl::streaming::keccak::get_alg(s);
  crate::hacl::hash::sha3::block_len(a1)
}

pub fn hash_len(s: &mut [crate::hacl::streaming::keccak::state]) -> u32
{
  let a1: crate::spec::hash::definitions::hash_alg = crate::hacl::streaming::keccak::get_alg(s);
  crate::hacl::hash::sha3::hash_len(a1)
}

pub fn shake128_hacl(
  inputByteLen: u32,
  input: &mut [u8],
  outputByteLen: u32,
  output: &mut [u8]
) ->
  ()
{
  crate::hacl::impl::sha3::keccak(
    1344u32,
    256u32,
    inputByteLen,
    input,
    0x1Fu8,
    outputByteLen,
    output
  )
}

pub fn shake256_hacl(
  inputByteLen: u32,
  input: &mut [u8],
  outputByteLen: u32,
  output: &mut [u8]
) ->
  ()
{
  crate::hacl::impl::sha3::keccak(
    1088u32,
    512u32,
    inputByteLen,
    input,
    0x1Fu8,
    outputByteLen,
    output
  )
}

pub fn sha3_224(inputByteLen: u32, input: &mut [u8], output: &mut [u8]) -> ()
{
  crate::hacl::impl::sha3::keccak(1152u32, 448u32, inputByteLen, input, 0x06u8, 28u32, output)
}

pub fn sha3_256(inputByteLen: u32, input: &mut [u8], output: &mut [u8]) -> ()
{
  crate::hacl::impl::sha3::keccak(1088u32, 512u32, inputByteLen, input, 0x06u8, 32u32, output)
}

pub fn sha3_384(inputByteLen: u32, input: &mut [u8], output: &mut [u8]) -> ()
{ crate::hacl::impl::sha3::keccak(832u32, 768u32, inputByteLen, input, 0x06u8, 48u32, output) }

pub fn sha3_512(inputByteLen: u32, input: &mut [u8], output: &mut [u8]) -> ()
{
  crate::hacl::impl::sha3::keccak(576u32, 1024u32, inputByteLen, input, 0x06u8, 64u32, output)
}

const keccak_rotc: [u32; 24] =
  [1u32,
    3u32,
    6u32,
    10u32,
    15u32,
    21u32,
    28u32,
    36u32,
    45u32,
    55u32,
    2u32,
    14u32,
    27u32,
    41u32,
    56u32,
    8u32,
    25u32,
    43u32,
    62u32,
    18u32,
    39u32,
    61u32,
    20u32,
    44u32];

const keccak_piln: [u32; 24] =
  [10u32,
    7u32,
    11u32,
    17u32,
    18u32,
    3u32,
    5u32,
    16u32,
    8u32,
    21u32,
    24u32,
    4u32,
    15u32,
    23u32,
    19u32,
    13u32,
    12u32,
    2u32,
    20u32,
    14u32,
    22u32,
    9u32,
    6u32,
    1u32];

const keccak_rndc: [u64; 24] =
  [0x0000000000000001u64,
    0x0000000000008082u64,
    0x800000000000808au64,
    0x8000000080008000u64,
    0x000000000000808bu64,
    0x0000000080000001u64,
    0x8000000080008081u64,
    0x8000000000008009u64,
    0x000000000000008au64,
    0x0000000000000088u64,
    0x0000000080008009u64,
    0x000000008000000au64,
    0x000000008000808bu64,
    0x800000000000008bu64,
    0x8000000000008089u64,
    0x8000000000008003u64,
    0x8000000000008002u64,
    0x8000000000000080u64,
    0x000000000000800au64,
    0x800000008000000au64,
    0x8000000080008081u64,
    0x8000000000008080u64,
    0x0000000080000001u64,
    0x8000000080008008u64];

pub fn state_permute(s: &mut [u64]) -> ()
for i in 0u32..24u32
{
  let mut _C: [u64; 5] = [0u64; 5u32 as usize];
  for i in 0u32..5u32
  {
    (&mut _C)[i as usize] =
      s[i.wrapping_add(0u32) as usize]
      ^
      (s[i.wrapping_add(5u32) as usize]
      ^
      (s[i.wrapping_add(10u32) as usize]
      ^
      (s[i.wrapping_add(15u32) as usize] ^ s[i.wrapping_add(20u32) as usize])))
  };
  for i in 0u32..5u32
  {
    let uu____0: u64 = (&mut _C)[i.wrapping_add(1u32).wrapping_rem(5u32) as usize];
    let _D: u64 =
      (&mut _C)[i.wrapping_add(4u32).wrapping_rem(5u32) as usize]
      ^
      (uu____0.wrapping_shl(1u32) | uu____0.wrapping_shr(63u32));
    for i in 0u32..5u32
    {
      s[i.wrapping_add(5u32.wrapping_mul(i)) as usize] =
        s[i.wrapping_add(5u32.wrapping_mul(i)) as usize] ^ _D
    }
  };
  let x: u64 = s[1u32 as usize];
  let mut current: u64 = x;
  for i in 0u32..24u32
  {
    let _Y: u32 = (&keccak_piln)[i as usize];
    let r: u32 = (&keccak_rotc)[i as usize];
    let temp: u64 = s[_Y as usize];
    let uu____1: u64 = current;
    s[_Y as usize] = uu____1.wrapping_shl(r) | uu____1.wrapping_shr(64u32.wrapping_sub(r));
    current = temp
  };
  for i in 0u32..5u32
  {
    let v0: u64 =
      s[0u32.wrapping_add(5u32.wrapping_mul(i)) as usize]
      ^
      ! s[1u32.wrapping_add(5u32.wrapping_mul(i)) as usize]
      &
      s[2u32.wrapping_add(5u32.wrapping_mul(i)) as usize];
    let v1: u64 =
      s[1u32.wrapping_add(5u32.wrapping_mul(i)) as usize]
      ^
      ! s[2u32.wrapping_add(5u32.wrapping_mul(i)) as usize]
      &
      s[3u32.wrapping_add(5u32.wrapping_mul(i)) as usize];
    let v2: u64 =
      s[2u32.wrapping_add(5u32.wrapping_mul(i)) as usize]
      ^
      ! s[3u32.wrapping_add(5u32.wrapping_mul(i)) as usize]
      &
      s[4u32.wrapping_add(5u32.wrapping_mul(i)) as usize];
    let v3: u64 =
      s[3u32.wrapping_add(5u32.wrapping_mul(i)) as usize]
      ^
      ! s[4u32.wrapping_add(5u32.wrapping_mul(i)) as usize]
      &
      s[0u32.wrapping_add(5u32.wrapping_mul(i)) as usize];
    let v4: u64 =
      s[4u32.wrapping_add(5u32.wrapping_mul(i)) as usize]
      ^
      ! s[0u32.wrapping_add(5u32.wrapping_mul(i)) as usize]
      &
      s[1u32.wrapping_add(5u32.wrapping_mul(i)) as usize];
    s[0u32.wrapping_add(5u32.wrapping_mul(i)) as usize] = v0;
    s[1u32.wrapping_add(5u32.wrapping_mul(i)) as usize] = v1;
    s[2u32.wrapping_add(5u32.wrapping_mul(i)) as usize] = v2;
    s[3u32.wrapping_add(5u32.wrapping_mul(i)) as usize] = v3;
    s[4u32.wrapping_add(5u32.wrapping_mul(i)) as usize] = v4
  };
  let c: u64 = (&keccak_rndc)[i as usize];
  s[0u32 as usize] = s[0u32 as usize] ^ c
}

pub fn loadState(rateInBytes: u32, input: &mut [u8], s: &mut [u64]) -> ()
{
  let mut block: [u8; 200] = [0u8; 200u32 as usize];
  ((&mut block)[0u32 as usize..0u32 as usize + rateInBytes as usize]).copy_from_slice(
    &input[0u32 as usize..0u32 as usize + rateInBytes as usize]
  );
  for i in 0u32..25u32
  {
    let u: u64 =
      crate::lowstar::endianness::load64_le(&mut (&mut block)[i.wrapping_mul(8u32) as usize..]);
    let x: u64 = u;
    s[i as usize] = s[i as usize] ^ x
  }
}

fn storeState(rateInBytes: u32, s: &mut [u64], res: &mut [u8]) -> ()
{
  let mut block: [u8; 200] = [0u8; 200u32 as usize];
  for i in 0u32..25u32
  {
    let sj: u64 = s[i as usize];
    crate::lowstar::endianness::store64_le(&mut (&mut block)[i.wrapping_mul(8u32) as usize..], sj)
  };
  (res[0u32 as usize..0u32 as usize + rateInBytes as usize]).copy_from_slice(
    &(&mut (&mut block)[0u32 as usize..])[0u32 as usize..0u32 as usize + rateInBytes as usize]
  )
}

pub fn absorb_inner(rateInBytes: u32, block: &mut [u8], s: &mut [u64]) -> ()
{
  loadState(rateInBytes, block, s);
  state_permute(s)
}

fn absorb(
  s: &mut [u64],
  rateInBytes: u32,
  inputByteLen: u32,
  input: &mut [u8],
  delimitedSuffix: u8
) ->
  ()
{
  let n_blocks: u32 = inputByteLen.wrapping_div(rateInBytes);
  let rem: u32 = inputByteLen.wrapping_rem(rateInBytes);
  for i in 0u32..rem
  {
    let block: (&mut [u8], &mut [u8]) =
      input.split_at_mut((i.wrapping_mul(rateInBytes) as usize).wrapping_add(0usize));
    absorb_inner(rateInBytes, block.1, s)
  };
  let last: (&mut [u8], &mut [u8]) =
    input.split_at_mut((n_blocks.wrapping_mul(rateInBytes) as usize).wrapping_add(0usize));
  let mut lastBlock_: [u8; 200] = [0u8; 200u32 as usize];
  let lastBlock: (&mut [u8], &mut [u8]) = (&mut lastBlock_).split_at_mut(0usize);
  (lastBlock.1[0u32 as usize..0u32 as usize + rem as usize]).copy_from_slice(
    &last.1[0u32 as usize..0u32 as usize + rem as usize]
  );
  lastBlock.1[rem as usize] = delimitedSuffix;
  loadState(rateInBytes, lastBlock.1, s);
  if ! delimitedSuffix & 0x80u8 == 0u8 && rem == rateInBytes.wrapping_sub(1u32)
  { state_permute(s) };
  let mut nextBlock_: [u8; 200] = [0u8; 200u32 as usize];
  let nextBlock: (&mut [u8], &mut [u8]) = (&mut nextBlock_).split_at_mut(0usize);
  nextBlock.1[rateInBytes.wrapping_sub(1u32) as usize] = 0x80u8;
  loadState(rateInBytes, nextBlock.1, s);
  state_permute(s)
}