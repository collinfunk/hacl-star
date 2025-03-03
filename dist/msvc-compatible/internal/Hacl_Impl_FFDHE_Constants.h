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


#ifndef __internal_Hacl_Impl_FFDHE_Constants_H
#define __internal_Hacl_Impl_FFDHE_Constants_H

#if defined(__cplusplus)
extern "C" {
#endif

static const uint8_t Hacl_Impl_FFDHE_Constants_ffdhe_g2[1U] = { 0x02U };

static const
uint8_t
Hacl_Impl_FFDHE_Constants_ffdhe_p2048[256U] =
  {
    0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xADU, 0xF8U, 0x54U, 0x58U, 0xA2U,
    0xBBU, 0x4AU, 0x9AU, 0xAFU, 0xDCU, 0x56U, 0x20U, 0x27U, 0x3DU, 0x3CU, 0xF1U, 0xD8U, 0xB9U,
    0xC5U, 0x83U, 0xCEU, 0x2DU, 0x36U, 0x95U, 0xA9U, 0xE1U, 0x36U, 0x41U, 0x14U, 0x64U, 0x33U,
    0xFBU, 0xCCU, 0x93U, 0x9DU, 0xCEU, 0x24U, 0x9BU, 0x3EU, 0xF9U, 0x7DU, 0x2FU, 0xE3U, 0x63U,
    0x63U, 0x0CU, 0x75U, 0xD8U, 0xF6U, 0x81U, 0xB2U, 0x02U, 0xAEU, 0xC4U, 0x61U, 0x7AU, 0xD3U,
    0xDFU, 0x1EU, 0xD5U, 0xD5U, 0xFDU, 0x65U, 0x61U, 0x24U, 0x33U, 0xF5U, 0x1FU, 0x5FU, 0x06U,
    0x6EU, 0xD0U, 0x85U, 0x63U, 0x65U, 0x55U, 0x3DU, 0xEDU, 0x1AU, 0xF3U, 0xB5U, 0x57U, 0x13U,
    0x5EU, 0x7FU, 0x57U, 0xC9U, 0x35U, 0x98U, 0x4FU, 0x0CU, 0x70U, 0xE0U, 0xE6U, 0x8BU, 0x77U,
    0xE2U, 0xA6U, 0x89U, 0xDAU, 0xF3U, 0xEFU, 0xE8U, 0x72U, 0x1DU, 0xF1U, 0x58U, 0xA1U, 0x36U,
    0xADU, 0xE7U, 0x35U, 0x30U, 0xACU, 0xCAU, 0x4FU, 0x48U, 0x3AU, 0x79U, 0x7AU, 0xBCU, 0x0AU,
    0xB1U, 0x82U, 0xB3U, 0x24U, 0xFBU, 0x61U, 0xD1U, 0x08U, 0xA9U, 0x4BU, 0xB2U, 0xC8U, 0xE3U,
    0xFBU, 0xB9U, 0x6AU, 0xDAU, 0xB7U, 0x60U, 0xD7U, 0xF4U, 0x68U, 0x1DU, 0x4FU, 0x42U, 0xA3U,
    0xDEU, 0x39U, 0x4DU, 0xF4U, 0xAEU, 0x56U, 0xEDU, 0xE7U, 0x63U, 0x72U, 0xBBU, 0x19U, 0x0BU,
    0x07U, 0xA7U, 0xC8U, 0xEEU, 0x0AU, 0x6DU, 0x70U, 0x9EU, 0x02U, 0xFCU, 0xE1U, 0xCDU, 0xF7U,
    0xE2U, 0xECU, 0xC0U, 0x34U, 0x04U, 0xCDU, 0x28U, 0x34U, 0x2FU, 0x61U, 0x91U, 0x72U, 0xFEU,
    0x9CU, 0xE9U, 0x85U, 0x83U, 0xFFU, 0x8EU, 0x4FU, 0x12U, 0x32U, 0xEEU, 0xF2U, 0x81U, 0x83U,
    0xC3U, 0xFEU, 0x3BU, 0x1BU, 0x4CU, 0x6FU, 0xADU, 0x73U, 0x3BU, 0xB5U, 0xFCU, 0xBCU, 0x2EU,
    0xC2U, 0x20U, 0x05U, 0xC5U, 0x8EU, 0xF1U, 0x83U, 0x7DU, 0x16U, 0x83U, 0xB2U, 0xC6U, 0xF3U,
    0x4AU, 0x26U, 0xC1U, 0xB2U, 0xEFU, 0xFAU, 0x88U, 0x6BU, 0x42U, 0x38U, 0x61U, 0x28U, 0x5CU,
    0x97U, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU
  };

static const
uint8_t
Hacl_Impl_FFDHE_Constants_ffdhe_p3072[384U] =
  {
    0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xADU, 0xF8U, 0x54U, 0x58U, 0xA2U,
    0xBBU, 0x4AU, 0x9AU, 0xAFU, 0xDCU, 0x56U, 0x20U, 0x27U, 0x3DU, 0x3CU, 0xF1U, 0xD8U, 0xB9U,
    0xC5U, 0x83U, 0xCEU, 0x2DU, 0x36U, 0x95U, 0xA9U, 0xE1U, 0x36U, 0x41U, 0x14U, 0x64U, 0x33U,
    0xFBU, 0xCCU, 0x93U, 0x9DU, 0xCEU, 0x24U, 0x9BU, 0x3EU, 0xF9U, 0x7DU, 0x2FU, 0xE3U, 0x63U,
    0x63U, 0x0CU, 0x75U, 0xD8U, 0xF6U, 0x81U, 0xB2U, 0x02U, 0xAEU, 0xC4U, 0x61U, 0x7AU, 0xD3U,
    0xDFU, 0x1EU, 0xD5U, 0xD5U, 0xFDU, 0x65U, 0x61U, 0x24U, 0x33U, 0xF5U, 0x1FU, 0x5FU, 0x06U,
    0x6EU, 0xD0U, 0x85U, 0x63U, 0x65U, 0x55U, 0x3DU, 0xEDU, 0x1AU, 0xF3U, 0xB5U, 0x57U, 0x13U,
    0x5EU, 0x7FU, 0x57U, 0xC9U, 0x35U, 0x98U, 0x4FU, 0x0CU, 0x70U, 0xE0U, 0xE6U, 0x8BU, 0x77U,
    0xE2U, 0xA6U, 0x89U, 0xDAU, 0xF3U, 0xEFU, 0xE8U, 0x72U, 0x1DU, 0xF1U, 0x58U, 0xA1U, 0x36U,
    0xADU, 0xE7U, 0x35U, 0x30U, 0xACU, 0xCAU, 0x4FU, 0x48U, 0x3AU, 0x79U, 0x7AU, 0xBCU, 0x0AU,
    0xB1U, 0x82U, 0xB3U, 0x24U, 0xFBU, 0x61U, 0xD1U, 0x08U, 0xA9U, 0x4BU, 0xB2U, 0xC8U, 0xE3U,
    0xFBU, 0xB9U, 0x6AU, 0xDAU, 0xB7U, 0x60U, 0xD7U, 0xF4U, 0x68U, 0x1DU, 0x4FU, 0x42U, 0xA3U,
    0xDEU, 0x39U, 0x4DU, 0xF4U, 0xAEU, 0x56U, 0xEDU, 0xE7U, 0x63U, 0x72U, 0xBBU, 0x19U, 0x0BU,
    0x07U, 0xA7U, 0xC8U, 0xEEU, 0x0AU, 0x6DU, 0x70U, 0x9EU, 0x02U, 0xFCU, 0xE1U, 0xCDU, 0xF7U,
    0xE2U, 0xECU, 0xC0U, 0x34U, 0x04U, 0xCDU, 0x28U, 0x34U, 0x2FU, 0x61U, 0x91U, 0x72U, 0xFEU,
    0x9CU, 0xE9U, 0x85U, 0x83U, 0xFFU, 0x8EU, 0x4FU, 0x12U, 0x32U, 0xEEU, 0xF2U, 0x81U, 0x83U,
    0xC3U, 0xFEU, 0x3BU, 0x1BU, 0x4CU, 0x6FU, 0xADU, 0x73U, 0x3BU, 0xB5U, 0xFCU, 0xBCU, 0x2EU,
    0xC2U, 0x20U, 0x05U, 0xC5U, 0x8EU, 0xF1U, 0x83U, 0x7DU, 0x16U, 0x83U, 0xB2U, 0xC6U, 0xF3U,
    0x4AU, 0x26U, 0xC1U, 0xB2U, 0xEFU, 0xFAU, 0x88U, 0x6BU, 0x42U, 0x38U, 0x61U, 0x1FU, 0xCFU,
    0xDCU, 0xDEU, 0x35U, 0x5BU, 0x3BU, 0x65U, 0x19U, 0x03U, 0x5BU, 0xBCU, 0x34U, 0xF4U, 0xDEU,
    0xF9U, 0x9CU, 0x02U, 0x38U, 0x61U, 0xB4U, 0x6FU, 0xC9U, 0xD6U, 0xE6U, 0xC9U, 0x07U, 0x7AU,
    0xD9U, 0x1DU, 0x26U, 0x91U, 0xF7U, 0xF7U, 0xEEU, 0x59U, 0x8CU, 0xB0U, 0xFAU, 0xC1U, 0x86U,
    0xD9U, 0x1CU, 0xAEU, 0xFEU, 0x13U, 0x09U, 0x85U, 0x13U, 0x92U, 0x70U, 0xB4U, 0x13U, 0x0CU,
    0x93U, 0xBCU, 0x43U, 0x79U, 0x44U, 0xF4U, 0xFDU, 0x44U, 0x52U, 0xE2U, 0xD7U, 0x4DU, 0xD3U,
    0x64U, 0xF2U, 0xE2U, 0x1EU, 0x71U, 0xF5U, 0x4BU, 0xFFU, 0x5CU, 0xAEU, 0x82U, 0xABU, 0x9CU,
    0x9DU, 0xF6U, 0x9EU, 0xE8U, 0x6DU, 0x2BU, 0xC5U, 0x22U, 0x36U, 0x3AU, 0x0DU, 0xABU, 0xC5U,
    0x21U, 0x97U, 0x9BU, 0x0DU, 0xEAU, 0xDAU, 0x1DU, 0xBFU, 0x9AU, 0x42U, 0xD5U, 0xC4U, 0x48U,
    0x4EU, 0x0AU, 0xBCU, 0xD0U, 0x6BU, 0xFAU, 0x53U, 0xDDU, 0xEFU, 0x3CU, 0x1BU, 0x20U, 0xEEU,
    0x3FU, 0xD5U, 0x9DU, 0x7CU, 0x25U, 0xE4U, 0x1DU, 0x2BU, 0x66U, 0xC6U, 0x2EU, 0x37U, 0xFFU,
    0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU
  };

static const
uint8_t
Hacl_Impl_FFDHE_Constants_ffdhe_p4096[512U] =
  {
    0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xADU, 0xF8U, 0x54U, 0x58U, 0xA2U,
    0xBBU, 0x4AU, 0x9AU, 0xAFU, 0xDCU, 0x56U, 0x20U, 0x27U, 0x3DU, 0x3CU, 0xF1U, 0xD8U, 0xB9U,
    0xC5U, 0x83U, 0xCEU, 0x2DU, 0x36U, 0x95U, 0xA9U, 0xE1U, 0x36U, 0x41U, 0x14U, 0x64U, 0x33U,
    0xFBU, 0xCCU, 0x93U, 0x9DU, 0xCEU, 0x24U, 0x9BU, 0x3EU, 0xF9U, 0x7DU, 0x2FU, 0xE3U, 0x63U,
    0x63U, 0x0CU, 0x75U, 0xD8U, 0xF6U, 0x81U, 0xB2U, 0x02U, 0xAEU, 0xC4U, 0x61U, 0x7AU, 0xD3U,
    0xDFU, 0x1EU, 0xD5U, 0xD5U, 0xFDU, 0x65U, 0x61U, 0x24U, 0x33U, 0xF5U, 0x1FU, 0x5FU, 0x06U,
    0x6EU, 0xD0U, 0x85U, 0x63U, 0x65U, 0x55U, 0x3DU, 0xEDU, 0x1AU, 0xF3U, 0xB5U, 0x57U, 0x13U,
    0x5EU, 0x7FU, 0x57U, 0xC9U, 0x35U, 0x98U, 0x4FU, 0x0CU, 0x70U, 0xE0U, 0xE6U, 0x8BU, 0x77U,
    0xE2U, 0xA6U, 0x89U, 0xDAU, 0xF3U, 0xEFU, 0xE8U, 0x72U, 0x1DU, 0xF1U, 0x58U, 0xA1U, 0x36U,
    0xADU, 0xE7U, 0x35U, 0x30U, 0xACU, 0xCAU, 0x4FU, 0x48U, 0x3AU, 0x79U, 0x7AU, 0xBCU, 0x0AU,
    0xB1U, 0x82U, 0xB3U, 0x24U, 0xFBU, 0x61U, 0xD1U, 0x08U, 0xA9U, 0x4BU, 0xB2U, 0xC8U, 0xE3U,
    0xFBU, 0xB9U, 0x6AU, 0xDAU, 0xB7U, 0x60U, 0xD7U, 0xF4U, 0x68U, 0x1DU, 0x4FU, 0x42U, 0xA3U,
    0xDEU, 0x39U, 0x4DU, 0xF4U, 0xAEU, 0x56U, 0xEDU, 0xE7U, 0x63U, 0x72U, 0xBBU, 0x19U, 0x0BU,
    0x07U, 0xA7U, 0xC8U, 0xEEU, 0x0AU, 0x6DU, 0x70U, 0x9EU, 0x02U, 0xFCU, 0xE1U, 0xCDU, 0xF7U,
    0xE2U, 0xECU, 0xC0U, 0x34U, 0x04U, 0xCDU, 0x28U, 0x34U, 0x2FU, 0x61U, 0x91U, 0x72U, 0xFEU,
    0x9CU, 0xE9U, 0x85U, 0x83U, 0xFFU, 0x8EU, 0x4FU, 0x12U, 0x32U, 0xEEU, 0xF2U, 0x81U, 0x83U,
    0xC3U, 0xFEU, 0x3BU, 0x1BU, 0x4CU, 0x6FU, 0xADU, 0x73U, 0x3BU, 0xB5U, 0xFCU, 0xBCU, 0x2EU,
    0xC2U, 0x20U, 0x05U, 0xC5U, 0x8EU, 0xF1U, 0x83U, 0x7DU, 0x16U, 0x83U, 0xB2U, 0xC6U, 0xF3U,
    0x4AU, 0x26U, 0xC1U, 0xB2U, 0xEFU, 0xFAU, 0x88U, 0x6BU, 0x42U, 0x38U, 0x61U, 0x1FU, 0xCFU,
    0xDCU, 0xDEU, 0x35U, 0x5BU, 0x3BU, 0x65U, 0x19U, 0x03U, 0x5BU, 0xBCU, 0x34U, 0xF4U, 0xDEU,
    0xF9U, 0x9CU, 0x02U, 0x38U, 0x61U, 0xB4U, 0x6FU, 0xC9U, 0xD6U, 0xE6U, 0xC9U, 0x07U, 0x7AU,
    0xD9U, 0x1DU, 0x26U, 0x91U, 0xF7U, 0xF7U, 0xEEU, 0x59U, 0x8CU, 0xB0U, 0xFAU, 0xC1U, 0x86U,
    0xD9U, 0x1CU, 0xAEU, 0xFEU, 0x13U, 0x09U, 0x85U, 0x13U, 0x92U, 0x70U, 0xB4U, 0x13U, 0x0CU,
    0x93U, 0xBCU, 0x43U, 0x79U, 0x44U, 0xF4U, 0xFDU, 0x44U, 0x52U, 0xE2U, 0xD7U, 0x4DU, 0xD3U,
    0x64U, 0xF2U, 0xE2U, 0x1EU, 0x71U, 0xF5U, 0x4BU, 0xFFU, 0x5CU, 0xAEU, 0x82U, 0xABU, 0x9CU,
    0x9DU, 0xF6U, 0x9EU, 0xE8U, 0x6DU, 0x2BU, 0xC5U, 0x22U, 0x36U, 0x3AU, 0x0DU, 0xABU, 0xC5U,
    0x21U, 0x97U, 0x9BU, 0x0DU, 0xEAU, 0xDAU, 0x1DU, 0xBFU, 0x9AU, 0x42U, 0xD5U, 0xC4U, 0x48U,
    0x4EU, 0x0AU, 0xBCU, 0xD0U, 0x6BU, 0xFAU, 0x53U, 0xDDU, 0xEFU, 0x3CU, 0x1BU, 0x20U, 0xEEU,
    0x3FU, 0xD5U, 0x9DU, 0x7CU, 0x25U, 0xE4U, 0x1DU, 0x2BU, 0x66U, 0x9EU, 0x1EU, 0xF1U, 0x6EU,
    0x6FU, 0x52U, 0xC3U, 0x16U, 0x4DU, 0xF4U, 0xFBU, 0x79U, 0x30U, 0xE9U, 0xE4U, 0xE5U, 0x88U,
    0x57U, 0xB6U, 0xACU, 0x7DU, 0x5FU, 0x42U, 0xD6U, 0x9FU, 0x6DU, 0x18U, 0x77U, 0x63U, 0xCFU,
    0x1DU, 0x55U, 0x03U, 0x40U, 0x04U, 0x87U, 0xF5U, 0x5BU, 0xA5U, 0x7EU, 0x31U, 0xCCU, 0x7AU,
    0x71U, 0x35U, 0xC8U, 0x86U, 0xEFU, 0xB4U, 0x31U, 0x8AU, 0xEDU, 0x6AU, 0x1EU, 0x01U, 0x2DU,
    0x9EU, 0x68U, 0x32U, 0xA9U, 0x07U, 0x60U, 0x0AU, 0x91U, 0x81U, 0x30U, 0xC4U, 0x6DU, 0xC7U,
    0x78U, 0xF9U, 0x71U, 0xADU, 0x00U, 0x38U, 0x09U, 0x29U, 0x99U, 0xA3U, 0x33U, 0xCBU, 0x8BU,
    0x7AU, 0x1AU, 0x1DU, 0xB9U, 0x3DU, 0x71U, 0x40U, 0x00U, 0x3CU, 0x2AU, 0x4EU, 0xCEU, 0xA9U,
    0xF9U, 0x8DU, 0x0AU, 0xCCU, 0x0AU, 0x82U, 0x91U, 0xCDU, 0xCEU, 0xC9U, 0x7DU, 0xCFU, 0x8EU,
    0xC9U, 0xB5U, 0x5AU, 0x7FU, 0x88U, 0xA4U, 0x6BU, 0x4DU, 0xB5U, 0xA8U, 0x51U, 0xF4U, 0x41U,
    0x82U, 0xE1U, 0xC6U, 0x8AU, 0x00U, 0x7EU, 0x5EU, 0x65U, 0x5FU, 0x6AU, 0xFFU, 0xFFU, 0xFFU,
    0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU
  };

static const
uint8_t
Hacl_Impl_FFDHE_Constants_ffdhe_p6144[768U] =
  {
    0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xADU, 0xF8U, 0x54U, 0x58U, 0xA2U,
    0xBBU, 0x4AU, 0x9AU, 0xAFU, 0xDCU, 0x56U, 0x20U, 0x27U, 0x3DU, 0x3CU, 0xF1U, 0xD8U, 0xB9U,
    0xC5U, 0x83U, 0xCEU, 0x2DU, 0x36U, 0x95U, 0xA9U, 0xE1U, 0x36U, 0x41U, 0x14U, 0x64U, 0x33U,
    0xFBU, 0xCCU, 0x93U, 0x9DU, 0xCEU, 0x24U, 0x9BU, 0x3EU, 0xF9U, 0x7DU, 0x2FU, 0xE3U, 0x63U,
    0x63U, 0x0CU, 0x75U, 0xD8U, 0xF6U, 0x81U, 0xB2U, 0x02U, 0xAEU, 0xC4U, 0x61U, 0x7AU, 0xD3U,
    0xDFU, 0x1EU, 0xD5U, 0xD5U, 0xFDU, 0x65U, 0x61U, 0x24U, 0x33U, 0xF5U, 0x1FU, 0x5FU, 0x06U,
    0x6EU, 0xD0U, 0x85U, 0x63U, 0x65U, 0x55U, 0x3DU, 0xEDU, 0x1AU, 0xF3U, 0xB5U, 0x57U, 0x13U,
    0x5EU, 0x7FU, 0x57U, 0xC9U, 0x35U, 0x98U, 0x4FU, 0x0CU, 0x70U, 0xE0U, 0xE6U, 0x8BU, 0x77U,
    0xE2U, 0xA6U, 0x89U, 0xDAU, 0xF3U, 0xEFU, 0xE8U, 0x72U, 0x1DU, 0xF1U, 0x58U, 0xA1U, 0x36U,
    0xADU, 0xE7U, 0x35U, 0x30U, 0xACU, 0xCAU, 0x4FU, 0x48U, 0x3AU, 0x79U, 0x7AU, 0xBCU, 0x0AU,
    0xB1U, 0x82U, 0xB3U, 0x24U, 0xFBU, 0x61U, 0xD1U, 0x08U, 0xA9U, 0x4BU, 0xB2U, 0xC8U, 0xE3U,
    0xFBU, 0xB9U, 0x6AU, 0xDAU, 0xB7U, 0x60U, 0xD7U, 0xF4U, 0x68U, 0x1DU, 0x4FU, 0x42U, 0xA3U,
    0xDEU, 0x39U, 0x4DU, 0xF4U, 0xAEU, 0x56U, 0xEDU, 0xE7U, 0x63U, 0x72U, 0xBBU, 0x19U, 0x0BU,
    0x07U, 0xA7U, 0xC8U, 0xEEU, 0x0AU, 0x6DU, 0x70U, 0x9EU, 0x02U, 0xFCU, 0xE1U, 0xCDU, 0xF7U,
    0xE2U, 0xECU, 0xC0U, 0x34U, 0x04U, 0xCDU, 0x28U, 0x34U, 0x2FU, 0x61U, 0x91U, 0x72U, 0xFEU,
    0x9CU, 0xE9U, 0x85U, 0x83U, 0xFFU, 0x8EU, 0x4FU, 0x12U, 0x32U, 0xEEU, 0xF2U, 0x81U, 0x83U,
    0xC3U, 0xFEU, 0x3BU, 0x1BU, 0x4CU, 0x6FU, 0xADU, 0x73U, 0x3BU, 0xB5U, 0xFCU, 0xBCU, 0x2EU,
    0xC2U, 0x20U, 0x05U, 0xC5U, 0x8EU, 0xF1U, 0x83U, 0x7DU, 0x16U, 0x83U, 0xB2U, 0xC6U, 0xF3U,
    0x4AU, 0x26U, 0xC1U, 0xB2U, 0xEFU, 0xFAU, 0x88U, 0x6BU, 0x42U, 0x38U, 0x61U, 0x1FU, 0xCFU,
    0xDCU, 0xDEU, 0x35U, 0x5BU, 0x3BU, 0x65U, 0x19U, 0x03U, 0x5BU, 0xBCU, 0x34U, 0xF4U, 0xDEU,
    0xF9U, 0x9CU, 0x02U, 0x38U, 0x61U, 0xB4U, 0x6FU, 0xC9U, 0xD6U, 0xE6U, 0xC9U, 0x07U, 0x7AU,
    0xD9U, 0x1DU, 0x26U, 0x91U, 0xF7U, 0xF7U, 0xEEU, 0x59U, 0x8CU, 0xB0U, 0xFAU, 0xC1U, 0x86U,
    0xD9U, 0x1CU, 0xAEU, 0xFEU, 0x13U, 0x09U, 0x85U, 0x13U, 0x92U, 0x70U, 0xB4U, 0x13U, 0x0CU,
    0x93U, 0xBCU, 0x43U, 0x79U, 0x44U, 0xF4U, 0xFDU, 0x44U, 0x52U, 0xE2U, 0xD7U, 0x4DU, 0xD3U,
    0x64U, 0xF2U, 0xE2U, 0x1EU, 0x71U, 0xF5U, 0x4BU, 0xFFU, 0x5CU, 0xAEU, 0x82U, 0xABU, 0x9CU,
    0x9DU, 0xF6U, 0x9EU, 0xE8U, 0x6DU, 0x2BU, 0xC5U, 0x22U, 0x36U, 0x3AU, 0x0DU, 0xABU, 0xC5U,
    0x21U, 0x97U, 0x9BU, 0x0DU, 0xEAU, 0xDAU, 0x1DU, 0xBFU, 0x9AU, 0x42U, 0xD5U, 0xC4U, 0x48U,
    0x4EU, 0x0AU, 0xBCU, 0xD0U, 0x6BU, 0xFAU, 0x53U, 0xDDU, 0xEFU, 0x3CU, 0x1BU, 0x20U, 0xEEU,
    0x3FU, 0xD5U, 0x9DU, 0x7CU, 0x25U, 0xE4U, 0x1DU, 0x2BU, 0x66U, 0x9EU, 0x1EU, 0xF1U, 0x6EU,
    0x6FU, 0x52U, 0xC3U, 0x16U, 0x4DU, 0xF4U, 0xFBU, 0x79U, 0x30U, 0xE9U, 0xE4U, 0xE5U, 0x88U,
    0x57U, 0xB6U, 0xACU, 0x7DU, 0x5FU, 0x42U, 0xD6U, 0x9FU, 0x6DU, 0x18U, 0x77U, 0x63U, 0xCFU,
    0x1DU, 0x55U, 0x03U, 0x40U, 0x04U, 0x87U, 0xF5U, 0x5BU, 0xA5U, 0x7EU, 0x31U, 0xCCU, 0x7AU,
    0x71U, 0x35U, 0xC8U, 0x86U, 0xEFU, 0xB4U, 0x31U, 0x8AU, 0xEDU, 0x6AU, 0x1EU, 0x01U, 0x2DU,
    0x9EU, 0x68U, 0x32U, 0xA9U, 0x07U, 0x60U, 0x0AU, 0x91U, 0x81U, 0x30U, 0xC4U, 0x6DU, 0xC7U,
    0x78U, 0xF9U, 0x71U, 0xADU, 0x00U, 0x38U, 0x09U, 0x29U, 0x99U, 0xA3U, 0x33U, 0xCBU, 0x8BU,
    0x7AU, 0x1AU, 0x1DU, 0xB9U, 0x3DU, 0x71U, 0x40U, 0x00U, 0x3CU, 0x2AU, 0x4EU, 0xCEU, 0xA9U,
    0xF9U, 0x8DU, 0x0AU, 0xCCU, 0x0AU, 0x82U, 0x91U, 0xCDU, 0xCEU, 0xC9U, 0x7DU, 0xCFU, 0x8EU,
    0xC9U, 0xB5U, 0x5AU, 0x7FU, 0x88U, 0xA4U, 0x6BU, 0x4DU, 0xB5U, 0xA8U, 0x51U, 0xF4U, 0x41U,
    0x82U, 0xE1U, 0xC6U, 0x8AU, 0x00U, 0x7EU, 0x5EU, 0x0DU, 0xD9U, 0x02U, 0x0BU, 0xFDU, 0x64U,
    0xB6U, 0x45U, 0x03U, 0x6CU, 0x7AU, 0x4EU, 0x67U, 0x7DU, 0x2CU, 0x38U, 0x53U, 0x2AU, 0x3AU,
    0x23U, 0xBAU, 0x44U, 0x42U, 0xCAU, 0xF5U, 0x3EU, 0xA6U, 0x3BU, 0xB4U, 0x54U, 0x32U, 0x9BU,
    0x76U, 0x24U, 0xC8U, 0x91U, 0x7BU, 0xDDU, 0x64U, 0xB1U, 0xC0U, 0xFDU, 0x4CU, 0xB3U, 0x8EU,
    0x8CU, 0x33U, 0x4CU, 0x70U, 0x1CU, 0x3AU, 0xCDU, 0xADU, 0x06U, 0x57U, 0xFCU, 0xCFU, 0xECU,
    0x71U, 0x9BU, 0x1FU, 0x5CU, 0x3EU, 0x4EU, 0x46U, 0x04U, 0x1FU, 0x38U, 0x81U, 0x47U, 0xFBU,
    0x4CU, 0xFDU, 0xB4U, 0x77U, 0xA5U, 0x24U, 0x71U, 0xF7U, 0xA9U, 0xA9U, 0x69U, 0x10U, 0xB8U,
    0x55U, 0x32U, 0x2EU, 0xDBU, 0x63U, 0x40U, 0xD8U, 0xA0U, 0x0EU, 0xF0U, 0x92U, 0x35U, 0x05U,
    0x11U, 0xE3U, 0x0AU, 0xBEU, 0xC1U, 0xFFU, 0xF9U, 0xE3U, 0xA2U, 0x6EU, 0x7FU, 0xB2U, 0x9FU,
    0x8CU, 0x18U, 0x30U, 0x23U, 0xC3U, 0x58U, 0x7EU, 0x38U, 0xDAU, 0x00U, 0x77U, 0xD9U, 0xB4U,
    0x76U, 0x3EU, 0x4EU, 0x4BU, 0x94U, 0xB2U, 0xBBU, 0xC1U, 0x94U, 0xC6U, 0x65U, 0x1EU, 0x77U,
    0xCAU, 0xF9U, 0x92U, 0xEEU, 0xAAU, 0xC0U, 0x23U, 0x2AU, 0x28U, 0x1BU, 0xF6U, 0xB3U, 0xA7U,
    0x39U, 0xC1U, 0x22U, 0x61U, 0x16U, 0x82U, 0x0AU, 0xE8U, 0xDBU, 0x58U, 0x47U, 0xA6U, 0x7CU,
    0xBEU, 0xF9U, 0xC9U, 0x09U, 0x1BU, 0x46U, 0x2DU, 0x53U, 0x8CU, 0xD7U, 0x2BU, 0x03U, 0x74U,
    0x6AU, 0xE7U, 0x7FU, 0x5EU, 0x62U, 0x29U, 0x2CU, 0x31U, 0x15U, 0x62U, 0xA8U, 0x46U, 0x50U,
    0x5DU, 0xC8U, 0x2DU, 0xB8U, 0x54U, 0x33U, 0x8AU, 0xE4U, 0x9FU, 0x52U, 0x35U, 0xC9U, 0x5BU,
    0x91U, 0x17U, 0x8CU, 0xCFU, 0x2DU, 0xD5U, 0xCAU, 0xCEU, 0xF4U, 0x03U, 0xECU, 0x9DU, 0x18U,
    0x10U, 0xC6U, 0x27U, 0x2BU, 0x04U, 0x5BU, 0x3BU, 0x71U, 0xF9U, 0xDCU, 0x6BU, 0x80U, 0xD6U,
    0x3FU, 0xDDU, 0x4AU, 0x8EU, 0x9AU, 0xDBU, 0x1EU, 0x69U, 0x62U, 0xA6U, 0x95U, 0x26U, 0xD4U,
    0x31U, 0x61U, 0xC1U, 0xA4U, 0x1DU, 0x57U, 0x0DU, 0x79U, 0x38U, 0xDAU, 0xD4U, 0xA4U, 0x0EU,
    0x32U, 0x9CU, 0xD0U, 0xE4U, 0x0EU, 0x65U, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU,
    0xFFU
  };

static const
uint8_t
Hacl_Impl_FFDHE_Constants_ffdhe_p8192[1024U] =
  {
    0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xADU, 0xF8U, 0x54U, 0x58U, 0xA2U,
    0xBBU, 0x4AU, 0x9AU, 0xAFU, 0xDCU, 0x56U, 0x20U, 0x27U, 0x3DU, 0x3CU, 0xF1U, 0xD8U, 0xB9U,
    0xC5U, 0x83U, 0xCEU, 0x2DU, 0x36U, 0x95U, 0xA9U, 0xE1U, 0x36U, 0x41U, 0x14U, 0x64U, 0x33U,
    0xFBU, 0xCCU, 0x93U, 0x9DU, 0xCEU, 0x24U, 0x9BU, 0x3EU, 0xF9U, 0x7DU, 0x2FU, 0xE3U, 0x63U,
    0x63U, 0x0CU, 0x75U, 0xD8U, 0xF6U, 0x81U, 0xB2U, 0x02U, 0xAEU, 0xC4U, 0x61U, 0x7AU, 0xD3U,
    0xDFU, 0x1EU, 0xD5U, 0xD5U, 0xFDU, 0x65U, 0x61U, 0x24U, 0x33U, 0xF5U, 0x1FU, 0x5FU, 0x06U,
    0x6EU, 0xD0U, 0x85U, 0x63U, 0x65U, 0x55U, 0x3DU, 0xEDU, 0x1AU, 0xF3U, 0xB5U, 0x57U, 0x13U,
    0x5EU, 0x7FU, 0x57U, 0xC9U, 0x35U, 0x98U, 0x4FU, 0x0CU, 0x70U, 0xE0U, 0xE6U, 0x8BU, 0x77U,
    0xE2U, 0xA6U, 0x89U, 0xDAU, 0xF3U, 0xEFU, 0xE8U, 0x72U, 0x1DU, 0xF1U, 0x58U, 0xA1U, 0x36U,
    0xADU, 0xE7U, 0x35U, 0x30U, 0xACU, 0xCAU, 0x4FU, 0x48U, 0x3AU, 0x79U, 0x7AU, 0xBCU, 0x0AU,
    0xB1U, 0x82U, 0xB3U, 0x24U, 0xFBU, 0x61U, 0xD1U, 0x08U, 0xA9U, 0x4BU, 0xB2U, 0xC8U, 0xE3U,
    0xFBU, 0xB9U, 0x6AU, 0xDAU, 0xB7U, 0x60U, 0xD7U, 0xF4U, 0x68U, 0x1DU, 0x4FU, 0x42U, 0xA3U,
    0xDEU, 0x39U, 0x4DU, 0xF4U, 0xAEU, 0x56U, 0xEDU, 0xE7U, 0x63U, 0x72U, 0xBBU, 0x19U, 0x0BU,
    0x07U, 0xA7U, 0xC8U, 0xEEU, 0x0AU, 0x6DU, 0x70U, 0x9EU, 0x02U, 0xFCU, 0xE1U, 0xCDU, 0xF7U,
    0xE2U, 0xECU, 0xC0U, 0x34U, 0x04U, 0xCDU, 0x28U, 0x34U, 0x2FU, 0x61U, 0x91U, 0x72U, 0xFEU,
    0x9CU, 0xE9U, 0x85U, 0x83U, 0xFFU, 0x8EU, 0x4FU, 0x12U, 0x32U, 0xEEU, 0xF2U, 0x81U, 0x83U,
    0xC3U, 0xFEU, 0x3BU, 0x1BU, 0x4CU, 0x6FU, 0xADU, 0x73U, 0x3BU, 0xB5U, 0xFCU, 0xBCU, 0x2EU,
    0xC2U, 0x20U, 0x05U, 0xC5U, 0x8EU, 0xF1U, 0x83U, 0x7DU, 0x16U, 0x83U, 0xB2U, 0xC6U, 0xF3U,
    0x4AU, 0x26U, 0xC1U, 0xB2U, 0xEFU, 0xFAU, 0x88U, 0x6BU, 0x42U, 0x38U, 0x61U, 0x1FU, 0xCFU,
    0xDCU, 0xDEU, 0x35U, 0x5BU, 0x3BU, 0x65U, 0x19U, 0x03U, 0x5BU, 0xBCU, 0x34U, 0xF4U, 0xDEU,
    0xF9U, 0x9CU, 0x02U, 0x38U, 0x61U, 0xB4U, 0x6FU, 0xC9U, 0xD6U, 0xE6U, 0xC9U, 0x07U, 0x7AU,
    0xD9U, 0x1DU, 0x26U, 0x91U, 0xF7U, 0xF7U, 0xEEU, 0x59U, 0x8CU, 0xB0U, 0xFAU, 0xC1U, 0x86U,
    0xD9U, 0x1CU, 0xAEU, 0xFEU, 0x13U, 0x09U, 0x85U, 0x13U, 0x92U, 0x70U, 0xB4U, 0x13U, 0x0CU,
    0x93U, 0xBCU, 0x43U, 0x79U, 0x44U, 0xF4U, 0xFDU, 0x44U, 0x52U, 0xE2U, 0xD7U, 0x4DU, 0xD3U,
    0x64U, 0xF2U, 0xE2U, 0x1EU, 0x71U, 0xF5U, 0x4BU, 0xFFU, 0x5CU, 0xAEU, 0x82U, 0xABU, 0x9CU,
    0x9DU, 0xF6U, 0x9EU, 0xE8U, 0x6DU, 0x2BU, 0xC5U, 0x22U, 0x36U, 0x3AU, 0x0DU, 0xABU, 0xC5U,
    0x21U, 0x97U, 0x9BU, 0x0DU, 0xEAU, 0xDAU, 0x1DU, 0xBFU, 0x9AU, 0x42U, 0xD5U, 0xC4U, 0x48U,
    0x4EU, 0x0AU, 0xBCU, 0xD0U, 0x6BU, 0xFAU, 0x53U, 0xDDU, 0xEFU, 0x3CU, 0x1BU, 0x20U, 0xEEU,
    0x3FU, 0xD5U, 0x9DU, 0x7CU, 0x25U, 0xE4U, 0x1DU, 0x2BU, 0x66U, 0x9EU, 0x1EU, 0xF1U, 0x6EU,
    0x6FU, 0x52U, 0xC3U, 0x16U, 0x4DU, 0xF4U, 0xFBU, 0x79U, 0x30U, 0xE9U, 0xE4U, 0xE5U, 0x88U,
    0x57U, 0xB6U, 0xACU, 0x7DU, 0x5FU, 0x42U, 0xD6U, 0x9FU, 0x6DU, 0x18U, 0x77U, 0x63U, 0xCFU,
    0x1DU, 0x55U, 0x03U, 0x40U, 0x04U, 0x87U, 0xF5U, 0x5BU, 0xA5U, 0x7EU, 0x31U, 0xCCU, 0x7AU,
    0x71U, 0x35U, 0xC8U, 0x86U, 0xEFU, 0xB4U, 0x31U, 0x8AU, 0xEDU, 0x6AU, 0x1EU, 0x01U, 0x2DU,
    0x9EU, 0x68U, 0x32U, 0xA9U, 0x07U, 0x60U, 0x0AU, 0x91U, 0x81U, 0x30U, 0xC4U, 0x6DU, 0xC7U,
    0x78U, 0xF9U, 0x71U, 0xADU, 0x00U, 0x38U, 0x09U, 0x29U, 0x99U, 0xA3U, 0x33U, 0xCBU, 0x8BU,
    0x7AU, 0x1AU, 0x1DU, 0xB9U, 0x3DU, 0x71U, 0x40U, 0x00U, 0x3CU, 0x2AU, 0x4EU, 0xCEU, 0xA9U,
    0xF9U, 0x8DU, 0x0AU, 0xCCU, 0x0AU, 0x82U, 0x91U, 0xCDU, 0xCEU, 0xC9U, 0x7DU, 0xCFU, 0x8EU,
    0xC9U, 0xB5U, 0x5AU, 0x7FU, 0x88U, 0xA4U, 0x6BU, 0x4DU, 0xB5U, 0xA8U, 0x51U, 0xF4U, 0x41U,
    0x82U, 0xE1U, 0xC6U, 0x8AU, 0x00U, 0x7EU, 0x5EU, 0x0DU, 0xD9U, 0x02U, 0x0BU, 0xFDU, 0x64U,
    0xB6U, 0x45U, 0x03U, 0x6CU, 0x7AU, 0x4EU, 0x67U, 0x7DU, 0x2CU, 0x38U, 0x53U, 0x2AU, 0x3AU,
    0x23U, 0xBAU, 0x44U, 0x42U, 0xCAU, 0xF5U, 0x3EU, 0xA6U, 0x3BU, 0xB4U, 0x54U, 0x32U, 0x9BU,
    0x76U, 0x24U, 0xC8U, 0x91U, 0x7BU, 0xDDU, 0x64U, 0xB1U, 0xC0U, 0xFDU, 0x4CU, 0xB3U, 0x8EU,
    0x8CU, 0x33U, 0x4CU, 0x70U, 0x1CU, 0x3AU, 0xCDU, 0xADU, 0x06U, 0x57U, 0xFCU, 0xCFU, 0xECU,
    0x71U, 0x9BU, 0x1FU, 0x5CU, 0x3EU, 0x4EU, 0x46U, 0x04U, 0x1FU, 0x38U, 0x81U, 0x47U, 0xFBU,
    0x4CU, 0xFDU, 0xB4U, 0x77U, 0xA5U, 0x24U, 0x71U, 0xF7U, 0xA9U, 0xA9U, 0x69U, 0x10U, 0xB8U,
    0x55U, 0x32U, 0x2EU, 0xDBU, 0x63U, 0x40U, 0xD8U, 0xA0U, 0x0EU, 0xF0U, 0x92U, 0x35U, 0x05U,
    0x11U, 0xE3U, 0x0AU, 0xBEU, 0xC1U, 0xFFU, 0xF9U, 0xE3U, 0xA2U, 0x6EU, 0x7FU, 0xB2U, 0x9FU,
    0x8CU, 0x18U, 0x30U, 0x23U, 0xC3U, 0x58U, 0x7EU, 0x38U, 0xDAU, 0x00U, 0x77U, 0xD9U, 0xB4U,
    0x76U, 0x3EU, 0x4EU, 0x4BU, 0x94U, 0xB2U, 0xBBU, 0xC1U, 0x94U, 0xC6U, 0x65U, 0x1EU, 0x77U,
    0xCAU, 0xF9U, 0x92U, 0xEEU, 0xAAU, 0xC0U, 0x23U, 0x2AU, 0x28U, 0x1BU, 0xF6U, 0xB3U, 0xA7U,
    0x39U, 0xC1U, 0x22U, 0x61U, 0x16U, 0x82U, 0x0AU, 0xE8U, 0xDBU, 0x58U, 0x47U, 0xA6U, 0x7CU,
    0xBEU, 0xF9U, 0xC9U, 0x09U, 0x1BU, 0x46U, 0x2DU, 0x53U, 0x8CU, 0xD7U, 0x2BU, 0x03U, 0x74U,
    0x6AU, 0xE7U, 0x7FU, 0x5EU, 0x62U, 0x29U, 0x2CU, 0x31U, 0x15U, 0x62U, 0xA8U, 0x46U, 0x50U,
    0x5DU, 0xC8U, 0x2DU, 0xB8U, 0x54U, 0x33U, 0x8AU, 0xE4U, 0x9FU, 0x52U, 0x35U, 0xC9U, 0x5BU,
    0x91U, 0x17U, 0x8CU, 0xCFU, 0x2DU, 0xD5U, 0xCAU, 0xCEU, 0xF4U, 0x03U, 0xECU, 0x9DU, 0x18U,
    0x10U, 0xC6U, 0x27U, 0x2BU, 0x04U, 0x5BU, 0x3BU, 0x71U, 0xF9U, 0xDCU, 0x6BU, 0x80U, 0xD6U,
    0x3FU, 0xDDU, 0x4AU, 0x8EU, 0x9AU, 0xDBU, 0x1EU, 0x69U, 0x62U, 0xA6U, 0x95U, 0x26U, 0xD4U,
    0x31U, 0x61U, 0xC1U, 0xA4U, 0x1DU, 0x57U, 0x0DU, 0x79U, 0x38U, 0xDAU, 0xD4U, 0xA4U, 0x0EU,
    0x32U, 0x9CU, 0xCFU, 0xF4U, 0x6AU, 0xAAU, 0x36U, 0xADU, 0x00U, 0x4CU, 0xF6U, 0x00U, 0xC8U,
    0x38U, 0x1EU, 0x42U, 0x5AU, 0x31U, 0xD9U, 0x51U, 0xAEU, 0x64U, 0xFDU, 0xB2U, 0x3FU, 0xCEU,
    0xC9U, 0x50U, 0x9DU, 0x43U, 0x68U, 0x7FU, 0xEBU, 0x69U, 0xEDU, 0xD1U, 0xCCU, 0x5EU, 0x0BU,
    0x8CU, 0xC3U, 0xBDU, 0xF6U, 0x4BU, 0x10U, 0xEFU, 0x86U, 0xB6U, 0x31U, 0x42U, 0xA3U, 0xABU,
    0x88U, 0x29U, 0x55U, 0x5BU, 0x2FU, 0x74U, 0x7CU, 0x93U, 0x26U, 0x65U, 0xCBU, 0x2CU, 0x0FU,
    0x1CU, 0xC0U, 0x1BU, 0xD7U, 0x02U, 0x29U, 0x38U, 0x88U, 0x39U, 0xD2U, 0xAFU, 0x05U, 0xE4U,
    0x54U, 0x50U, 0x4AU, 0xC7U, 0x8BU, 0x75U, 0x82U, 0x82U, 0x28U, 0x46U, 0xC0U, 0xBAU, 0x35U,
    0xC3U, 0x5FU, 0x5CU, 0x59U, 0x16U, 0x0CU, 0xC0U, 0x46U, 0xFDU, 0x82U, 0x51U, 0x54U, 0x1FU,
    0xC6U, 0x8CU, 0x9CU, 0x86U, 0xB0U, 0x22U, 0xBBU, 0x70U, 0x99U, 0x87U, 0x6AU, 0x46U, 0x0EU,
    0x74U, 0x51U, 0xA8U, 0xA9U, 0x31U, 0x09U, 0x70U, 0x3FU, 0xEEU, 0x1CU, 0x21U, 0x7EU, 0x6CU,
    0x38U, 0x26U, 0xE5U, 0x2CU, 0x51U, 0xAAU, 0x69U, 0x1EU, 0x0EU, 0x42U, 0x3CU, 0xFCU, 0x99U,
    0xE9U, 0xE3U, 0x16U, 0x50U, 0xC1U, 0x21U, 0x7BU, 0x62U, 0x48U, 0x16U, 0xCDU, 0xADU, 0x9AU,
    0x95U, 0xF9U, 0xD5U, 0xB8U, 0x01U, 0x94U, 0x88U, 0xD9U, 0xC0U, 0xA0U, 0xA1U, 0xFEU, 0x30U,
    0x75U, 0xA5U, 0x77U, 0xE2U, 0x31U, 0x83U, 0xF8U, 0x1DU, 0x4AU, 0x3FU, 0x2FU, 0xA4U, 0x57U,
    0x1EU, 0xFCU, 0x8CU, 0xE0U, 0xBAU, 0x8AU, 0x4FU, 0xE8U, 0xB6U, 0x85U, 0x5DU, 0xFEU, 0x72U,
    0xB0U, 0xA6U, 0x6EU, 0xDEU, 0xD2U, 0xFBU, 0xABU, 0xFBU, 0xE5U, 0x8AU, 0x30U, 0xFAU, 0xFAU,
    0xBEU, 0x1CU, 0x5DU, 0x71U, 0xA8U, 0x7EU, 0x2FU, 0x74U, 0x1EU, 0xF8U, 0xC1U, 0xFEU, 0x86U,
    0xFEU, 0xA6U, 0xBBU, 0xFDU, 0xE5U, 0x30U, 0x67U, 0x7FU, 0x0DU, 0x97U, 0xD1U, 0x1DU, 0x49U,
    0xF7U, 0xA8U, 0x44U, 0x3DU, 0x08U, 0x22U, 0xE5U, 0x06U, 0xA9U, 0xF4U, 0x61U, 0x4EU, 0x01U,
    0x1EU, 0x2AU, 0x94U, 0x83U, 0x8FU, 0xF8U, 0x8CU, 0xD6U, 0x8CU, 0x8BU, 0xB7U, 0xC5U, 0xC6U,
    0x42U, 0x4CU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU, 0xFFU
  };

#if defined(__cplusplus)
}
#endif

#define __internal_Hacl_Impl_FFDHE_Constants_H_DEFINED
#endif
