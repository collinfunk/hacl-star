fn bn_add(aLen: u32, a: &mut [u64], bLen: u32, b: &mut [u64], res: &mut [u64]) -> u64
{
  let a0: (&mut [u64], &mut [u64]) = a.split_at_mut(0usize);
  let res0: (&mut [u64], &mut [u64]) = res.split_at_mut(0usize);
  let mut c: u64 = 0u64;
  for i in 0u32..b.wrapping_div(4u32)
  {
    let t1: u64 = a0.1[4u32.wrapping_mul(i) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i) as usize];
    let res_i: (&mut [u64], &mut [u64]) =
      res0.1.split_at_mut((4u32.wrapping_mul(i) as usize).wrapping_add(0usize));
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1);
    let t1: u64 = a0.1[4u32.wrapping_mul(i).wrapping_add(1u32) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i).wrapping_add(1u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1);
    let t1: u64 = a0.1[4u32.wrapping_mul(i).wrapping_add(2u32) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i).wrapping_add(2u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1);
    let t1: u64 = a0.1[4u32.wrapping_mul(i).wrapping_add(3u32) as usize];
    let t2: u64 = b[4u32.wrapping_mul(i).wrapping_add(3u32) as usize];
    let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1)
  };
  for i in b.wrapping_div(4u32).wrapping_mul(4u32)..b
  {
    let t1: u64 = a0.1[i as usize];
    let t2: u64 = b[i as usize];
    let res_i: (&mut [u64], &mut [u64]) = res0.1.split_at_mut((i as usize).wrapping_add(0usize));
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1)
  };
  let c0: u64 = c;
  if bLen < aLen
  {
    let a1: (&mut [u64], &mut [u64]) = a0.1.split_at_mut((bLen as usize).wrapping_add(0usize));
    let res1: (&mut [u64], &mut [u64]) = res0.1.split_at_mut((bLen as usize).wrapping_add(0usize));
    let mut c: u64 = c0;
    for i in 0u32..a.wrapping_sub(b).wrapping_div(4u32)
    {
      let t1: u64 = a1.1[4u32.wrapping_mul(i) as usize];
      let res_i: (&mut [u64], &mut [u64]) =
        res1.1.split_at_mut((4u32.wrapping_mul(i) as usize).wrapping_add(0usize));
      c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, 0u64, res_i.1);
      let t1: u64 = a1.1[4u32.wrapping_mul(i).wrapping_add(1u32) as usize];
      let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
      c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, 0u64, res_i.1);
      let t1: u64 = a1.1[4u32.wrapping_mul(i).wrapping_add(2u32) as usize];
      let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
      c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, 0u64, res_i.1);
      let t1: u64 = a1.1[4u32.wrapping_mul(i).wrapping_add(3u32) as usize];
      let res_i: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
      c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, 0u64, res_i.1)
    };
    for i in a.wrapping_sub(b).wrapping_div(4u32).wrapping_mul(4u32)..a.wrapping_sub(b)
    {
      let t1: u64 = a1.1[i as usize];
      let res_i: (&mut [u64], &mut [u64]) = res1.1.split_at_mut((i as usize).wrapping_add(0usize));
      c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, 0u64, res_i.1)
    };
    let c1: u64 = c;
    c1
  }
  else
  { c0 }
}

fn add4(a: &mut [u64], b: &mut [u64], res: &mut [u64]) -> u64
{
  let mut c: u64 = 0u64;
  for i in 0u32..1u32
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
  for i in 4u32..4u32
  {
    let t1: u64 = a[i as usize];
    let t2: u64 = b[i as usize];
    let res_i: (&mut [u64], &mut [u64]) = res.split_at_mut((i as usize).wrapping_add(0usize));
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1)
  };
  c
}

fn add_mod4(n: &mut [u64], a: &mut [u64], b: &mut [u64], res: &mut [u64]) -> ()
{
  let mut c: u64 = 0u64;
  for i in 0u32..1u32
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
  for i in 4u32..4u32
  {
    let t1: u64 = a[i as usize];
    let t2: u64 = b[i as usize];
    let res_i: (&mut [u64], &mut [u64]) = res.split_at_mut((i as usize).wrapping_add(0usize));
    c = crate::lib::inttypes::intrinsics::add_carry_u64(c, t1, t2, res_i.1)
  };
  let c0: u64 = c;
  let mut tmp: [u64; 4] = [0u64; 4u32 as usize];
  let mut c: u64 = 0u64;
  for i in 0u32..1u32
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
  for i in 4u32..4u32
  {
    let t1: u64 = res[i as usize];
    let t2: u64 = n[i as usize];
    let res_i: (&mut [u64], &mut [u64]) =
      (&mut tmp).split_at_mut((i as usize).wrapping_add(0usize));
    c = crate::lib::inttypes::intrinsics::sub_borrow_u64(c, t1, t2, res_i.1)
  };
  let c1: u64 = c;
  let c: u64 = c0.wrapping_sub(c1);
  for i in 0u32..4u32
  {
    let os: (&mut [u64], &mut [u64]) = res.split_at_mut(0usize);
    let x: u64 = c & os.1[i as usize] | ! c & (&mut tmp)[i as usize];
    os.1[i as usize] = x
  }
}

fn is_qelem_zero(f: &mut [u64]) -> u64
{
  let mut bn_zero: [u64; 4] = [0u64; 4u32 as usize];
  let mut mask: u64 = 0xFFFFFFFFFFFFFFFFu64;
  for i in 0u32..4u32
  {
    let uu____0: u64 = crate::fstar::uint64::eq_mask(f[i as usize], (&mut bn_zero)[i as usize]);
    mask = uu____0 & mask
  };
  let mask1: u64 = mask;
  let res: u64 = mask1;
  res
}

fn is_qelem_zero_vartime(f: &mut [u64]) -> bool
{
  let f0: u64 = f[0u32 as usize];
  let f1: u64 = f[1u32 as usize];
  let f2: u64 = f[2u32 as usize];
  let f3: u64 = f[3u32 as usize];
  f0 == 0u64 && f1 == 0u64 && f2 == 0u64 && f3 == 0u64
}

fn load_qelem_check(f: &mut [u64], b: &mut [u8]) -> u64
{
  let mut n: [u64; 4] = [0u64; 4u32 as usize];
  (&mut n)[0u32 as usize] = 0xbfd25e8cd0364141u64;
  (&mut n)[1u32 as usize] = 0xbaaedce6af48a03bu64;
  (&mut n)[2u32 as usize] = 0xfffffffffffffffeu64;
  (&mut n)[3u32 as usize] = 0xffffffffffffffffu64;
  for i in 0u32..4u32
  {
    let os: (&mut [u64], &mut [u64]) = f.split_at_mut(0usize);
    let u: u64 =
      crate::lowstar::endianness::load64_be(
        &mut b[4u32.wrapping_sub(i).wrapping_sub(1u32).wrapping_mul(8u32) as usize..]
      );
    let x: u64 = u;
    os.1[i as usize] = x
  };
  let is_zero: u64 = is_qelem_zero(f);
  let mut acc: u64 = 0u64;
  for i in 0u32..4u32
  {
    let beq: u64 = crate::fstar::uint64::eq_mask(f[i as usize], (&mut n)[i as usize]);
    let blt: u64 = ! crate::fstar::uint64::gte_mask(f[i as usize], (&mut n)[i as usize]);
    acc = beq & acc | ! beq & (blt & 0xFFFFFFFFFFFFFFFFu64 | ! blt & 0u64)
  };
  let is_lt_q: u64 = acc;
  ! is_zero & is_lt_q
}

fn load_qelem_vartime(f: &mut [u64], b: &mut [u8]) -> bool
{
  for i in 0u32..4u32
  {
    let os: (&mut [u64], &mut [u64]) = f.split_at_mut(0usize);
    let u: u64 =
      crate::lowstar::endianness::load64_be(
        &mut b[4u32.wrapping_sub(i).wrapping_sub(1u32).wrapping_mul(8u32) as usize..]
      );
    let x: u64 = u;
    os.1[i as usize] = x
  };
  let is_zero: bool = is_qelem_zero_vartime(f);
  let a0: u64 = f[0u32 as usize];
  let a1: u64 = f[1u32 as usize];
  let a2: u64 = f[2u32 as usize];
  let a3: u64 = f[3u32 as usize];
  let is_lt_q_b: bool =
    if a3 < 0xffffffffffffffffu64
    { truebool }
    else
    if a2 < 0xfffffffffffffffeu64
    { truebool }
    else
    if a2 > 0xfffffffffffffffeu64
    { falsebool }
    else
    if a1 < 0xbaaedce6af48a03bu64
    { truebool }
    else
    if a1 > 0xbaaedce6af48a03bu64 { falsebool } else { a0 < 0xbfd25e8cd0364141u64 };
  ! is_zero && is_lt_q_b
}

fn modq_short(out: &mut [u64], a: &mut [u64]) -> ()
{
  let mut tmp: [u64; 4] = [0u64; 4u32 as usize];
  (&mut tmp)[0u32 as usize] = 0x402da1732fc9bebfu64;
  (&mut tmp)[1u32 as usize] = 0x4551231950b75fc4u64;
  (&mut tmp)[2u32 as usize] = 0x1u64;
  (&mut tmp)[3u32 as usize] = 0x0u64;
  let c: u64 = add4(a, &mut tmp, out);
  let mask: u64 = 0u64.wrapping_sub(c);
  for i in 0u32..4u32
  {
    let os: (&mut [u64], &mut [u64]) = out.split_at_mut(0usize);
    let x: u64 = mask & os.1[i as usize] | ! mask & a[i as usize];
    os.1[i as usize] = x
  }
}

fn load_qelem_modq(f: &mut [u64], b: &mut [u8]) -> ()
{
  let mut tmp: [u64; 4] = [0u64; 4u32 as usize];
  for i in 0u32..4u32
  {
    let os: (&mut [u64], &mut [u64]) = f.split_at_mut(0usize);
    let u: u64 =
      crate::lowstar::endianness::load64_be(
        &mut b[4u32.wrapping_sub(i).wrapping_sub(1u32).wrapping_mul(8u32) as usize..]
      );
    let x: u64 = u;
    os.1[i as usize] = x
  };
  ((&mut tmp)[0u32 as usize..0u32 as usize + 4u32 as usize]).copy_from_slice(
    &f[0u32 as usize..0u32 as usize + 4u32 as usize]
  );
  modq_short(f, &mut tmp)
}

fn qadd(out: &mut [u64], f1: &mut [u64], f2: &mut [u64]) -> ()
{
  let mut n: [u64; 4] = [0u64; 4u32 as usize];
  (&mut n)[0u32 as usize] = 0xbfd25e8cd0364141u64;
  (&mut n)[1u32 as usize] = 0xbaaedce6af48a03bu64;
  (&mut n)[2u32 as usize] = 0xfffffffffffffffeu64;
  (&mut n)[3u32 as usize] = 0xffffffffffffffffu64;
  add_mod4(&mut n, f1, f2, out)
}

fn qmul(out: &mut [u64], f1: &mut [u64], f2: &mut [u64]) -> ()
{
  let mut tmp: [u64; 8] = [0u64; 8u32 as usize];
  crate::hacl::k256::scalar::mul4(f1, f2, &mut tmp);
  crate::hacl::k256::scalar::modq(out, &mut tmp)
}

fn qsqr(out: &mut [u64], f: &mut [u64]) -> ()
{
  let mut tmp: [u64; 8] = [0u64; 8u32 as usize];
  crate::hacl::k256::scalar::sqr4(f, &mut tmp);
  crate::hacl::k256::scalar::modq(out, &mut tmp)
}

fn qnegate_conditional_vartime(f: &mut [u64], is_negate: bool) -> ()
{
  let mut n: [u64; 4] = [0u64; 4u32 as usize];
  (&mut n)[0u32 as usize] = 0xbfd25e8cd0364141u64;
  (&mut n)[1u32 as usize] = 0xbaaedce6af48a03bu64;
  (&mut n)[2u32 as usize] = 0xfffffffffffffffeu64;
  (&mut n)[3u32 as usize] = 0xffffffffffffffffu64;
  let mut zero: [u64; 4] = [0u64; 4u32 as usize];
  if is_negate { crate::hacl::k256::scalar::sub_mod4(&mut n, &mut zero, f, f) }
}

fn is_qelem_le_q_halved_vartime(f: &mut [u64]) -> bool
{
  let a0: u64 = f[0u32 as usize];
  let a1: u64 = f[1u32 as usize];
  let a2: u64 = f[2u32 as usize];
  let a3: u64 = f[3u32 as usize];
  if a3 < 0x7fffffffffffffffu64
  { truebool }
  else
  if a3 > 0x7fffffffffffffffu64
  { falsebool }
  else
  if a2 < 0xffffffffffffffffu64
  { truebool }
  else
  if a2 > 0xffffffffffffffffu64
  { falsebool }
  else
  if a1 < 0x5d576e7357a4501du64
  { truebool }
  else
  if a1 > 0x5d576e7357a4501du64 { falsebool } else { a0 <= 0xdfe92f46681b20a0u64 }
}

fn qsquare_times_in_place(out: &mut [u64], b: u32) -> () for i in 0u32..i { qsqr(out, out) }



fn qinv(out: &mut [u64], f: &mut [u64]) -> ()
{
  let mut x_10: [u64; 4] = [0u64; 4u32 as usize];
  let mut x_11: [u64; 4] = [0u64; 4u32 as usize];
  let mut x_101: [u64; 4] = [0u64; 4u32 as usize];
  let mut x_111: [u64; 4] = [0u64; 4u32 as usize];
  let mut x_1001: [u64; 4] = [0u64; 4u32 as usize];
  let mut x_1011: [u64; 4] = [0u64; 4u32 as usize];
  let mut x_1101: [u64; 4] = [0u64; 4u32 as usize];
  qsquare_times(&mut x_10, f, 1u32);
  qmul(&mut x_11, &mut x_10, f);
  qmul(&mut x_101, &mut x_10, &mut x_11);
  qmul(&mut x_111, &mut x_10, &mut x_101);
  qmul(&mut x_1001, &mut x_10, &mut x_111);
  qmul(&mut x_1011, &mut x_10, &mut x_1001);
  qmul(&mut x_1101, &mut x_10, &mut x_1011);
  let mut x6: [u64; 4] = [0u64; 4u32 as usize];
  let mut x8: [u64; 4] = [0u64; 4u32 as usize];
  let mut x14: [u64; 4] = [0u64; 4u32 as usize];
  qsquare_times(&mut x6, &mut x_1101, 2u32);
  qmul(&mut x6, &mut x6, &mut x_1011);
  qsquare_times(&mut x8, &mut x6, 2u32);
  qmul(&mut x8, &mut x8, &mut x_11);
  qsquare_times(&mut x14, &mut x8, 6u32);
  qmul(&mut x14, &mut x14, &mut x6);
  let mut x56: [u64; 4] = [0u64; 4u32 as usize];
  qsquare_times(out, &mut x14, 14u32);
  qmul(out, out, &mut x14);
  qsquare_times(&mut x56, out, 28u32);
  qmul(&mut x56, &mut x56, out);
  qsquare_times(out, &mut x56, 56u32);
  qmul(out, out, &mut x56);
  qsquare_times_in_place(out, 14u32);
  qmul(out, out, &mut x14);
  qsquare_times_in_place(out, 3u32);
  qmul(out, out, &mut x_101);
  qsquare_times_in_place(out, 4u32);
  qmul(out, out, &mut x_111);
  qsquare_times_in_place(out, 4u32);
  qmul(out, out, &mut x_101);
  qsquare_times_in_place(out, 5u32);
  qmul(out, out, &mut x_1011);
  qsquare_times_in_place(out, 4u32);
  qmul(out, out, &mut x_1011);
  qsquare_times_in_place(out, 4u32);
  qmul(out, out, &mut x_111);
  qsquare_times_in_place(out, 5u32);
  qmul(out, out, &mut x_111);
  qsquare_times_in_place(out, 6u32);
  qmul(out, out, &mut x_1101);
  qsquare_times_in_place(out, 4u32);
  qmul(out, out, &mut x_101);
  qsquare_times_in_place(out, 3u32);
  qmul(out, out, &mut x_111);
  qsquare_times_in_place(out, 5u32);
  qmul(out, out, &mut x_1001);
  qsquare_times_in_place(out, 6u32);
  qmul(out, out, &mut x_101);
  qsquare_times_in_place(out, 10u32);
  qmul(out, out, &mut x_111);
  qsquare_times_in_place(out, 4u32);
  qmul(out, out, &mut x_111);
  qsquare_times_in_place(out, 9u32);
  qmul(out, out, &mut x8);
  qsquare_times_in_place(out, 5u32);
  qmul(out, out, &mut x_1001);
  qsquare_times_in_place(out, 6u32);
  qmul(out, out, &mut x_1011);
  qsquare_times_in_place(out, 4u32);
  qmul(out, out, &mut x_1101);
  qsquare_times_in_place(out, 5u32);
  qmul(out, out, &mut x_11);
  qsquare_times_in_place(out, 6u32);
  qmul(out, out, &mut x_1101);
  qsquare_times_in_place(out, 10u32);
  qmul(out, out, &mut x_1101);
  qsquare_times_in_place(out, 4u32);
  qmul(out, out, &mut x_1001);
  qsquare_times_in_place(out, 6u32);
  qmul(out, out, f);
  qsquare_times_in_place(out, 8u32);
  qmul(out, out, &mut x6)
}

fn to_aff_point(p_aff: &mut [u64], p: &mut [u64]) -> ()
{
  let x: (&mut [u64], &mut [u64]) = p_aff.split_at_mut(0usize);
  let y: (&mut [u64], &mut [u64]) = x.1.split_at_mut(5usize);
  let x1: (&mut [u64], &mut [u64]) = p.split_at_mut(0usize);
  let y1: (&mut [u64], &mut [u64]) = x1.1.split_at_mut(5usize);
  let z1: (&mut [u64], &mut [u64]) = y1.1.split_at_mut(5usize);
  let mut zinv: [u64; 5] = [0u64; 5u32 as usize];
  crate::hacl::bignum_k256::finv(&mut zinv, z1.1);
  crate::hacl::bignum_k256::fmul(y.0, y1.0, &mut zinv);
  crate::hacl::bignum_k256::fmul(y.1, z1.0, &mut zinv);
  crate::hacl::bignum_k256::fnormalize(y.0, y.0);
  crate::hacl::bignum_k256::fnormalize(y.1, y.1)
}

fn to_aff_point_x(x: &mut [u64], p: &mut [u64]) -> ()
{
  let x1: (&mut [u64], &mut [u64]) = p.split_at_mut(0usize);
  let z1: (&mut [u64], &mut [u64]) = x1.1.split_at_mut(10usize);
  let mut zinv: [u64; 5] = [0u64; 5u32 as usize];
  crate::hacl::bignum_k256::finv(&mut zinv, z1.1);
  crate::hacl::bignum_k256::fmul(x, z1.0, &mut zinv);
  crate::hacl::bignum_k256::fnormalize(x, x)
}

fn is_on_curve_vartime(p: &mut [u64]) -> bool
{
  let mut y2_exp: [u64; 5] = [0u64; 5u32 as usize];
  let x: (&mut [u64], &mut [u64]) = p.split_at_mut(0usize);
  let y: (&mut [u64], &mut [u64]) = x.1.split_at_mut(5usize);
  let mut b: [u64; 5] = [0u64; 5u32 as usize];
  (&mut b)[0u32 as usize] = 0x7u64;
  (&mut b)[1u32 as usize] = 0u64;
  (&mut b)[2u32 as usize] = 0u64;
  (&mut b)[3u32 as usize] = 0u64;
  (&mut b)[4u32 as usize] = 0u64;
  crate::hacl::bignum_k256::fsqr(&mut y2_exp, y.0);
  crate::hacl::bignum_k256::fmul(&mut y2_exp, &mut y2_exp, y.0);
  crate::hacl::bignum_k256::fadd(&mut y2_exp, &mut y2_exp, &mut b);
  crate::hacl::bignum_k256::fnormalize(&mut y2_exp, &mut y2_exp);
  let mut y2_comp: [u64; 5] = [0u64; 5u32 as usize];
  crate::hacl::bignum_k256::fsqr(&mut y2_comp, y.1);
  crate::hacl::bignum_k256::fnormalize(&mut y2_comp, &mut y2_comp);
  let res: bool = crate::hacl::bignum_k256::is_felem_eq_vartime(&mut y2_exp, &mut y2_comp);
  let res: bool = res;
  res
}

pub fn point_negate(out: &mut [u64], p: &mut [u64]) -> ()
{
  let px: (&mut [u64], &mut [u64]) = p.split_at_mut(0usize);
  let py: (&mut [u64], &mut [u64]) = px.1.split_at_mut(5usize);
  let pz: (&mut [u64], &mut [u64]) = py.1.split_at_mut(5usize);
  let ox: (&mut [u64], &mut [u64]) = out.split_at_mut(0usize);
  let oy: (&mut [u64], &mut [u64]) = ox.1.split_at_mut(5usize);
  let oz: (&mut [u64], &mut [u64]) = oy.1.split_at_mut(5usize);
  oy.0[0u32 as usize] = py.0[0u32 as usize];
  oy.0[1u32 as usize] = py.0[1u32 as usize];
  oy.0[2u32 as usize] = py.0[2u32 as usize];
  oy.0[3u32 as usize] = py.0[3u32 as usize];
  oy.0[4u32 as usize] = py.0[4u32 as usize];
  oz.1[0u32 as usize] = pz.1[0u32 as usize];
  oz.1[1u32 as usize] = pz.1[1u32 as usize];
  oz.1[2u32 as usize] = pz.1[2u32 as usize];
  oz.1[3u32 as usize] = pz.1[3u32 as usize];
  oz.1[4u32 as usize] = pz.1[4u32 as usize];
  let a0: u64 = pz.0[0u32 as usize];
  let a1: u64 = pz.0[1u32 as usize];
  let a2: u64 = pz.0[2u32 as usize];
  let a3: u64 = pz.0[3u32 as usize];
  let a4: u64 = pz.0[4u32 as usize];
  let r0: u64 = 18014381329608892u64.wrapping_sub(a0);
  let r1: u64 = 18014398509481980u64.wrapping_sub(a1);
  let r2: u64 = 18014398509481980u64.wrapping_sub(a2);
  let r3: u64 = 18014398509481980u64.wrapping_sub(a3);
  let r4: u64 = 1125899906842620u64.wrapping_sub(a4);
  let f0: u64 = r0;
  let f1: u64 = r1;
  let f2: u64 = r2;
  let f3: u64 = r3;
  let f4: u64 = r4;
  oz.0[0u32 as usize] = f0;
  oz.0[1u32 as usize] = f1;
  oz.0[2u32 as usize] = f2;
  oz.0[3u32 as usize] = f3;
  oz.0[4u32 as usize] = f4;
  crate::hacl::bignum_k256::fnormalize_weak(oz.0, oz.0)
}

fn point_negate_conditional_vartime(p: &mut [u64], is_negate: bool) -> ()
if is_negate { point_negate(p, p) }

fn aff_point_store(out: &mut [u8], p: &mut [u64]) -> ()
{
  let px: (&mut [u64], &mut [u64]) = p.split_at_mut(0usize);
  let py: (&mut [u64], &mut [u64]) = px.1.split_at_mut(5usize);
  crate::hacl::bignum_k256::store_felem(&mut out[0u32 as usize..], py.0);
  crate::hacl::bignum_k256::store_felem(&mut out[32u32 as usize..], py.1)
}

pub fn point_store(out: &mut [u8], p: &mut [u64]) -> ()
{
  let mut p_aff: [u64; 10] = [0u64; 10u32 as usize];
  to_aff_point(&mut p_aff, p);
  aff_point_store(out, &mut p_aff)
}

pub fn aff_point_load_vartime(p: &mut [u64], b: &mut [u8]) -> bool
{
  let px: (&mut [u8], &mut [u8]) = b.split_at_mut(0usize);
  let py: (&mut [u8], &mut [u8]) = px.1.split_at_mut(32usize);
  let bn_px: (&mut [u64], &mut [u64]) = p.split_at_mut(0usize);
  let bn_py: (&mut [u64], &mut [u64]) = bn_px.1.split_at_mut(5usize);
  let is_x_valid: bool = crate::hacl::bignum_k256::load_felem_lt_prime_vartime(bn_py.0, py.0);
  let is_y_valid: bool = crate::hacl::bignum_k256::load_felem_lt_prime_vartime(bn_py.1, py.1);
  if is_x_valid && is_y_valid { is_on_curve_vartime(bn_py.0) } else { falsebool }
}

fn aff_point_decompress_vartime(x: &mut [u64], y: &mut [u64], s: &mut [u8]) -> bool
{
  let s0: u8 = s[0u32 as usize];
  let s01: u8 = s0;
  if ! (s01 == 0x02u8 || s01 == 0x03u8)
  { falsebool }
  else
  {
    let xb: (&mut [u8], &mut [u8]) = s.split_at_mut(1usize);
    let is_x_valid: bool = crate::hacl::bignum_k256::load_felem_lt_prime_vartime(x, xb.1);
    let is_y_odd: bool = s01 == 0x03u8;
    if ! is_x_valid
    { falsebool }
    else
    {
      let mut y2: [u64; 5] = [0u64; 5u32 as usize];
      let mut b: [u64; 5] = [0u64; 5u32 as usize];
      (&mut b)[0u32 as usize] = 0x7u64;
      (&mut b)[1u32 as usize] = 0u64;
      (&mut b)[2u32 as usize] = 0u64;
      (&mut b)[3u32 as usize] = 0u64;
      (&mut b)[4u32 as usize] = 0u64;
      crate::hacl::bignum_k256::fsqr(&mut y2, x);
      crate::hacl::bignum_k256::fmul(&mut y2, &mut y2, x);
      crate::hacl::bignum_k256::fadd(&mut y2, &mut y2, &mut b);
      crate::hacl::bignum_k256::fnormalize(&mut y2, &mut y2);
      crate::hacl::bignum_k256::fsqrt(y, &mut y2);
      crate::hacl::bignum_k256::fnormalize(y, y);
      let mut y2_comp: [u64; 5] = [0u64; 5u32 as usize];
      crate::hacl::bignum_k256::fsqr(&mut y2_comp, y);
      crate::hacl::bignum_k256::fnormalize(&mut y2_comp, &mut y2_comp);
      let res: bool = crate::hacl::bignum_k256::is_felem_eq_vartime(&mut y2, &mut y2_comp);
      let is_y_valid: bool = res;
      let is_y_valid: bool = is_y_valid;
      if ! is_y_valid
      { falsebool }
      else
      {
        let x0: u64 = y[0u32 as usize];
        let is_y_odd1: bool = x0 & 1u64 == 1u64;
        crate::hacl::bignum_k256::fnegate_conditional_vartime(y, is_y_odd1 != is_y_odd);
        truebool
      }
    }
  }
}

pub fn point_double(out: &mut [u64], p: &mut [u64]) -> ()
{
  let mut tmp: [u64; 25] = [0u64; 25u32 as usize];
  let x1: (&mut [u64], &mut [u64]) = p.split_at_mut(0usize);
  let y1: (&mut [u64], &mut [u64]) = x1.1.split_at_mut(5usize);
  let z1: (&mut [u64], &mut [u64]) = y1.1.split_at_mut(5usize);
  let x3: (&mut [u64], &mut [u64]) = out.split_at_mut(0usize);
  let y3: (&mut [u64], &mut [u64]) = x3.1.split_at_mut(5usize);
  let z3: (&mut [u64], &mut [u64]) = y3.1.split_at_mut(5usize);
  let yy: (&mut [u64], &mut [u64]) = (&mut tmp).split_at_mut(0usize);
  let zz: (&mut [u64], &mut [u64]) = yy.1.split_at_mut(5usize);
  let bzz3: (&mut [u64], &mut [u64]) = zz.1.split_at_mut(5usize);
  let bzz9: (&mut [u64], &mut [u64]) = bzz3.1.split_at_mut(5usize);
  let tmp1: (&mut [u64], &mut [u64]) = bzz9.1.split_at_mut(5usize);
  crate::hacl::bignum_k256::fsqr(zz.0, z1.0);
  crate::hacl::bignum_k256::fsqr(bzz3.0, z1.1);
  crate::hacl::bignum_k256::fmul_small_num(y3.0, y1.0, 2u64);
  crate::hacl::bignum_k256::fmul(y3.0, y3.0, z1.0);
  crate::hacl::bignum_k256::fmul(tmp1.1, zz.0, z1.0);
  crate::hacl::bignum_k256::fmul(z3.1, tmp1.1, z1.1);
  crate::hacl::bignum_k256::fmul_small_num(z3.1, z3.1, 8u64);
  crate::hacl::bignum_k256::fnormalize_weak(z3.1, z3.1);
  crate::hacl::bignum_k256::fmul_small_num(bzz9.0, bzz3.0, 21u64);
  crate::hacl::bignum_k256::fnormalize_weak(bzz9.0, bzz9.0);
  crate::hacl::bignum_k256::fmul_small_num(tmp1.0, bzz9.0, 3u64);
  crate::hacl::bignum_k256::fsub(tmp1.0, zz.0, tmp1.0, 6u64);
  crate::hacl::bignum_k256::fadd(tmp1.1, zz.0, bzz9.0);
  crate::hacl::bignum_k256::fmul(tmp1.1, tmp1.0, tmp1.1);
  crate::hacl::bignum_k256::fmul(z3.0, zz.0, bzz3.0);
  crate::hacl::bignum_k256::fmul(y3.0, y3.0, tmp1.0);
  crate::hacl::bignum_k256::fmul_small_num(z3.0, z3.0, 168u64);
  crate::hacl::bignum_k256::fadd(z3.0, tmp1.1, z3.0);
  crate::hacl::bignum_k256::fnormalize_weak(z3.0, z3.0)
}

pub fn point_add(out: &mut [u64], p: &mut [u64], q: &mut [u64]) -> ()
{
  let mut tmp: [u64; 45] = [0u64; 45u32 as usize];
  let x1: (&mut [u64], &mut [u64]) = p.split_at_mut(0usize);
  let y1: (&mut [u64], &mut [u64]) = x1.1.split_at_mut(5usize);
  let z1: (&mut [u64], &mut [u64]) = y1.1.split_at_mut(5usize);
  let x2: (&mut [u64], &mut [u64]) = q.split_at_mut(0usize);
  let y2: (&mut [u64], &mut [u64]) = x2.1.split_at_mut(5usize);
  let z2: (&mut [u64], &mut [u64]) = y2.1.split_at_mut(5usize);
  let x3: (&mut [u64], &mut [u64]) = out.split_at_mut(0usize);
  let y3: (&mut [u64], &mut [u64]) = x3.1.split_at_mut(5usize);
  let z3: (&mut [u64], &mut [u64]) = y3.1.split_at_mut(5usize);
  let xx: (&mut [u64], &mut [u64]) = (&mut tmp).split_at_mut(0usize);
  let yy: (&mut [u64], &mut [u64]) = xx.1.split_at_mut(5usize);
  let zz: (&mut [u64], &mut [u64]) = yy.1.split_at_mut(5usize);
  let xy_pairs: (&mut [u64], &mut [u64]) = zz.1.split_at_mut(5usize);
  let yz_pairs: (&mut [u64], &mut [u64]) = xy_pairs.1.split_at_mut(5usize);
  let xz_pairs: (&mut [u64], &mut [u64]) = yz_pairs.1.split_at_mut(5usize);
  let yy_m_bzz3: (&mut [u64], &mut [u64]) = xz_pairs.1.split_at_mut(5usize);
  let yy_p_bzz3: (&mut [u64], &mut [u64]) = yy_m_bzz3.1.split_at_mut(5usize);
  let tmp1: (&mut [u64], &mut [u64]) = yy_p_bzz3.1.split_at_mut(5usize);
  crate::hacl::bignum_k256::fmul(yy.0, y1.0, y2.0);
  crate::hacl::bignum_k256::fmul(zz.0, z1.0, z2.0);
  crate::hacl::bignum_k256::fmul(xy_pairs.0, z1.1, z2.1);
  crate::hacl::bignum_k256::fadd(yz_pairs.0, y1.0, z1.0);
  crate::hacl::bignum_k256::fadd(tmp1.1, y2.0, z2.0);
  crate::hacl::bignum_k256::fmul(yz_pairs.0, yz_pairs.0, tmp1.1);
  crate::hacl::bignum_k256::fadd(tmp1.1, yy.0, zz.0);
  crate::hacl::bignum_k256::fsub(yz_pairs.0, yz_pairs.0, tmp1.1, 4u64);
  crate::hacl::bignum_k256::fadd(xz_pairs.0, z1.0, z1.1);
  crate::hacl::bignum_k256::fadd(tmp1.1, z2.0, z2.1);
  crate::hacl::bignum_k256::fmul(xz_pairs.0, xz_pairs.0, tmp1.1);
  crate::hacl::bignum_k256::fadd(tmp1.1, zz.0, xy_pairs.0);
  crate::hacl::bignum_k256::fsub(xz_pairs.0, xz_pairs.0, tmp1.1, 4u64);
  crate::hacl::bignum_k256::fadd(yy_m_bzz3.0, y1.0, z1.1);
  crate::hacl::bignum_k256::fadd(tmp1.1, y2.0, z2.1);
  crate::hacl::bignum_k256::fmul(yy_m_bzz3.0, yy_m_bzz3.0, tmp1.1);
  crate::hacl::bignum_k256::fadd(tmp1.1, yy.0, xy_pairs.0);
  crate::hacl::bignum_k256::fsub(yy_m_bzz3.0, yy_m_bzz3.0, tmp1.1, 4u64);
  crate::hacl::bignum_k256::fmul_small_num(tmp1.1, xy_pairs.0, 21u64);
  crate::hacl::bignum_k256::fnormalize_weak(tmp1.1, tmp1.1);
  crate::hacl::bignum_k256::fsub(yy_p_bzz3.0, zz.0, tmp1.1, 2u64);
  crate::hacl::bignum_k256::fadd(tmp1.0, zz.0, tmp1.1);
  crate::hacl::bignum_k256::fmul_small_num(y3.0, xz_pairs.0, 21u64);
  crate::hacl::bignum_k256::fnormalize_weak(y3.0, y3.0);
  crate::hacl::bignum_k256::fmul_small_num(z3.1, yy.0, 3u64);
  crate::hacl::bignum_k256::fmul_small_num(z3.0, z3.1, 21u64);
  crate::hacl::bignum_k256::fnormalize_weak(z3.0, z3.0);
  crate::hacl::bignum_k256::fmul(tmp1.1, yz_pairs.0, yy_p_bzz3.0);
  crate::hacl::bignum_k256::fmul(y3.0, y3.0, yy_m_bzz3.0);
  crate::hacl::bignum_k256::fsub(y3.0, tmp1.1, y3.0, 2u64);
  crate::hacl::bignum_k256::fnormalize_weak(y3.0, y3.0);
  crate::hacl::bignum_k256::fmul(tmp1.1, tmp1.0, yy_p_bzz3.0);
  crate::hacl::bignum_k256::fmul(z3.0, z3.0, yy_m_bzz3.0);
  crate::hacl::bignum_k256::fadd(z3.0, tmp1.1, z3.0);
  crate::hacl::bignum_k256::fnormalize_weak(z3.0, z3.0);
  crate::hacl::bignum_k256::fmul(tmp1.1, xz_pairs.0, tmp1.0);
  crate::hacl::bignum_k256::fmul(z3.1, z3.1, yz_pairs.0);
  crate::hacl::bignum_k256::fadd(z3.1, tmp1.1, z3.1);
  crate::hacl::bignum_k256::fnormalize_weak(z3.1, z3.1)
}

fn scalar_split_lambda(r1: &mut [u64], r2: &mut [u64], k: &mut [u64]) -> ()
{
  let mut tmp1: [u64; 4] = [0u64; 4u32 as usize];
  let mut tmp2: [u64; 4] = [0u64; 4u32 as usize];
  (&mut tmp1)[0u32 as usize] = 0xe893209a45dbb031u64;
  (&mut tmp1)[1u32 as usize] = 0x3daa8a1471e8ca7fu64;
  (&mut tmp1)[2u32 as usize] = 0xe86c90e49284eb15u64;
  (&mut tmp1)[3u32 as usize] = 0x3086d221a7d46bcdu64;
  (&mut tmp2)[0u32 as usize] = 0x1571b4ae8ac47f71u64;
  (&mut tmp2)[1u32 as usize] = 0x221208ac9df506c6u64;
  (&mut tmp2)[2u32 as usize] = 0x6f547fa90abfe4c4u64;
  (&mut tmp2)[3u32 as usize] = 0xe4437ed6010e8828u64;
  crate::hacl::k256::scalar::qmul_shift_384(r1, k, &mut tmp1);
  crate::hacl::k256::scalar::qmul_shift_384(r2, k, &mut tmp2);
  (&mut tmp1)[0u32 as usize] = 0x6f547fa90abfe4c3u64;
  (&mut tmp1)[1u32 as usize] = 0xe4437ed6010e8828u64;
  (&mut tmp1)[2u32 as usize] = 0x0u64;
  (&mut tmp1)[3u32 as usize] = 0x0u64;
  (&mut tmp2)[0u32 as usize] = 0xd765cda83db1562cu64;
  (&mut tmp2)[1u32 as usize] = 0x8a280ac50774346du64;
  (&mut tmp2)[2u32 as usize] = 0xfffffffffffffffeu64;
  (&mut tmp2)[3u32 as usize] = 0xffffffffffffffffu64;
  qmul(r1, r1, &mut tmp1);
  qmul(r2, r2, &mut tmp2);
  (&mut tmp1)[0u32 as usize] = 0xe0cfc810b51283cfu64;
  (&mut tmp1)[1u32 as usize] = 0xa880b9fc8ec739c2u64;
  (&mut tmp1)[2u32 as usize] = 0x5ad9e3fd77ed9ba4u64;
  (&mut tmp1)[3u32 as usize] = 0xac9c52b33fa3cf1fu64;
  qadd(r2, r1, r2);
  qmul(&mut tmp2, r2, &mut tmp1);
  qadd(r1, k, &mut tmp2)
}

fn point_mul_lambda(res: &mut [u64], p: &mut [u64]) -> ()
{
  let rx: (&mut [u64], &mut [u64]) = res.split_at_mut(0usize);
  let ry: (&mut [u64], &mut [u64]) = rx.1.split_at_mut(5usize);
  let rz: (&mut [u64], &mut [u64]) = ry.1.split_at_mut(5usize);
  let px: (&mut [u64], &mut [u64]) = p.split_at_mut(0usize);
  let py: (&mut [u64], &mut [u64]) = px.1.split_at_mut(5usize);
  let pz: (&mut [u64], &mut [u64]) = py.1.split_at_mut(5usize);
  let mut beta: [u64; 5] = [0u64; 5u32 as usize];
  (&mut beta)[0u32 as usize] = 0x96c28719501eeu64;
  (&mut beta)[1u32 as usize] = 0x7512f58995c13u64;
  (&mut beta)[2u32 as usize] = 0xc3434e99cf049u64;
  (&mut beta)[3u32 as usize] = 0x7106e64479eau64;
  (&mut beta)[4u32 as usize] = 0x7ae96a2b657cu64;
  crate::hacl::bignum_k256::fmul(ry.0, &mut beta, py.0);
  rz.0[0u32 as usize] = pz.0[0u32 as usize];
  rz.0[1u32 as usize] = pz.0[1u32 as usize];
  rz.0[2u32 as usize] = pz.0[2u32 as usize];
  rz.0[3u32 as usize] = pz.0[3u32 as usize];
  rz.0[4u32 as usize] = pz.0[4u32 as usize];
  rz.1[0u32 as usize] = pz.1[0u32 as usize];
  rz.1[1u32 as usize] = pz.1[1u32 as usize];
  rz.1[2u32 as usize] = pz.1[2u32 as usize];
  rz.1[3u32 as usize] = pz.1[3u32 as usize];
  rz.1[4u32 as usize] = pz.1[4u32 as usize]
}

fn point_mul_lambda_inplace(res: &mut [u64]) -> ()
{
  let rx: (&mut [u64], &mut [u64]) = res.split_at_mut(0usize);
  let mut beta: [u64; 5] = [0u64; 5u32 as usize];
  (&mut beta)[0u32 as usize] = 0x96c28719501eeu64;
  (&mut beta)[1u32 as usize] = 0x7512f58995c13u64;
  (&mut beta)[2u32 as usize] = 0xc3434e99cf049u64;
  (&mut beta)[3u32 as usize] = 0x7106e64479eau64;
  (&mut beta)[4u32 as usize] = 0x7ae96a2b657cu64;
  crate::hacl::bignum_k256::fmul(rx.1, &mut beta, rx.1)
}

fn precomp_get_consttime(table: &[u64], bits_l: u64, tmp: &mut [u64]) -> ()
{
  (tmp[0u32 as usize..0u32 as usize + 15u32 as usize]).copy_from_slice(
    &(&mut table[0u32 as usize..] as &mut [u64])[0u32 as usize..0u32 as usize + 15u32 as usize]
  );
  for i in 0u32..15u32
  {
    let c: u64 = crate::fstar::uint64::eq_mask(bits_l, i.wrapping_add(1u32) as u64);
    let res_j: (&[u64], &[u64]) =
      table.split_at_mut((i.wrapping_add(1u32).wrapping_mul(15u32) as usize).wrapping_add(0usize));
    for i in 0u32..15u32
    {
      let os: (&mut [u64], &mut [u64]) = tmp.split_at_mut(0usize);
      let x: u64 = c & res_j.1[i as usize] | ! c & os.1[i as usize];
      os.1[i as usize] = x
    }
  }
}

fn check_ecmult_endo_split(r1: &mut [u64], r2: &mut [u64], r3: &mut [u64], r4: &mut [u64]) ->
  bool
{
  let f2: u64 = r1[2u32 as usize];
  let f3: u64 = r1[3u32 as usize];
  let b1: bool = f2 == 0u64 && f3 == 0u64;
  let f2: u64 = r2[2u32 as usize];
  let f3: u64 = r2[3u32 as usize];
  let b2: bool = f2 == 0u64 && f3 == 0u64;
  let f2: u64 = r3[2u32 as usize];
  let f3: u64 = r3[3u32 as usize];
  let b3: bool = f2 == 0u64 && f3 == 0u64;
  let f2: u64 = r4[2u32 as usize];
  let f3: u64 = r4[3u32 as usize];
  let b4: bool = f2 == 0u64 && f3 == 0u64;
  b1 && b2 && b3 && b4
}

fn fmul_eq_vartime(r: &mut [u64], z: &mut [u64], x: &mut [u64]) -> bool
{
  let mut tmp: [u64; 5] = [0u64; 5u32 as usize];
  crate::hacl::bignum_k256::fmul(&mut tmp, r, z);
  crate::hacl::bignum_k256::fnormalize(&mut tmp, &mut tmp);
  let b: bool = crate::hacl::bignum_k256::is_felem_eq_vartime(&mut tmp, x);
  b
}

pub fn ecdsa_sign_sha256(
  signature: &mut [u8],
  msg_len: u32,
  msg: &mut [u8],
  private_key: &mut [u8],
  nonce: &mut [u8]
) ->
  bool
{
  let mut msgHash: [u8; 32] = [0u8; 32u32 as usize];
  crate::hacl::hash_sha2::hash_256(msg, msg_len, &mut msgHash);
  let b: bool =
    crate::hacl::k256::ecdsa::ecdsa_sign_hashed_msg(signature, &mut msgHash, private_key, nonce);
  b
}

pub fn ecdsa_verify_hashed_msg(m: &mut [u8], public_key: &mut [u8], signature: &mut [u8]) ->
  bool
{
  let mut tmp: [u64; 35] = [0u64; 35u32 as usize];
  let pk: (&mut [u64], &mut [u64]) = (&mut tmp).split_at_mut(0usize);
  let r_q: (&mut [u64], &mut [u64]) = pk.1.split_at_mut(15usize);
  let s_q: (&mut [u64], &mut [u64]) = r_q.1.split_at_mut(4usize);
  let u1: (&mut [u64], &mut [u64]) = s_q.1.split_at_mut(4usize);
  let u2: (&mut [u64], &mut [u64]) = u1.1.split_at_mut(4usize);
  let m_q: (&mut [u64], &mut [u64]) = u2.1.split_at_mut(4usize);
  let is_pk_valid: bool = crate::hacl::impl::k256::point::load_point_vartime(r_q.0, public_key);
  let is_r_valid: bool = load_qelem_vartime(s_q.0, &mut signature[0u32 as usize..]);
  let is_s_valid: bool = load_qelem_vartime(u1.0, &mut signature[32u32 as usize..]);
  let is_rs_valid: bool = is_r_valid && is_s_valid;
  load_qelem_modq(m_q.1, m);
  if ! (is_pk_valid && is_rs_valid)
  { falsebool }
  else
  {
    let mut sinv: [u64; 4] = [0u64; 4u32 as usize];
    qinv(&mut sinv, u1.0);
    qmul(u2.0, m_q.1, &mut sinv);
    qmul(m_q.0, s_q.0, &mut sinv);
    let mut res: [u64; 15] = [0u64; 15u32 as usize];
    crate::hacl::impl::k256::glv::point_mul_g_double_split_lambda_vartime(
      &mut res,
      u2.0,
      m_q.0,
      r_q.0
    );
    let mut tmp1: [u64; 5] = [0u64; 5u32 as usize];
    let pz: (&mut [u64], &mut [u64]) = (&mut res).split_at_mut(10usize);
    crate::hacl::bignum_k256::fnormalize(&mut tmp1, pz.1);
    let b: bool = crate::hacl::bignum_k256::is_felem_zero_vartime(&mut tmp1);
    if b
    { falsebool }
    else
    {
      let x: (&mut [u64], &mut [u64]) = pz.0.split_at_mut(0usize);
      let z: (&mut [u64], &mut [u64]) = pz.1.split_at_mut(0usize);
      let mut r_bytes: [u8; 32] = [0u8; 32u32 as usize];
      let mut r_fe: [u64; 5] = [0u64; 5u32 as usize];
      let mut tmp_q: [u64; 5] = [0u64; 5u32 as usize];
      let mut tmp_x: [u64; 5] = [0u64; 5u32 as usize];
      crate::hacl::k256::scalar::store_qelem(&mut r_bytes, s_q.0);
      crate::hacl::bignum_k256::load_felem(&mut r_fe, &mut r_bytes);
      crate::hacl::bignum_k256::fnormalize(&mut tmp_x, x.1);
      let is_rz_x: bool = fmul_eq_vartime(&mut r_fe, z.1, &mut tmp_x);
      if ! is_rz_x
      {
        let is_r_lt_p_m_q: bool =
          crate::hacl::bignum_k256::is_felem_lt_prime_minus_order_vartime(&mut r_fe);
        if is_r_lt_p_m_q
        {
          (&mut tmp_q)[0u32 as usize] = 0x25e8cd0364141u64;
          (&mut tmp_q)[1u32 as usize] = 0xe6af48a03bbfdu64;
          (&mut tmp_q)[2u32 as usize] = 0xffffffebaaedcu64;
          (&mut tmp_q)[3u32 as usize] = 0xfffffffffffffu64;
          (&mut tmp_q)[4u32 as usize] = 0xffffffffffffu64;
          crate::hacl::bignum_k256::fadd(&mut tmp_q, &mut r_fe, &mut tmp_q);
          fmul_eq_vartime(&mut tmp_q, z.1, &mut tmp_x)
        }
        else
        { falsebool }
      }
      else
      { truebool }
    }
  }
}

pub fn ecdsa_verify_sha256(
  msg_len: u32,
  msg: &mut [u8],
  public_key: &mut [u8],
  signature: &mut [u8]
) ->
  bool
{
  let mut mHash: [u8; 32] = [0u8; 32u32 as usize];
  crate::hacl::hash_sha2::hash_256(msg, msg_len, &mut mHash);
  let b: bool = ecdsa_verify_hashed_msg(&mut mHash, public_key, signature);
  b
}

pub fn secp256k1_ecdsa_signature_normalize(signature: &mut [u8]) -> bool
{
  let mut s_q: [u64; 4] = [0u64; 4u32 as usize];
  let s: (&mut [u8], &mut [u8]) = signature.split_at_mut(32usize);
  let is_sk_valid: bool = load_qelem_vartime(&mut s_q, s.1);
  if ! is_sk_valid
  { falsebool }
  else
  {
    let is_sk_lt_q_halved: bool = is_qelem_le_q_halved_vartime(&mut s_q);
    qnegate_conditional_vartime(&mut s_q, ! is_sk_lt_q_halved);
    crate::hacl::k256::scalar::store_qelem(&mut s.1[32u32 as usize..], &mut s_q);
    truebool
  }
}

pub fn secp256k1_ecdsa_is_signature_normalized(signature: &mut [u8]) -> bool
{
  let mut s_q: [u64; 4] = [0u64; 4u32 as usize];
  let s: (&mut [u8], &mut [u8]) = signature.split_at_mut(32usize);
  let is_s_valid: bool = load_qelem_vartime(&mut s_q, s.1);
  let is_s_lt_q_halved: bool = is_qelem_le_q_halved_vartime(&mut s_q);
  is_s_valid && is_s_lt_q_halved
}

pub fn secp256k1_ecdsa_sign_hashed_msg(
  signature: &mut [u8],
  msgHash: &mut [u8],
  private_key: &mut [u8],
  nonce: &mut [u8]
) ->
  bool
{
  let b: bool =
    crate::hacl::k256::ecdsa::ecdsa_sign_hashed_msg(signature, msgHash, private_key, nonce);
  if b { secp256k1_ecdsa_signature_normalize(signature) } else { falsebool }
}

pub fn secp256k1_ecdsa_sign_sha256(
  signature: &mut [u8],
  msg_len: u32,
  msg: &mut [u8],
  private_key: &mut [u8],
  nonce: &mut [u8]
) ->
  bool
{
  let mut msgHash: [u8; 32] = [0u8; 32u32 as usize];
  crate::hacl::hash_sha2::hash_256(msg, msg_len, &mut msgHash);
  let b: bool = secp256k1_ecdsa_sign_hashed_msg(signature, &mut msgHash, private_key, nonce);
  b
}

pub fn secp256k1_ecdsa_verify_hashed_msg(
  msgHash: &mut [u8],
  public_key: &mut [u8],
  signature: &mut [u8]
) ->
  bool
{
  let is_s_normalized: bool = secp256k1_ecdsa_is_signature_normalized(signature);
  if ! is_s_normalized
  { falsebool }
  else
  { ecdsa_verify_hashed_msg(msgHash, public_key, signature) }
}

pub fn secp256k1_ecdsa_verify_sha256(
  msg_len: u32,
  msg: &mut [u8],
  public_key: &mut [u8],
  signature: &mut [u8]
) ->
  bool
{
  let mut mHash: [u8; 32] = [0u8; 32u32 as usize];
  crate::hacl::hash_sha2::hash_256(msg, msg_len, &mut mHash);
  let b: bool = secp256k1_ecdsa_verify_hashed_msg(&mut mHash, public_key, signature);
  b
}

pub fn public_key_uncompressed_to_raw(pk_raw: &mut [u8], pk: &mut [u8]) -> bool
{
  let pk0: u8 = pk[0u32 as usize];
  if pk0 != 0x04u8
  { falsebool }
  else
  {
    (pk_raw[0u32 as usize..0u32 as usize + 64u32 as usize]).copy_from_slice(
      &(&mut pk[1u32 as usize..])[0u32 as usize..0u32 as usize + 64u32 as usize]
    );
    truebool
  }
}

pub fn public_key_uncompressed_from_raw(pk: &mut [u8], pk_raw: &mut [u8]) -> ()
{
  pk[0u32 as usize] = 0x04u8;
  (pk[1u32 as usize..1u32 as usize + 64u32 as usize]).copy_from_slice(
    &pk_raw[0u32 as usize..0u32 as usize + 64u32 as usize]
  )
}

pub fn public_key_compressed_to_raw(pk_raw: &mut [u8], pk: &mut [u8]) -> bool
{
  let mut xa: [u64; 5] = [0u64; 5u32 as usize];
  let mut ya: [u64; 5] = [0u64; 5u32 as usize];
  let pk_xb: (&mut [u8], &mut [u8]) = pk.split_at_mut(1usize);
  let b: bool = aff_point_decompress_vartime(&mut xa, &mut ya, pk_xb.1);
  if b
  {
    (pk_raw[0u32 as usize..0u32 as usize + 32u32 as usize]).copy_from_slice(
      &pk_xb.1[0u32 as usize..0u32 as usize + 32u32 as usize]
    );
    crate::hacl::bignum_k256::store_felem(&mut pk_raw[32u32 as usize..], &mut ya)
  };
  b
}

pub fn public_key_compressed_from_raw(pk: &mut [u8], pk_raw: &mut [u8]) -> ()
{
  let pk_x: (&mut [u8], &mut [u8]) = pk_raw.split_at_mut(0usize);
  let pk_y: (&mut [u8], &mut [u8]) = pk_x.1.split_at_mut(32usize);
  let x0: u8 = pk_y.1[31u32 as usize];
  let is_pk_y_odd: bool = x0 & 1u8 == 1u8;
  let ite: u8 = if is_pk_y_odd { 0x03u8 } else { 0x02u8 };
  pk[0u32 as usize] = ite;
  (pk[1u32 as usize..1u32 as usize + 32u32 as usize]).copy_from_slice(
    &pk_y.0[0u32 as usize..0u32 as usize + 32u32 as usize]
  )
}

pub fn is_public_key_valid(public_key: &mut [u8]) -> bool
{
  let mut p: [u64; 15] = [0u64; 15u32 as usize];
  let is_pk_valid: bool = crate::hacl::impl::k256::point::load_point_vartime(&mut p, public_key);
  is_pk_valid
}

pub fn is_private_key_valid(private_key: &mut [u8]) -> bool
{
  let mut s_q: [u64; 4] = [0u64; 4u32 as usize];
  let res: u64 = load_qelem_check(&mut s_q, private_key);
  res == 0xFFFFFFFFFFFFFFFFu64
}

pub fn secret_to_public(public_key: &mut [u8], private_key: &mut [u8]) -> bool
{
  let mut tmp: [u64; 19] = [0u64; 19u32 as usize];
  let pk: (&mut [u64], &mut [u64]) = (&mut tmp).split_at_mut(0usize);
  let sk: (&mut [u64], &mut [u64]) = pk.1.split_at_mut(15usize);
  let is_b_valid: u64 = load_qelem_check(sk.1, private_key);
  let mut oneq: [u64; 4] = [0x1u64, 0x0u64, 0x0u64, 0x0u64];
  for i in 0u32..4u32
  {
    let os: (&mut [u64], &mut [u64]) = sk.1.split_at_mut(0usize);
    let uu____0: u64 = (&mut oneq)[i as usize];
    let x: u64 = uu____0 ^ is_b_valid & (os.1[i as usize] ^ uu____0);
    os.1[i as usize] = x
  };
  let is_sk_valid: u64 = is_b_valid;
  crate::hacl::impl::k256::pointmul::point_mul_g(sk.0, sk.1);
  point_store(public_key, sk.0);
  is_sk_valid == 0xFFFFFFFFFFFFFFFFu64
}

pub fn ecdh(shared_secret: &mut [u8], their_pubkey: &mut [u8], private_key: &mut [u8]) -> bool
{
  let mut tmp: [u64; 34] = [0u64; 34u32 as usize];
  let pk: (&mut [u64], &mut [u64]) = (&mut tmp).split_at_mut(0usize);
  let ss: (&mut [u64], &mut [u64]) = pk.1.split_at_mut(15usize);
  let sk: (&mut [u64], &mut [u64]) = ss.1.split_at_mut(15usize);
  let is_pk_valid: bool = crate::hacl::impl::k256::point::load_point_vartime(ss.0, their_pubkey);
  let is_b_valid: u64 = load_qelem_check(sk.1, private_key);
  let mut oneq: [u64; 4] = [0x1u64, 0x0u64, 0x0u64, 0x0u64];
  for i in 0u32..4u32
  {
    let os: (&mut [u64], &mut [u64]) = sk.1.split_at_mut(0usize);
    let uu____0: u64 = (&mut oneq)[i as usize];
    let x: u64 = uu____0 ^ is_b_valid & (os.1[i as usize] ^ uu____0);
    os.1[i as usize] = x
  };
  let is_sk_valid: u64 = is_b_valid;
  if is_pk_valid
  {
    crate::hacl::impl::k256::pointmul::point_mul(sk.0, sk.1, ss.0);
    point_store(shared_secret, sk.0)
  };
  is_sk_valid == 0xFFFFFFFFFFFFFFFFu64 && is_pk_valid
}