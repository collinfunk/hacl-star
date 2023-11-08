pub fn add(a: &mut [u64], b: &mut [u64], res: &mut [u64]) -> u64
{
  let mut c: u64 = 0u64;
  for i in 0u32..16u32
  {
    let t1: u64 = a[4u32.wrapping_mul(i) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i) as usize];
    let res_i: (&mut [u64], &mut [u64]) =
      res.split_at_mut((4u32.wrapping_mul(i) as usize).wrapping_add(0usize));
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1);
    let t1: u64 = a[4u32.wrapping_mul(i).wrapping_add(1u32) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i).wrapping_add(1u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1);
    let t1: u64 = a[4u32.wrapping_mul(i).wrapping_add(2u32) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i).wrapping_add(2u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1);
    let t1: u64 = a[4u32.wrapping_mul(i).wrapping_add(3u32) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i).wrapping_add(3u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1)
  };
  for i in 64u32..64u32
  {
    let t1: u64 = a[i as usize];
    let t2: u64 = b[i as usize];
    let res_i: (&mut [u64], &mut [u64]) = res.split_at_mut((i as usize).wrapping_add(0usize));
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1)
  };
  c
}

pub fn sub(a: &mut [u64], b: &mut [u64], res: &mut [u64]) -> u64
{
  let mut c: u64 = 0u64;
  for i in 0u32..16u32
  {
    let t1: u64 = a[4u32.wrapping_mul(i) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i) as usize];
    let res_i: (&mut [u64], &mut [u64]) =
      res.split_at_mut((4u32.wrapping_mul(i) as usize).wrapping_add(0usize));
    c = crate::lib::inttypes::intrinsics::sub_borrow_u64(c, t1, t2, res_i.1);
    let t1: u64 = a[4u32.wrapping_mul(i).wrapping_add(1u32) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i).wrapping_add(1u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c = crate::lib::inttypes::intrinsics::sub_borrow_u64(c, t1, t2, res_i.1);
    let t1: u64 = a[4u32.wrapping_mul(i).wrapping_add(2u32) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i).wrapping_add(2u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c = crate::lib::inttypes::intrinsics::sub_borrow_u64(c, t1, t2, res_i.1);
    let t1: u64 = a[4u32.wrapping_mul(i).wrapping_add(3u32) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i).wrapping_add(3u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c = crate::lib::inttypes::intrinsics::sub_borrow_u64(c, t1, t2, res_i.1)
  };
  for i in 64u32..64u32
  {
    let t1: u64 = a[i as usize];
    let t2: u64 = b[i as usize];
    let res_i: (&mut [u64], &mut [u64]) = res.split_at_mut((i as usize).wrapping_add(0usize));
    c = crate::lib::inttypes::intrinsics::sub_borrow_u64(c, t1, t2, res_i.1)
  };
  c
}

pub fn add_mod(n: &mut [u64], a: &mut [u64], b: &mut [u64], res: &mut [u64]) -> ()
{
  let mut c: u64 = 0u64;
  for i in 0u32..16u32
  {
    let t1: u64 = a[4u32.wrapping_mul(i) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i) as usize];
    let res_i: (&mut [u64], &mut [u64]) =
      res.split_at_mut((4u32.wrapping_mul(i) as usize).wrapping_add(0usize));
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1);
    let t1: u64 = a[4u32.wrapping_mul(i).wrapping_add(1u32) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i).wrapping_add(1u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1);
    let t1: u64 = a[4u32.wrapping_mul(i).wrapping_add(2u32) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i).wrapping_add(2u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1);
    let t1: u64 = a[4u32.wrapping_mul(i).wrapping_add(3u32) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i).wrapping_add(3u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1)
  };
  for i in 64u32..64u32
  {
    let t1: u64 = a[i as usize];
    let t2: u64 = b[i as usize];
    let res_i: (&mut [u64], &mut [u64]) = res.split_at_mut((i as usize).wrapping_add(0usize));
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1)
  };
  let c0: u64 = c;
  let mut tmp: [u64; 64] = [0u64; 64u32 as usize];
  let mut c: u64 = 0u64;
  for i in 0u32..16u32
  {
    let t1: u64 = res[4u32.wrapping_mul(i) as usize];
    let t2: u64 = n[4u32.wrapping_mul(i) as usize];
    let res_i: (&mut [u64], &mut [u64]) =
      (&mut tmp).split_at_mut((4u32.wrapping_mul(i) as usize).wrapping_add(0usize));
    c = crate::lib::inttypes::intrinsics::sub_borrow_u64(c, t1, t2, res_i.1);
    let t1: u64 = res[4u32.wrapping_mul(i).wrapping_add(1u32) as usize];
    let t2: u64 = n[4u32.wrapping_mul(i).wrapping_add(1u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c = crate::lib::inttypes::intrinsics::sub_borrow_u64(c, t1, t2, res_i.1);
    let t1: u64 = res[4u32.wrapping_mul(i).wrapping_add(2u32) as usize];
    let t2: u64 = n[4u32.wrapping_mul(i).wrapping_add(2u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c = crate::lib::inttypes::intrinsics::sub_borrow_u64(c, t1, t2, res_i.1);
    let t1: u64 = res[4u32.wrapping_mul(i).wrapping_add(3u32) as usize];
    let t2: u64 = n[4u32.wrapping_mul(i).wrapping_add(3u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c = crate::lib::inttypes::intrinsics::sub_borrow_u64(c, t1, t2, res_i.1)
  };
  for i in 64u32..64u32
  {
    let t1: u64 = res[i as usize];
    let t2: u64 = n[i as usize];
    let res_i: (&mut [u64], &mut [u64]) =
      (&mut tmp).split_at_mut((i as usize).wrapping_add(0usize));
    c = crate::lib::inttypes::intrinsics::sub_borrow_u64(c, t1, t2, res_i.1)
  };
  let c1: u64 = c;
  let c: u64 = c0.wrapping_sub(c1);
  for i in 0u32..64u32
  {
    let os: (&mut [u64], &mut [u64]) = res.split_at_mut(0usize);
    let x: u64 = c & os.1[i as usize] | ! c & (&mut tmp)[i as usize];
    os.1[i as usize] = x
  }
}

pub fn mul(a: &mut [u64], b: &mut [u64], res: &mut [u64]) -> ()
{
  let mut tmp: [u64; 256] = [0u64; 256u32 as usize];
  crate::hacl::bignum::karatsuba::bn_karatsuba_mul_uint64(64u32, a, b, &mut tmp, res)
}

pub fn sqr(a: &mut [u64], res: &mut [u64]) -> ()
{
  let mut tmp: [u64; 256] = [0u64; 256u32 as usize];
  crate::hacl::bignum::karatsuba::bn_karatsuba_sqr_uint64(64u32, a, &mut tmp, res)
}

fn reduction(n: &mut [u64], nInv: u64, c: &mut [u64], res: &mut [u64]) -> ()
{
  let mut c0: u64 = 0u64;
  for i in 0u32..64u32
  {
    let qj: u64 = nInv.wrapping_mul(c[i as usize]);
    let res_j: (&mut [u64], &mut [u64]) = c.split_at_mut((i as usize).wrapping_add(0usize));
    let mut c1: u64 = 0u64;
    for i in 0u32..16u32
    {
      let a_i: u64 = n[4u32.wrapping_mul(i) as usize];
      let res_i: (&mut [u64], &mut [u64]) =
        res_j.1.split_at_mut((4u32.wrapping_mul(i) as usize).wrapping_add(0usize));
      c1 = crate::hacl::bignum_base::mul_wide_add2_u64(a_i, qj, c1, res_i.1);
      let a_i: u64 = n[4u32.wrapping_mul(i).wrapping_add(1u32) as usize];
      let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
      c1 = crate::hacl::bignum_base::mul_wide_add2_u64(a_i, qj, c1, res_i.1);
      let a_i: u64 = n[4u32.wrapping_mul(i).wrapping_add(2u32) as usize];
      let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
      c1 = crate::hacl::bignum_base::mul_wide_add2_u64(a_i, qj, c1, res_i.1);
      let a_i: u64 = n[4u32.wrapping_mul(i).wrapping_add(3u32) as usize];
      let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
      c1 = crate::hacl::bignum_base::mul_wide_add2_u64(a_i, qj, c1, res_i.1)
    };
    for i in 64u32..64u32
    {
      let a_i: u64 = n[i as usize];
      let res_i: (&mut [u64], &mut [u64]) = res_j.1.split_at_mut((i as usize).wrapping_add(0usize));
      c1 = crate::hacl::bignum_base::mul_wide_add2_u64(a_i, qj, c1, res_i.1)
    };
    let r: u64 = c1;
    let c1: u64 = r;
    let resb: (&mut [u64], &mut [u64]) = res_j.1.split_at_mut(64usize);
    let res_j: u64 = resb.0[64u32.wrapping_add(i) as usize];
    c0 = crate::lib::inttypes::intrinsics::add_carry_u64(c0, c1, res_j, resb.1)
  };
  (res[0u32 as usize..0u32 as usize + 64u32 as usize]).copy_from_slice(
    &(&mut c[64u32 as usize..])[0u32 as usize..0u32 as usize + 64u32 as usize]
  );
  let c0: u64 = c0;
  let mut tmp: [u64; 64] = [0u64; 64u32 as usize];
  let mut c1: u64 = 0u64;
  for i in 0u32..16u32
  {
    let t1: u64 = res[4u32.wrapping_mul(i) as usize];
    let t2: u64 = n[4u32.wrapping_mul(i) as usize];
    let res_i: (&mut [u64], &mut [u64]) =
      (&mut tmp).split_at_mut((4u32.wrapping_mul(i) as usize).wrapping_add(0usize));
    c1 = crate::lib::inttypes::intrinsics::sub_borrow_u64(c1, t1, t2, res_i.1);
    let t1: u64 = res[4u32.wrapping_mul(i).wrapping_add(1u32) as usize];
    let t2: u64 = n[4u32.wrapping_mul(i).wrapping_add(1u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c1 = crate::lib::inttypes::intrinsics::sub_borrow_u64(c1, t1, t2, res_i.1);
    let t1: u64 = res[4u32.wrapping_mul(i).wrapping_add(2u32) as usize];
    let t2: u64 = n[4u32.wrapping_mul(i).wrapping_add(2u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c1 = crate::lib::inttypes::intrinsics::sub_borrow_u64(c1, t1, t2, res_i.1);
    let t1: u64 = res[4u32.wrapping_mul(i).wrapping_add(3u32) as usize];
    let t2: u64 = n[4u32.wrapping_mul(i).wrapping_add(3u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c1 = crate::lib::inttypes::intrinsics::sub_borrow_u64(c1, t1, t2, res_i.1)
  };
  for i in 64u32..64u32
  {
    let t1: u64 = res[i as usize];
    let t2: u64 = n[i as usize];
    let res_i: (&mut [u64], &mut [u64]) =
      (&mut tmp).split_at_mut((i as usize).wrapping_add(0usize));
    c1 = crate::lib::inttypes::intrinsics::sub_borrow_u64(c1, t1, t2, res_i.1)
  };
  let c1: u64 = c1;
  let c2: u64 = c0.wrapping_sub(c1);
  for i in 0u32..64u32
  {
    let os: (&mut [u64], &mut [u64]) = res.split_at_mut(0usize);
    let x: u64 = c2 & os.1[i as usize] | ! c2 & (&mut tmp)[i as usize];
    os.1[i as usize] = x
  }
}

fn from(n: &mut [u64], nInv_u64: u64, aM: &mut [u64], a: &mut [u64]) -> ()
{
  let mut tmp: [u64; 128] = [0u64; 128u32 as usize];
  ((&mut tmp)[0u32 as usize..0u32 as usize + 64u32 as usize]).copy_from_slice(
    &aM[0u32 as usize..0u32 as usize + 64u32 as usize]
  );
  reduction(n, nInv_u64, &mut tmp, a)
}

fn amont_mul(n: &mut [u64], nInv_u64: u64, aM: &mut [u64], bM: &mut [u64], resM: &mut [u64]) ->
  ()
{
  let mut c: [u64; 128] = [0u64; 128u32 as usize];
  let mut tmp: [u64; 256] = [0u64; 256u32 as usize];
  crate::hacl::bignum::karatsuba::bn_karatsuba_mul_uint64(64u32, aM, bM, &mut tmp, &mut c);
  areduction(n, nInv_u64, &mut c, resM)
}

fn amont_sqr(n: &mut [u64], nInv_u64: u64, aM: &mut [u64], resM: &mut [u64]) -> ()
{
  let mut c: [u64; 128] = [0u64; 128u32 as usize];
  let mut tmp: [u64; 256] = [0u64; 256u32 as usize];
  crate::hacl::bignum::karatsuba::bn_karatsuba_sqr_uint64(64u32, aM, &mut tmp, &mut c);
  areduction(n, nInv_u64, &mut c, resM)
}

fn exp_vartime(
  nBits: u32,
  n: &mut [u64],
  a: &mut [u64],
  bBits: u32,
  b: &mut [u64],
  res: &mut [u64]
) ->
  ()
{
  let mut r2: [u64; 64] = [0u64; 64u32 as usize];
  precompr2(nBits, n, &mut r2);
  let mu: u64 = crate::hacl::bignum::mod_inv_uint64(n[0u32 as usize]);
  exp_vartime_precomp(n, mu, &mut r2, a, bBits, b, res)
}

fn exp_consttime(
  nBits: u32,
  n: &mut [u64],
  a: &mut [u64],
  bBits: u32,
  b: &mut [u64],
  res: &mut [u64]
) ->
  ()
{
  let mut r2: [u64; 64] = [0u64; 64u32 as usize];
  precompr2(nBits, n, &mut r2);
  let mu: u64 = crate::hacl::bignum::mod_inv_uint64(n[0u32 as usize]);
  exp_consttime_precomp(n, mu, &mut r2, a, bBits, b, res)
}

pub fn op_Bang_Star__Hacl_Bignum_MontArithmetic_bn_mont_ctx'  uint64_t* uint64_t(
  p: &mut [crate::hacl::bignum::montarithmetic::bn_mont_ctx_u64]
) ->
  crate::hacl::bignum::montarithmetic::bn_mont_ctx_u64
{ p[0u32 as usize] }

pub fn lt_mask(a: &mut [u64], b: &mut [u64]) -> u64
{
  let mut acc: u64 = 0u64;
  for i in 0u32..64u32
  {
    let beq: u64 = crate::fstar::uint64::eq_mask(a[i as usize], b[i as usize]);
    let blt: u64 = ! crate::fstar::uint64::gte_mask(a[i as usize], b[i as usize]);
    acc = beq & acc | ! beq & (blt & 0xFFFFFFFFFFFFFFFFu64 | ! blt & 0u64)
  };
  acc
}

pub fn eq_mask(a: &mut [u64], b: &mut [u64]) -> u64
{
  let mut mask: u64 = 0xFFFFFFFFFFFFFFFFu64;
  for i in 0u32..64u32
  {
    let uu____0: u64 = crate::fstar::uint64::eq_mask(a[i as usize], b[i as usize]);
    mask = uu____0 & mask
  };
  let mask1: u64 = mask;
  mask1
}