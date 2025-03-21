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


#ifndef __internal_Hacl_Spec_H
#define __internal_Hacl_Spec_H

#if defined(__cplusplus)
extern "C" {
#endif

#include "../Hacl_Spec.h"

/* SNIPPET_START: Spec_Cipher_Expansion_impl */

#define Spec_Cipher_Expansion_Hacl_CHACHA20 0
#define Spec_Cipher_Expansion_Vale_AES128 1
#define Spec_Cipher_Expansion_Vale_AES256 2

/* SNIPPET_END: Spec_Cipher_Expansion_impl */

typedef uint8_t Spec_Cipher_Expansion_impl;

/* SNIPPET_START: Spec_Frodo_Params_frodo_gen_a */

#define Spec_Frodo_Params_SHAKE128 0
#define Spec_Frodo_Params_AES128 1

/* SNIPPET_END: Spec_Frodo_Params_frodo_gen_a */

typedef uint8_t Spec_Frodo_Params_frodo_gen_a;

#if defined(__cplusplus)
}
#endif

#define __internal_Hacl_Spec_H_DEFINED
#endif
