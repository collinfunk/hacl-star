/* MIT License
 *
 * Copyright (c) 2016-2020 INRIA, CMU and Microsoft Corporation
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


#ifndef __Hacl_Impl_Sparkle1_H
#define __Hacl_Impl_Sparkle1_H
#include "kremlin/internal/types.h"
#include "kremlin/lowstar_endianness.h"
#include <string.h>


#include "Hacl_Impl_Sparkle.h"

extern uint32_t Hacl_Impl_Sparkle1_vsize_rcon;

extern const uint32_t Hacl_Impl_Sparkle1_rcon[8U];

typedef uint32_t Hacl_Impl_Sparkle1_branch_len;

Spec_SPARKLE2_branch1 Hacl_Impl_Sparkle1_xor(uint32_t l, uint32_t *b);

void
Hacl_Impl_Sparkle1_xor_x(uint32_t n, uint32_t *b, uint32_t lty, uint32_t ltx, uint32_t *temp);

void Hacl_Impl_Sparkle1_l_step(uint32_t n, uint32_t *m1, uint32_t i, uint32_t *right);

void Hacl_Impl_Sparkle1_l(uint32_t n, uint32_t *b);

void Hacl_Impl_Sparkle1_add2(uint32_t n, uint32_t i, uint32_t *b);

void Hacl_Impl_Sparkle1_toBranch(uint32_t n, uint8_t *i, uint32_t *o);

void Hacl_Impl_Sparkle1_fromBranch(uint32_t n, uint32_t *i, uint8_t *o);

void Hacl_Impl_Sparkle1_arx_n_step(uint32_t n, uint32_t i, uint32_t *b);

void Hacl_Impl_Sparkle1_sparkle256(uint32_t steps, uint8_t *i, uint8_t *o);

void Hacl_Impl_Sparkle1_sparkle384(uint32_t steps, uint8_t *i, uint8_t *o);

void Hacl_Impl_Sparkle1_sparkle512(uint32_t steps, uint8_t *i, uint8_t *o);


#define __Hacl_Impl_Sparkle1_H_DEFINED
#endif
