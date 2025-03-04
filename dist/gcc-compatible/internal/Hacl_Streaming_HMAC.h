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


#ifndef __internal_Hacl_Streaming_HMAC_H
#define __internal_Hacl_Streaming_HMAC_H

#if defined(__cplusplus)
extern "C" {
#endif

#include <string.h>
#include "krml/internal/types.h"
#include "krml/lowstar_endianness.h"
#include "krml/internal/target.h"

#include "../Hacl_Streaming_HMAC.h"
#include "libintvector-shim.h"

#define Hacl_Agile_Hash_MD5_s 0
#define Hacl_Agile_Hash_SHA1_s 1
#define Hacl_Agile_Hash_SHA2_224_s 2
#define Hacl_Agile_Hash_SHA2_256_s 3
#define Hacl_Agile_Hash_SHA2_384_s 4
#define Hacl_Agile_Hash_SHA2_512_s 5
#define Hacl_Agile_Hash_SHA3_224_s 6
#define Hacl_Agile_Hash_SHA3_256_s 7
#define Hacl_Agile_Hash_SHA3_384_s 8
#define Hacl_Agile_Hash_SHA3_512_s 9
#define Hacl_Agile_Hash_Blake2S_s 10
#define Hacl_Agile_Hash_Blake2S_128_s 11
#define Hacl_Agile_Hash_Blake2B_s 12
#define Hacl_Agile_Hash_Blake2B_256_s 13

typedef uint8_t Hacl_Agile_Hash_state_s_tags;

typedef struct Hacl_Agile_Hash_state_s_s
{
  Hacl_Agile_Hash_state_s_tags tag;
  union {
    uint32_t *case_MD5_s;
    uint32_t *case_SHA1_s;
    uint32_t *case_SHA2_224_s;
    uint32_t *case_SHA2_256_s;
    uint64_t *case_SHA2_384_s;
    uint64_t *case_SHA2_512_s;
    uint64_t *case_SHA3_224_s;
    uint64_t *case_SHA3_256_s;
    uint64_t *case_SHA3_384_s;
    uint64_t *case_SHA3_512_s;
    uint32_t *case_Blake2S_s;
    Lib_IntVector_Intrinsics_vec128 *case_Blake2S_128_s;
    uint64_t *case_Blake2B_s;
    Lib_IntVector_Intrinsics_vec256 *case_Blake2B_256_s;
  }
  ;
}
Hacl_Agile_Hash_state_s;

typedef struct Hacl_Streaming_HMAC_agile_state_s
{
  Hacl_Streaming_HMAC_Definitions_two_state block_state;
  uint8_t *buf;
  uint64_t total_len;
}
Hacl_Streaming_HMAC_agile_state;

#if defined(__cplusplus)
}
#endif

#define __internal_Hacl_Streaming_HMAC_H_DEFINED
#endif
