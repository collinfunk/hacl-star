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


#ifndef __internal_EverCrypt_HMAC_H
#define __internal_EverCrypt_HMAC_H

#if defined(__cplusplus)
extern "C" {
#endif

#include "../EverCrypt_HMAC.h"

/* SNIPPET_START: EverCrypt_HMAC_compute_sha1 */

void
EverCrypt_HMAC_compute_sha1(
  uint8_t *dst,
  uint8_t *key,
  uint32_t key_len,
  uint8_t *data,
  uint32_t data_len
);

/* SNIPPET_END: EverCrypt_HMAC_compute_sha1 */

/* SNIPPET_START: EverCrypt_HMAC_compute_sha2_256 */

void
EverCrypt_HMAC_compute_sha2_256(
  uint8_t *dst,
  uint8_t *key,
  uint32_t key_len,
  uint8_t *data,
  uint32_t data_len
);

/* SNIPPET_END: EverCrypt_HMAC_compute_sha2_256 */

/* SNIPPET_START: EverCrypt_HMAC_compute_sha2_384 */

void
EverCrypt_HMAC_compute_sha2_384(
  uint8_t *dst,
  uint8_t *key,
  uint32_t key_len,
  uint8_t *data,
  uint32_t data_len
);

/* SNIPPET_END: EverCrypt_HMAC_compute_sha2_384 */

/* SNIPPET_START: EverCrypt_HMAC_compute_sha2_512 */

void
EverCrypt_HMAC_compute_sha2_512(
  uint8_t *dst,
  uint8_t *key,
  uint32_t key_len,
  uint8_t *data,
  uint32_t data_len
);

/* SNIPPET_END: EverCrypt_HMAC_compute_sha2_512 */

/* SNIPPET_START: EverCrypt_HMAC_compute_blake2s */

void
EverCrypt_HMAC_compute_blake2s(
  uint8_t *dst,
  uint8_t *key,
  uint32_t key_len,
  uint8_t *data,
  uint32_t data_len
);

/* SNIPPET_END: EverCrypt_HMAC_compute_blake2s */

/* SNIPPET_START: EverCrypt_HMAC_compute_blake2b */

void
EverCrypt_HMAC_compute_blake2b(
  uint8_t *dst,
  uint8_t *key,
  uint32_t key_len,
  uint8_t *data,
  uint32_t data_len
);

/* SNIPPET_END: EverCrypt_HMAC_compute_blake2b */

#if defined(__cplusplus)
}
#endif

#define __internal_EverCrypt_HMAC_H_DEFINED
#endif
