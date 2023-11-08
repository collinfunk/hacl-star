pub fn load_acc4(acc: &mut [crate::lib::intvector::intrinsics::vec256], b: &mut [u8]) -> ()
{
  let mut e: [crate::lib::intvector::intrinsics::vec256; 5] =
    [crate::lib::intvector::intrinsics::vec256_zero; 5u32 as usize];
  let lo: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64_le(&mut b[0u32 as usize..]);
  let hi: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64_le(&mut b[32u32 as usize..]);
  let mask26: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64);
  let m0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low128(lo, hi);
  let m1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_high128(lo, hi);
  let m2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right(m0, 48u32);
  let m3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right(m1, 48u32);
  let m4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_high64(m0, m1);
  let t0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low64(m0, m1);
  let t3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low64(m2, m3);
  let t2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(t3, 4u32);
  let o2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t2, mask26);
  let t1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(t0, 26u32);
  let o1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t1, mask26);
  let o5: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t0, mask26);
  let t31: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(t3, 30u32);
  let o3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t31, mask26);
  let o4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(m4, 40u32);
  let o0: crate::lib::intvector::intrinsics::vec256 = o5;
  let o1: crate::lib::intvector::intrinsics::vec256 = o1;
  let o2: crate::lib::intvector::intrinsics::vec256 = o2;
  let o3: crate::lib::intvector::intrinsics::vec256 = o3;
  let o4: crate::lib::intvector::intrinsics::vec256 = o4;
  (&mut e)[0u32 as usize] = o0;
  (&mut e)[1u32 as usize] = o1;
  (&mut e)[2u32 as usize] = o2;
  (&mut e)[3u32 as usize] = o3;
  (&mut e)[4u32 as usize] = o4;
  let b1: u64 = 0x1000000u64;
  let mask: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64(b1);
  let f4: crate::lib::intvector::intrinsics::vec256 = (&mut e)[4u32 as usize];
  (&mut e)[4u32 as usize] = crate::lib::intvector::intrinsics::vec256_or(f4, mask);
  let acc0: crate::lib::intvector::intrinsics::vec256 = acc[0u32 as usize];
  let acc1: crate::lib::intvector::intrinsics::vec256 = acc[1u32 as usize];
  let acc2: crate::lib::intvector::intrinsics::vec256 = acc[2u32 as usize];
  let acc3: crate::lib::intvector::intrinsics::vec256 = acc[3u32 as usize];
  let acc4: crate::lib::intvector::intrinsics::vec256 = acc[4u32 as usize];
  let e0: crate::lib::intvector::intrinsics::vec256 = (&mut e)[0u32 as usize];
  let e1: crate::lib::intvector::intrinsics::vec256 = (&mut e)[1u32 as usize];
  let e2: crate::lib::intvector::intrinsics::vec256 = (&mut e)[2u32 as usize];
  let e3: crate::lib::intvector::intrinsics::vec256 = (&mut e)[3u32 as usize];
  let e4: crate::lib::intvector::intrinsics::vec256 = (&mut e)[4u32 as usize];
  let r0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_zero;
  let r1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_zero;
  let r2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_zero;
  let r3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_zero;
  let r4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_zero;
  let r01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_insert64(
      r0,
      crate::lib::intvector::intrinsics::vec256_extract64(acc0, 0u32),
      0u32
    );
  let r11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_insert64(
      r1,
      crate::lib::intvector::intrinsics::vec256_extract64(acc1, 0u32),
      0u32
    );
  let r21: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_insert64(
      r2,
      crate::lib::intvector::intrinsics::vec256_extract64(acc2, 0u32),
      0u32
    );
  let r31: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_insert64(
      r3,
      crate::lib::intvector::intrinsics::vec256_extract64(acc3, 0u32),
      0u32
    );
  let r41: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_insert64(
      r4,
      crate::lib::intvector::intrinsics::vec256_extract64(acc4, 0u32),
      0u32
    );
  let f0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(r01, e0);
  let f1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(r11, e1);
  let f2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(r21, e2);
  let f3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(r31, e3);
  let f4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(r41, e4);
  let acc01: crate::lib::intvector::intrinsics::vec256 = f0;
  let acc11: crate::lib::intvector::intrinsics::vec256 = f1;
  let acc21: crate::lib::intvector::intrinsics::vec256 = f2;
  let acc31: crate::lib::intvector::intrinsics::vec256 = f3;
  let acc41: crate::lib::intvector::intrinsics::vec256 = f4;
  acc[0u32 as usize] = acc01;
  acc[1u32 as usize] = acc11;
  acc[2u32 as usize] = acc21;
  acc[3u32 as usize] = acc31;
  acc[4u32 as usize] = acc41
}

pub fn fmul_r4_normalize(
  out: &mut [crate::lib::intvector::intrinsics::vec256],
  p: &mut [crate::lib::intvector::intrinsics::vec256]
) ->
  ()
{
  let
  r:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    p.split_at_mut(0usize);
  let
  r_5:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    r.1.split_at_mut(5usize);
  let
  r4:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    r_5.1.split_at_mut(5usize);
  let a0: crate::lib::intvector::intrinsics::vec256 = out[0u32 as usize];
  let a1: crate::lib::intvector::intrinsics::vec256 = out[1u32 as usize];
  let a2: crate::lib::intvector::intrinsics::vec256 = out[2u32 as usize];
  let a3: crate::lib::intvector::intrinsics::vec256 = out[3u32 as usize];
  let a4: crate::lib::intvector::intrinsics::vec256 = out[4u32 as usize];
  let r10: crate::lib::intvector::intrinsics::vec256 = r_5.0[0u32 as usize];
  let r11: crate::lib::intvector::intrinsics::vec256 = r_5.0[1u32 as usize];
  let r12: crate::lib::intvector::intrinsics::vec256 = r_5.0[2u32 as usize];
  let r13: crate::lib::intvector::intrinsics::vec256 = r_5.0[3u32 as usize];
  let r14: crate::lib::intvector::intrinsics::vec256 = r_5.0[4u32 as usize];
  let r151: crate::lib::intvector::intrinsics::vec256 = r4.0[1u32 as usize];
  let r152: crate::lib::intvector::intrinsics::vec256 = r4.0[2u32 as usize];
  let r153: crate::lib::intvector::intrinsics::vec256 = r4.0[3u32 as usize];
  let r154: crate::lib::intvector::intrinsics::vec256 = r4.0[4u32 as usize];
  let r40: crate::lib::intvector::intrinsics::vec256 = r4.1[0u32 as usize];
  let r41: crate::lib::intvector::intrinsics::vec256 = r4.1[1u32 as usize];
  let r42: crate::lib::intvector::intrinsics::vec256 = r4.1[2u32 as usize];
  let r43: crate::lib::intvector::intrinsics::vec256 = r4.1[3u32 as usize];
  let r44: crate::lib::intvector::intrinsics::vec256 = r4.1[4u32 as usize];
  let a01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r10, r10);
  let a11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r11, r10);
  let a21: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r12, r10);
  let a31: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r13, r10);
  let a41: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r14, r10);
  let a02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a01,
      crate::lib::intvector::intrinsics::vec256_mul64(r154, r11)
    );
  let a12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a11,
      crate::lib::intvector::intrinsics::vec256_mul64(r10, r11)
    );
  let a22: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a21,
      crate::lib::intvector::intrinsics::vec256_mul64(r11, r11)
    );
  let a32: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a31,
      crate::lib::intvector::intrinsics::vec256_mul64(r12, r11)
    );
  let a42: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a41,
      crate::lib::intvector::intrinsics::vec256_mul64(r13, r11)
    );
  let a03: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a02,
      crate::lib::intvector::intrinsics::vec256_mul64(r153, r12)
    );
  let a13: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a12,
      crate::lib::intvector::intrinsics::vec256_mul64(r154, r12)
    );
  let a23: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a22,
      crate::lib::intvector::intrinsics::vec256_mul64(r10, r12)
    );
  let a33: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a32,
      crate::lib::intvector::intrinsics::vec256_mul64(r11, r12)
    );
  let a43: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a42,
      crate::lib::intvector::intrinsics::vec256_mul64(r12, r12)
    );
  let a04: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a03,
      crate::lib::intvector::intrinsics::vec256_mul64(r152, r13)
    );
  let a14: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a13,
      crate::lib::intvector::intrinsics::vec256_mul64(r153, r13)
    );
  let a24: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a23,
      crate::lib::intvector::intrinsics::vec256_mul64(r154, r13)
    );
  let a34: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a33,
      crate::lib::intvector::intrinsics::vec256_mul64(r10, r13)
    );
  let a44: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a43,
      crate::lib::intvector::intrinsics::vec256_mul64(r11, r13)
    );
  let a05: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a04,
      crate::lib::intvector::intrinsics::vec256_mul64(r151, r14)
    );
  let a15: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a14,
      crate::lib::intvector::intrinsics::vec256_mul64(r152, r14)
    );
  let a25: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a24,
      crate::lib::intvector::intrinsics::vec256_mul64(r153, r14)
    );
  let a35: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a34,
      crate::lib::intvector::intrinsics::vec256_mul64(r154, r14)
    );
  let a45: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a44,
      crate::lib::intvector::intrinsics::vec256_mul64(r10, r14)
    );
  let t0: crate::lib::intvector::intrinsics::vec256 = a05;
  let t1: crate::lib::intvector::intrinsics::vec256 = a15;
  let t2: crate::lib::intvector::intrinsics::vec256 = a25;
  let t3: crate::lib::intvector::intrinsics::vec256 = a35;
  let t4: crate::lib::intvector::intrinsics::vec256 = a45;
  let mask26: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64);
  let z0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(t0, 26u32);
  let z1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(t3, 26u32);
  let x0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t0, mask26);
  let x3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t3, mask26);
  let x1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t1, z0);
  let x4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t4, z1);
  let z01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x1, 26u32);
  let z11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x4, 26u32);
  let t: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_left64(z11, 2u32);
  let z12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(z11, t);
  let x11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x1, mask26);
  let x41: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x4, mask26);
  let x2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t2, z01);
  let x01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x0, z12);
  let z02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x2, 26u32);
  let z13: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x01, 26u32);
  let x21: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x2, mask26);
  let x02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x01, mask26);
  let x31: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x3, z02);
  let x12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x11, z13);
  let z03: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x31, 26u32);
  let x32: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x31, mask26);
  let x42: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x41, z03);
  let r20: crate::lib::intvector::intrinsics::vec256 = x02;
  let r21: crate::lib::intvector::intrinsics::vec256 = x12;
  let r22: crate::lib::intvector::intrinsics::vec256 = x21;
  let r23: crate::lib::intvector::intrinsics::vec256 = x32;
  let r24: crate::lib::intvector::intrinsics::vec256 = x42;
  let a01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r10, r20);
  let a11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r11, r20);
  let a21: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r12, r20);
  let a31: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r13, r20);
  let a41: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r14, r20);
  let a02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a01,
      crate::lib::intvector::intrinsics::vec256_mul64(r154, r21)
    );
  let a12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a11,
      crate::lib::intvector::intrinsics::vec256_mul64(r10, r21)
    );
  let a22: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a21,
      crate::lib::intvector::intrinsics::vec256_mul64(r11, r21)
    );
  let a32: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a31,
      crate::lib::intvector::intrinsics::vec256_mul64(r12, r21)
    );
  let a42: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a41,
      crate::lib::intvector::intrinsics::vec256_mul64(r13, r21)
    );
  let a03: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a02,
      crate::lib::intvector::intrinsics::vec256_mul64(r153, r22)
    );
  let a13: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a12,
      crate::lib::intvector::intrinsics::vec256_mul64(r154, r22)
    );
  let a23: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a22,
      crate::lib::intvector::intrinsics::vec256_mul64(r10, r22)
    );
  let a33: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a32,
      crate::lib::intvector::intrinsics::vec256_mul64(r11, r22)
    );
  let a43: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a42,
      crate::lib::intvector::intrinsics::vec256_mul64(r12, r22)
    );
  let a04: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a03,
      crate::lib::intvector::intrinsics::vec256_mul64(r152, r23)
    );
  let a14: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a13,
      crate::lib::intvector::intrinsics::vec256_mul64(r153, r23)
    );
  let a24: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a23,
      crate::lib::intvector::intrinsics::vec256_mul64(r154, r23)
    );
  let a34: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a33,
      crate::lib::intvector::intrinsics::vec256_mul64(r10, r23)
    );
  let a44: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a43,
      crate::lib::intvector::intrinsics::vec256_mul64(r11, r23)
    );
  let a05: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a04,
      crate::lib::intvector::intrinsics::vec256_mul64(r151, r24)
    );
  let a15: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a14,
      crate::lib::intvector::intrinsics::vec256_mul64(r152, r24)
    );
  let a25: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a24,
      crate::lib::intvector::intrinsics::vec256_mul64(r153, r24)
    );
  let a35: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a34,
      crate::lib::intvector::intrinsics::vec256_mul64(r154, r24)
    );
  let a45: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a44,
      crate::lib::intvector::intrinsics::vec256_mul64(r10, r24)
    );
  let t0: crate::lib::intvector::intrinsics::vec256 = a05;
  let t1: crate::lib::intvector::intrinsics::vec256 = a15;
  let t2: crate::lib::intvector::intrinsics::vec256 = a25;
  let t3: crate::lib::intvector::intrinsics::vec256 = a35;
  let t4: crate::lib::intvector::intrinsics::vec256 = a45;
  let mask26: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64);
  let z0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(t0, 26u32);
  let z1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(t3, 26u32);
  let x0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t0, mask26);
  let x3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t3, mask26);
  let x1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t1, z0);
  let x4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t4, z1);
  let z01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x1, 26u32);
  let z11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x4, 26u32);
  let t: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_left64(z11, 2u32);
  let z12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(z11, t);
  let x11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x1, mask26);
  let x41: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x4, mask26);
  let x2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t2, z01);
  let x01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x0, z12);
  let z02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x2, 26u32);
  let z13: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x01, 26u32);
  let x21: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x2, mask26);
  let x02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x01, mask26);
  let x31: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x3, z02);
  let x12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x11, z13);
  let z03: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x31, 26u32);
  let x32: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x31, mask26);
  let x42: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x41, z03);
  let r30: crate::lib::intvector::intrinsics::vec256 = x02;
  let r31: crate::lib::intvector::intrinsics::vec256 = x12;
  let r32: crate::lib::intvector::intrinsics::vec256 = x21;
  let r33: crate::lib::intvector::intrinsics::vec256 = x32;
  let r34: crate::lib::intvector::intrinsics::vec256 = x42;
  let v12120: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low64(r20, r10);
  let v34340: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low64(r40, r30);
  let r12340: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low128(v34340, v12120);
  let v12121: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low64(r21, r11);
  let v34341: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low64(r41, r31);
  let r12341: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low128(v34341, v12121);
  let v12122: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low64(r22, r12);
  let v34342: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low64(r42, r32);
  let r12342: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low128(v34342, v12122);
  let v12123: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low64(r23, r13);
  let v34343: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low64(r43, r33);
  let r12343: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low128(v34343, v12123);
  let v12124: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low64(r24, r14);
  let v34344: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low64(r44, r34);
  let r12344: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_low128(v34344, v12124);
  let r123451: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_smul64(r12341, 5u64);
  let r123452: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_smul64(r12342, 5u64);
  let r123453: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_smul64(r12343, 5u64);
  let r123454: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_smul64(r12344, 5u64);
  let a01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r12340, a0);
  let a11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r12341, a0);
  let a21: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r12342, a0);
  let a31: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r12343, a0);
  let a41: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r12344, a0);
  let a02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a01,
      crate::lib::intvector::intrinsics::vec256_mul64(r123454, a1)
    );
  let a12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a11,
      crate::lib::intvector::intrinsics::vec256_mul64(r12340, a1)
    );
  let a22: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a21,
      crate::lib::intvector::intrinsics::vec256_mul64(r12341, a1)
    );
  let a32: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a31,
      crate::lib::intvector::intrinsics::vec256_mul64(r12342, a1)
    );
  let a42: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a41,
      crate::lib::intvector::intrinsics::vec256_mul64(r12343, a1)
    );
  let a03: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a02,
      crate::lib::intvector::intrinsics::vec256_mul64(r123453, a2)
    );
  let a13: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a12,
      crate::lib::intvector::intrinsics::vec256_mul64(r123454, a2)
    );
  let a23: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a22,
      crate::lib::intvector::intrinsics::vec256_mul64(r12340, a2)
    );
  let a33: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a32,
      crate::lib::intvector::intrinsics::vec256_mul64(r12341, a2)
    );
  let a43: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a42,
      crate::lib::intvector::intrinsics::vec256_mul64(r12342, a2)
    );
  let a04: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a03,
      crate::lib::intvector::intrinsics::vec256_mul64(r123452, a3)
    );
  let a14: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a13,
      crate::lib::intvector::intrinsics::vec256_mul64(r123453, a3)
    );
  let a24: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a23,
      crate::lib::intvector::intrinsics::vec256_mul64(r123454, a3)
    );
  let a34: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a33,
      crate::lib::intvector::intrinsics::vec256_mul64(r12340, a3)
    );
  let a44: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a43,
      crate::lib::intvector::intrinsics::vec256_mul64(r12341, a3)
    );
  let a05: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a04,
      crate::lib::intvector::intrinsics::vec256_mul64(r123451, a4)
    );
  let a15: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a14,
      crate::lib::intvector::intrinsics::vec256_mul64(r123452, a4)
    );
  let a25: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a24,
      crate::lib::intvector::intrinsics::vec256_mul64(r123453, a4)
    );
  let a35: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a34,
      crate::lib::intvector::intrinsics::vec256_mul64(r123454, a4)
    );
  let a45: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a44,
      crate::lib::intvector::intrinsics::vec256_mul64(r12340, a4)
    );
  let t0: crate::lib::intvector::intrinsics::vec256 = a05;
  let t1: crate::lib::intvector::intrinsics::vec256 = a15;
  let t2: crate::lib::intvector::intrinsics::vec256 = a25;
  let t3: crate::lib::intvector::intrinsics::vec256 = a35;
  let t4: crate::lib::intvector::intrinsics::vec256 = a45;
  let mask26: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64);
  let z0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(t0, 26u32);
  let z1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(t3, 26u32);
  let x0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t0, mask26);
  let x3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t3, mask26);
  let x1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t1, z0);
  let x4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t4, z1);
  let z01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x1, 26u32);
  let z11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x4, 26u32);
  let t: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_left64(z11, 2u32);
  let z12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(z11, t);
  let x11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x1, mask26);
  let x41: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x4, mask26);
  let x2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t2, z01);
  let x01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x0, z12);
  let z02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x2, 26u32);
  let z13: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x01, 26u32);
  let x21: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x2, mask26);
  let x02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x01, mask26);
  let x31: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x3, z02);
  let x12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x11, z13);
  let z03: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x31, 26u32);
  let x32: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x31, mask26);
  let x42: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x41, z03);
  let o0: crate::lib::intvector::intrinsics::vec256 = x02;
  let o1: crate::lib::intvector::intrinsics::vec256 = x12;
  let o2: crate::lib::intvector::intrinsics::vec256 = x21;
  let o3: crate::lib::intvector::intrinsics::vec256 = x32;
  let o4: crate::lib::intvector::intrinsics::vec256 = x42;
  let v00: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_high128(o0, o0);
  let v10: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(o0, v00);
  let v10h: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_high64(v10, v10);
  let v20: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(v10, v10h);
  let v01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_high128(o1, o1);
  let v11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(o1, v01);
  let v11h: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_high64(v11, v11);
  let v21: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(v11, v11h);
  let v02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_high128(o2, o2);
  let v12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(o2, v02);
  let v12h: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_high64(v12, v12);
  let v22: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(v12, v12h);
  let v03: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_high128(o3, o3);
  let v13: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(o3, v03);
  let v13h: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_high64(v13, v13);
  let v23: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(v13, v13h);
  let v04: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_high128(o4, o4);
  let v14: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(o4, v04);
  let v14h: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_interleave_high64(v14, v14);
  let v24: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(v14, v14h);
  let l: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      v20,
      crate::lib::intvector::intrinsics::vec256_zero
    );
  let tmp0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      l,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let c0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(l, 26u32);
  let l: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(v21, c0);
  let tmp1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      l,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let c1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(l, 26u32);
  let l: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(v22, c1);
  let tmp2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      l,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let c2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(l, 26u32);
  let l: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(v23, c2);
  let tmp3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      l,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let c3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(l, 26u32);
  let l: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(v24, c3);
  let tmp4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      l,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let c4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(l, 26u32);
  let o0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      tmp0,
      crate::lib::intvector::intrinsics::vec256_smul64(c4, 5u64)
    );
  let o1: crate::lib::intvector::intrinsics::vec256 = tmp1;
  let o2: crate::lib::intvector::intrinsics::vec256 = tmp2;
  let o3: crate::lib::intvector::intrinsics::vec256 = tmp3;
  let o4: crate::lib::intvector::intrinsics::vec256 = tmp4;
  out[0u32 as usize] = o0;
  out[1u32 as usize] = o1;
  out[2u32 as usize] = o2;
  out[3u32 as usize] = o3;
  out[4u32 as usize] = o4
}

pub fn poly1305_init(ctx: &mut [crate::lib::intvector::intrinsics::vec256], key: &mut [u8]) ->
  ()
{
  let
  acc:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    ctx.split_at_mut(0usize);
  let
  pre:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    acc.1.split_at_mut(5usize);
  let kr: (&mut [u8], &mut [u8]) = key.split_at_mut(0usize);
  pre.0[0u32 as usize] = crate::lib::intvector::intrinsics::vec256_zero;
  pre.0[1u32 as usize] = crate::lib::intvector::intrinsics::vec256_zero;
  pre.0[2u32 as usize] = crate::lib::intvector::intrinsics::vec256_zero;
  pre.0[3u32 as usize] = crate::lib::intvector::intrinsics::vec256_zero;
  pre.0[4u32 as usize] = crate::lib::intvector::intrinsics::vec256_zero;
  let u: u64 = crate::lowstar::endianness::load64_le(&mut kr.1[0u32 as usize..]);
  let lo: u64 = u;
  let u: u64 = crate::lowstar::endianness::load64_le(&mut kr.1[8u32 as usize..]);
  let hi: u64 = u;
  let mask0: u64 = 0x0ffffffc0fffffffu64;
  let mask1: u64 = 0x0ffffffc0ffffffcu64;
  let lo1: u64 = lo & mask0;
  let hi1: u64 = hi & mask1;
  let
  r:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    pre.1.split_at_mut(0usize);
  let
  r5:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    r.1.split_at_mut(5usize);
  let
  rn:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    r5.1.split_at_mut(5usize);
  let
  rn_5:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    rn.1.split_at_mut(5usize);
  let r_vec0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64(lo1);
  let r_vec1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64(hi1);
  let f0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      r_vec0,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let f1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      crate::lib::intvector::intrinsics::vec256_shift_right64(r_vec0, 26u32),
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let f2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_or(
      crate::lib::intvector::intrinsics::vec256_shift_right64(r_vec0, 52u32),
      crate::lib::intvector::intrinsics::vec256_shift_left64(
        crate::lib::intvector::intrinsics::vec256_and(
          r_vec1,
          crate::lib::intvector::intrinsics::vec256_load64(0x3fffu64)
        ),
        12u32
      )
    );
  let f3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      crate::lib::intvector::intrinsics::vec256_shift_right64(r_vec1, 14u32),
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let f4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(r_vec1, 40u32);
  let f0: crate::lib::intvector::intrinsics::vec256 = f0;
  let f1: crate::lib::intvector::intrinsics::vec256 = f1;
  let f2: crate::lib::intvector::intrinsics::vec256 = f2;
  let f3: crate::lib::intvector::intrinsics::vec256 = f3;
  let f4: crate::lib::intvector::intrinsics::vec256 = f4;
  r5.0[0u32 as usize] = f0;
  r5.0[1u32 as usize] = f1;
  r5.0[2u32 as usize] = f2;
  r5.0[3u32 as usize] = f3;
  r5.0[4u32 as usize] = f4;
  let f20: crate::lib::intvector::intrinsics::vec256 = r5.0[0u32 as usize];
  let f21: crate::lib::intvector::intrinsics::vec256 = r5.0[1u32 as usize];
  let f22: crate::lib::intvector::intrinsics::vec256 = r5.0[2u32 as usize];
  let f23: crate::lib::intvector::intrinsics::vec256 = r5.0[3u32 as usize];
  let f24: crate::lib::intvector::intrinsics::vec256 = r5.0[4u32 as usize];
  rn.0[0u32 as usize] = crate::lib::intvector::intrinsics::vec256_smul64(f20, 5u64);
  rn.0[1u32 as usize] = crate::lib::intvector::intrinsics::vec256_smul64(f21, 5u64);
  rn.0[2u32 as usize] = crate::lib::intvector::intrinsics::vec256_smul64(f22, 5u64);
  rn.0[3u32 as usize] = crate::lib::intvector::intrinsics::vec256_smul64(f23, 5u64);
  rn.0[4u32 as usize] = crate::lib::intvector::intrinsics::vec256_smul64(f24, 5u64);
  let r0: crate::lib::intvector::intrinsics::vec256 = r5.0[0u32 as usize];
  let r1: crate::lib::intvector::intrinsics::vec256 = r5.0[1u32 as usize];
  let r2: crate::lib::intvector::intrinsics::vec256 = r5.0[2u32 as usize];
  let r3: crate::lib::intvector::intrinsics::vec256 = r5.0[3u32 as usize];
  let r4: crate::lib::intvector::intrinsics::vec256 = r5.0[4u32 as usize];
  let r51: crate::lib::intvector::intrinsics::vec256 = rn.0[1u32 as usize];
  let r52: crate::lib::intvector::intrinsics::vec256 = rn.0[2u32 as usize];
  let r53: crate::lib::intvector::intrinsics::vec256 = rn.0[3u32 as usize];
  let r54: crate::lib::intvector::intrinsics::vec256 = rn.0[4u32 as usize];
  let f10: crate::lib::intvector::intrinsics::vec256 = r5.0[0u32 as usize];
  let f11: crate::lib::intvector::intrinsics::vec256 = r5.0[1u32 as usize];
  let f12: crate::lib::intvector::intrinsics::vec256 = r5.0[2u32 as usize];
  let f13: crate::lib::intvector::intrinsics::vec256 = r5.0[3u32 as usize];
  let f14: crate::lib::intvector::intrinsics::vec256 = r5.0[4u32 as usize];
  let a0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r0, f10);
  let a1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r1, f10);
  let a2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r2, f10);
  let a3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r3, f10);
  let a4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r4, f10);
  let a01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a0,
      crate::lib::intvector::intrinsics::vec256_mul64(r54, f11)
    );
  let a11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a1,
      crate::lib::intvector::intrinsics::vec256_mul64(r0, f11)
    );
  let a21: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a2,
      crate::lib::intvector::intrinsics::vec256_mul64(r1, f11)
    );
  let a31: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a3,
      crate::lib::intvector::intrinsics::vec256_mul64(r2, f11)
    );
  let a41: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a4,
      crate::lib::intvector::intrinsics::vec256_mul64(r3, f11)
    );
  let a02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a01,
      crate::lib::intvector::intrinsics::vec256_mul64(r53, f12)
    );
  let a12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a11,
      crate::lib::intvector::intrinsics::vec256_mul64(r54, f12)
    );
  let a22: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a21,
      crate::lib::intvector::intrinsics::vec256_mul64(r0, f12)
    );
  let a32: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a31,
      crate::lib::intvector::intrinsics::vec256_mul64(r1, f12)
    );
  let a42: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a41,
      crate::lib::intvector::intrinsics::vec256_mul64(r2, f12)
    );
  let a03: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a02,
      crate::lib::intvector::intrinsics::vec256_mul64(r52, f13)
    );
  let a13: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a12,
      crate::lib::intvector::intrinsics::vec256_mul64(r53, f13)
    );
  let a23: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a22,
      crate::lib::intvector::intrinsics::vec256_mul64(r54, f13)
    );
  let a33: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a32,
      crate::lib::intvector::intrinsics::vec256_mul64(r0, f13)
    );
  let a43: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a42,
      crate::lib::intvector::intrinsics::vec256_mul64(r1, f13)
    );
  let a04: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a03,
      crate::lib::intvector::intrinsics::vec256_mul64(r51, f14)
    );
  let a14: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a13,
      crate::lib::intvector::intrinsics::vec256_mul64(r52, f14)
    );
  let a24: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a23,
      crate::lib::intvector::intrinsics::vec256_mul64(r53, f14)
    );
  let a34: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a33,
      crate::lib::intvector::intrinsics::vec256_mul64(r54, f14)
    );
  let a44: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a43,
      crate::lib::intvector::intrinsics::vec256_mul64(r0, f14)
    );
  let t0: crate::lib::intvector::intrinsics::vec256 = a04;
  let t1: crate::lib::intvector::intrinsics::vec256 = a14;
  let t2: crate::lib::intvector::intrinsics::vec256 = a24;
  let t3: crate::lib::intvector::intrinsics::vec256 = a34;
  let t4: crate::lib::intvector::intrinsics::vec256 = a44;
  let mask26: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64);
  let z0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(t0, 26u32);
  let z1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(t3, 26u32);
  let x0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t0, mask26);
  let x3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t3, mask26);
  let x1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t1, z0);
  let x4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t4, z1);
  let z01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x1, 26u32);
  let z11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x4, 26u32);
  let t: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_left64(z11, 2u32);
  let z12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(z11, t);
  let x11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x1, mask26);
  let x41: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x4, mask26);
  let x2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t2, z01);
  let x01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x0, z12);
  let z02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x2, 26u32);
  let z13: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x01, 26u32);
  let x21: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x2, mask26);
  let x02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x01, mask26);
  let x31: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x3, z02);
  let x12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x11, z13);
  let z03: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x31, 26u32);
  let x32: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x31, mask26);
  let x42: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x41, z03);
  let o0: crate::lib::intvector::intrinsics::vec256 = x02;
  let o1: crate::lib::intvector::intrinsics::vec256 = x12;
  let o2: crate::lib::intvector::intrinsics::vec256 = x21;
  let o3: crate::lib::intvector::intrinsics::vec256 = x32;
  let o4: crate::lib::intvector::intrinsics::vec256 = x42;
  rn_5.0[0u32 as usize] = o0;
  rn_5.0[1u32 as usize] = o1;
  rn_5.0[2u32 as usize] = o2;
  rn_5.0[3u32 as usize] = o3;
  rn_5.0[4u32 as usize] = o4;
  let f20: crate::lib::intvector::intrinsics::vec256 = rn_5.0[0u32 as usize];
  let f21: crate::lib::intvector::intrinsics::vec256 = rn_5.0[1u32 as usize];
  let f22: crate::lib::intvector::intrinsics::vec256 = rn_5.0[2u32 as usize];
  let f23: crate::lib::intvector::intrinsics::vec256 = rn_5.0[3u32 as usize];
  let f24: crate::lib::intvector::intrinsics::vec256 = rn_5.0[4u32 as usize];
  rn_5.1[0u32 as usize] = crate::lib::intvector::intrinsics::vec256_smul64(f20, 5u64);
  rn_5.1[1u32 as usize] = crate::lib::intvector::intrinsics::vec256_smul64(f21, 5u64);
  rn_5.1[2u32 as usize] = crate::lib::intvector::intrinsics::vec256_smul64(f22, 5u64);
  rn_5.1[3u32 as usize] = crate::lib::intvector::intrinsics::vec256_smul64(f23, 5u64);
  rn_5.1[4u32 as usize] = crate::lib::intvector::intrinsics::vec256_smul64(f24, 5u64);
  let r0: crate::lib::intvector::intrinsics::vec256 = rn_5.0[0u32 as usize];
  let r1: crate::lib::intvector::intrinsics::vec256 = rn_5.0[1u32 as usize];
  let r2: crate::lib::intvector::intrinsics::vec256 = rn_5.0[2u32 as usize];
  let r3: crate::lib::intvector::intrinsics::vec256 = rn_5.0[3u32 as usize];
  let r4: crate::lib::intvector::intrinsics::vec256 = rn_5.0[4u32 as usize];
  let r51: crate::lib::intvector::intrinsics::vec256 = rn_5.1[1u32 as usize];
  let r52: crate::lib::intvector::intrinsics::vec256 = rn_5.1[2u32 as usize];
  let r53: crate::lib::intvector::intrinsics::vec256 = rn_5.1[3u32 as usize];
  let r54: crate::lib::intvector::intrinsics::vec256 = rn_5.1[4u32 as usize];
  let f10: crate::lib::intvector::intrinsics::vec256 = rn_5.0[0u32 as usize];
  let f11: crate::lib::intvector::intrinsics::vec256 = rn_5.0[1u32 as usize];
  let f12: crate::lib::intvector::intrinsics::vec256 = rn_5.0[2u32 as usize];
  let f13: crate::lib::intvector::intrinsics::vec256 = rn_5.0[3u32 as usize];
  let f14: crate::lib::intvector::intrinsics::vec256 = rn_5.0[4u32 as usize];
  let a0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r0, f10);
  let a1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r1, f10);
  let a2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r2, f10);
  let a3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r3, f10);
  let a4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r4, f10);
  let a01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a0,
      crate::lib::intvector::intrinsics::vec256_mul64(r54, f11)
    );
  let a11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a1,
      crate::lib::intvector::intrinsics::vec256_mul64(r0, f11)
    );
  let a21: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a2,
      crate::lib::intvector::intrinsics::vec256_mul64(r1, f11)
    );
  let a31: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a3,
      crate::lib::intvector::intrinsics::vec256_mul64(r2, f11)
    );
  let a41: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a4,
      crate::lib::intvector::intrinsics::vec256_mul64(r3, f11)
    );
  let a02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a01,
      crate::lib::intvector::intrinsics::vec256_mul64(r53, f12)
    );
  let a12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a11,
      crate::lib::intvector::intrinsics::vec256_mul64(r54, f12)
    );
  let a22: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a21,
      crate::lib::intvector::intrinsics::vec256_mul64(r0, f12)
    );
  let a32: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a31,
      crate::lib::intvector::intrinsics::vec256_mul64(r1, f12)
    );
  let a42: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a41,
      crate::lib::intvector::intrinsics::vec256_mul64(r2, f12)
    );
  let a03: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a02,
      crate::lib::intvector::intrinsics::vec256_mul64(r52, f13)
    );
  let a13: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a12,
      crate::lib::intvector::intrinsics::vec256_mul64(r53, f13)
    );
  let a23: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a22,
      crate::lib::intvector::intrinsics::vec256_mul64(r54, f13)
    );
  let a33: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a32,
      crate::lib::intvector::intrinsics::vec256_mul64(r0, f13)
    );
  let a43: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a42,
      crate::lib::intvector::intrinsics::vec256_mul64(r1, f13)
    );
  let a04: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a03,
      crate::lib::intvector::intrinsics::vec256_mul64(r51, f14)
    );
  let a14: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a13,
      crate::lib::intvector::intrinsics::vec256_mul64(r52, f14)
    );
  let a24: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a23,
      crate::lib::intvector::intrinsics::vec256_mul64(r53, f14)
    );
  let a34: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a33,
      crate::lib::intvector::intrinsics::vec256_mul64(r54, f14)
    );
  let a44: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a43,
      crate::lib::intvector::intrinsics::vec256_mul64(r0, f14)
    );
  let t0: crate::lib::intvector::intrinsics::vec256 = a04;
  let t1: crate::lib::intvector::intrinsics::vec256 = a14;
  let t2: crate::lib::intvector::intrinsics::vec256 = a24;
  let t3: crate::lib::intvector::intrinsics::vec256 = a34;
  let t4: crate::lib::intvector::intrinsics::vec256 = a44;
  let mask26: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64);
  let z0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(t0, 26u32);
  let z1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(t3, 26u32);
  let x0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t0, mask26);
  let x3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t3, mask26);
  let x1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t1, z0);
  let x4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t4, z1);
  let z01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x1, 26u32);
  let z11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x4, 26u32);
  let t: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_left64(z11, 2u32);
  let z12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(z11, t);
  let x11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x1, mask26);
  let x41: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x4, mask26);
  let x2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t2, z01);
  let x01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x0, z12);
  let z02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x2, 26u32);
  let z13: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x01, 26u32);
  let x21: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x2, mask26);
  let x02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x01, mask26);
  let x31: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x3, z02);
  let x12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x11, z13);
  let z03: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x31, 26u32);
  let x32: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x31, mask26);
  let x42: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x41, z03);
  let o0: crate::lib::intvector::intrinsics::vec256 = x02;
  let o1: crate::lib::intvector::intrinsics::vec256 = x12;
  let o2: crate::lib::intvector::intrinsics::vec256 = x21;
  let o3: crate::lib::intvector::intrinsics::vec256 = x32;
  let o4: crate::lib::intvector::intrinsics::vec256 = x42;
  rn_5.0[0u32 as usize] = o0;
  rn_5.0[1u32 as usize] = o1;
  rn_5.0[2u32 as usize] = o2;
  rn_5.0[3u32 as usize] = o3;
  rn_5.0[4u32 as usize] = o4;
  let f20: crate::lib::intvector::intrinsics::vec256 = rn_5.0[0u32 as usize];
  let f21: crate::lib::intvector::intrinsics::vec256 = rn_5.0[1u32 as usize];
  let f22: crate::lib::intvector::intrinsics::vec256 = rn_5.0[2u32 as usize];
  let f23: crate::lib::intvector::intrinsics::vec256 = rn_5.0[3u32 as usize];
  let f24: crate::lib::intvector::intrinsics::vec256 = rn_5.0[4u32 as usize];
  rn_5.1[0u32 as usize] = crate::lib::intvector::intrinsics::vec256_smul64(f20, 5u64);
  rn_5.1[1u32 as usize] = crate::lib::intvector::intrinsics::vec256_smul64(f21, 5u64);
  rn_5.1[2u32 as usize] = crate::lib::intvector::intrinsics::vec256_smul64(f22, 5u64);
  rn_5.1[3u32 as usize] = crate::lib::intvector::intrinsics::vec256_smul64(f23, 5u64);
  rn_5.1[4u32 as usize] = crate::lib::intvector::intrinsics::vec256_smul64(f24, 5u64)
}

pub fn poly1305_update1(ctx: &mut [crate::lib::intvector::intrinsics::vec256], text: &mut [u8]) ->
  ()
{
  let
  pre:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    ctx.split_at_mut(5usize);
  let
  acc:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    pre.0.split_at_mut(0usize);
  let mut e: [crate::lib::intvector::intrinsics::vec256; 5] =
    [crate::lib::intvector::intrinsics::vec256_zero; 5u32 as usize];
  let u: u64 = crate::lowstar::endianness::load64_le(&mut text[0u32 as usize..]);
  let lo: u64 = u;
  let u: u64 = crate::lowstar::endianness::load64_le(&mut text[8u32 as usize..]);
  let hi: u64 = u;
  let f0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64(lo);
  let f1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64(hi);
  let f01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      f0,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let f11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      crate::lib::intvector::intrinsics::vec256_shift_right64(f0, 26u32),
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let f2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_or(
      crate::lib::intvector::intrinsics::vec256_shift_right64(f0, 52u32),
      crate::lib::intvector::intrinsics::vec256_shift_left64(
        crate::lib::intvector::intrinsics::vec256_and(
          f1,
          crate::lib::intvector::intrinsics::vec256_load64(0x3fffu64)
        ),
        12u32
      )
    );
  let f3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      crate::lib::intvector::intrinsics::vec256_shift_right64(f1, 14u32),
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let f4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(f1, 40u32);
  let f01: crate::lib::intvector::intrinsics::vec256 = f01;
  let f11: crate::lib::intvector::intrinsics::vec256 = f11;
  let f2: crate::lib::intvector::intrinsics::vec256 = f2;
  let f3: crate::lib::intvector::intrinsics::vec256 = f3;
  let f4: crate::lib::intvector::intrinsics::vec256 = f4;
  (&mut e)[0u32 as usize] = f01;
  (&mut e)[1u32 as usize] = f11;
  (&mut e)[2u32 as usize] = f2;
  (&mut e)[3u32 as usize] = f3;
  (&mut e)[4u32 as usize] = f4;
  let b: u64 = 0x1000000u64;
  let mask: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64(b);
  let f4: crate::lib::intvector::intrinsics::vec256 = (&mut e)[4u32 as usize];
  (&mut e)[4u32 as usize] = crate::lib::intvector::intrinsics::vec256_or(f4, mask);
  let
  r:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    pre.1.split_at_mut(0usize);
  let
  r5:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    r.1.split_at_mut(5usize);
  let r0: crate::lib::intvector::intrinsics::vec256 = r5.0[0u32 as usize];
  let r1: crate::lib::intvector::intrinsics::vec256 = r5.0[1u32 as usize];
  let r2: crate::lib::intvector::intrinsics::vec256 = r5.0[2u32 as usize];
  let r3: crate::lib::intvector::intrinsics::vec256 = r5.0[3u32 as usize];
  let r4: crate::lib::intvector::intrinsics::vec256 = r5.0[4u32 as usize];
  let r51: crate::lib::intvector::intrinsics::vec256 = r5.1[1u32 as usize];
  let r52: crate::lib::intvector::intrinsics::vec256 = r5.1[2u32 as usize];
  let r53: crate::lib::intvector::intrinsics::vec256 = r5.1[3u32 as usize];
  let r54: crate::lib::intvector::intrinsics::vec256 = r5.1[4u32 as usize];
  let f10: crate::lib::intvector::intrinsics::vec256 = (&mut e)[0u32 as usize];
  let f11: crate::lib::intvector::intrinsics::vec256 = (&mut e)[1u32 as usize];
  let f12: crate::lib::intvector::intrinsics::vec256 = (&mut e)[2u32 as usize];
  let f13: crate::lib::intvector::intrinsics::vec256 = (&mut e)[3u32 as usize];
  let f14: crate::lib::intvector::intrinsics::vec256 = (&mut e)[4u32 as usize];
  let a0: crate::lib::intvector::intrinsics::vec256 = acc.1[0u32 as usize];
  let a1: crate::lib::intvector::intrinsics::vec256 = acc.1[1u32 as usize];
  let a2: crate::lib::intvector::intrinsics::vec256 = acc.1[2u32 as usize];
  let a3: crate::lib::intvector::intrinsics::vec256 = acc.1[3u32 as usize];
  let a4: crate::lib::intvector::intrinsics::vec256 = acc.1[4u32 as usize];
  let a01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(a0, f10);
  let a11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(a1, f11);
  let a21: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(a2, f12);
  let a31: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(a3, f13);
  let a41: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(a4, f14);
  let a02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r0, a01);
  let a12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r1, a01);
  let a22: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r2, a01);
  let a32: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r3, a01);
  let a42: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_mul64(r4, a01);
  let a03: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a02,
      crate::lib::intvector::intrinsics::vec256_mul64(r54, a11)
    );
  let a13: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a12,
      crate::lib::intvector::intrinsics::vec256_mul64(r0, a11)
    );
  let a23: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a22,
      crate::lib::intvector::intrinsics::vec256_mul64(r1, a11)
    );
  let a33: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a32,
      crate::lib::intvector::intrinsics::vec256_mul64(r2, a11)
    );
  let a43: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a42,
      crate::lib::intvector::intrinsics::vec256_mul64(r3, a11)
    );
  let a04: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a03,
      crate::lib::intvector::intrinsics::vec256_mul64(r53, a21)
    );
  let a14: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a13,
      crate::lib::intvector::intrinsics::vec256_mul64(r54, a21)
    );
  let a24: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a23,
      crate::lib::intvector::intrinsics::vec256_mul64(r0, a21)
    );
  let a34: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a33,
      crate::lib::intvector::intrinsics::vec256_mul64(r1, a21)
    );
  let a44: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a43,
      crate::lib::intvector::intrinsics::vec256_mul64(r2, a21)
    );
  let a05: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a04,
      crate::lib::intvector::intrinsics::vec256_mul64(r52, a31)
    );
  let a15: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a14,
      crate::lib::intvector::intrinsics::vec256_mul64(r53, a31)
    );
  let a25: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a24,
      crate::lib::intvector::intrinsics::vec256_mul64(r54, a31)
    );
  let a35: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a34,
      crate::lib::intvector::intrinsics::vec256_mul64(r0, a31)
    );
  let a45: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a44,
      crate::lib::intvector::intrinsics::vec256_mul64(r1, a31)
    );
  let a06: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a05,
      crate::lib::intvector::intrinsics::vec256_mul64(r51, a41)
    );
  let a16: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a15,
      crate::lib::intvector::intrinsics::vec256_mul64(r52, a41)
    );
  let a26: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a25,
      crate::lib::intvector::intrinsics::vec256_mul64(r53, a41)
    );
  let a36: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a35,
      crate::lib::intvector::intrinsics::vec256_mul64(r54, a41)
    );
  let a46: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      a45,
      crate::lib::intvector::intrinsics::vec256_mul64(r0, a41)
    );
  let t0: crate::lib::intvector::intrinsics::vec256 = a06;
  let t1: crate::lib::intvector::intrinsics::vec256 = a16;
  let t2: crate::lib::intvector::intrinsics::vec256 = a26;
  let t3: crate::lib::intvector::intrinsics::vec256 = a36;
  let t4: crate::lib::intvector::intrinsics::vec256 = a46;
  let mask26: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64);
  let z0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(t0, 26u32);
  let z1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(t3, 26u32);
  let x0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t0, mask26);
  let x3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(t3, mask26);
  let x1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t1, z0);
  let x4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t4, z1);
  let z01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x1, 26u32);
  let z11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x4, 26u32);
  let t: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_left64(z11, 2u32);
  let z12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(z11, t);
  let x11: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x1, mask26);
  let x41: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x4, mask26);
  let x2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(t2, z01);
  let x01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x0, z12);
  let z02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x2, 26u32);
  let z13: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x01, 26u32);
  let x21: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x2, mask26);
  let x02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x01, mask26);
  let x31: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x3, z02);
  let x12: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x11, z13);
  let z03: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(x31, 26u32);
  let x32: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(x31, mask26);
  let x42: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(x41, z03);
  let o0: crate::lib::intvector::intrinsics::vec256 = x02;
  let o1: crate::lib::intvector::intrinsics::vec256 = x12;
  let o2: crate::lib::intvector::intrinsics::vec256 = x21;
  let o3: crate::lib::intvector::intrinsics::vec256 = x32;
  let o4: crate::lib::intvector::intrinsics::vec256 = x42;
  acc.1[0u32 as usize] = o0;
  acc.1[1u32 as usize] = o1;
  acc.1[2u32 as usize] = o2;
  acc.1[3u32 as usize] = o3;
  acc.1[4u32 as usize] = o4
}

pub fn poly1305_update(
  ctx: &mut [crate::lib::intvector::intrinsics::vec256],
  len: u32,
  text: &mut [u8]
) ->
  ()
{
  let
  pre:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    ctx.split_at_mut(5usize);
  let
  acc:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    pre.0.split_at_mut(0usize);
  let sz_block: u32 = 64u32;
  let len0: u32 = len.wrapping_div(sz_block).wrapping_mul(sz_block);
  let t0: (&mut [u8], &mut [u8]) = text.split_at_mut(0usize);
  if len0 > 0u32
  {
    let bs: u32 = 64u32;
    let text0: (&mut [u8], &mut [u8]) = t0.1.split_at_mut(0usize);
    load_acc4(acc.1, text0.1);
    let len1: u32 = len0.wrapping_sub(bs);
    let text1: (&mut [u8], &mut [u8]) = text0.1.split_at_mut((bs as usize).wrapping_add(0usize));
    let nb: u32 = len1.wrapping_div(bs);
    for i in 0u32..i
    {
      let block: (&mut [u8], &mut [u8]) =
        text1.1.split_at_mut((i.wrapping_mul(bs) as usize).wrapping_add(0usize));
      let mut e: [crate::lib::intvector::intrinsics::vec256; 5] =
        [crate::lib::intvector::intrinsics::vec256_zero; 5u32 as usize];
      let lo: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_load64_le(&mut block.1[0u32 as usize..]);
      let hi: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_load64_le(&mut block.1[32u32 as usize..]);
      let mask26: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64);
      let m0: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_interleave_low128(lo, hi);
      let m1: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_interleave_high128(lo, hi);
      let m2: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_shift_right(m0, 48u32);
      let m3: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_shift_right(m1, 48u32);
      let m4: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_interleave_high64(m0, m1);
      let t01: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_interleave_low64(m0, m1);
      let t3: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_interleave_low64(m2, m3);
      let t2: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_shift_right64(t3, 4u32);
      let o2: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_and(t2, mask26);
      let t1: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_shift_right64(t01, 26u32);
      let o1: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_and(t1, mask26);
      let o5: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_and(t01, mask26);
      let t31: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_shift_right64(t3, 30u32);
      let o3: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_and(t31, mask26);
      let o4: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_shift_right64(m4, 40u32);
      let o0: crate::lib::intvector::intrinsics::vec256 = o5;
      let o1: crate::lib::intvector::intrinsics::vec256 = o1;
      let o2: crate::lib::intvector::intrinsics::vec256 = o2;
      let o3: crate::lib::intvector::intrinsics::vec256 = o3;
      let o4: crate::lib::intvector::intrinsics::vec256 = o4;
      (&mut e)[0u32 as usize] = o0;
      (&mut e)[1u32 as usize] = o1;
      (&mut e)[2u32 as usize] = o2;
      (&mut e)[3u32 as usize] = o3;
      (&mut e)[4u32 as usize] = o4;
      let b: u64 = 0x1000000u64;
      let mask: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_load64(b);
      let f4: crate::lib::intvector::intrinsics::vec256 = (&mut e)[4u32 as usize];
      (&mut e)[4u32 as usize] = crate::lib::intvector::intrinsics::vec256_or(f4, mask);
      let
      rn:
      (&mut [crate::lib::intvector::intrinsics::vec256],
      &mut [crate::lib::intvector::intrinsics::vec256])
      =
        pre.1.split_at_mut(10usize);
      let
      rn5:
      (&mut [crate::lib::intvector::intrinsics::vec256],
      &mut [crate::lib::intvector::intrinsics::vec256])
      =
        rn.1.split_at_mut(5usize);
      let r0: crate::lib::intvector::intrinsics::vec256 = rn5.0[0u32 as usize];
      let r1: crate::lib::intvector::intrinsics::vec256 = rn5.0[1u32 as usize];
      let r2: crate::lib::intvector::intrinsics::vec256 = rn5.0[2u32 as usize];
      let r3: crate::lib::intvector::intrinsics::vec256 = rn5.0[3u32 as usize];
      let r4: crate::lib::intvector::intrinsics::vec256 = rn5.0[4u32 as usize];
      let r51: crate::lib::intvector::intrinsics::vec256 = rn5.1[1u32 as usize];
      let r52: crate::lib::intvector::intrinsics::vec256 = rn5.1[2u32 as usize];
      let r53: crate::lib::intvector::intrinsics::vec256 = rn5.1[3u32 as usize];
      let r54: crate::lib::intvector::intrinsics::vec256 = rn5.1[4u32 as usize];
      let f10: crate::lib::intvector::intrinsics::vec256 = acc.1[0u32 as usize];
      let f11: crate::lib::intvector::intrinsics::vec256 = acc.1[1u32 as usize];
      let f12: crate::lib::intvector::intrinsics::vec256 = acc.1[2u32 as usize];
      let f13: crate::lib::intvector::intrinsics::vec256 = acc.1[3u32 as usize];
      let f14: crate::lib::intvector::intrinsics::vec256 = acc.1[4u32 as usize];
      let a0: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_mul64(r0, f10);
      let a1: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_mul64(r1, f10);
      let a2: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_mul64(r2, f10);
      let a3: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_mul64(r3, f10);
      let a4: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_mul64(r4, f10);
      let a01: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a0,
          crate::lib::intvector::intrinsics::vec256_mul64(r54, f11)
        );
      let a11: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a1,
          crate::lib::intvector::intrinsics::vec256_mul64(r0, f11)
        );
      let a21: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a2,
          crate::lib::intvector::intrinsics::vec256_mul64(r1, f11)
        );
      let a31: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a3,
          crate::lib::intvector::intrinsics::vec256_mul64(r2, f11)
        );
      let a41: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a4,
          crate::lib::intvector::intrinsics::vec256_mul64(r3, f11)
        );
      let a02: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a01,
          crate::lib::intvector::intrinsics::vec256_mul64(r53, f12)
        );
      let a12: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a11,
          crate::lib::intvector::intrinsics::vec256_mul64(r54, f12)
        );
      let a22: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a21,
          crate::lib::intvector::intrinsics::vec256_mul64(r0, f12)
        );
      let a32: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a31,
          crate::lib::intvector::intrinsics::vec256_mul64(r1, f12)
        );
      let a42: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a41,
          crate::lib::intvector::intrinsics::vec256_mul64(r2, f12)
        );
      let a03: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a02,
          crate::lib::intvector::intrinsics::vec256_mul64(r52, f13)
        );
      let a13: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a12,
          crate::lib::intvector::intrinsics::vec256_mul64(r53, f13)
        );
      let a23: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a22,
          crate::lib::intvector::intrinsics::vec256_mul64(r54, f13)
        );
      let a33: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a32,
          crate::lib::intvector::intrinsics::vec256_mul64(r0, f13)
        );
      let a43: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a42,
          crate::lib::intvector::intrinsics::vec256_mul64(r1, f13)
        );
      let a04: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a03,
          crate::lib::intvector::intrinsics::vec256_mul64(r51, f14)
        );
      let a14: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a13,
          crate::lib::intvector::intrinsics::vec256_mul64(r52, f14)
        );
      let a24: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a23,
          crate::lib::intvector::intrinsics::vec256_mul64(r53, f14)
        );
      let a34: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a33,
          crate::lib::intvector::intrinsics::vec256_mul64(r54, f14)
        );
      let a44: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(
          a43,
          crate::lib::intvector::intrinsics::vec256_mul64(r0, f14)
        );
      let t01: crate::lib::intvector::intrinsics::vec256 = a04;
      let t1: crate::lib::intvector::intrinsics::vec256 = a14;
      let t2: crate::lib::intvector::intrinsics::vec256 = a24;
      let t3: crate::lib::intvector::intrinsics::vec256 = a34;
      let t4: crate::lib::intvector::intrinsics::vec256 = a44;
      let mask26: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64);
      let z0: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_shift_right64(t01, 26u32);
      let z1: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_shift_right64(t3, 26u32);
      let x0: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_and(t01, mask26);
      let x3: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_and(t3, mask26);
      let x1: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(t1, z0);
      let x4: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(t4, z1);
      let z01: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_shift_right64(x1, 26u32);
      let z11: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_shift_right64(x4, 26u32);
      let t: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_shift_left64(z11, 2u32);
      let z12: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(z11, t);
      let x11: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_and(x1, mask26);
      let x41: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_and(x4, mask26);
      let x2: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(t2, z01);
      let x01: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(x0, z12);
      let z02: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_shift_right64(x2, 26u32);
      let z13: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_shift_right64(x01, 26u32);
      let x21: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_and(x2, mask26);
      let x02: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_and(x01, mask26);
      let x31: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(x3, z02);
      let x12: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(x11, z13);
      let z03: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_shift_right64(x31, 26u32);
      let x32: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_and(x31, mask26);
      let x42: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(x41, z03);
      let o0: crate::lib::intvector::intrinsics::vec256 = x02;
      let o1: crate::lib::intvector::intrinsics::vec256 = x12;
      let o2: crate::lib::intvector::intrinsics::vec256 = x21;
      let o3: crate::lib::intvector::intrinsics::vec256 = x32;
      let o4: crate::lib::intvector::intrinsics::vec256 = x42;
      acc.1[0u32 as usize] = o0;
      acc.1[1u32 as usize] = o1;
      acc.1[2u32 as usize] = o2;
      acc.1[3u32 as usize] = o3;
      acc.1[4u32 as usize] = o4;
      let f10: crate::lib::intvector::intrinsics::vec256 = acc.1[0u32 as usize];
      let f11: crate::lib::intvector::intrinsics::vec256 = acc.1[1u32 as usize];
      let f12: crate::lib::intvector::intrinsics::vec256 = acc.1[2u32 as usize];
      let f13: crate::lib::intvector::intrinsics::vec256 = acc.1[3u32 as usize];
      let f14: crate::lib::intvector::intrinsics::vec256 = acc.1[4u32 as usize];
      let f20: crate::lib::intvector::intrinsics::vec256 = (&mut e)[0u32 as usize];
      let f21: crate::lib::intvector::intrinsics::vec256 = (&mut e)[1u32 as usize];
      let f22: crate::lib::intvector::intrinsics::vec256 = (&mut e)[2u32 as usize];
      let f23: crate::lib::intvector::intrinsics::vec256 = (&mut e)[3u32 as usize];
      let f24: crate::lib::intvector::intrinsics::vec256 = (&mut e)[4u32 as usize];
      let o0: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(f10, f20);
      let o1: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(f11, f21);
      let o2: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(f12, f22);
      let o3: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(f13, f23);
      let o4: crate::lib::intvector::intrinsics::vec256 =
        crate::lib::intvector::intrinsics::vec256_add64(f14, f24);
      acc.1[0u32 as usize] = o0;
      acc.1[1u32 as usize] = o1;
      acc.1[2u32 as usize] = o2;
      acc.1[3u32 as usize] = o3;
      acc.1[4u32 as usize] = o4
    };
    fmul_r4_normalize(acc.1, pre.1)
  };
  let len1: u32 = len.wrapping_sub(len0);
  let t1: (&mut [u8], &mut [u8]) = t0.1.split_at_mut((len0 as usize).wrapping_add(0usize));
  let nb: u32 = len1.wrapping_div(16u32);
  let rem: u32 = len1.wrapping_rem(16u32);
  for i in 0u32..rem
  {
    let block: (&mut [u8], &mut [u8]) =
      t1.1.split_at_mut((i.wrapping_mul(16u32) as usize).wrapping_add(0usize));
    let mut e: [crate::lib::intvector::intrinsics::vec256; 5] =
      [crate::lib::intvector::intrinsics::vec256_zero; 5u32 as usize];
    let u: u64 = crate::lowstar::endianness::load64_le(&mut block.1[0u32 as usize..]);
    let lo: u64 = u;
    let u: u64 = crate::lowstar::endianness::load64_le(&mut block.1[8u32 as usize..]);
    let hi: u64 = u;
    let f0: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_load64(lo);
    let f1: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_load64(hi);
    let f01: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(
        f0,
        crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
      );
    let f11: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(
        crate::lib::intvector::intrinsics::vec256_shift_right64(f0, 26u32),
        crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
      );
    let f2: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_or(
        crate::lib::intvector::intrinsics::vec256_shift_right64(f0, 52u32),
        crate::lib::intvector::intrinsics::vec256_shift_left64(
          crate::lib::intvector::intrinsics::vec256_and(
            f1,
            crate::lib::intvector::intrinsics::vec256_load64(0x3fffu64)
          ),
          12u32
        )
      );
    let f3: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(
        crate::lib::intvector::intrinsics::vec256_shift_right64(f1, 14u32),
        crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
      );
    let f4: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(f1, 40u32);
    let f01: crate::lib::intvector::intrinsics::vec256 = f01;
    let f11: crate::lib::intvector::intrinsics::vec256 = f11;
    let f2: crate::lib::intvector::intrinsics::vec256 = f2;
    let f3: crate::lib::intvector::intrinsics::vec256 = f3;
    let f4: crate::lib::intvector::intrinsics::vec256 = f4;
    (&mut e)[0u32 as usize] = f01;
    (&mut e)[1u32 as usize] = f11;
    (&mut e)[2u32 as usize] = f2;
    (&mut e)[3u32 as usize] = f3;
    (&mut e)[4u32 as usize] = f4;
    let b: u64 = 0x1000000u64;
    let mask: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_load64(b);
    let f4: crate::lib::intvector::intrinsics::vec256 = (&mut e)[4u32 as usize];
    (&mut e)[4u32 as usize] = crate::lib::intvector::intrinsics::vec256_or(f4, mask);
    let
    r:
    (&mut [crate::lib::intvector::intrinsics::vec256],
    &mut [crate::lib::intvector::intrinsics::vec256])
    =
      pre.1.split_at_mut(0usize);
    let
    r5:
    (&mut [crate::lib::intvector::intrinsics::vec256],
    &mut [crate::lib::intvector::intrinsics::vec256])
    =
      r.1.split_at_mut(5usize);
    let r0: crate::lib::intvector::intrinsics::vec256 = r5.0[0u32 as usize];
    let r1: crate::lib::intvector::intrinsics::vec256 = r5.0[1u32 as usize];
    let r2: crate::lib::intvector::intrinsics::vec256 = r5.0[2u32 as usize];
    let r3: crate::lib::intvector::intrinsics::vec256 = r5.0[3u32 as usize];
    let r4: crate::lib::intvector::intrinsics::vec256 = r5.0[4u32 as usize];
    let r51: crate::lib::intvector::intrinsics::vec256 = r5.1[1u32 as usize];
    let r52: crate::lib::intvector::intrinsics::vec256 = r5.1[2u32 as usize];
    let r53: crate::lib::intvector::intrinsics::vec256 = r5.1[3u32 as usize];
    let r54: crate::lib::intvector::intrinsics::vec256 = r5.1[4u32 as usize];
    let f10: crate::lib::intvector::intrinsics::vec256 = (&mut e)[0u32 as usize];
    let f11: crate::lib::intvector::intrinsics::vec256 = (&mut e)[1u32 as usize];
    let f12: crate::lib::intvector::intrinsics::vec256 = (&mut e)[2u32 as usize];
    let f13: crate::lib::intvector::intrinsics::vec256 = (&mut e)[3u32 as usize];
    let f14: crate::lib::intvector::intrinsics::vec256 = (&mut e)[4u32 as usize];
    let a0: crate::lib::intvector::intrinsics::vec256 = acc.1[0u32 as usize];
    let a1: crate::lib::intvector::intrinsics::vec256 = acc.1[1u32 as usize];
    let a2: crate::lib::intvector::intrinsics::vec256 = acc.1[2u32 as usize];
    let a3: crate::lib::intvector::intrinsics::vec256 = acc.1[3u32 as usize];
    let a4: crate::lib::intvector::intrinsics::vec256 = acc.1[4u32 as usize];
    let a01: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(a0, f10);
    let a11: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(a1, f11);
    let a21: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(a2, f12);
    let a31: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(a3, f13);
    let a41: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(a4, f14);
    let a02: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_mul64(r0, a01);
    let a12: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_mul64(r1, a01);
    let a22: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_mul64(r2, a01);
    let a32: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_mul64(r3, a01);
    let a42: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_mul64(r4, a01);
    let a03: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a02,
        crate::lib::intvector::intrinsics::vec256_mul64(r54, a11)
      );
    let a13: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a12,
        crate::lib::intvector::intrinsics::vec256_mul64(r0, a11)
      );
    let a23: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a22,
        crate::lib::intvector::intrinsics::vec256_mul64(r1, a11)
      );
    let a33: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a32,
        crate::lib::intvector::intrinsics::vec256_mul64(r2, a11)
      );
    let a43: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a42,
        crate::lib::intvector::intrinsics::vec256_mul64(r3, a11)
      );
    let a04: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a03,
        crate::lib::intvector::intrinsics::vec256_mul64(r53, a21)
      );
    let a14: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a13,
        crate::lib::intvector::intrinsics::vec256_mul64(r54, a21)
      );
    let a24: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a23,
        crate::lib::intvector::intrinsics::vec256_mul64(r0, a21)
      );
    let a34: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a33,
        crate::lib::intvector::intrinsics::vec256_mul64(r1, a21)
      );
    let a44: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a43,
        crate::lib::intvector::intrinsics::vec256_mul64(r2, a21)
      );
    let a05: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a04,
        crate::lib::intvector::intrinsics::vec256_mul64(r52, a31)
      );
    let a15: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a14,
        crate::lib::intvector::intrinsics::vec256_mul64(r53, a31)
      );
    let a25: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a24,
        crate::lib::intvector::intrinsics::vec256_mul64(r54, a31)
      );
    let a35: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a34,
        crate::lib::intvector::intrinsics::vec256_mul64(r0, a31)
      );
    let a45: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a44,
        crate::lib::intvector::intrinsics::vec256_mul64(r1, a31)
      );
    let a06: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a05,
        crate::lib::intvector::intrinsics::vec256_mul64(r51, a41)
      );
    let a16: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a15,
        crate::lib::intvector::intrinsics::vec256_mul64(r52, a41)
      );
    let a26: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a25,
        crate::lib::intvector::intrinsics::vec256_mul64(r53, a41)
      );
    let a36: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a35,
        crate::lib::intvector::intrinsics::vec256_mul64(r54, a41)
      );
    let a46: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a45,
        crate::lib::intvector::intrinsics::vec256_mul64(r0, a41)
      );
    let t01: crate::lib::intvector::intrinsics::vec256 = a06;
    let t11: crate::lib::intvector::intrinsics::vec256 = a16;
    let t2: crate::lib::intvector::intrinsics::vec256 = a26;
    let t3: crate::lib::intvector::intrinsics::vec256 = a36;
    let t4: crate::lib::intvector::intrinsics::vec256 = a46;
    let mask26: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64);
    let z0: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(t01, 26u32);
    let z1: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(t3, 26u32);
    let x0: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(t01, mask26);
    let x3: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(t3, mask26);
    let x1: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(t11, z0);
    let x4: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(t4, z1);
    let z01: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(x1, 26u32);
    let z11: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(x4, 26u32);
    let t: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_left64(z11, 2u32);
    let z12: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(z11, t);
    let x11: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(x1, mask26);
    let x41: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(x4, mask26);
    let x2: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(t2, z01);
    let x01: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(x0, z12);
    let z02: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(x2, 26u32);
    let z13: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(x01, 26u32);
    let x21: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(x2, mask26);
    let x02: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(x01, mask26);
    let x31: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(x3, z02);
    let x12: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(x11, z13);
    let z03: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(x31, 26u32);
    let x32: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(x31, mask26);
    let x42: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(x41, z03);
    let o0: crate::lib::intvector::intrinsics::vec256 = x02;
    let o1: crate::lib::intvector::intrinsics::vec256 = x12;
    let o2: crate::lib::intvector::intrinsics::vec256 = x21;
    let o3: crate::lib::intvector::intrinsics::vec256 = x32;
    let o4: crate::lib::intvector::intrinsics::vec256 = x42;
    acc.1[0u32 as usize] = o0;
    acc.1[1u32 as usize] = o1;
    acc.1[2u32 as usize] = o2;
    acc.1[3u32 as usize] = o3;
    acc.1[4u32 as usize] = o4
  };
  if rem > 0u32
  {
    let last: (&mut [u8], &mut [u8]) =
      t1.1.split_at_mut((nb.wrapping_mul(16u32) as usize).wrapping_add(0usize));
    let mut e: [crate::lib::intvector::intrinsics::vec256; 5] =
      [crate::lib::intvector::intrinsics::vec256_zero; 5u32 as usize];
    let mut tmp: [u8; 16] = [0u8; 16u32 as usize];
    ((&mut tmp)[0u32 as usize..0u32 as usize + rem as usize]).copy_from_slice(
      &last.1[0u32 as usize..0u32 as usize + rem as usize]
    );
    let u: u64 = crate::lowstar::endianness::load64_le(&mut (&mut tmp)[0u32 as usize..]);
    let lo: u64 = u;
    let u: u64 = crate::lowstar::endianness::load64_le(&mut (&mut tmp)[8u32 as usize..]);
    let hi: u64 = u;
    let f0: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_load64(lo);
    let f1: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_load64(hi);
    let f01: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(
        f0,
        crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
      );
    let f11: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(
        crate::lib::intvector::intrinsics::vec256_shift_right64(f0, 26u32),
        crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
      );
    let f2: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_or(
        crate::lib::intvector::intrinsics::vec256_shift_right64(f0, 52u32),
        crate::lib::intvector::intrinsics::vec256_shift_left64(
          crate::lib::intvector::intrinsics::vec256_and(
            f1,
            crate::lib::intvector::intrinsics::vec256_load64(0x3fffu64)
          ),
          12u32
        )
      );
    let f3: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(
        crate::lib::intvector::intrinsics::vec256_shift_right64(f1, 14u32),
        crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
      );
    let f4: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(f1, 40u32);
    let f01: crate::lib::intvector::intrinsics::vec256 = f01;
    let f11: crate::lib::intvector::intrinsics::vec256 = f11;
    let f2: crate::lib::intvector::intrinsics::vec256 = f2;
    let f3: crate::lib::intvector::intrinsics::vec256 = f3;
    let f4: crate::lib::intvector::intrinsics::vec256 = f4;
    (&mut e)[0u32 as usize] = f01;
    (&mut e)[1u32 as usize] = f11;
    (&mut e)[2u32 as usize] = f2;
    (&mut e)[3u32 as usize] = f3;
    (&mut e)[4u32 as usize] = f4;
    let b: u64 = 1u64.wrapping_shl(rem.wrapping_mul(8u32).wrapping_rem(26u32));
    let mask: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_load64(b);
    let fi: crate::lib::intvector::intrinsics::vec256 =
      (&mut e)[rem.wrapping_mul(8u32).wrapping_div(26u32) as usize];
    (&mut e)[rem.wrapping_mul(8u32).wrapping_div(26u32) as usize] =
      crate::lib::intvector::intrinsics::vec256_or(fi, mask);
    let
    r:
    (&mut [crate::lib::intvector::intrinsics::vec256],
    &mut [crate::lib::intvector::intrinsics::vec256])
    =
      pre.1.split_at_mut(0usize);
    let
    r5:
    (&mut [crate::lib::intvector::intrinsics::vec256],
    &mut [crate::lib::intvector::intrinsics::vec256])
    =
      r.1.split_at_mut(5usize);
    let r0: crate::lib::intvector::intrinsics::vec256 = r5.0[0u32 as usize];
    let r1: crate::lib::intvector::intrinsics::vec256 = r5.0[1u32 as usize];
    let r2: crate::lib::intvector::intrinsics::vec256 = r5.0[2u32 as usize];
    let r3: crate::lib::intvector::intrinsics::vec256 = r5.0[3u32 as usize];
    let r4: crate::lib::intvector::intrinsics::vec256 = r5.0[4u32 as usize];
    let r51: crate::lib::intvector::intrinsics::vec256 = r5.1[1u32 as usize];
    let r52: crate::lib::intvector::intrinsics::vec256 = r5.1[2u32 as usize];
    let r53: crate::lib::intvector::intrinsics::vec256 = r5.1[3u32 as usize];
    let r54: crate::lib::intvector::intrinsics::vec256 = r5.1[4u32 as usize];
    let f10: crate::lib::intvector::intrinsics::vec256 = (&mut e)[0u32 as usize];
    let f11: crate::lib::intvector::intrinsics::vec256 = (&mut e)[1u32 as usize];
    let f12: crate::lib::intvector::intrinsics::vec256 = (&mut e)[2u32 as usize];
    let f13: crate::lib::intvector::intrinsics::vec256 = (&mut e)[3u32 as usize];
    let f14: crate::lib::intvector::intrinsics::vec256 = (&mut e)[4u32 as usize];
    let a0: crate::lib::intvector::intrinsics::vec256 = acc.1[0u32 as usize];
    let a1: crate::lib::intvector::intrinsics::vec256 = acc.1[1u32 as usize];
    let a2: crate::lib::intvector::intrinsics::vec256 = acc.1[2u32 as usize];
    let a3: crate::lib::intvector::intrinsics::vec256 = acc.1[3u32 as usize];
    let a4: crate::lib::intvector::intrinsics::vec256 = acc.1[4u32 as usize];
    let a01: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(a0, f10);
    let a11: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(a1, f11);
    let a21: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(a2, f12);
    let a31: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(a3, f13);
    let a41: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(a4, f14);
    let a02: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_mul64(r0, a01);
    let a12: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_mul64(r1, a01);
    let a22: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_mul64(r2, a01);
    let a32: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_mul64(r3, a01);
    let a42: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_mul64(r4, a01);
    let a03: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a02,
        crate::lib::intvector::intrinsics::vec256_mul64(r54, a11)
      );
    let a13: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a12,
        crate::lib::intvector::intrinsics::vec256_mul64(r0, a11)
      );
    let a23: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a22,
        crate::lib::intvector::intrinsics::vec256_mul64(r1, a11)
      );
    let a33: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a32,
        crate::lib::intvector::intrinsics::vec256_mul64(r2, a11)
      );
    let a43: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a42,
        crate::lib::intvector::intrinsics::vec256_mul64(r3, a11)
      );
    let a04: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a03,
        crate::lib::intvector::intrinsics::vec256_mul64(r53, a21)
      );
    let a14: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a13,
        crate::lib::intvector::intrinsics::vec256_mul64(r54, a21)
      );
    let a24: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a23,
        crate::lib::intvector::intrinsics::vec256_mul64(r0, a21)
      );
    let a34: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a33,
        crate::lib::intvector::intrinsics::vec256_mul64(r1, a21)
      );
    let a44: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a43,
        crate::lib::intvector::intrinsics::vec256_mul64(r2, a21)
      );
    let a05: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a04,
        crate::lib::intvector::intrinsics::vec256_mul64(r52, a31)
      );
    let a15: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a14,
        crate::lib::intvector::intrinsics::vec256_mul64(r53, a31)
      );
    let a25: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a24,
        crate::lib::intvector::intrinsics::vec256_mul64(r54, a31)
      );
    let a35: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a34,
        crate::lib::intvector::intrinsics::vec256_mul64(r0, a31)
      );
    let a45: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a44,
        crate::lib::intvector::intrinsics::vec256_mul64(r1, a31)
      );
    let a06: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a05,
        crate::lib::intvector::intrinsics::vec256_mul64(r51, a41)
      );
    let a16: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a15,
        crate::lib::intvector::intrinsics::vec256_mul64(r52, a41)
      );
    let a26: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a25,
        crate::lib::intvector::intrinsics::vec256_mul64(r53, a41)
      );
    let a36: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a35,
        crate::lib::intvector::intrinsics::vec256_mul64(r54, a41)
      );
    let a46: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(
        a45,
        crate::lib::intvector::intrinsics::vec256_mul64(r0, a41)
      );
    let t01: crate::lib::intvector::intrinsics::vec256 = a06;
    let t11: crate::lib::intvector::intrinsics::vec256 = a16;
    let t2: crate::lib::intvector::intrinsics::vec256 = a26;
    let t3: crate::lib::intvector::intrinsics::vec256 = a36;
    let t4: crate::lib::intvector::intrinsics::vec256 = a46;
    let mask26: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64);
    let z0: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(t01, 26u32);
    let z1: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(t3, 26u32);
    let x0: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(t01, mask26);
    let x3: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(t3, mask26);
    let x1: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(t11, z0);
    let x4: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(t4, z1);
    let z01: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(x1, 26u32);
    let z11: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(x4, 26u32);
    let t: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_left64(z11, 2u32);
    let z12: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(z11, t);
    let x11: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(x1, mask26);
    let x41: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(x4, mask26);
    let x2: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(t2, z01);
    let x01: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(x0, z12);
    let z02: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(x2, 26u32);
    let z13: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(x01, 26u32);
    let x21: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(x2, mask26);
    let x02: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(x01, mask26);
    let x31: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(x3, z02);
    let x12: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(x11, z13);
    let z03: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_shift_right64(x31, 26u32);
    let x32: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_and(x31, mask26);
    let x42: crate::lib::intvector::intrinsics::vec256 =
      crate::lib::intvector::intrinsics::vec256_add64(x41, z03);
    let o0: crate::lib::intvector::intrinsics::vec256 = x02;
    let o1: crate::lib::intvector::intrinsics::vec256 = x12;
    let o2: crate::lib::intvector::intrinsics::vec256 = x21;
    let o3: crate::lib::intvector::intrinsics::vec256 = x32;
    let o4: crate::lib::intvector::intrinsics::vec256 = x42;
    acc.1[0u32 as usize] = o0;
    acc.1[1u32 as usize] = o1;
    acc.1[2u32 as usize] = o2;
    acc.1[3u32 as usize] = o3;
    acc.1[4u32 as usize] = o4
  }
}

pub fn poly1305_finish(
  tag: &mut [u8],
  key: &mut [u8],
  ctx: &mut [crate::lib::intvector::intrinsics::vec256]
) ->
  ()
{
  let
  acc:
  (&mut [crate::lib::intvector::intrinsics::vec256],
  &mut [crate::lib::intvector::intrinsics::vec256])
  =
    ctx.split_at_mut(0usize);
  let ks: (&mut [u8], &mut [u8]) = key.split_at_mut(16usize);
  let f0: crate::lib::intvector::intrinsics::vec256 = acc.1[0u32 as usize];
  let f1: crate::lib::intvector::intrinsics::vec256 = acc.1[1u32 as usize];
  let f2: crate::lib::intvector::intrinsics::vec256 = acc.1[2u32 as usize];
  let f3: crate::lib::intvector::intrinsics::vec256 = acc.1[3u32 as usize];
  let f4: crate::lib::intvector::intrinsics::vec256 = acc.1[4u32 as usize];
  let l: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      f0,
      crate::lib::intvector::intrinsics::vec256_zero
    );
  let tmp0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      l,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let c0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(l, 26u32);
  let l: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(f1, c0);
  let tmp1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      l,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let c1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(l, 26u32);
  let l: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(f2, c1);
  let tmp2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      l,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let c2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(l, 26u32);
  let l: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(f3, c2);
  let tmp3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      l,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let c3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(l, 26u32);
  let l: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(f4, c3);
  let tmp4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      l,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let c4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(l, 26u32);
  let f01: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      tmp0,
      crate::lib::intvector::intrinsics::vec256_smul64(c4, 5u64)
    );
  let f11: crate::lib::intvector::intrinsics::vec256 = tmp1;
  let f21: crate::lib::intvector::intrinsics::vec256 = tmp2;
  let f31: crate::lib::intvector::intrinsics::vec256 = tmp3;
  let f41: crate::lib::intvector::intrinsics::vec256 = tmp4;
  let l: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      f01,
      crate::lib::intvector::intrinsics::vec256_zero
    );
  let tmp0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      l,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let c0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(l, 26u32);
  let l: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(f11, c0);
  let tmp1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      l,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let c1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(l, 26u32);
  let l: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(f21, c1);
  let tmp2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      l,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let c2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(l, 26u32);
  let l: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(f31, c2);
  let tmp3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      l,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let c3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(l, 26u32);
  let l: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(f41, c3);
  let tmp4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      l,
      crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64)
    );
  let c4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_shift_right64(l, 26u32);
  let f02: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_add64(
      tmp0,
      crate::lib::intvector::intrinsics::vec256_smul64(c4, 5u64)
    );
  let f12: crate::lib::intvector::intrinsics::vec256 = tmp1;
  let f22: crate::lib::intvector::intrinsics::vec256 = tmp2;
  let f32: crate::lib::intvector::intrinsics::vec256 = tmp3;
  let f42: crate::lib::intvector::intrinsics::vec256 = tmp4;
  let mh: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64(0x3ffffffu64);
  let ml: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_load64(0x3fffffbu64);
  let mask: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_eq64(f42, mh);
  let mask1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      mask,
      crate::lib::intvector::intrinsics::vec256_eq64(f32, mh)
    );
  let mask2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      mask1,
      crate::lib::intvector::intrinsics::vec256_eq64(f22, mh)
    );
  let mask3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      mask2,
      crate::lib::intvector::intrinsics::vec256_eq64(f12, mh)
    );
  let mask4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(
      mask3,
      crate::lib::intvector::intrinsics::vec256_lognot(
        crate::lib::intvector::intrinsics::vec256_gt64(ml, f02)
      )
    );
  let ph: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(mask4, mh);
  let pl: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_and(mask4, ml);
  let o0: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_sub64(f02, pl);
  let o1: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_sub64(f12, ph);
  let o2: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_sub64(f22, ph);
  let o3: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_sub64(f32, ph);
  let o4: crate::lib::intvector::intrinsics::vec256 =
    crate::lib::intvector::intrinsics::vec256_sub64(f42, ph);
  let f01: crate::lib::intvector::intrinsics::vec256 = o0;
  let f11: crate::lib::intvector::intrinsics::vec256 = o1;
  let f21: crate::lib::intvector::intrinsics::vec256 = o2;
  let f31: crate::lib::intvector::intrinsics::vec256 = o3;
  let f41: crate::lib::intvector::intrinsics::vec256 = o4;
  acc.1[0u32 as usize] = f01;
  acc.1[1u32 as usize] = f11;
  acc.1[2u32 as usize] = f21;
  acc.1[3u32 as usize] = f31;
  acc.1[4u32 as usize] = f41;
  let f0: crate::lib::intvector::intrinsics::vec256 = acc.1[0u32 as usize];
  let f1: crate::lib::intvector::intrinsics::vec256 = acc.1[1u32 as usize];
  let f2: crate::lib::intvector::intrinsics::vec256 = acc.1[2u32 as usize];
  let f3: crate::lib::intvector::intrinsics::vec256 = acc.1[3u32 as usize];
  let f4: crate::lib::intvector::intrinsics::vec256 = acc.1[4u32 as usize];
  let f01: u64 = crate::lib::intvector::intrinsics::vec256_extract64(f0, 0u32);
  let f11: u64 = crate::lib::intvector::intrinsics::vec256_extract64(f1, 0u32);
  let f21: u64 = crate::lib::intvector::intrinsics::vec256_extract64(f2, 0u32);
  let f31: u64 = crate::lib::intvector::intrinsics::vec256_extract64(f3, 0u32);
  let f41: u64 = crate::lib::intvector::intrinsics::vec256_extract64(f4, 0u32);
  let lo: u64 = f01 | f11.wrapping_shl(26u32) | f21.wrapping_shl(52u32);
  let hi: u64 = f21.wrapping_shr(12u32) | f31.wrapping_shl(14u32) | f41.wrapping_shl(40u32);
  let f10: u64 = lo;
  let f11: u64 = hi;
  let u: u64 = crate::lowstar::endianness::load64_le(&mut ks.1[0u32 as usize..]);
  let lo: u64 = u;
  let u: u64 = crate::lowstar::endianness::load64_le(&mut ks.1[8u32 as usize..]);
  let hi: u64 = u;
  let f20: u64 = lo;
  let f21: u64 = hi;
  let r0: u64 = f10.wrapping_add(f20);
  let r1: u64 = f11.wrapping_add(f21);
  let c: u64 = (r0 ^ (r0 ^ f20 | r0.wrapping_sub(f20) ^ f20)).wrapping_shr(63u32);
  let r11: u64 = r1.wrapping_add(c);
  let f30: u64 = r0;
  let f31: u64 = r11;
  crate::lowstar::endianness::store64_le(&mut tag[0u32 as usize..], f30);
  crate::lowstar::endianness::store64_le(&mut tag[8u32 as usize..], f31)
}

pub fn poly1305_mac(tag: &mut [u8], len: u32, text: &mut [u8], key: &mut [u8]) -> ()
{
  let mut ctx: [crate::lib::intvector::intrinsics::vec256; 25] =
    [crate::lib::intvector::intrinsics::vec256_zero; 25u32 as usize];
  poly1305_init(&mut ctx, key);
  poly1305_update(&mut ctx, len, text);
  poly1305_finish(tag, key, &mut ctx)
}