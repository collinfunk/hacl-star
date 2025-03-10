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


#ifndef __internal_Hacl_Streaming_Types_H
#define __internal_Hacl_Streaming_Types_H

#if defined(__cplusplus)
extern "C" {
#endif

#include "../Hacl_Streaming_Types.h"
#include "libintvector.h"

/* SNIPPET_START: Hacl_Streaming_MD_state_32 */

typedef struct Hacl_Streaming_MD_state_32_s
{
  uint32_t *block_state;
  uint8_t *buf;
  uint64_t total_len;
}
Hacl_Streaming_MD_state_32;

/* SNIPPET_END: Hacl_Streaming_MD_state_32 */

/* SNIPPET_START: Hacl_Streaming_MD_state_64 */

typedef struct Hacl_Streaming_MD_state_64_s
{
  uint64_t *block_state;
  uint8_t *buf;
  uint64_t total_len;
}
Hacl_Streaming_MD_state_64;

/* SNIPPET_END: Hacl_Streaming_MD_state_64 */

/* SNIPPET_START: Hacl_Streaming_Blake2_Types_two_vec128 */

typedef struct Hacl_Streaming_Blake2_Types_two_vec128_s
{
  Lib_IntVector_Intrinsics_vec128 *fst;
  Lib_IntVector_Intrinsics_vec128 *snd;
}
Hacl_Streaming_Blake2_Types_two_vec128;

/* SNIPPET_END: Hacl_Streaming_Blake2_Types_two_vec128 */

/* SNIPPET_START: Hacl_Streaming_Blake2_Types_two_vec256 */

typedef struct Hacl_Streaming_Blake2_Types_two_vec256_s
{
  Lib_IntVector_Intrinsics_vec256 *fst;
  Lib_IntVector_Intrinsics_vec256 *snd;
}
Hacl_Streaming_Blake2_Types_two_vec256;

/* SNIPPET_END: Hacl_Streaming_Blake2_Types_two_vec256 */

/* SNIPPET_START: Hacl_Streaming_Blake2_Types_block_state_blake2b_32 */

typedef struct Hacl_Streaming_Blake2_Types_block_state_blake2b_32_s
{
  uint8_t fst;
  uint8_t snd;
  bool thd;
  K____uint64_t___uint64_t_ f3;
}
Hacl_Streaming_Blake2_Types_block_state_blake2b_32;

/* SNIPPET_END: Hacl_Streaming_Blake2_Types_block_state_blake2b_32 */

/* SNIPPET_START: Hacl_Streaming_Blake2_Types_block_state_blake2b_256 */

typedef struct Hacl_Streaming_Blake2_Types_block_state_blake2b_256_s
{
  uint8_t fst;
  uint8_t snd;
  bool thd;
  Hacl_Streaming_Blake2_Types_two_vec256 f3;
}
Hacl_Streaming_Blake2_Types_block_state_blake2b_256;

/* SNIPPET_END: Hacl_Streaming_Blake2_Types_block_state_blake2b_256 */

/* SNIPPET_START: K____uint32_t___uint32_t_ */

typedef struct K____uint32_t___uint32_t__s
{
  uint32_t *fst;
  uint32_t *snd;
}
K____uint32_t___uint32_t_;

/* SNIPPET_END: K____uint32_t___uint32_t_ */

/* SNIPPET_START: Hacl_Streaming_Blake2_Types_block_state_blake2s_32 */

typedef struct Hacl_Streaming_Blake2_Types_block_state_blake2s_32_s
{
  uint8_t fst;
  uint8_t snd;
  bool thd;
  K____uint32_t___uint32_t_ f3;
}
Hacl_Streaming_Blake2_Types_block_state_blake2s_32;

/* SNIPPET_END: Hacl_Streaming_Blake2_Types_block_state_blake2s_32 */

/* SNIPPET_START: Hacl_Streaming_Blake2_Types_block_state_blake2s_128 */

typedef struct Hacl_Streaming_Blake2_Types_block_state_blake2s_128_s
{
  uint8_t fst;
  uint8_t snd;
  bool thd;
  Hacl_Streaming_Blake2_Types_two_vec128 f3;
}
Hacl_Streaming_Blake2_Types_block_state_blake2s_128;

/* SNIPPET_END: Hacl_Streaming_Blake2_Types_block_state_blake2s_128 */

/* SNIPPET_START: Hacl_Streaming_Types_optional_32_tags */

#define Hacl_Streaming_Types_None 0
#define Hacl_Streaming_Types_Some 1

/* SNIPPET_END: Hacl_Streaming_Types_optional_32_tags */

typedef uint8_t Hacl_Streaming_Types_optional_32_tags;

/* SNIPPET_START: Hacl_Streaming_Types_optional_32 */

typedef struct Hacl_Streaming_Types_optional_32_s
{
  Hacl_Streaming_Types_optional_32_tags tag;
  uint32_t *v;
}
Hacl_Streaming_Types_optional_32;

/* SNIPPET_END: Hacl_Streaming_Types_optional_32 */

/* SNIPPET_START: Hacl_Streaming_Types_optional_64 */

typedef struct Hacl_Streaming_Types_optional_64_s
{
  Hacl_Streaming_Types_optional_32_tags tag;
  uint64_t *v;
}
Hacl_Streaming_Types_optional_64;

/* SNIPPET_END: Hacl_Streaming_Types_optional_64 */

/* SNIPPET_START: Hacl_Streaming_Types_optional_unit */

typedef Hacl_Streaming_Types_optional_32_tags Hacl_Streaming_Types_optional_unit;

/* SNIPPET_END: Hacl_Streaming_Types_optional_unit */

#if defined(__cplusplus)
}
#endif

#define __internal_Hacl_Streaming_Types_H_DEFINED
#endif
