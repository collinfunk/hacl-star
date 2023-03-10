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


#include "Hacl_Streaming_Blake2b_256.h"

/**
  State allocation function when there is no key
*/
Hacl_Streaming_Blake2b_256_state_t *Hacl_Streaming_Blake2b_256_malloc(void)
{
  uint8_t *buf = (uint8_t *)KRML_HOST_CALLOC((uint32_t)128U, sizeof (uint8_t));
  Lib_IntVector_Intrinsics_vec256
  *wv =
    (Lib_IntVector_Intrinsics_vec256 *)KRML_ALIGNED_MALLOC(32,
      sizeof (Lib_IntVector_Intrinsics_vec256) * (uint32_t)4U);
  memset(wv, 0U, (uint32_t)4U * sizeof (Lib_IntVector_Intrinsics_vec256));
  Lib_IntVector_Intrinsics_vec256
  *b =
    (Lib_IntVector_Intrinsics_vec256 *)KRML_ALIGNED_MALLOC(32,
      sizeof (Lib_IntVector_Intrinsics_vec256) * (uint32_t)4U);
  memset(b, 0U, (uint32_t)4U * sizeof (Lib_IntVector_Intrinsics_vec256));
  Hacl_Streaming_Blake2b_256_block_state_t block_state = { .fst = wv, .snd = b };
  Hacl_Streaming_Blake2b_256_state_t
  s = { .block_state = block_state, .buf = buf, .total_len = (uint64_t)(uint32_t)0U };
  Hacl_Streaming_Blake2b_256_state_t
  *p =
    (Hacl_Streaming_Blake2b_256_state_t *)KRML_HOST_MALLOC(sizeof (
        Hacl_Streaming_Blake2b_256_state_t
      ));
  p[0U] = s;
  Hacl_Blake2b_256_init(block_state.snd, (uint32_t)0U, (uint32_t)64U);
  return p;
}

/**
  (Re-)initialization function when there is no key
*/
void Hacl_Streaming_Blake2b_256_reset(Hacl_Streaming_Blake2b_256_state_t *state)
{
  Hacl_Streaming_Blake2b_256_state_t scrut = *state;
  uint8_t *buf = scrut.buf;
  Hacl_Streaming_Blake2b_256_block_state_t block_state = scrut.block_state;
  Hacl_Blake2b_256_init(block_state.snd, (uint32_t)0U, (uint32_t)64U);
  Hacl_Streaming_Blake2b_256_state_t
  tmp = { .block_state = block_state, .buf = buf, .total_len = (uint64_t)(uint32_t)0U };
  state[0U] = tmp;
}

/**
  Update function when there is no key; 0 = success, 1 = max length exceeded
*/
uint32_t
Hacl_Streaming_Blake2b_256_update(
  Hacl_Streaming_Blake2b_256_state_t *state,
  uint8_t *chunk,
  uint32_t chunk_len
)
{
  Hacl_Streaming_Blake2b_256_state_t s = *state;
  uint64_t total_len = s.total_len;
  if ((uint64_t)chunk_len > (uint64_t)0xffffffffffffffffU - total_len)
  {
    return (uint32_t)1U;
  }
  uint32_t sz;
  if (total_len % (uint64_t)(uint32_t)128U == (uint64_t)0U && total_len > (uint64_t)0U)
  {
    sz = (uint32_t)128U;
  }
  else
  {
    sz = (uint32_t)(total_len % (uint64_t)(uint32_t)128U);
  }
  if (chunk_len <= (uint32_t)128U - sz)
  {
    Hacl_Streaming_Blake2b_256_state_t s1 = *state;
    Hacl_Streaming_Blake2b_256_block_state_t block_state1 = s1.block_state;
    uint8_t *buf = s1.buf;
    uint64_t total_len1 = s1.total_len;
    uint32_t sz1;
    if (total_len1 % (uint64_t)(uint32_t)128U == (uint64_t)0U && total_len1 > (uint64_t)0U)
    {
      sz1 = (uint32_t)128U;
    }
    else
    {
      sz1 = (uint32_t)(total_len1 % (uint64_t)(uint32_t)128U);
    }
    uint8_t *buf2 = buf + sz1;
    memcpy(buf2, chunk, chunk_len * sizeof (uint8_t));
    uint64_t total_len2 = total_len1 + (uint64_t)chunk_len;
    *state
    =
      (
        (Hacl_Streaming_Blake2b_256_state_t){
          .block_state = block_state1,
          .buf = buf,
          .total_len = total_len2
        }
      );
  }
  else if (sz == (uint32_t)0U)
  {
    Hacl_Streaming_Blake2b_256_state_t s1 = *state;
    Hacl_Streaming_Blake2b_256_block_state_t block_state1 = s1.block_state;
    uint8_t *buf = s1.buf;
    uint64_t total_len1 = s1.total_len;
    uint32_t sz1;
    if (total_len1 % (uint64_t)(uint32_t)128U == (uint64_t)0U && total_len1 > (uint64_t)0U)
    {
      sz1 = (uint32_t)128U;
    }
    else
    {
      sz1 = (uint32_t)(total_len1 % (uint64_t)(uint32_t)128U);
    }
    if (!(sz1 == (uint32_t)0U))
    {
      uint64_t prevlen = total_len1 - (uint64_t)sz1;
      Lib_IntVector_Intrinsics_vec256 *wv = block_state1.fst;
      Lib_IntVector_Intrinsics_vec256 *hash = block_state1.snd;
      uint32_t nb = (uint32_t)1U;
      Hacl_Blake2b_256_update_multi((uint32_t)128U,
        wv,
        hash,
        FStar_UInt128_uint64_to_uint128(prevlen),
        buf,
        nb);
    }
    uint32_t ite;
    if
    (
      (uint64_t)chunk_len
      % (uint64_t)(uint32_t)128U
      == (uint64_t)0U
      && (uint64_t)chunk_len > (uint64_t)0U
    )
    {
      ite = (uint32_t)128U;
    }
    else
    {
      ite = (uint32_t)((uint64_t)chunk_len % (uint64_t)(uint32_t)128U);
    }
    uint32_t n_blocks = (chunk_len - ite) / (uint32_t)128U;
    uint32_t data1_len = n_blocks * (uint32_t)128U;
    uint32_t data2_len = chunk_len - data1_len;
    uint8_t *data1 = chunk;
    uint8_t *data2 = chunk + data1_len;
    Lib_IntVector_Intrinsics_vec256 *wv = block_state1.fst;
    Lib_IntVector_Intrinsics_vec256 *hash = block_state1.snd;
    uint32_t nb = data1_len / (uint32_t)128U;
    Hacl_Blake2b_256_update_multi(data1_len,
      wv,
      hash,
      FStar_UInt128_uint64_to_uint128(total_len1),
      data1,
      nb);
    uint8_t *dst = buf;
    memcpy(dst, data2, data2_len * sizeof (uint8_t));
    *state
    =
      (
        (Hacl_Streaming_Blake2b_256_state_t){
          .block_state = block_state1,
          .buf = buf,
          .total_len = total_len1 + (uint64_t)chunk_len
        }
      );
  }
  else
  {
    uint32_t diff = (uint32_t)128U - sz;
    uint8_t *chunk1 = chunk;
    uint8_t *chunk2 = chunk + diff;
    Hacl_Streaming_Blake2b_256_state_t s1 = *state;
    Hacl_Streaming_Blake2b_256_block_state_t block_state10 = s1.block_state;
    uint8_t *buf0 = s1.buf;
    uint64_t total_len10 = s1.total_len;
    uint32_t sz10;
    if (total_len10 % (uint64_t)(uint32_t)128U == (uint64_t)0U && total_len10 > (uint64_t)0U)
    {
      sz10 = (uint32_t)128U;
    }
    else
    {
      sz10 = (uint32_t)(total_len10 % (uint64_t)(uint32_t)128U);
    }
    uint8_t *buf2 = buf0 + sz10;
    memcpy(buf2, chunk1, diff * sizeof (uint8_t));
    uint64_t total_len2 = total_len10 + (uint64_t)diff;
    *state
    =
      (
        (Hacl_Streaming_Blake2b_256_state_t){
          .block_state = block_state10,
          .buf = buf0,
          .total_len = total_len2
        }
      );
    Hacl_Streaming_Blake2b_256_state_t s10 = *state;
    Hacl_Streaming_Blake2b_256_block_state_t block_state1 = s10.block_state;
    uint8_t *buf = s10.buf;
    uint64_t total_len1 = s10.total_len;
    uint32_t sz1;
    if (total_len1 % (uint64_t)(uint32_t)128U == (uint64_t)0U && total_len1 > (uint64_t)0U)
    {
      sz1 = (uint32_t)128U;
    }
    else
    {
      sz1 = (uint32_t)(total_len1 % (uint64_t)(uint32_t)128U);
    }
    if (!(sz1 == (uint32_t)0U))
    {
      uint64_t prevlen = total_len1 - (uint64_t)sz1;
      Lib_IntVector_Intrinsics_vec256 *wv = block_state1.fst;
      Lib_IntVector_Intrinsics_vec256 *hash = block_state1.snd;
      uint32_t nb = (uint32_t)1U;
      Hacl_Blake2b_256_update_multi((uint32_t)128U,
        wv,
        hash,
        FStar_UInt128_uint64_to_uint128(prevlen),
        buf,
        nb);
    }
    uint32_t ite;
    if
    (
      (uint64_t)(chunk_len - diff)
      % (uint64_t)(uint32_t)128U
      == (uint64_t)0U
      && (uint64_t)(chunk_len - diff) > (uint64_t)0U
    )
    {
      ite = (uint32_t)128U;
    }
    else
    {
      ite = (uint32_t)((uint64_t)(chunk_len - diff) % (uint64_t)(uint32_t)128U);
    }
    uint32_t n_blocks = (chunk_len - diff - ite) / (uint32_t)128U;
    uint32_t data1_len = n_blocks * (uint32_t)128U;
    uint32_t data2_len = chunk_len - diff - data1_len;
    uint8_t *data1 = chunk2;
    uint8_t *data2 = chunk2 + data1_len;
    Lib_IntVector_Intrinsics_vec256 *wv = block_state1.fst;
    Lib_IntVector_Intrinsics_vec256 *hash = block_state1.snd;
    uint32_t nb = data1_len / (uint32_t)128U;
    Hacl_Blake2b_256_update_multi(data1_len,
      wv,
      hash,
      FStar_UInt128_uint64_to_uint128(total_len1),
      data1,
      nb);
    uint8_t *dst = buf;
    memcpy(dst, data2, data2_len * sizeof (uint8_t));
    *state
    =
      (
        (Hacl_Streaming_Blake2b_256_state_t){
          .block_state = block_state1,
          .buf = buf,
          .total_len = total_len1 + (uint64_t)(chunk_len - diff)
        }
      );
  }
  return (uint32_t)0U;
}

/**
  Finish function when there is no key
*/
void
Hacl_Streaming_Blake2b_256_digest(Hacl_Streaming_Blake2b_256_state_t *state, uint8_t *output)
{
  Hacl_Streaming_Blake2b_256_state_t scrut = *state;
  Hacl_Streaming_Blake2b_256_block_state_t block_state = scrut.block_state;
  uint8_t *buf_ = scrut.buf;
  uint64_t total_len = scrut.total_len;
  uint32_t r;
  if (total_len % (uint64_t)(uint32_t)128U == (uint64_t)0U && total_len > (uint64_t)0U)
  {
    r = (uint32_t)128U;
  }
  else
  {
    r = (uint32_t)(total_len % (uint64_t)(uint32_t)128U);
  }
  uint8_t *buf_1 = buf_;
  KRML_PRE_ALIGN(32) Lib_IntVector_Intrinsics_vec256 wv0[4U] KRML_POST_ALIGN(32) = { 0U };
  KRML_PRE_ALIGN(32) Lib_IntVector_Intrinsics_vec256 b[4U] KRML_POST_ALIGN(32) = { 0U };
  Hacl_Streaming_Blake2b_256_block_state_t tmp_block_state = { .fst = wv0, .snd = b };
  Lib_IntVector_Intrinsics_vec256 *src_b = block_state.snd;
  Lib_IntVector_Intrinsics_vec256 *dst_b = tmp_block_state.snd;
  memcpy(dst_b, src_b, (uint32_t)4U * sizeof (Lib_IntVector_Intrinsics_vec256));
  uint64_t prev_len = total_len - (uint64_t)r;
  uint32_t ite;
  if (r % (uint32_t)128U == (uint32_t)0U && r > (uint32_t)0U)
  {
    ite = (uint32_t)128U;
  }
  else
  {
    ite = r % (uint32_t)128U;
  }
  uint8_t *buf_last = buf_1 + r - ite;
  uint8_t *buf_multi = buf_1;
  Lib_IntVector_Intrinsics_vec256 *wv1 = tmp_block_state.fst;
  Lib_IntVector_Intrinsics_vec256 *hash0 = tmp_block_state.snd;
  uint32_t nb = (uint32_t)0U;
  Hacl_Blake2b_256_update_multi((uint32_t)0U,
    wv1,
    hash0,
    FStar_UInt128_uint64_to_uint128(prev_len),
    buf_multi,
    nb);
  uint64_t prev_len_last = total_len - (uint64_t)r;
  Lib_IntVector_Intrinsics_vec256 *wv = tmp_block_state.fst;
  Lib_IntVector_Intrinsics_vec256 *hash = tmp_block_state.snd;
  Hacl_Blake2b_256_update_last(r,
    wv,
    hash,
    FStar_UInt128_uint64_to_uint128(prev_len_last),
    r,
    buf_last);
  Hacl_Blake2b_256_finish((uint32_t)64U, output, tmp_block_state.snd);
}

/**
  Free state function when there is no key
*/
void Hacl_Streaming_Blake2b_256_free(Hacl_Streaming_Blake2b_256_state_t *state)
{
  Hacl_Streaming_Blake2b_256_state_t scrut = *state;
  uint8_t *buf = scrut.buf;
  Hacl_Streaming_Blake2b_256_block_state_t block_state = scrut.block_state;
  Lib_IntVector_Intrinsics_vec256 *wv = block_state.fst;
  Lib_IntVector_Intrinsics_vec256 *b = block_state.snd;
  KRML_ALIGNED_FREE(wv);
  KRML_ALIGNED_FREE(b);
  KRML_HOST_FREE(buf);
  KRML_HOST_FREE(state);
}

