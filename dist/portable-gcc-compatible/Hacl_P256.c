/* MIT License
 *
 * Copyright (c) 2016-2022 INRIA, CMU and Microsoft Corporation
 * Copyright (c) 2022-2023 HACL* Contributors
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */


#include "internal/Hacl_P256.h"

#include "Hacl_Krmllib.h"
#include "Hacl_Hash_SHA2.h"
#include "internal/Hacl_P256_PrecompTable.h"
#include "internal/Hacl_Krmllib.h"
#include "internal/Hacl_Bignum_Base.h"
#include "lib_intrinsics.h"

/* SNIPPET_START: bn_is_zero_mask4 */

static inline uint64_t bn_is_zero_mask4(uint64_t *f)
{
  uint64_t bn_zero[4U] = { 0U };
  uint64_t mask = 0xFFFFFFFFFFFFFFFFULL;
  KRML_MAYBE_FOR4(i,
    0U,
    4U,
    1U,
    uint64_t uu____0 = FStar_UInt64_eq_mask(f[i], bn_zero[i]);
    mask = uu____0 & mask;);
  uint64_t mask1 = mask;
  uint64_t res = mask1;
  return res;
}

/* SNIPPET_END: bn_is_zero_mask4 */

/* SNIPPET_START: bn_is_zero_vartime4 */

static inline bool bn_is_zero_vartime4(uint64_t *f)
{
  uint64_t m = bn_is_zero_mask4(f);
  return m == 0xFFFFFFFFFFFFFFFFULL;
}

/* SNIPPET_END: bn_is_zero_vartime4 */

/* SNIPPET_START: bn_is_eq_mask4 */

static inline uint64_t bn_is_eq_mask4(uint64_t *a, uint64_t *b)
{
  uint64_t mask = 0xFFFFFFFFFFFFFFFFULL;
  KRML_MAYBE_FOR4(i,
    0U,
    4U,
    1U,
    uint64_t uu____0 = FStar_UInt64_eq_mask(a[i], b[i]);
    mask = uu____0 & mask;);
  uint64_t mask1 = mask;
  return mask1;
}

/* SNIPPET_END: bn_is_eq_mask4 */

/* SNIPPET_START: bn_is_eq_vartime4 */

static inline bool bn_is_eq_vartime4(uint64_t *a, uint64_t *b)
{
  uint64_t m = bn_is_eq_mask4(a, b);
  return m == 0xFFFFFFFFFFFFFFFFULL;
}

/* SNIPPET_END: bn_is_eq_vartime4 */

/* SNIPPET_START: bn_cmovznz4 */

static inline void bn_cmovznz4(uint64_t *res, uint64_t cin, uint64_t *x, uint64_t *y)
{
  uint64_t mask = ~FStar_UInt64_eq_mask(cin, 0ULL);
  KRML_MAYBE_FOR4(i,
    0U,
    4U,
    1U,
    uint64_t *os = res;
    uint64_t uu____0 = x[i];
    uint64_t x1 = uu____0 ^ (mask & (y[i] ^ uu____0));
    os[i] = x1;);
}

/* SNIPPET_END: bn_cmovznz4 */

/* SNIPPET_START: bn_add_mod4 */

static inline void bn_add_mod4(uint64_t *res, uint64_t *n, uint64_t *x, uint64_t *y)
{
  uint64_t c0 = 0ULL;
  {
    uint64_t t1 = x[4U * 0U];
    uint64_t t20 = y[4U * 0U];
    uint64_t *res_i0 = res + 4U * 0U;
    c0 = Lib_IntTypes_Intrinsics_add_carry_u64(c0, t1, t20, res_i0);
    uint64_t t10 = x[4U * 0U + 1U];
    uint64_t t21 = y[4U * 0U + 1U];
    uint64_t *res_i1 = res + 4U * 0U + 1U;
    c0 = Lib_IntTypes_Intrinsics_add_carry_u64(c0, t10, t21, res_i1);
    uint64_t t11 = x[4U * 0U + 2U];
    uint64_t t22 = y[4U * 0U + 2U];
    uint64_t *res_i2 = res + 4U * 0U + 2U;
    c0 = Lib_IntTypes_Intrinsics_add_carry_u64(c0, t11, t22, res_i2);
    uint64_t t12 = x[4U * 0U + 3U];
    uint64_t t2 = y[4U * 0U + 3U];
    uint64_t *res_i = res + 4U * 0U + 3U;
    c0 = Lib_IntTypes_Intrinsics_add_carry_u64(c0, t12, t2, res_i);
  }
  uint64_t c00 = c0;
  uint64_t tmp[4U] = { 0U };
  uint64_t c = 0ULL;
  {
    uint64_t t1 = res[4U * 0U];
    uint64_t t20 = n[4U * 0U];
    uint64_t *res_i0 = tmp + 4U * 0U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t1, t20, res_i0);
    uint64_t t10 = res[4U * 0U + 1U];
    uint64_t t21 = n[4U * 0U + 1U];
    uint64_t *res_i1 = tmp + 4U * 0U + 1U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t10, t21, res_i1);
    uint64_t t11 = res[4U * 0U + 2U];
    uint64_t t22 = n[4U * 0U + 2U];
    uint64_t *res_i2 = tmp + 4U * 0U + 2U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t11, t22, res_i2);
    uint64_t t12 = res[4U * 0U + 3U];
    uint64_t t2 = n[4U * 0U + 3U];
    uint64_t *res_i = tmp + 4U * 0U + 3U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t12, t2, res_i);
  }
  uint64_t c1 = c;
  uint64_t c2 = c00 - c1;
  KRML_MAYBE_FOR4(i,
    0U,
    4U,
    1U,
    uint64_t *os = res;
    uint64_t x1 = (c2 & res[i]) | (~c2 & tmp[i]);
    os[i] = x1;);
}

/* SNIPPET_END: bn_add_mod4 */

/* SNIPPET_START: bn_sub4 */

static inline uint64_t bn_sub4(uint64_t *res, uint64_t *x, uint64_t *y)
{
  uint64_t c = 0ULL;
  {
    uint64_t t1 = x[4U * 0U];
    uint64_t t20 = y[4U * 0U];
    uint64_t *res_i0 = res + 4U * 0U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t1, t20, res_i0);
    uint64_t t10 = x[4U * 0U + 1U];
    uint64_t t21 = y[4U * 0U + 1U];
    uint64_t *res_i1 = res + 4U * 0U + 1U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t10, t21, res_i1);
    uint64_t t11 = x[4U * 0U + 2U];
    uint64_t t22 = y[4U * 0U + 2U];
    uint64_t *res_i2 = res + 4U * 0U + 2U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t11, t22, res_i2);
    uint64_t t12 = x[4U * 0U + 3U];
    uint64_t t2 = y[4U * 0U + 3U];
    uint64_t *res_i = res + 4U * 0U + 3U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t12, t2, res_i);
  }
  uint64_t c0 = c;
  return c0;
}

/* SNIPPET_END: bn_sub4 */

/* SNIPPET_START: bn_sub_mod4 */

static inline void bn_sub_mod4(uint64_t *res, uint64_t *n, uint64_t *x, uint64_t *y)
{
  uint64_t c0 = 0ULL;
  {
    uint64_t t1 = x[4U * 0U];
    uint64_t t20 = y[4U * 0U];
    uint64_t *res_i0 = res + 4U * 0U;
    c0 = Lib_IntTypes_Intrinsics_sub_borrow_u64(c0, t1, t20, res_i0);
    uint64_t t10 = x[4U * 0U + 1U];
    uint64_t t21 = y[4U * 0U + 1U];
    uint64_t *res_i1 = res + 4U * 0U + 1U;
    c0 = Lib_IntTypes_Intrinsics_sub_borrow_u64(c0, t10, t21, res_i1);
    uint64_t t11 = x[4U * 0U + 2U];
    uint64_t t22 = y[4U * 0U + 2U];
    uint64_t *res_i2 = res + 4U * 0U + 2U;
    c0 = Lib_IntTypes_Intrinsics_sub_borrow_u64(c0, t11, t22, res_i2);
    uint64_t t12 = x[4U * 0U + 3U];
    uint64_t t2 = y[4U * 0U + 3U];
    uint64_t *res_i = res + 4U * 0U + 3U;
    c0 = Lib_IntTypes_Intrinsics_sub_borrow_u64(c0, t12, t2, res_i);
  }
  uint64_t c00 = c0;
  uint64_t tmp[4U] = { 0U };
  uint64_t c = 0ULL;
  {
    uint64_t t1 = res[4U * 0U];
    uint64_t t20 = n[4U * 0U];
    uint64_t *res_i0 = tmp + 4U * 0U;
    c = Lib_IntTypes_Intrinsics_add_carry_u64(c, t1, t20, res_i0);
    uint64_t t10 = res[4U * 0U + 1U];
    uint64_t t21 = n[4U * 0U + 1U];
    uint64_t *res_i1 = tmp + 4U * 0U + 1U;
    c = Lib_IntTypes_Intrinsics_add_carry_u64(c, t10, t21, res_i1);
    uint64_t t11 = res[4U * 0U + 2U];
    uint64_t t22 = n[4U * 0U + 2U];
    uint64_t *res_i2 = tmp + 4U * 0U + 2U;
    c = Lib_IntTypes_Intrinsics_add_carry_u64(c, t11, t22, res_i2);
    uint64_t t12 = res[4U * 0U + 3U];
    uint64_t t2 = n[4U * 0U + 3U];
    uint64_t *res_i = tmp + 4U * 0U + 3U;
    c = Lib_IntTypes_Intrinsics_add_carry_u64(c, t12, t2, res_i);
  }
  uint64_t c1 = c;
  KRML_MAYBE_UNUSED_VAR(c1);
  uint64_t c2 = 0ULL - c00;
  KRML_MAYBE_FOR4(i,
    0U,
    4U,
    1U,
    uint64_t *os = res;
    uint64_t x1 = (c2 & tmp[i]) | (~c2 & res[i]);
    os[i] = x1;);
}

/* SNIPPET_END: bn_sub_mod4 */

/* SNIPPET_START: bn_mul4 */

static inline void bn_mul4(uint64_t *res, uint64_t *x, uint64_t *y)
{
  memset(res, 0U, 8U * sizeof (uint64_t));
  KRML_MAYBE_FOR4(i0,
    0U,
    4U,
    1U,
    uint64_t bj = y[i0];
    uint64_t *res_j = res + i0;
    uint64_t c = 0ULL;
    {
      uint64_t a_i = x[4U * 0U];
      uint64_t *res_i0 = res_j + 4U * 0U;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i, bj, c, res_i0);
      uint64_t a_i0 = x[4U * 0U + 1U];
      uint64_t *res_i1 = res_j + 4U * 0U + 1U;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i0, bj, c, res_i1);
      uint64_t a_i1 = x[4U * 0U + 2U];
      uint64_t *res_i2 = res_j + 4U * 0U + 2U;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i1, bj, c, res_i2);
      uint64_t a_i2 = x[4U * 0U + 3U];
      uint64_t *res_i = res_j + 4U * 0U + 3U;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i2, bj, c, res_i);
    }
    uint64_t r = c;
    res[4U + i0] = r;);
}

/* SNIPPET_END: bn_mul4 */

/* SNIPPET_START: bn_sqr4 */

static inline void bn_sqr4(uint64_t *res, uint64_t *x)
{
  memset(res, 0U, 8U * sizeof (uint64_t));
  KRML_MAYBE_FOR4(i0,
    0U,
    4U,
    1U,
    uint64_t *ab = x;
    uint64_t a_j = x[i0];
    uint64_t *res_j = res + i0;
    uint64_t c = 0ULL;
    for (uint32_t i = 0U; i < i0 / 4U; i++)
    {
      uint64_t a_i = ab[4U * i];
      uint64_t *res_i0 = res_j + 4U * i;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i, a_j, c, res_i0);
      uint64_t a_i0 = ab[4U * i + 1U];
      uint64_t *res_i1 = res_j + 4U * i + 1U;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i0, a_j, c, res_i1);
      uint64_t a_i1 = ab[4U * i + 2U];
      uint64_t *res_i2 = res_j + 4U * i + 2U;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i1, a_j, c, res_i2);
      uint64_t a_i2 = ab[4U * i + 3U];
      uint64_t *res_i = res_j + 4U * i + 3U;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i2, a_j, c, res_i);
    }
    for (uint32_t i = i0 / 4U * 4U; i < i0; i++)
    {
      uint64_t a_i = ab[i];
      uint64_t *res_i = res_j + i;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i, a_j, c, res_i);
    }
    uint64_t r = c;
    res[i0 + i0] = r;);
  uint64_t c0 = Hacl_Bignum_Addition_bn_add_eq_len_u64(8U, res, res, res);
  KRML_MAYBE_UNUSED_VAR(c0);
  uint64_t tmp[8U] = { 0U };
  KRML_MAYBE_FOR4(i,
    0U,
    4U,
    1U,
    FStar_UInt128_uint128 res1 = FStar_UInt128_mul_wide(x[i], x[i]);
    uint64_t hi = FStar_UInt128_uint128_to_uint64(FStar_UInt128_shift_right(res1, 64U));
    uint64_t lo = FStar_UInt128_uint128_to_uint64(res1);
    tmp[2U * i] = lo;
    tmp[2U * i + 1U] = hi;);
  uint64_t c1 = Hacl_Bignum_Addition_bn_add_eq_len_u64(8U, res, tmp, res);
  KRML_MAYBE_UNUSED_VAR(c1);
}

/* SNIPPET_END: bn_sqr4 */

/* SNIPPET_START: bn_to_bytes_be4 */

static inline void bn_to_bytes_be4(uint8_t *res, uint64_t *f)
{
  uint8_t tmp[32U] = { 0U };
  KRML_MAYBE_UNUSED_VAR(tmp);
  KRML_MAYBE_FOR4(i, 0U, 4U, 1U, store64_be(res + i * 8U, f[4U - i - 1U]););
}

/* SNIPPET_END: bn_to_bytes_be4 */

/* SNIPPET_START: bn_from_bytes_be4 */

static inline void bn_from_bytes_be4(uint64_t *res, uint8_t *b)
{
  KRML_MAYBE_FOR4(i,
    0U,
    4U,
    1U,
    uint64_t *os = res;
    uint64_t u = load64_be(b + (4U - i - 1U) * 8U);
    uint64_t x = u;
    os[i] = x;);
}

/* SNIPPET_END: bn_from_bytes_be4 */

/* SNIPPET_START: bn2_to_bytes_be4 */

static inline void bn2_to_bytes_be4(uint8_t *res, uint64_t *x, uint64_t *y)
{
  bn_to_bytes_be4(res, x);
  bn_to_bytes_be4(res + 32U, y);
}

/* SNIPPET_END: bn2_to_bytes_be4 */

/* SNIPPET_START: make_prime */

static inline void make_prime(uint64_t *n)
{
  n[0U] = 0xffffffffffffffffULL;
  n[1U] = 0xffffffffULL;
  n[2U] = 0x0ULL;
  n[3U] = 0xffffffff00000001ULL;
}

/* SNIPPET_END: make_prime */

/* SNIPPET_START: make_order */

static inline void make_order(uint64_t *n)
{
  n[0U] = 0xf3b9cac2fc632551ULL;
  n[1U] = 0xbce6faada7179e84ULL;
  n[2U] = 0xffffffffffffffffULL;
  n[3U] = 0xffffffff00000000ULL;
}

/* SNIPPET_END: make_order */

/* SNIPPET_START: make_a_coeff */

static inline void make_a_coeff(uint64_t *a)
{
  a[0U] = 0xfffffffffffffffcULL;
  a[1U] = 0x3ffffffffULL;
  a[2U] = 0x0ULL;
  a[3U] = 0xfffffffc00000004ULL;
}

/* SNIPPET_END: make_a_coeff */

/* SNIPPET_START: make_b_coeff */

static inline void make_b_coeff(uint64_t *b)
{
  b[0U] = 0xd89cdf6229c4bddfULL;
  b[1U] = 0xacf005cd78843090ULL;
  b[2U] = 0xe5a220abf7212ed6ULL;
  b[3U] = 0xdc30061d04874834ULL;
}

/* SNIPPET_END: make_b_coeff */

/* SNIPPET_START: make_g_x */

static inline void make_g_x(uint64_t *n)
{
  n[0U] = 0x79e730d418a9143cULL;
  n[1U] = 0x75ba95fc5fedb601ULL;
  n[2U] = 0x79fb732b77622510ULL;
  n[3U] = 0x18905f76a53755c6ULL;
}

/* SNIPPET_END: make_g_x */

/* SNIPPET_START: make_g_y */

static inline void make_g_y(uint64_t *n)
{
  n[0U] = 0xddf25357ce95560aULL;
  n[1U] = 0x8b4ab8e4ba19e45cULL;
  n[2U] = 0xd2e88688dd21f325ULL;
  n[3U] = 0x8571ff1825885d85ULL;
}

/* SNIPPET_END: make_g_y */

/* SNIPPET_START: make_fmont_R2 */

static inline void make_fmont_R2(uint64_t *n)
{
  n[0U] = 0x3ULL;
  n[1U] = 0xfffffffbffffffffULL;
  n[2U] = 0xfffffffffffffffeULL;
  n[3U] = 0x4fffffffdULL;
}

/* SNIPPET_END: make_fmont_R2 */

/* SNIPPET_START: make_fzero */

static inline void make_fzero(uint64_t *n)
{
  n[0U] = 0ULL;
  n[1U] = 0ULL;
  n[2U] = 0ULL;
  n[3U] = 0ULL;
}

/* SNIPPET_END: make_fzero */

/* SNIPPET_START: make_fone */

static inline void make_fone(uint64_t *n)
{
  n[0U] = 0x1ULL;
  n[1U] = 0xffffffff00000000ULL;
  n[2U] = 0xffffffffffffffffULL;
  n[3U] = 0xfffffffeULL;
}

/* SNIPPET_END: make_fone */

/* SNIPPET_START: bn_is_lt_prime_mask4 */

static inline uint64_t bn_is_lt_prime_mask4(uint64_t *f)
{
  uint64_t tmp[4U] = { 0U };
  make_prime(tmp);
  uint64_t c = bn_sub4(tmp, f, tmp);
  return 0ULL - c;
}

/* SNIPPET_END: bn_is_lt_prime_mask4 */

/* SNIPPET_START: feq_mask */

static inline uint64_t feq_mask(uint64_t *a, uint64_t *b)
{
  uint64_t r = bn_is_eq_mask4(a, b);
  return r;
}

/* SNIPPET_END: feq_mask */

/* SNIPPET_START: fadd0 */

static inline void fadd0(uint64_t *res, uint64_t *x, uint64_t *y)
{
  uint64_t n[4U] = { 0U };
  make_prime(n);
  bn_add_mod4(res, n, x, y);
}

/* SNIPPET_END: fadd0 */

/* SNIPPET_START: fsub0 */

static inline void fsub0(uint64_t *res, uint64_t *x, uint64_t *y)
{
  uint64_t n[4U] = { 0U };
  make_prime(n);
  bn_sub_mod4(res, n, x, y);
}

/* SNIPPET_END: fsub0 */

/* SNIPPET_START: fnegate_conditional_vartime */

static inline void fnegate_conditional_vartime(uint64_t *f, bool is_negate)
{
  uint64_t zero[4U] = { 0U };
  if (is_negate)
  {
    fsub0(f, zero, f);
  }
}

/* SNIPPET_END: fnegate_conditional_vartime */

/* SNIPPET_START: mont_reduction */

static inline void mont_reduction(uint64_t *res, uint64_t *x)
{
  uint64_t n[4U] = { 0U };
  make_prime(n);
  uint64_t c0 = 0ULL;
  KRML_MAYBE_FOR4(i0,
    0U,
    4U,
    1U,
    uint64_t qj = 1ULL * x[i0];
    uint64_t *res_j0 = x + i0;
    uint64_t c = 0ULL;
    {
      uint64_t a_i = n[4U * 0U];
      uint64_t *res_i0 = res_j0 + 4U * 0U;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i, qj, c, res_i0);
      uint64_t a_i0 = n[4U * 0U + 1U];
      uint64_t *res_i1 = res_j0 + 4U * 0U + 1U;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i0, qj, c, res_i1);
      uint64_t a_i1 = n[4U * 0U + 2U];
      uint64_t *res_i2 = res_j0 + 4U * 0U + 2U;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i1, qj, c, res_i2);
      uint64_t a_i2 = n[4U * 0U + 3U];
      uint64_t *res_i = res_j0 + 4U * 0U + 3U;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i2, qj, c, res_i);
    }
    uint64_t r = c;
    uint64_t c1 = r;
    uint64_t *resb = x + 4U + i0;
    uint64_t res_j = x[4U + i0];
    c0 = Lib_IntTypes_Intrinsics_add_carry_u64(c0, c1, res_j, resb););
  memcpy(res, x + 4U, 4U * sizeof (uint64_t));
  uint64_t c00 = c0;
  uint64_t tmp[4U] = { 0U };
  uint64_t c = 0ULL;
  {
    uint64_t t1 = res[4U * 0U];
    uint64_t t20 = n[4U * 0U];
    uint64_t *res_i0 = tmp + 4U * 0U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t1, t20, res_i0);
    uint64_t t10 = res[4U * 0U + 1U];
    uint64_t t21 = n[4U * 0U + 1U];
    uint64_t *res_i1 = tmp + 4U * 0U + 1U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t10, t21, res_i1);
    uint64_t t11 = res[4U * 0U + 2U];
    uint64_t t22 = n[4U * 0U + 2U];
    uint64_t *res_i2 = tmp + 4U * 0U + 2U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t11, t22, res_i2);
    uint64_t t12 = res[4U * 0U + 3U];
    uint64_t t2 = n[4U * 0U + 3U];
    uint64_t *res_i = tmp + 4U * 0U + 3U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t12, t2, res_i);
  }
  uint64_t c1 = c;
  uint64_t c2 = c00 - c1;
  KRML_MAYBE_FOR4(i,
    0U,
    4U,
    1U,
    uint64_t *os = res;
    uint64_t x1 = (c2 & res[i]) | (~c2 & tmp[i]);
    os[i] = x1;);
}

/* SNIPPET_END: mont_reduction */

/* SNIPPET_START: fmul0 */

static inline void fmul0(uint64_t *res, uint64_t *x, uint64_t *y)
{
  uint64_t tmp[8U] = { 0U };
  bn_mul4(tmp, x, y);
  mont_reduction(res, tmp);
}

/* SNIPPET_END: fmul0 */

/* SNIPPET_START: fsqr0 */

static inline void fsqr0(uint64_t *res, uint64_t *x)
{
  uint64_t tmp[8U] = { 0U };
  bn_sqr4(tmp, x);
  mont_reduction(res, tmp);
}

/* SNIPPET_END: fsqr0 */

/* SNIPPET_START: from_mont */

static inline void from_mont(uint64_t *res, uint64_t *a)
{
  uint64_t tmp[8U] = { 0U };
  memcpy(tmp, a, 4U * sizeof (uint64_t));
  mont_reduction(res, tmp);
}

/* SNIPPET_END: from_mont */

/* SNIPPET_START: to_mont */

static inline void to_mont(uint64_t *res, uint64_t *a)
{
  uint64_t r2modn[4U] = { 0U };
  make_fmont_R2(r2modn);
  fmul0(res, a, r2modn);
}

/* SNIPPET_END: to_mont */

/* SNIPPET_START: fmul_by_b_coeff */

static inline void fmul_by_b_coeff(uint64_t *res, uint64_t *x)
{
  uint64_t b_coeff[4U] = { 0U };
  make_b_coeff(b_coeff);
  fmul0(res, b_coeff, x);
}

/* SNIPPET_END: fmul_by_b_coeff */

/* SNIPPET_START: fcube */

static inline void fcube(uint64_t *res, uint64_t *x)
{
  fsqr0(res, x);
  fmul0(res, res, x);
}

/* SNIPPET_END: fcube */

/* SNIPPET_START: finv */

static inline void finv(uint64_t *res, uint64_t *a)
{
  uint64_t tmp[16U] = { 0U };
  uint64_t *x30 = tmp;
  uint64_t *x2 = tmp + 4U;
  uint64_t *tmp1 = tmp + 8U;
  uint64_t *tmp2 = tmp + 12U;
  memcpy(x2, a, 4U * sizeof (uint64_t));
  {
    fsqr0(x2, x2);
  }
  fmul0(x2, x2, a);
  memcpy(x30, x2, 4U * sizeof (uint64_t));
  {
    fsqr0(x30, x30);
  }
  fmul0(x30, x30, a);
  memcpy(tmp1, x30, 4U * sizeof (uint64_t));
  KRML_MAYBE_FOR3(i, 0U, 3U, 1U, fsqr0(tmp1, tmp1););
  fmul0(tmp1, tmp1, x30);
  memcpy(tmp2, tmp1, 4U * sizeof (uint64_t));
  KRML_MAYBE_FOR6(i, 0U, 6U, 1U, fsqr0(tmp2, tmp2););
  fmul0(tmp2, tmp2, tmp1);
  memcpy(tmp1, tmp2, 4U * sizeof (uint64_t));
  KRML_MAYBE_FOR3(i, 0U, 3U, 1U, fsqr0(tmp1, tmp1););
  fmul0(tmp1, tmp1, x30);
  memcpy(x30, tmp1, 4U * sizeof (uint64_t));
  KRML_MAYBE_FOR15(i, 0U, 15U, 1U, fsqr0(x30, x30););
  fmul0(x30, x30, tmp1);
  memcpy(tmp1, x30, 4U * sizeof (uint64_t));
  KRML_MAYBE_FOR2(i, 0U, 2U, 1U, fsqr0(tmp1, tmp1););
  fmul0(tmp1, tmp1, x2);
  memcpy(x2, tmp1, 4U * sizeof (uint64_t));
  for (uint32_t i = 0U; i < 32U; i++)
  {
    fsqr0(x2, x2);
  }
  fmul0(x2, x2, a);
  for (uint32_t i = 0U; i < 128U; i++)
  {
    fsqr0(x2, x2);
  }
  fmul0(x2, x2, tmp1);
  for (uint32_t i = 0U; i < 32U; i++)
  {
    fsqr0(x2, x2);
  }
  fmul0(x2, x2, tmp1);
  for (uint32_t i = 0U; i < 30U; i++)
  {
    fsqr0(x2, x2);
  }
  fmul0(x2, x2, x30);
  KRML_MAYBE_FOR2(i, 0U, 2U, 1U, fsqr0(x2, x2););
  fmul0(tmp1, x2, a);
  memcpy(res, tmp1, 4U * sizeof (uint64_t));
}

/* SNIPPET_END: finv */

/* SNIPPET_START: fsqrt */

static inline void fsqrt(uint64_t *res, uint64_t *a)
{
  uint64_t tmp[8U] = { 0U };
  uint64_t *tmp1 = tmp;
  uint64_t *tmp2 = tmp + 4U;
  memcpy(tmp1, a, 4U * sizeof (uint64_t));
  {
    fsqr0(tmp1, tmp1);
  }
  fmul0(tmp1, tmp1, a);
  memcpy(tmp2, tmp1, 4U * sizeof (uint64_t));
  KRML_MAYBE_FOR2(i, 0U, 2U, 1U, fsqr0(tmp2, tmp2););
  fmul0(tmp2, tmp2, tmp1);
  memcpy(tmp1, tmp2, 4U * sizeof (uint64_t));
  KRML_MAYBE_FOR4(i, 0U, 4U, 1U, fsqr0(tmp1, tmp1););
  fmul0(tmp1, tmp1, tmp2);
  memcpy(tmp2, tmp1, 4U * sizeof (uint64_t));
  KRML_MAYBE_FOR8(i, 0U, 8U, 1U, fsqr0(tmp2, tmp2););
  fmul0(tmp2, tmp2, tmp1);
  memcpy(tmp1, tmp2, 4U * sizeof (uint64_t));
  KRML_MAYBE_FOR16(i, 0U, 16U, 1U, fsqr0(tmp1, tmp1););
  fmul0(tmp1, tmp1, tmp2);
  memcpy(tmp2, tmp1, 4U * sizeof (uint64_t));
  for (uint32_t i = 0U; i < 32U; i++)
  {
    fsqr0(tmp2, tmp2);
  }
  fmul0(tmp2, tmp2, a);
  for (uint32_t i = 0U; i < 96U; i++)
  {
    fsqr0(tmp2, tmp2);
  }
  fmul0(tmp2, tmp2, a);
  for (uint32_t i = 0U; i < 94U; i++)
  {
    fsqr0(tmp2, tmp2);
  }
  memcpy(res, tmp2, 4U * sizeof (uint64_t));
}

/* SNIPPET_END: fsqrt */

/* SNIPPET_START: make_base_point */

static inline void make_base_point(uint64_t *p)
{
  uint64_t *x = p;
  uint64_t *y = p + 4U;
  uint64_t *z = p + 8U;
  make_g_x(x);
  make_g_y(y);
  make_fone(z);
}

/* SNIPPET_END: make_base_point */

/* SNIPPET_START: make_point_at_inf */

static inline void make_point_at_inf(uint64_t *p)
{
  uint64_t *x = p;
  uint64_t *y = p + 4U;
  uint64_t *z = p + 8U;
  make_fzero(x);
  make_fone(y);
  make_fzero(z);
}

/* SNIPPET_END: make_point_at_inf */

/* SNIPPET_START: is_point_at_inf_vartime */

static inline bool is_point_at_inf_vartime(uint64_t *p)
{
  uint64_t *pz = p + 8U;
  return bn_is_zero_vartime4(pz);
}

/* SNIPPET_END: is_point_at_inf_vartime */

/* SNIPPET_START: to_aff_point */

static inline void to_aff_point(uint64_t *res, uint64_t *p)
{
  uint64_t zinv[4U] = { 0U };
  uint64_t *px = p;
  uint64_t *py = p + 4U;
  uint64_t *pz = p + 8U;
  uint64_t *x = res;
  uint64_t *y = res + 4U;
  finv(zinv, pz);
  fmul0(x, px, zinv);
  fmul0(y, py, zinv);
  from_mont(x, x);
  from_mont(y, y);
}

/* SNIPPET_END: to_aff_point */

/* SNIPPET_START: to_aff_point_x */

static inline void to_aff_point_x(uint64_t *res, uint64_t *p)
{
  uint64_t zinv[4U] = { 0U };
  uint64_t *px = p;
  uint64_t *pz = p + 8U;
  finv(zinv, pz);
  fmul0(res, px, zinv);
  from_mont(res, res);
}

/* SNIPPET_END: to_aff_point_x */

/* SNIPPET_START: to_proj_point */

static inline void to_proj_point(uint64_t *res, uint64_t *p)
{
  uint64_t *px = p;
  uint64_t *py = p + 4U;
  uint64_t *rx = res;
  uint64_t *ry = res + 4U;
  uint64_t *rz = res + 8U;
  to_mont(rx, px);
  to_mont(ry, py);
  make_fone(rz);
}

/* SNIPPET_END: to_proj_point */

/* SNIPPET_START: is_on_curve_vartime */

static inline bool is_on_curve_vartime(uint64_t *p)
{
  uint64_t rp[4U] = { 0U };
  uint64_t tx[4U] = { 0U };
  uint64_t ty[4U] = { 0U };
  uint64_t *px = p;
  uint64_t *py = p + 4U;
  to_mont(tx, px);
  to_mont(ty, py);
  uint64_t tmp[4U] = { 0U };
  fcube(rp, tx);
  make_a_coeff(tmp);
  fmul0(tmp, tmp, tx);
  fadd0(rp, tmp, rp);
  make_b_coeff(tmp);
  fadd0(rp, tmp, rp);
  fsqr0(ty, ty);
  uint64_t r = feq_mask(ty, rp);
  bool r0 = r == 0xFFFFFFFFFFFFFFFFULL;
  return r0;
}

/* SNIPPET_END: is_on_curve_vartime */

/* SNIPPET_START: aff_point_store */

static inline void aff_point_store(uint8_t *res, uint64_t *p)
{
  uint64_t *px = p;
  uint64_t *py = p + 4U;
  bn2_to_bytes_be4(res, px, py);
}

/* SNIPPET_END: aff_point_store */

/* SNIPPET_START: point_store */

static inline void point_store(uint8_t *res, uint64_t *p)
{
  uint64_t aff_p[8U] = { 0U };
  to_aff_point(aff_p, p);
  aff_point_store(res, aff_p);
}

/* SNIPPET_END: point_store */

/* SNIPPET_START: aff_point_load_vartime */

static inline bool aff_point_load_vartime(uint64_t *p, uint8_t *b)
{
  uint8_t *p_x = b;
  uint8_t *p_y = b + 32U;
  uint64_t *bn_p_x = p;
  uint64_t *bn_p_y = p + 4U;
  bn_from_bytes_be4(bn_p_x, p_x);
  bn_from_bytes_be4(bn_p_y, p_y);
  uint64_t *px = p;
  uint64_t *py = p + 4U;
  uint64_t lessX = bn_is_lt_prime_mask4(px);
  uint64_t lessY = bn_is_lt_prime_mask4(py);
  uint64_t res = lessX & lessY;
  bool is_xy_valid = res == 0xFFFFFFFFFFFFFFFFULL;
  if (!is_xy_valid)
  {
    return false;
  }
  return is_on_curve_vartime(p);
}

/* SNIPPET_END: aff_point_load_vartime */

/* SNIPPET_START: load_point_vartime */

static inline bool load_point_vartime(uint64_t *p, uint8_t *b)
{
  uint64_t p_aff[8U] = { 0U };
  bool res = aff_point_load_vartime(p_aff, b);
  if (res)
  {
    to_proj_point(p, p_aff);
  }
  return res;
}

/* SNIPPET_END: load_point_vartime */

/* SNIPPET_START: aff_point_decompress_vartime */

static inline bool aff_point_decompress_vartime(uint64_t *x, uint64_t *y, uint8_t *s)
{
  uint8_t s0 = s[0U];
  uint8_t s01 = s0;
  if (!(s01 == 0x02U || s01 == 0x03U))
  {
    return false;
  }
  uint8_t *xb = s + 1U;
  bn_from_bytes_be4(x, xb);
  uint64_t is_x_valid = bn_is_lt_prime_mask4(x);
  bool is_x_valid1 = is_x_valid == 0xFFFFFFFFFFFFFFFFULL;
  bool is_y_odd = s01 == 0x03U;
  if (!is_x_valid1)
  {
    return false;
  }
  uint64_t y2M[4U] = { 0U };
  uint64_t xM[4U] = { 0U };
  uint64_t yM[4U] = { 0U };
  to_mont(xM, x);
  uint64_t tmp[4U] = { 0U };
  fcube(y2M, xM);
  make_a_coeff(tmp);
  fmul0(tmp, tmp, xM);
  fadd0(y2M, tmp, y2M);
  make_b_coeff(tmp);
  fadd0(y2M, tmp, y2M);
  fsqrt(yM, y2M);
  from_mont(y, yM);
  fsqr0(yM, yM);
  uint64_t r = feq_mask(yM, y2M);
  bool is_y_valid = r == 0xFFFFFFFFFFFFFFFFULL;
  bool is_y_valid0 = is_y_valid;
  if (!is_y_valid0)
  {
    return false;
  }
  uint64_t is_y_odd1 = y[0U] & 1ULL;
  bool is_y_odd2 = is_y_odd1 == 1ULL;
  fnegate_conditional_vartime(y, is_y_odd2 != is_y_odd);
  return true;
}

/* SNIPPET_END: aff_point_decompress_vartime */

/* SNIPPET_START: point_double */

static inline void point_double(uint64_t *res, uint64_t *p)
{
  uint64_t tmp[20U] = { 0U };
  uint64_t *x = p;
  uint64_t *z = p + 8U;
  uint64_t *x3 = res;
  uint64_t *y3 = res + 4U;
  uint64_t *z3 = res + 8U;
  uint64_t *t0 = tmp;
  uint64_t *t1 = tmp + 4U;
  uint64_t *t2 = tmp + 8U;
  uint64_t *t3 = tmp + 12U;
  uint64_t *t4 = tmp + 16U;
  uint64_t *x1 = p;
  uint64_t *y = p + 4U;
  uint64_t *z1 = p + 8U;
  fsqr0(t0, x1);
  fsqr0(t1, y);
  fsqr0(t2, z1);
  fmul0(t3, x1, y);
  fadd0(t3, t3, t3);
  fmul0(t4, y, z1);
  fmul0(z3, x, z);
  fadd0(z3, z3, z3);
  fmul_by_b_coeff(y3, t2);
  fsub0(y3, y3, z3);
  fadd0(x3, y3, y3);
  fadd0(y3, x3, y3);
  fsub0(x3, t1, y3);
  fadd0(y3, t1, y3);
  fmul0(y3, x3, y3);
  fmul0(x3, x3, t3);
  fadd0(t3, t2, t2);
  fadd0(t2, t2, t3);
  fmul_by_b_coeff(z3, z3);
  fsub0(z3, z3, t2);
  fsub0(z3, z3, t0);
  fadd0(t3, z3, z3);
  fadd0(z3, z3, t3);
  fadd0(t3, t0, t0);
  fadd0(t0, t3, t0);
  fsub0(t0, t0, t2);
  fmul0(t0, t0, z3);
  fadd0(y3, y3, t0);
  fadd0(t0, t4, t4);
  fmul0(z3, t0, z3);
  fsub0(x3, x3, z3);
  fmul0(z3, t0, t1);
  fadd0(z3, z3, z3);
  fadd0(z3, z3, z3);
}

/* SNIPPET_END: point_double */

/* SNIPPET_START: point_add */

static inline void point_add(uint64_t *res, uint64_t *p, uint64_t *q)
{
  uint64_t tmp[36U] = { 0U };
  uint64_t *t0 = tmp;
  uint64_t *t1 = tmp + 24U;
  uint64_t *x3 = t1;
  uint64_t *y3 = t1 + 4U;
  uint64_t *z3 = t1 + 8U;
  uint64_t *t01 = t0;
  uint64_t *t11 = t0 + 4U;
  uint64_t *t2 = t0 + 8U;
  uint64_t *t3 = t0 + 12U;
  uint64_t *t4 = t0 + 16U;
  uint64_t *t5 = t0 + 20U;
  uint64_t *x1 = p;
  uint64_t *y1 = p + 4U;
  uint64_t *z10 = p + 8U;
  uint64_t *x20 = q;
  uint64_t *y20 = q + 4U;
  uint64_t *z20 = q + 8U;
  fmul0(t01, x1, x20);
  fmul0(t11, y1, y20);
  fmul0(t2, z10, z20);
  fadd0(t3, x1, y1);
  fadd0(t4, x20, y20);
  fmul0(t3, t3, t4);
  fadd0(t4, t01, t11);
  uint64_t *y10 = p + 4U;
  uint64_t *z11 = p + 8U;
  uint64_t *y2 = q + 4U;
  uint64_t *z21 = q + 8U;
  fsub0(t3, t3, t4);
  fadd0(t4, y10, z11);
  fadd0(t5, y2, z21);
  fmul0(t4, t4, t5);
  fadd0(t5, t11, t2);
  fsub0(t4, t4, t5);
  uint64_t *x10 = p;
  uint64_t *z1 = p + 8U;
  uint64_t *x2 = q;
  uint64_t *z2 = q + 8U;
  fadd0(x3, x10, z1);
  fadd0(y3, x2, z2);
  fmul0(x3, x3, y3);
  fadd0(y3, t01, t2);
  fsub0(y3, x3, y3);
  fmul_by_b_coeff(z3, t2);
  fsub0(x3, y3, z3);
  fadd0(z3, x3, x3);
  fadd0(x3, x3, z3);
  fsub0(z3, t11, x3);
  fadd0(x3, t11, x3);
  fmul_by_b_coeff(y3, y3);
  fadd0(t11, t2, t2);
  fadd0(t2, t11, t2);
  fsub0(y3, y3, t2);
  fsub0(y3, y3, t01);
  fadd0(t11, y3, y3);
  fadd0(y3, t11, y3);
  fadd0(t11, t01, t01);
  fadd0(t01, t11, t01);
  fsub0(t01, t01, t2);
  fmul0(t11, t4, y3);
  fmul0(t2, t01, y3);
  fmul0(y3, x3, z3);
  fadd0(y3, y3, t2);
  fmul0(x3, t3, x3);
  fsub0(x3, x3, t11);
  fmul0(z3, t4, z3);
  fmul0(t11, t3, t01);
  fadd0(z3, z3, t11);
  memcpy(res, t1, 12U * sizeof (uint64_t));
}

/* SNIPPET_END: point_add */

/* SNIPPET_START: point_mul */

static inline void point_mul(uint64_t *res, uint64_t *scalar, uint64_t *p)
{
  uint64_t table[192U] = { 0U };
  uint64_t tmp[12U] = { 0U };
  uint64_t *t0 = table;
  uint64_t *t1 = table + 12U;
  make_point_at_inf(t0);
  memcpy(t1, p, 12U * sizeof (uint64_t));
  KRML_MAYBE_FOR7(i,
    0U,
    7U,
    1U,
    uint64_t *t11 = table + (i + 1U) * 12U;
    point_double(tmp, t11);
    memcpy(table + (2U * i + 2U) * 12U, tmp, 12U * sizeof (uint64_t));
    uint64_t *t2 = table + (2U * i + 2U) * 12U;
    point_add(tmp, p, t2);
    memcpy(table + (2U * i + 3U) * 12U, tmp, 12U * sizeof (uint64_t)););
  make_point_at_inf(res);
  uint64_t tmp0[12U] = { 0U };
  for (uint32_t i0 = 0U; i0 < 64U; i0++)
  {
    KRML_MAYBE_FOR4(i, 0U, 4U, 1U, point_double(res, res););
    uint32_t k = 256U - 4U * i0 - 4U;
    uint64_t bits_l = Hacl_Bignum_Lib_bn_get_bits_u64(4U, scalar, k, 4U);
    memcpy(tmp0, (uint64_t *)table, 12U * sizeof (uint64_t));
    KRML_MAYBE_FOR15(i1,
      0U,
      15U,
      1U,
      uint64_t c = FStar_UInt64_eq_mask(bits_l, (uint64_t)(i1 + 1U));
      const uint64_t *res_j = table + (i1 + 1U) * 12U;
      KRML_MAYBE_FOR12(i,
        0U,
        12U,
        1U,
        uint64_t *os = tmp0;
        uint64_t x = (c & res_j[i]) | (~c & tmp0[i]);
        os[i] = x;););
    point_add(res, res, tmp0);
  }
}

/* SNIPPET_END: point_mul */

/* SNIPPET_START: precomp_get_consttime */

static inline void precomp_get_consttime(const uint64_t *table, uint64_t bits_l, uint64_t *tmp)
{
  memcpy(tmp, (uint64_t *)table, 12U * sizeof (uint64_t));
  KRML_MAYBE_FOR15(i0,
    0U,
    15U,
    1U,
    uint64_t c = FStar_UInt64_eq_mask(bits_l, (uint64_t)(i0 + 1U));
    const uint64_t *res_j = table + (i0 + 1U) * 12U;
    KRML_MAYBE_FOR12(i,
      0U,
      12U,
      1U,
      uint64_t *os = tmp;
      uint64_t x = (c & res_j[i]) | (~c & tmp[i]);
      os[i] = x;););
}

/* SNIPPET_END: precomp_get_consttime */

/* SNIPPET_START: point_mul_g */

static inline void point_mul_g(uint64_t *res, uint64_t *scalar)
{
  uint64_t q1[12U] = { 0U };
  make_base_point(q1);
  uint64_t
  q2[12U] =
    {
      1499621593102562565ULL, 16692369783039433128ULL, 15337520135922861848ULL,
      5455737214495366228ULL, 17827017231032529600ULL, 12413621606240782649ULL,
      2290483008028286132ULL, 15752017553340844820ULL, 4846430910634234874ULL,
      10861682798464583253ULL, 15404737222404363049ULL, 363586619281562022ULL
    };
  uint64_t
  q3[12U] =
    {
      14619254753077084366ULL, 13913835116514008593ULL, 15060744674088488145ULL,
      17668414598203068685ULL, 10761169236902342334ULL, 15467027479157446221ULL,
      14989185522423469618ULL, 14354539272510107003ULL, 14298211796392133693ULL,
      13270323784253711450ULL, 13380964971965046957ULL, 8686204248456909699ULL
    };
  uint64_t
  q4[12U] =
    {
      7870395003430845958ULL, 18001862936410067720ULL, 8006461232116967215ULL,
      5921313779532424762ULL, 10702113371959864307ULL, 8070517410642379879ULL,
      7139806720777708306ULL, 8253938546650739833ULL, 17490482834545705718ULL,
      1065249776797037500ULL, 5018258455937968775ULL, 14100621120178668337ULL
    };
  uint64_t *r1 = scalar;
  uint64_t *r2 = scalar + 1U;
  uint64_t *r3 = scalar + 2U;
  uint64_t *r4 = scalar + 3U;
  make_point_at_inf(res);
  uint64_t tmp[12U] = { 0U };
  KRML_MAYBE_FOR16(i,
    0U,
    16U,
    1U,
    KRML_MAYBE_FOR4(i0, 0U, 4U, 1U, point_double(res, res););
    uint32_t k = 64U - 4U * i - 4U;
    uint64_t bits_l = Hacl_Bignum_Lib_bn_get_bits_u64(1U, r4, k, 4U);
    precomp_get_consttime(Hacl_P256_PrecompTable_precomp_g_pow2_192_table_w4, bits_l, tmp);
    point_add(res, res, tmp);
    uint32_t k0 = 64U - 4U * i - 4U;
    uint64_t bits_l0 = Hacl_Bignum_Lib_bn_get_bits_u64(1U, r3, k0, 4U);
    precomp_get_consttime(Hacl_P256_PrecompTable_precomp_g_pow2_128_table_w4, bits_l0, tmp);
    point_add(res, res, tmp);
    uint32_t k1 = 64U - 4U * i - 4U;
    uint64_t bits_l1 = Hacl_Bignum_Lib_bn_get_bits_u64(1U, r2, k1, 4U);
    precomp_get_consttime(Hacl_P256_PrecompTable_precomp_g_pow2_64_table_w4, bits_l1, tmp);
    point_add(res, res, tmp);
    uint32_t k2 = 64U - 4U * i - 4U;
    uint64_t bits_l2 = Hacl_Bignum_Lib_bn_get_bits_u64(1U, r1, k2, 4U);
    precomp_get_consttime(Hacl_P256_PrecompTable_precomp_basepoint_table_w4, bits_l2, tmp);
    point_add(res, res, tmp););
  KRML_MAYBE_UNUSED_VAR(q1);
  KRML_MAYBE_UNUSED_VAR(q2);
  KRML_MAYBE_UNUSED_VAR(q3);
  KRML_MAYBE_UNUSED_VAR(q4);
}

/* SNIPPET_END: point_mul_g */

/* SNIPPET_START: point_mul_double_g */

static inline void
point_mul_double_g(uint64_t *res, uint64_t *scalar1, uint64_t *scalar2, uint64_t *q2)
{
  uint64_t q1[12U] = { 0U };
  make_base_point(q1);
  uint64_t table2[384U] = { 0U };
  uint64_t tmp[12U] = { 0U };
  uint64_t *t0 = table2;
  uint64_t *t1 = table2 + 12U;
  make_point_at_inf(t0);
  memcpy(t1, q2, 12U * sizeof (uint64_t));
  KRML_MAYBE_FOR15(i,
    0U,
    15U,
    1U,
    uint64_t *t11 = table2 + (i + 1U) * 12U;
    point_double(tmp, t11);
    memcpy(table2 + (2U * i + 2U) * 12U, tmp, 12U * sizeof (uint64_t));
    uint64_t *t2 = table2 + (2U * i + 2U) * 12U;
    point_add(tmp, q2, t2);
    memcpy(table2 + (2U * i + 3U) * 12U, tmp, 12U * sizeof (uint64_t)););
  uint64_t tmp0[12U] = { 0U };
  uint32_t i0 = 255U;
  uint64_t bits_c = Hacl_Bignum_Lib_bn_get_bits_u64(4U, scalar1, i0, 5U);
  uint32_t bits_l32 = (uint32_t)bits_c;
  const uint64_t *a_bits_l = Hacl_P256_PrecompTable_precomp_basepoint_table_w5 + bits_l32 * 12U;
  memcpy(res, (uint64_t *)a_bits_l, 12U * sizeof (uint64_t));
  uint32_t i1 = 255U;
  uint64_t bits_c0 = Hacl_Bignum_Lib_bn_get_bits_u64(4U, scalar2, i1, 5U);
  uint32_t bits_l320 = (uint32_t)bits_c0;
  const uint64_t *a_bits_l0 = table2 + bits_l320 * 12U;
  memcpy(tmp0, (uint64_t *)a_bits_l0, 12U * sizeof (uint64_t));
  point_add(res, res, tmp0);
  uint64_t tmp1[12U] = { 0U };
  for (uint32_t i = 0U; i < 51U; i++)
  {
    KRML_MAYBE_FOR5(i2, 0U, 5U, 1U, point_double(res, res););
    uint32_t k = 255U - 5U * i - 5U;
    uint64_t bits_l = Hacl_Bignum_Lib_bn_get_bits_u64(4U, scalar2, k, 5U);
    uint32_t bits_l321 = (uint32_t)bits_l;
    const uint64_t *a_bits_l1 = table2 + bits_l321 * 12U;
    memcpy(tmp1, (uint64_t *)a_bits_l1, 12U * sizeof (uint64_t));
    point_add(res, res, tmp1);
    uint32_t k0 = 255U - 5U * i - 5U;
    uint64_t bits_l0 = Hacl_Bignum_Lib_bn_get_bits_u64(4U, scalar1, k0, 5U);
    uint32_t bits_l322 = (uint32_t)bits_l0;
    const
    uint64_t
    *a_bits_l2 = Hacl_P256_PrecompTable_precomp_basepoint_table_w5 + bits_l322 * 12U;
    memcpy(tmp1, (uint64_t *)a_bits_l2, 12U * sizeof (uint64_t));
    point_add(res, res, tmp1);
  }
}

/* SNIPPET_END: point_mul_double_g */

/* SNIPPET_START: bn_is_lt_order_mask4 */

static inline uint64_t bn_is_lt_order_mask4(uint64_t *f)
{
  uint64_t tmp[4U] = { 0U };
  make_order(tmp);
  uint64_t c = bn_sub4(tmp, f, tmp);
  return 0ULL - c;
}

/* SNIPPET_END: bn_is_lt_order_mask4 */

/* SNIPPET_START: bn_is_lt_order_and_gt_zero_mask4 */

static inline uint64_t bn_is_lt_order_and_gt_zero_mask4(uint64_t *f)
{
  uint64_t is_lt_order = bn_is_lt_order_mask4(f);
  uint64_t is_eq_zero = bn_is_zero_mask4(f);
  return is_lt_order & ~is_eq_zero;
}

/* SNIPPET_END: bn_is_lt_order_and_gt_zero_mask4 */

/* SNIPPET_START: qmod_short */

static inline void qmod_short(uint64_t *res, uint64_t *x)
{
  uint64_t tmp[4U] = { 0U };
  make_order(tmp);
  uint64_t c = bn_sub4(tmp, x, tmp);
  bn_cmovznz4(res, c, tmp, x);
}

/* SNIPPET_END: qmod_short */

/* SNIPPET_START: qadd */

static inline void qadd(uint64_t *res, uint64_t *x, uint64_t *y)
{
  uint64_t n[4U] = { 0U };
  make_order(n);
  bn_add_mod4(res, n, x, y);
}

/* SNIPPET_END: qadd */

/* SNIPPET_START: qmont_reduction */

static inline void qmont_reduction(uint64_t *res, uint64_t *x)
{
  uint64_t n[4U] = { 0U };
  make_order(n);
  uint64_t c0 = 0ULL;
  KRML_MAYBE_FOR4(i0,
    0U,
    4U,
    1U,
    uint64_t qj = 0xccd1c8aaee00bc4fULL * x[i0];
    uint64_t *res_j0 = x + i0;
    uint64_t c = 0ULL;
    {
      uint64_t a_i = n[4U * 0U];
      uint64_t *res_i0 = res_j0 + 4U * 0U;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i, qj, c, res_i0);
      uint64_t a_i0 = n[4U * 0U + 1U];
      uint64_t *res_i1 = res_j0 + 4U * 0U + 1U;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i0, qj, c, res_i1);
      uint64_t a_i1 = n[4U * 0U + 2U];
      uint64_t *res_i2 = res_j0 + 4U * 0U + 2U;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i1, qj, c, res_i2);
      uint64_t a_i2 = n[4U * 0U + 3U];
      uint64_t *res_i = res_j0 + 4U * 0U + 3U;
      c = Hacl_Bignum_Base_mul_wide_add2_u64(a_i2, qj, c, res_i);
    }
    uint64_t r = c;
    uint64_t c1 = r;
    uint64_t *resb = x + 4U + i0;
    uint64_t res_j = x[4U + i0];
    c0 = Lib_IntTypes_Intrinsics_add_carry_u64(c0, c1, res_j, resb););
  memcpy(res, x + 4U, 4U * sizeof (uint64_t));
  uint64_t c00 = c0;
  uint64_t tmp[4U] = { 0U };
  uint64_t c = 0ULL;
  {
    uint64_t t1 = res[4U * 0U];
    uint64_t t20 = n[4U * 0U];
    uint64_t *res_i0 = tmp + 4U * 0U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t1, t20, res_i0);
    uint64_t t10 = res[4U * 0U + 1U];
    uint64_t t21 = n[4U * 0U + 1U];
    uint64_t *res_i1 = tmp + 4U * 0U + 1U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t10, t21, res_i1);
    uint64_t t11 = res[4U * 0U + 2U];
    uint64_t t22 = n[4U * 0U + 2U];
    uint64_t *res_i2 = tmp + 4U * 0U + 2U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t11, t22, res_i2);
    uint64_t t12 = res[4U * 0U + 3U];
    uint64_t t2 = n[4U * 0U + 3U];
    uint64_t *res_i = tmp + 4U * 0U + 3U;
    c = Lib_IntTypes_Intrinsics_sub_borrow_u64(c, t12, t2, res_i);
  }
  uint64_t c1 = c;
  uint64_t c2 = c00 - c1;
  KRML_MAYBE_FOR4(i,
    0U,
    4U,
    1U,
    uint64_t *os = res;
    uint64_t x1 = (c2 & res[i]) | (~c2 & tmp[i]);
    os[i] = x1;);
}

/* SNIPPET_END: qmont_reduction */

/* SNIPPET_START: from_qmont */

static inline void from_qmont(uint64_t *res, uint64_t *x)
{
  uint64_t tmp[8U] = { 0U };
  memcpy(tmp, x, 4U * sizeof (uint64_t));
  qmont_reduction(res, tmp);
}

/* SNIPPET_END: from_qmont */

/* SNIPPET_START: qmul */

static inline void qmul(uint64_t *res, uint64_t *x, uint64_t *y)
{
  uint64_t tmp[8U] = { 0U };
  bn_mul4(tmp, x, y);
  qmont_reduction(res, tmp);
}

/* SNIPPET_END: qmul */

/* SNIPPET_START: qsqr */

static inline void qsqr(uint64_t *res, uint64_t *x)
{
  uint64_t tmp[8U] = { 0U };
  bn_sqr4(tmp, x);
  qmont_reduction(res, tmp);
}

/* SNIPPET_END: qsqr */

/* SNIPPET_START: Hacl_Impl_P256_DH_ecp256dh_i */

bool Hacl_Impl_P256_DH_ecp256dh_i(uint8_t *public_key, uint8_t *private_key)
{
  uint64_t tmp[16U] = { 0U };
  uint64_t *sk = tmp;
  uint64_t *pk = tmp + 4U;
  bn_from_bytes_be4(sk, private_key);
  uint64_t is_b_valid = bn_is_lt_order_and_gt_zero_mask4(sk);
  uint64_t oneq[4U] = { 0U };
  oneq[0U] = 1ULL;
  oneq[1U] = 0ULL;
  oneq[2U] = 0ULL;
  oneq[3U] = 0ULL;
  KRML_MAYBE_FOR4(i,
    0U,
    4U,
    1U,
    uint64_t *os = sk;
    uint64_t uu____0 = oneq[i];
    uint64_t x = uu____0 ^ (is_b_valid & (sk[i] ^ uu____0));
    os[i] = x;);
  uint64_t is_sk_valid = is_b_valid;
  point_mul_g(pk, sk);
  point_store(public_key, pk);
  return is_sk_valid == 0xFFFFFFFFFFFFFFFFULL;
}

/* SNIPPET_END: Hacl_Impl_P256_DH_ecp256dh_i */

/* SNIPPET_START: Hacl_Impl_P256_DH_ecp256dh_r */

bool
Hacl_Impl_P256_DH_ecp256dh_r(
  uint8_t *shared_secret,
  uint8_t *their_pubkey,
  uint8_t *private_key
)
{
  uint64_t tmp[16U] = { 0U };
  uint64_t *sk = tmp;
  uint64_t *pk = tmp + 4U;
  bool is_pk_valid = load_point_vartime(pk, their_pubkey);
  bn_from_bytes_be4(sk, private_key);
  uint64_t is_b_valid = bn_is_lt_order_and_gt_zero_mask4(sk);
  uint64_t oneq[4U] = { 0U };
  oneq[0U] = 1ULL;
  oneq[1U] = 0ULL;
  oneq[2U] = 0ULL;
  oneq[3U] = 0ULL;
  KRML_MAYBE_FOR4(i,
    0U,
    4U,
    1U,
    uint64_t *os = sk;
    uint64_t uu____0 = oneq[i];
    uint64_t x = uu____0 ^ (is_b_valid & (sk[i] ^ uu____0));
    os[i] = x;);
  uint64_t is_sk_valid = is_b_valid;
  uint64_t ss_proj[12U] = { 0U };
  if (is_pk_valid)
  {
    point_mul(ss_proj, sk, pk);
    point_store(shared_secret, ss_proj);
  }
  return is_sk_valid == 0xFFFFFFFFFFFFFFFFULL && is_pk_valid;
}

/* SNIPPET_END: Hacl_Impl_P256_DH_ecp256dh_r */

/* SNIPPET_START: qinv */

static inline void qinv(uint64_t *res, uint64_t *r)
{
  uint64_t tmp[28U] = { 0U };
  uint64_t *x6 = tmp;
  uint64_t *x_11 = tmp + 4U;
  uint64_t *x_101 = tmp + 8U;
  uint64_t *x_111 = tmp + 12U;
  uint64_t *x_1111 = tmp + 16U;
  uint64_t *x_10101 = tmp + 20U;
  uint64_t *x_101111 = tmp + 24U;
  memcpy(x6, r, 4U * sizeof (uint64_t));
  {
    qsqr(x6, x6);
  }
  qmul(x_11, x6, r);
  qmul(x_101, x6, x_11);
  qmul(x_111, x6, x_101);
  memcpy(x6, x_101, 4U * sizeof (uint64_t));
  {
    qsqr(x6, x6);
  }
  qmul(x_1111, x_101, x6);
  {
    qsqr(x6, x6);
  }
  qmul(x_10101, x6, r);
  memcpy(x6, x_10101, 4U * sizeof (uint64_t));
  {
    qsqr(x6, x6);
  }
  qmul(x_101111, x_101, x6);
  qmul(x6, x_10101, x6);
  uint64_t tmp1[4U] = { 0U };
  KRML_MAYBE_FOR2(i, 0U, 2U, 1U, qsqr(x6, x6););
  qmul(x6, x6, x_11);
  memcpy(tmp1, x6, 4U * sizeof (uint64_t));
  KRML_MAYBE_FOR8(i, 0U, 8U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x6);
  memcpy(x6, tmp1, 4U * sizeof (uint64_t));
  KRML_MAYBE_FOR16(i, 0U, 16U, 1U, qsqr(x6, x6););
  qmul(x6, x6, tmp1);
  memcpy(tmp1, x6, 4U * sizeof (uint64_t));
  for (uint32_t i = 0U; i < 64U; i++)
  {
    qsqr(tmp1, tmp1);
  }
  qmul(tmp1, tmp1, x6);
  for (uint32_t i = 0U; i < 32U; i++)
  {
    qsqr(tmp1, tmp1);
  }
  qmul(tmp1, tmp1, x6);
  KRML_MAYBE_FOR6(i, 0U, 6U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_101111);
  KRML_MAYBE_FOR5(i, 0U, 5U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_111);
  KRML_MAYBE_FOR4(i, 0U, 4U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_11);
  KRML_MAYBE_FOR5(i, 0U, 5U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_1111);
  KRML_MAYBE_FOR5(i, 0U, 5U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_10101);
  KRML_MAYBE_FOR4(i, 0U, 4U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_101);
  KRML_MAYBE_FOR3(i, 0U, 3U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_101);
  KRML_MAYBE_FOR3(i, 0U, 3U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_101);
  KRML_MAYBE_FOR5(i, 0U, 5U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_111);
  KRML_MAYBE_FOR9(i, 0U, 9U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_101111);
  KRML_MAYBE_FOR6(i, 0U, 6U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_1111);
  KRML_MAYBE_FOR2(i, 0U, 2U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, r);
  KRML_MAYBE_FOR5(i, 0U, 5U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, r);
  KRML_MAYBE_FOR6(i, 0U, 6U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_1111);
  KRML_MAYBE_FOR5(i, 0U, 5U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_111);
  KRML_MAYBE_FOR4(i, 0U, 4U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_111);
  KRML_MAYBE_FOR5(i, 0U, 5U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_111);
  KRML_MAYBE_FOR5(i, 0U, 5U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_101);
  KRML_MAYBE_FOR3(i, 0U, 3U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_11);
  KRML_MAYBE_FOR10(i, 0U, 10U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_101111);
  KRML_MAYBE_FOR2(i, 0U, 2U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_11);
  KRML_MAYBE_FOR5(i, 0U, 5U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_11);
  KRML_MAYBE_FOR5(i, 0U, 5U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_11);
  KRML_MAYBE_FOR3(i, 0U, 3U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, r);
  KRML_MAYBE_FOR7(i, 0U, 7U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_10101);
  KRML_MAYBE_FOR6(i, 0U, 6U, 1U, qsqr(tmp1, tmp1););
  qmul(tmp1, tmp1, x_1111);
  memcpy(x6, tmp1, 4U * sizeof (uint64_t));
  memcpy(res, x6, 4U * sizeof (uint64_t));
}

/* SNIPPET_END: qinv */

/* SNIPPET_START: qmul_mont */

static inline void qmul_mont(uint64_t *sinv, uint64_t *b, uint64_t *res)
{
  uint64_t tmp[4U] = { 0U };
  from_qmont(tmp, b);
  qmul(res, sinv, tmp);
}

/* SNIPPET_END: qmul_mont */

/* SNIPPET_START: ecdsa_verify_msg_as_qelem */

static inline bool
ecdsa_verify_msg_as_qelem(
  uint64_t *m_q,
  uint8_t *public_key,
  uint8_t *signature_r,
  uint8_t *signature_s
)
{
  uint64_t tmp[28U] = { 0U };
  uint64_t *pk = tmp;
  uint64_t *r_q = tmp + 12U;
  uint64_t *s_q = tmp + 16U;
  uint64_t *u1 = tmp + 20U;
  uint64_t *u2 = tmp + 24U;
  bool is_pk_valid = load_point_vartime(pk, public_key);
  bn_from_bytes_be4(r_q, signature_r);
  bn_from_bytes_be4(s_q, signature_s);
  uint64_t is_r_valid = bn_is_lt_order_and_gt_zero_mask4(r_q);
  uint64_t is_s_valid = bn_is_lt_order_and_gt_zero_mask4(s_q);
  bool is_rs_valid = is_r_valid == 0xFFFFFFFFFFFFFFFFULL && is_s_valid == 0xFFFFFFFFFFFFFFFFULL;
  if (!(is_pk_valid && is_rs_valid))
  {
    return false;
  }
  uint64_t sinv[4U] = { 0U };
  qinv(sinv, s_q);
  qmul_mont(sinv, m_q, u1);
  qmul_mont(sinv, r_q, u2);
  uint64_t res[12U] = { 0U };
  point_mul_double_g(res, u1, u2, pk);
  if (is_point_at_inf_vartime(res))
  {
    return false;
  }
  uint64_t x[4U] = { 0U };
  to_aff_point_x(x, res);
  qmod_short(x, x);
  bool res1 = bn_is_eq_vartime4(x, r_q);
  return res1;
}

/* SNIPPET_END: ecdsa_verify_msg_as_qelem */

/* SNIPPET_START: ecdsa_sign_msg_as_qelem */

static inline bool
ecdsa_sign_msg_as_qelem(
  uint8_t *signature,
  uint64_t *m_q,
  uint8_t *private_key,
  uint8_t *nonce
)
{
  uint64_t rsdk_q[16U] = { 0U };
  uint64_t *r_q = rsdk_q;
  uint64_t *s_q = rsdk_q + 4U;
  uint64_t *d_a = rsdk_q + 8U;
  uint64_t *k_q = rsdk_q + 12U;
  bn_from_bytes_be4(d_a, private_key);
  uint64_t is_b_valid0 = bn_is_lt_order_and_gt_zero_mask4(d_a);
  uint64_t oneq0[4U] = { 0U };
  oneq0[0U] = 1ULL;
  oneq0[1U] = 0ULL;
  oneq0[2U] = 0ULL;
  oneq0[3U] = 0ULL;
  KRML_MAYBE_FOR4(i,
    0U,
    4U,
    1U,
    uint64_t *os = d_a;
    uint64_t uu____0 = oneq0[i];
    uint64_t x = uu____0 ^ (is_b_valid0 & (d_a[i] ^ uu____0));
    os[i] = x;);
  uint64_t is_sk_valid = is_b_valid0;
  bn_from_bytes_be4(k_q, nonce);
  uint64_t is_b_valid = bn_is_lt_order_and_gt_zero_mask4(k_q);
  uint64_t oneq[4U] = { 0U };
  oneq[0U] = 1ULL;
  oneq[1U] = 0ULL;
  oneq[2U] = 0ULL;
  oneq[3U] = 0ULL;
  KRML_MAYBE_FOR4(i,
    0U,
    4U,
    1U,
    uint64_t *os = k_q;
    uint64_t uu____1 = oneq[i];
    uint64_t x = uu____1 ^ (is_b_valid & (k_q[i] ^ uu____1));
    os[i] = x;);
  uint64_t is_nonce_valid = is_b_valid;
  uint64_t are_sk_nonce_valid = is_sk_valid & is_nonce_valid;
  uint64_t p[12U] = { 0U };
  point_mul_g(p, k_q);
  to_aff_point_x(r_q, p);
  qmod_short(r_q, r_q);
  uint64_t kinv[4U] = { 0U };
  qinv(kinv, k_q);
  qmul(s_q, r_q, d_a);
  from_qmont(m_q, m_q);
  qadd(s_q, m_q, s_q);
  qmul(s_q, kinv, s_q);
  bn2_to_bytes_be4(signature, r_q, s_q);
  uint64_t is_r_zero = bn_is_zero_mask4(r_q);
  uint64_t is_s_zero = bn_is_zero_mask4(s_q);
  uint64_t m = are_sk_nonce_valid & (~is_r_zero & ~is_s_zero);
  bool res = m == 0xFFFFFFFFFFFFFFFFULL;
  return res;
}

/* SNIPPET_END: ecdsa_sign_msg_as_qelem */

/* SNIPPET_START: Hacl_P256_ecdsa_sign_p256_sha2 */


/*******************************************************************************

 Verified C library for ECDSA and ECDH functions over the P-256 NIST curve.

 This module implements signing and verification, key validation, conversions
 between various point representations, and ECDH key agreement.

*******************************************************************************/

/*****************/
/* ECDSA signing */
/*****************/

/*
  As per the standard, a hash function *shall* be used. Therefore, we recommend
  using one of the three combined hash-and-sign variants.
*/

/**
Create an ECDSA signature using SHA2-256.

  The function returns `true` for successful creation of an ECDSA signature and `false` otherwise.

  The outparam `signature` (R || S) points to 64 bytes of valid memory, i.e., uint8_t[64].
  The argument `msg` points to `msg_len` bytes of valid memory, i.e., uint8_t[msg_len].
  The arguments `private_key` and `nonce` point to 32 bytes of valid memory, i.e., uint8_t[32].

  The function also checks whether `private_key` and `nonce` are valid:
    • 0 < `private_key` < the order of the curve
    • 0 < `nonce` < the order of the curve
*/
bool
Hacl_P256_ecdsa_sign_p256_sha2(
  uint8_t *signature,
  uint32_t msg_len,
  uint8_t *msg,
  uint8_t *private_key,
  uint8_t *nonce
)
{
  uint64_t m_q[4U] = { 0U };
  uint8_t mHash[32U] = { 0U };
  Hacl_Hash_SHA2_hash_256(mHash, msg, msg_len);
  KRML_MAYBE_UNUSED_VAR(msg_len);
  uint8_t *mHash32 = mHash;
  bn_from_bytes_be4(m_q, mHash32);
  qmod_short(m_q, m_q);
  bool res = ecdsa_sign_msg_as_qelem(signature, m_q, private_key, nonce);
  return res;
}

/* SNIPPET_END: Hacl_P256_ecdsa_sign_p256_sha2 */

/* SNIPPET_START: Hacl_P256_ecdsa_sign_p256_sha384 */

/**
Create an ECDSA signature using SHA2-384.

  The function returns `true` for successful creation of an ECDSA signature and `false` otherwise.

  The outparam `signature` (R || S) points to 64 bytes of valid memory, i.e., uint8_t[64].
  The argument `msg` points to `msg_len` bytes of valid memory, i.e., uint8_t[msg_len].
  The arguments `private_key` and `nonce` point to 32 bytes of valid memory, i.e., uint8_t[32].

  The function also checks whether `private_key` and `nonce` are valid:
    • 0 < `private_key` < the order of the curve
    • 0 < `nonce` < the order of the curve
*/
bool
Hacl_P256_ecdsa_sign_p256_sha384(
  uint8_t *signature,
  uint32_t msg_len,
  uint8_t *msg,
  uint8_t *private_key,
  uint8_t *nonce
)
{
  uint64_t m_q[4U] = { 0U };
  uint8_t mHash[48U] = { 0U };
  Hacl_Hash_SHA2_hash_384(mHash, msg, msg_len);
  KRML_MAYBE_UNUSED_VAR(msg_len);
  uint8_t *mHash32 = mHash;
  bn_from_bytes_be4(m_q, mHash32);
  qmod_short(m_q, m_q);
  bool res = ecdsa_sign_msg_as_qelem(signature, m_q, private_key, nonce);
  return res;
}

/* SNIPPET_END: Hacl_P256_ecdsa_sign_p256_sha384 */

/* SNIPPET_START: Hacl_P256_ecdsa_sign_p256_sha512 */

/**
Create an ECDSA signature using SHA2-512.

  The function returns `true` for successful creation of an ECDSA signature and `false` otherwise.

  The outparam `signature` (R || S) points to 64 bytes of valid memory, i.e., uint8_t[64].
  The argument `msg` points to `msg_len` bytes of valid memory, i.e., uint8_t[msg_len].
  The arguments `private_key` and `nonce` point to 32 bytes of valid memory, i.e., uint8_t[32].

  The function also checks whether `private_key` and `nonce` are valid:
    • 0 < `private_key` < the order of the curve
    • 0 < `nonce` < the order of the curve
*/
bool
Hacl_P256_ecdsa_sign_p256_sha512(
  uint8_t *signature,
  uint32_t msg_len,
  uint8_t *msg,
  uint8_t *private_key,
  uint8_t *nonce
)
{
  uint64_t m_q[4U] = { 0U };
  uint8_t mHash[64U] = { 0U };
  Hacl_Hash_SHA2_hash_512(mHash, msg, msg_len);
  KRML_MAYBE_UNUSED_VAR(msg_len);
  uint8_t *mHash32 = mHash;
  bn_from_bytes_be4(m_q, mHash32);
  qmod_short(m_q, m_q);
  bool res = ecdsa_sign_msg_as_qelem(signature, m_q, private_key, nonce);
  return res;
}

/* SNIPPET_END: Hacl_P256_ecdsa_sign_p256_sha512 */

/* SNIPPET_START: Hacl_P256_ecdsa_sign_p256_without_hash */

/**
Create an ECDSA signature WITHOUT hashing first.

  This function is intended to receive a hash of the input.
  For convenience, we recommend using one of the hash-and-sign combined functions above.

  The argument `msg` MUST be at least 32 bytes (i.e. `msg_len >= 32`).

  NOTE: The equivalent functions in OpenSSL and Fiat-Crypto both accept inputs
  smaller than 32 bytes. These libraries left-pad the input with enough zeroes to
  reach the minimum 32 byte size. Clients who need behavior identical to OpenSSL
  need to perform the left-padding themselves.

  The function returns `true` for successful creation of an ECDSA signature and `false` otherwise.

  The outparam `signature` (R || S) points to 64 bytes of valid memory, i.e., uint8_t[64].
  The argument `msg` points to `msg_len` bytes of valid memory, i.e., uint8_t[msg_len].
  The arguments `private_key` and `nonce` point to 32 bytes of valid memory, i.e., uint8_t[32].

  The function also checks whether `private_key` and `nonce` are valid values:
    • 0 < `private_key` < the order of the curve
    • 0 < `nonce` < the order of the curve
*/
bool
Hacl_P256_ecdsa_sign_p256_without_hash(
  uint8_t *signature,
  uint32_t msg_len,
  uint8_t *msg,
  uint8_t *private_key,
  uint8_t *nonce
)
{
  uint64_t m_q[4U] = { 0U };
  uint8_t mHash[32U] = { 0U };
  memcpy(mHash, msg, 32U * sizeof (uint8_t));
  KRML_MAYBE_UNUSED_VAR(msg_len);
  uint8_t *mHash32 = mHash;
  bn_from_bytes_be4(m_q, mHash32);
  qmod_short(m_q, m_q);
  bool res = ecdsa_sign_msg_as_qelem(signature, m_q, private_key, nonce);
  return res;
}

/* SNIPPET_END: Hacl_P256_ecdsa_sign_p256_without_hash */

/* SNIPPET_START: Hacl_P256_ecdsa_verif_p256_sha2 */


/**********************/
/* ECDSA verification */
/**********************/

/**
Verify an ECDSA signature using SHA2-256.

  The function returns `true` if the signature is valid and `false` otherwise.

  The argument `msg` points to `msg_len` bytes of valid memory, i.e., uint8_t[msg_len].
  The argument `public_key` (x || y) points to 64 bytes of valid memory, i.e., uint8_t[64].
  The arguments `signature_r` and `signature_s` point to 32 bytes of valid memory, i.e., uint8_t[32].

  The function also checks whether `public_key` is valid
*/
bool
Hacl_P256_ecdsa_verif_p256_sha2(
  uint32_t msg_len,
  uint8_t *msg,
  uint8_t *public_key,
  uint8_t *signature_r,
  uint8_t *signature_s
)
{
  uint64_t m_q[4U] = { 0U };
  uint8_t mHash[32U] = { 0U };
  Hacl_Hash_SHA2_hash_256(mHash, msg, msg_len);
  KRML_MAYBE_UNUSED_VAR(msg_len);
  uint8_t *mHash32 = mHash;
  bn_from_bytes_be4(m_q, mHash32);
  qmod_short(m_q, m_q);
  bool res = ecdsa_verify_msg_as_qelem(m_q, public_key, signature_r, signature_s);
  return res;
}

/* SNIPPET_END: Hacl_P256_ecdsa_verif_p256_sha2 */

/* SNIPPET_START: Hacl_P256_ecdsa_verif_p256_sha384 */

/**
Verify an ECDSA signature using SHA2-384.

  The function returns `true` if the signature is valid and `false` otherwise.

  The argument `msg` points to `msg_len` bytes of valid memory, i.e., uint8_t[msg_len].
  The argument `public_key` (x || y) points to 64 bytes of valid memory, i.e., uint8_t[64].
  The arguments `signature_r` and `signature_s` point to 32 bytes of valid memory, i.e., uint8_t[32].

  The function also checks whether `public_key` is valid
*/
bool
Hacl_P256_ecdsa_verif_p256_sha384(
  uint32_t msg_len,
  uint8_t *msg,
  uint8_t *public_key,
  uint8_t *signature_r,
  uint8_t *signature_s
)
{
  uint64_t m_q[4U] = { 0U };
  uint8_t mHash[48U] = { 0U };
  Hacl_Hash_SHA2_hash_384(mHash, msg, msg_len);
  KRML_MAYBE_UNUSED_VAR(msg_len);
  uint8_t *mHash32 = mHash;
  bn_from_bytes_be4(m_q, mHash32);
  qmod_short(m_q, m_q);
  bool res = ecdsa_verify_msg_as_qelem(m_q, public_key, signature_r, signature_s);
  return res;
}

/* SNIPPET_END: Hacl_P256_ecdsa_verif_p256_sha384 */

/* SNIPPET_START: Hacl_P256_ecdsa_verif_p256_sha512 */

/**
Verify an ECDSA signature using SHA2-512.

  The function returns `true` if the signature is valid and `false` otherwise.

  The argument `msg` points to `msg_len` bytes of valid memory, i.e., uint8_t[msg_len].
  The argument `public_key` (x || y) points to 64 bytes of valid memory, i.e., uint8_t[64].
  The arguments `signature_r` and `signature_s` point to 32 bytes of valid memory, i.e., uint8_t[32].

  The function also checks whether `public_key` is valid
*/
bool
Hacl_P256_ecdsa_verif_p256_sha512(
  uint32_t msg_len,
  uint8_t *msg,
  uint8_t *public_key,
  uint8_t *signature_r,
  uint8_t *signature_s
)
{
  uint64_t m_q[4U] = { 0U };
  uint8_t mHash[64U] = { 0U };
  Hacl_Hash_SHA2_hash_512(mHash, msg, msg_len);
  KRML_MAYBE_UNUSED_VAR(msg_len);
  uint8_t *mHash32 = mHash;
  bn_from_bytes_be4(m_q, mHash32);
  qmod_short(m_q, m_q);
  bool res = ecdsa_verify_msg_as_qelem(m_q, public_key, signature_r, signature_s);
  return res;
}

/* SNIPPET_END: Hacl_P256_ecdsa_verif_p256_sha512 */

/* SNIPPET_START: Hacl_P256_ecdsa_verif_without_hash */

/**
Verify an ECDSA signature WITHOUT hashing first.

  This function is intended to receive a hash of the input.
  For convenience, we recommend using one of the hash-and-verify combined functions above.

  The argument `msg` MUST be at least 32 bytes (i.e. `msg_len >= 32`).

  The function returns `true` if the signature is valid and `false` otherwise.

  The argument `msg` points to `msg_len` bytes of valid memory, i.e., uint8_t[msg_len].
  The argument `public_key` (x || y) points to 64 bytes of valid memory, i.e., uint8_t[64].
  The arguments `signature_r` and `signature_s` point to 32 bytes of valid memory, i.e., uint8_t[32].

  The function also checks whether `public_key` is valid
*/
bool
Hacl_P256_ecdsa_verif_without_hash(
  uint32_t msg_len,
  uint8_t *msg,
  uint8_t *public_key,
  uint8_t *signature_r,
  uint8_t *signature_s
)
{
  uint64_t m_q[4U] = { 0U };
  uint8_t mHash[32U] = { 0U };
  memcpy(mHash, msg, 32U * sizeof (uint8_t));
  KRML_MAYBE_UNUSED_VAR(msg_len);
  uint8_t *mHash32 = mHash;
  bn_from_bytes_be4(m_q, mHash32);
  qmod_short(m_q, m_q);
  bool res = ecdsa_verify_msg_as_qelem(m_q, public_key, signature_r, signature_s);
  return res;
}

/* SNIPPET_END: Hacl_P256_ecdsa_verif_without_hash */

/* SNIPPET_START: Hacl_P256_validate_public_key */


/******************/
/* Key validation */
/******************/

/**
Public key validation.

  The function returns `true` if a public key is valid and `false` otherwise.

  The argument `public_key` points to 64 bytes of valid memory, i.e., uint8_t[64].

  The public key (x || y) is valid (with respect to SP 800-56A):
    • the public key is not the “point at infinity”, represented as O.
    • the affine x and y coordinates of the point represented by the public key are
      in the range [0, p – 1] where p is the prime defining the finite field.
    • y^2 = x^3 + ax + b where a and b are the coefficients of the curve equation.
  The last extract is taken from: https://neilmadden.blog/2017/05/17/so-how-do-you-validate-nist-ecdh-public-keys/
*/
bool Hacl_P256_validate_public_key(uint8_t *public_key)
{
  uint64_t point_jac[12U] = { 0U };
  bool res = load_point_vartime(point_jac, public_key);
  return res;
}

/* SNIPPET_END: Hacl_P256_validate_public_key */

/* SNIPPET_START: Hacl_P256_validate_private_key */

/**
Private key validation.

  The function returns `true` if a private key is valid and `false` otherwise.

  The argument `private_key` points to 32 bytes of valid memory, i.e., uint8_t[32].

  The private key is valid:
    • 0 < `private_key` < the order of the curve
*/
bool Hacl_P256_validate_private_key(uint8_t *private_key)
{
  uint64_t bn_sk[4U] = { 0U };
  bn_from_bytes_be4(bn_sk, private_key);
  uint64_t res = bn_is_lt_order_and_gt_zero_mask4(bn_sk);
  return res == 0xFFFFFFFFFFFFFFFFULL;
}

/* SNIPPET_END: Hacl_P256_validate_private_key */

/* SNIPPET_START: Hacl_P256_uncompressed_to_raw */

/*******************************************************************************
  Parsing and Serializing public keys.

  A public key is a point (x, y) on the P-256 NIST curve.

  The point can be represented in the following three ways.
    • raw          = [ x || y ], 64 bytes
    • uncompressed = [ 0x04 || x || y ], 65 bytes
    • compressed   = [ (0x02 for even `y` and 0x03 for odd `y`) || x ], 33 bytes

*******************************************************************************/


/**
Convert a public key from uncompressed to its raw form.

  The function returns `true` for successful conversion of a public key and `false` otherwise.

  The outparam `pk_raw` points to 64 bytes of valid memory, i.e., uint8_t[64].
  The argument `pk` points to 65 bytes of valid memory, i.e., uint8_t[65].

  The function DOESN'T check whether (x, y) is a valid point.
*/
bool Hacl_P256_uncompressed_to_raw(uint8_t *pk, uint8_t *pk_raw)
{
  uint8_t pk0 = pk[0U];
  if (pk0 != 0x04U)
  {
    return false;
  }
  memcpy(pk_raw, pk + 1U, 64U * sizeof (uint8_t));
  return true;
}

/* SNIPPET_END: Hacl_P256_uncompressed_to_raw */

/* SNIPPET_START: Hacl_P256_compressed_to_raw */

/**
Convert a public key from compressed to its raw form.

  The function returns `true` for successful conversion of a public key and `false` otherwise.

  The outparam `pk_raw` points to 64 bytes of valid memory, i.e., uint8_t[64].
  The argument `pk` points to 33 bytes of valid memory, i.e., uint8_t[33].

  The function also checks whether (x, y) is a valid point.
*/
bool Hacl_P256_compressed_to_raw(uint8_t *pk, uint8_t *pk_raw)
{
  uint64_t xa[4U] = { 0U };
  uint64_t ya[4U] = { 0U };
  uint8_t *pk_xb = pk + 1U;
  bool b = aff_point_decompress_vartime(xa, ya, pk);
  if (b)
  {
    memcpy(pk_raw, pk_xb, 32U * sizeof (uint8_t));
    bn_to_bytes_be4(pk_raw + 32U, ya);
  }
  return b;
}

/* SNIPPET_END: Hacl_P256_compressed_to_raw */

/* SNIPPET_START: Hacl_P256_raw_to_uncompressed */

/**
Convert a public key from raw to its uncompressed form.

  The outparam `pk` points to 65 bytes of valid memory, i.e., uint8_t[65].
  The argument `pk_raw` points to 64 bytes of valid memory, i.e., uint8_t[64].

  The function DOESN'T check whether (x, y) is a valid point.
*/
void Hacl_P256_raw_to_uncompressed(uint8_t *pk_raw, uint8_t *pk)
{
  pk[0U] = 0x04U;
  memcpy(pk + 1U, pk_raw, 64U * sizeof (uint8_t));
}

/* SNIPPET_END: Hacl_P256_raw_to_uncompressed */

/* SNIPPET_START: Hacl_P256_raw_to_compressed */

/**
Convert a public key from raw to its compressed form.

  The outparam `pk` points to 33 bytes of valid memory, i.e., uint8_t[33].
  The argument `pk_raw` points to 64 bytes of valid memory, i.e., uint8_t[64].

  The function DOESN'T check whether (x, y) is a valid point.
*/
void Hacl_P256_raw_to_compressed(uint8_t *pk_raw, uint8_t *pk)
{
  uint8_t *pk_x = pk_raw;
  uint8_t *pk_y = pk_raw + 32U;
  uint64_t bn_f[4U] = { 0U };
  bn_from_bytes_be4(bn_f, pk_y);
  uint64_t is_odd_f = bn_f[0U] & 1ULL;
  pk[0U] = (uint32_t)(uint8_t)is_odd_f + 0x02U;
  memcpy(pk + 1U, pk_x, 32U * sizeof (uint8_t));
}

/* SNIPPET_END: Hacl_P256_raw_to_compressed */

/* SNIPPET_START: Hacl_P256_dh_initiator */


/******************/
/* ECDH agreement */
/******************/

/**
Compute the public key from the private key.

  The function returns `true` if a private key is valid and `false` otherwise.

  The outparam `public_key`  points to 64 bytes of valid memory, i.e., uint8_t[64].
  The argument `private_key` points to 32 bytes of valid memory, i.e., uint8_t[32].

  The private key is valid:
    • 0 < `private_key` < the order of the curve.
*/
bool Hacl_P256_dh_initiator(uint8_t *public_key, uint8_t *private_key)
{
  return Hacl_Impl_P256_DH_ecp256dh_i(public_key, private_key);
}

/* SNIPPET_END: Hacl_P256_dh_initiator */

/* SNIPPET_START: Hacl_P256_dh_responder */

/**
Execute the diffie-hellmann key exchange.

  The function returns `true` for successful creation of an ECDH shared secret and
  `false` otherwise.

  The outparam `shared_secret` points to 64 bytes of valid memory, i.e., uint8_t[64].
  The argument `their_pubkey` points to 64 bytes of valid memory, i.e., uint8_t[64].
  The argument `private_key` points to 32 bytes of valid memory, i.e., uint8_t[32].

  The function also checks whether `private_key` and `their_pubkey` are valid.
*/
bool
Hacl_P256_dh_responder(uint8_t *shared_secret, uint8_t *their_pubkey, uint8_t *private_key)
{
  return Hacl_Impl_P256_DH_ecp256dh_r(shared_secret, their_pubkey, private_key);
}

/* SNIPPET_END: Hacl_P256_dh_responder */

