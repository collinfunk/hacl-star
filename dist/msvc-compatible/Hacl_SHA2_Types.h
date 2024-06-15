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


#ifndef __Hacl_SHA2_Types_H
#define __Hacl_SHA2_Types_H

#if defined(__cplusplus)
extern "C" {
#endif

#include <string.h>
#include "krml/internal/types.h"
#include "krml/lowstar_endianness.h"
#include "krml/internal/target.h"

#include "Hacl_Hash_SHA3_Simd256.h"

typedef K____uint8_t___uint8_t_ Hacl_Hash_SHA2_uint8_2p;

typedef K____uint8_t__K____uint8_t___uint8_t_ Hacl_Hash_SHA2_uint8_3p;

typedef K____uint8_t____uint8_t____K____uint8_t___uint8_t__ Hacl_Hash_SHA2_uint8_4p;

typedef uint8_t *Hacl_Hash_SHA2_bufx1;

typedef K____uint8_t____uint8_t____K____uint8_t___uint8_t__ Hacl_Hash_SHA2_bufx4;

#if defined(__cplusplus)
}
#endif

#define __Hacl_SHA2_Types_H_DEFINED
#endif
