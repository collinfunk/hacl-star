#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unreachable_patterns)]
#![allow(const_item_mutation)]

pub type pbn_mont_ctx_u64 <'a> = &'a [crate::hacl::bignum::bn_mont_ctx_u64];

pub fn field_modulus_check(len: u32, n: &[u64]) -> bool
{
    let m: u64 = crate::hacl::bignum::bn_check_modulus_u64(len, n);
    m == 0xFFFFFFFFFFFFFFFFu64
}

pub fn field_init(len: u32, n: &[u64]) -> Vec<crate::hacl::bignum::bn_mont_ctx_u64>
{
    let mut r2: Vec<u64> = vec![0u64; len as usize];
    let mut n1: Vec<u64> = vec![0u64; len as usize];
    let r21: &mut [u64] = &mut r2;
    let n11: &mut [u64] = &mut n1;
    (n11[0usize..len as usize]).copy_from_slice(&n[0usize..len as usize]);
    let nBits: u32 =
        64u32.wrapping_mul(crate::hacl::bignum_base::bn_get_top_index_u64(len, n) as u32);
    crate::hacl::bignum::bn_precomp_r2_mod_n_u64(len, nBits, n, r21);
    let mu: u64 = crate::hacl::bignum::mod_inv_uint64(n[0usize]);
    let res: crate::hacl::bignum::bn_mont_ctx_u64 =
        crate::hacl::bignum::bn_mont_ctx_u64 { len: len, n: n11.to_vec(), mu: mu, r2: r21.to_vec() };
    let buf: Vec<crate::hacl::bignum::bn_mont_ctx_u64> =
        {
            let mut tmp: Vec<crate::hacl::bignum::bn_mont_ctx_u64> = Vec::new();
            tmp.push(res);
            tmp
        };
    buf
}

pub fn field_get_len(k: &[crate::hacl::bignum::bn_mont_ctx_u64]) -> u32 { (k[0usize]).len }

pub fn to_field(k: &[crate::hacl::bignum::bn_mont_ctx_u64], a: &[u64], aM: &mut [u64])
{
    let len1: u32 = field_get_len(k);
    let uu____0: &crate::hacl::bignum::bn_mont_ctx_u64 = &k[0usize];
    crate::hacl::bignum::bn_to_mont_u64(len1, &(*uu____0).n, (*uu____0).mu, &(*uu____0).r2, a, aM)
}

pub fn from_field(k: &[crate::hacl::bignum::bn_mont_ctx_u64], aM: &[u64], a: &mut [u64])
{
    let len1: u32 = field_get_len(k);
    let uu____0: &crate::hacl::bignum::bn_mont_ctx_u64 = &k[0usize];
    crate::hacl::bignum::bn_from_mont_u64(len1, &(*uu____0).n, (*uu____0).mu, aM, a)
}

pub fn add(k: &[crate::hacl::bignum::bn_mont_ctx_u64], aM: &[u64], bM: &[u64], cM: &mut [u64])
{
    let len1: u32 = field_get_len(k);
    let uu____0: &crate::hacl::bignum::bn_mont_ctx_u64 = &k[0usize];
    let mut a_copy: Vec<u64> = vec![0u64; len1 as usize];
    let mut b_copy: Vec<u64> = vec![0u64; len1 as usize];
    ((&mut a_copy)[0usize..len1 as usize]).copy_from_slice(&aM[0usize..len1 as usize]);
    ((&mut b_copy)[0usize..len1 as usize]).copy_from_slice(&bM[0usize..len1 as usize]);
    crate::hacl::bignum::bn_add_mod_n_u64(len1, &(*uu____0).n, &a_copy, &b_copy, cM)
}

pub fn sub(k: &[crate::hacl::bignum::bn_mont_ctx_u64], aM: &[u64], bM: &[u64], cM: &mut [u64])
{
    let len1: u32 = field_get_len(k);
    crate::hacl::bignum::bn_sub_mod_n_u64(len1, &(k[0usize]).n, aM, bM, cM)
}

pub fn mul(k: &[crate::hacl::bignum::bn_mont_ctx_u64], aM: &[u64], bM: &[u64], cM: &mut [u64])
{
    let len1: u32 = field_get_len(k);
    let uu____0: &crate::hacl::bignum::bn_mont_ctx_u64 = &k[0usize];
    crate::hacl::bignum::bn_mont_mul_u64(len1, &(*uu____0).n, (*uu____0).mu, aM, bM, cM)
}

pub fn sqr(k: &[crate::hacl::bignum::bn_mont_ctx_u64], aM: &[u64], cM: &mut [u64])
{
    let len1: u32 = field_get_len(k);
    let uu____0: &crate::hacl::bignum::bn_mont_ctx_u64 = &k[0usize];
    crate::hacl::bignum::bn_mont_sqr_u64(len1, &(*uu____0).n, (*uu____0).mu, aM, cM)
}

pub fn one(k: &[crate::hacl::bignum::bn_mont_ctx_u64], oneM: &mut [u64])
{
    let len1: u32 = field_get_len(k);
    let uu____0: &crate::hacl::bignum::bn_mont_ctx_u64 = &k[0usize];
    crate::hacl::bignum::bn_from_mont_u64(len1, &(*uu____0).n, (*uu____0).mu, &(*uu____0).r2, oneM)
}

pub fn exp_consttime(
    k: &[crate::hacl::bignum::bn_mont_ctx_u64],
    aM: &[u64],
    bBits: u32,
    b: &[u64],
    resM: &mut [u64]
)
{
    let len1: u32 = field_get_len(k);
    let uu____0: &crate::hacl::bignum::bn_mont_ctx_u64 = &k[0usize];
    let mut aMc: Vec<u64> = vec![0u64; (*uu____0).len as usize];
    ((&mut aMc)[0usize..(*uu____0).len as usize]).copy_from_slice(
        &aM[0usize..(*uu____0).len as usize]
    );
    if bBits < 200u32
    {
        let mut ctx: Vec<u64> = vec![0u64; len1.wrapping_add(len1) as usize];
        ((&mut ctx)[0usize..len1 as usize]).copy_from_slice(&(&(*uu____0).n)[0usize..len1 as usize]);
        ((&mut ctx)[len1 as usize..len1 as usize + len1 as usize]).copy_from_slice(
            &(&(*uu____0).r2)[0usize..len1 as usize]
        );
        let mut sw: [u64; 1] = [0u64; 1usize];
        let ctx_n: (&[u64], &[u64]) = (&ctx).split_at(0usize);
        let ctx_r2: (&[u64], &[u64]) = ctx_n.1.split_at(len1 as usize);
        crate::hacl::bignum::bn_from_mont_u64(len1, ctx_r2.0, (*uu____0).mu, ctx_r2.1, resM);
        for i in 0u32..bBits
        {
            let i1: u32 = bBits.wrapping_sub(i).wrapping_sub(1u32).wrapping_div(64u32);
            let j: u32 = bBits.wrapping_sub(i).wrapping_sub(1u32).wrapping_rem(64u32);
            let tmp: u64 = b[i1 as usize];
            let bit: u64 = tmp.wrapping_shr(j) & 1u64;
            let sw1: u64 = bit ^ (&sw)[0usize];
            for i0 in 0u32..len1
            {
                let dummy: u64 = 0u64.wrapping_sub(sw1) & (resM[i0 as usize] ^ (&aMc)[i0 as usize]);
                resM[i0 as usize] = resM[i0 as usize] ^ dummy;
                (&mut aMc)[i0 as usize] = (&aMc)[i0 as usize] ^ dummy
            };
            let ctx_n0: (&[u64], &[u64]) = ctx_r2.1.split_at(0usize - len1 as usize);
            crate::hacl::bignum::bn_mont_mul_u64(
                len1,
                ctx_n0.1,
                (*uu____0).mu,
                &aMc,
                resM,
                &mut aMc
            );
            let ctx_n1: (&[u64], &[u64]) = ctx_n0.1.split_at(0usize);
            crate::hacl::bignum::bn_mont_sqr_u64(len1, ctx_n1.1, (*uu____0).mu, resM, resM);
            (&mut sw)[0usize] = bit
        };
        let sw0: u64 = (&sw)[0usize];
        for i in 0u32..len1
        {
            let dummy: u64 = 0u64.wrapping_sub(sw0) & (resM[i as usize] ^ (&aMc)[i as usize]);
            resM[i as usize] = resM[i as usize] ^ dummy;
            (&mut aMc)[i as usize] = (&aMc)[i as usize] ^ dummy
        }
    }
    else
    {
        let bLen: u32 =
            if bBits == 0u32
            { 1u32 }
            else
            { bBits.wrapping_sub(1u32).wrapping_div(64u32).wrapping_add(1u32) };
        let mut ctx: Vec<u64> = vec![0u64; len1.wrapping_add(len1) as usize];
        ((&mut ctx)[0usize..len1 as usize]).copy_from_slice(&(&(*uu____0).n)[0usize..len1 as usize]);
        ((&mut ctx)[len1 as usize..len1 as usize + len1 as usize]).copy_from_slice(
            &(&(*uu____0).r2)[0usize..len1 as usize]
        );
        let mut table: Vec<u64> = vec![0u64; 16u32.wrapping_mul(len1) as usize];
        let mut tmp: Vec<u64> = vec![0u64; len1 as usize];
        let t0: (&mut [u64], &mut [u64]) = (&mut table).split_at_mut(0usize);
        let t1: (&mut [u64], &mut [u64]) = t0.1.split_at_mut(len1 as usize);
        let ctx_n: (&[u64], &[u64]) = (&ctx).split_at(0usize);
        let ctx_r2: (&[u64], &[u64]) = ctx_n.1.split_at(len1 as usize);
        crate::hacl::bignum::bn_from_mont_u64(len1, ctx_r2.0, (*uu____0).mu, ctx_r2.1, t1.0);
        (t1.1[0usize..len1 as usize]).copy_from_slice(&(&aMc)[0usize..len1 as usize]);
        crate::lowstar::ignore::ignore::<&[u64]>(&table);
        krml::unroll_for!(
            7,
            "i",
            0u32,
            1u32,
            {
                let t11: (&[u64], &[u64]) =
                    (&table).split_at(i.wrapping_add(1u32).wrapping_mul(len1) as usize);
                let ctx_n0: (&[u64], &[u64]) = ctx_r2.1.split_at(0usize - len1 as usize);
                crate::hacl::bignum::bn_mont_sqr_u64(len1, ctx_n0.1, (*uu____0).mu, t11.1, &mut tmp);
                ((&mut table)[2u32.wrapping_mul(i).wrapping_add(2u32).wrapping_mul(len1) as usize..2u32.wrapping_mul(
                    i
                ).wrapping_add(2u32).wrapping_mul(len1)
                as
                usize
                +
                len1 as usize]).copy_from_slice(&(&tmp)[0usize..len1 as usize]);
                let t2: (&[u64], &[u64]) =
                    (&table).split_at(
                        2u32.wrapping_mul(i).wrapping_add(2u32).wrapping_mul(len1) as usize
                    );
                let ctx_n1: (&[u64], &[u64]) = ctx_n0.1.split_at(0usize);
                crate::hacl::bignum::bn_mont_mul_u64(
                    len1,
                    ctx_n1.1,
                    (*uu____0).mu,
                    &aMc,
                    t2.1,
                    &mut tmp
                );
                ((&mut table)[2u32.wrapping_mul(i).wrapping_add(3u32).wrapping_mul(len1) as usize..2u32.wrapping_mul(
                    i
                ).wrapping_add(3u32).wrapping_mul(len1)
                as
                usize
                +
                len1 as usize]).copy_from_slice(&(&tmp)[0usize..len1 as usize])
            }
        );
        if bBits.wrapping_rem(4u32) != 0u32
        {
            let i: u32 = bBits.wrapping_div(4u32).wrapping_mul(4u32);
            let bits_c: u64 = crate::hacl::bignum_base::bn_get_bits_u64(bLen, b, i, 4u32);
            (resM[0usize..len1 as usize]).copy_from_slice(
                &(&(&table)[0u32.wrapping_mul(len1) as usize..] as &[u64])[0usize..len1 as usize]
            );
            krml::unroll_for!(
                15,
                "i0",
                0u32,
                1u32,
                {
                    let c: u64 =
                        crate::fstar::uint64::eq_mask(bits_c, i0.wrapping_add(1u32) as u64);
                    let res_j: (&[u64], &[u64]) =
                        (&table).split_at(i0.wrapping_add(1u32).wrapping_mul(len1) as usize);
                    for i1 in 0u32..len1
                    {
                        let x: u64 = c & res_j.1[i1 as usize] | ! c & resM[i1 as usize];
                        let os: (&mut [u64], &mut [u64]) = resM.split_at_mut(0usize);
                        os.1[i1 as usize] = x
                    }
                }
            )
        }
        else
        {
            let ctx_n0: (&[u64], &[u64]) = ctx_r2.1.split_at(0usize - len1 as usize);
            let ctx_r20: (&[u64], &[u64]) = ctx_n0.1.split_at(len1 as usize);
            crate::hacl::bignum::bn_from_mont_u64(len1, ctx_r20.0, (*uu____0).mu, ctx_r20.1, resM)
        };
        let mut tmp0: Vec<u64> = vec![0u64; len1 as usize];
        for i in 0u32..bBits.wrapping_div(4u32)
        {
            krml::unroll_for!(
                4,
                "_i",
                0u32,
                1u32,
                {
                    let ctx_n0: (&[u64], &[u64]) = ctx_r2.1.split_at(0usize - len1 as usize);
                    crate::hacl::bignum::bn_mont_sqr_u64(len1, ctx_n0.1, (*uu____0).mu, resM, resM)
                }
            );
            let k2: u32 =
                bBits.wrapping_sub(bBits.wrapping_rem(4u32)).wrapping_sub(4u32.wrapping_mul(i)).wrapping_sub(
                    4u32
                );
            let bits_l: u64 = crate::hacl::bignum_base::bn_get_bits_u64(bLen, b, k2, 4u32);
            crate::lowstar::ignore::ignore::<&[u64]>(&table);
            ((&mut tmp0)[0usize..len1 as usize]).copy_from_slice(
                &(&(&table)[0u32.wrapping_mul(len1) as usize..] as &[u64])[0usize..len1 as usize]
            );
            krml::unroll_for!(
                15,
                "i0",
                0u32,
                1u32,
                {
                    let c: u64 =
                        crate::fstar::uint64::eq_mask(bits_l, i0.wrapping_add(1u32) as u64);
                    let res_j: (&[u64], &[u64]) =
                        (&table).split_at(i0.wrapping_add(1u32).wrapping_mul(len1) as usize);
                    for i1 in 0u32..len1
                    {
                        let x: u64 = c & res_j.1[i1 as usize] | ! c & (&tmp0)[i1 as usize];
                        let os: (&mut [u64], &mut [u64]) = (&mut tmp0).split_at_mut(0usize);
                        os.1[i1 as usize] = x
                    }
                }
            );
            let ctx_n0: (&[u64], &[u64]) = ctx_r2.1.split_at(0usize - len1 as usize);
            crate::hacl::bignum::bn_mont_mul_u64(len1, ctx_n0.1, (*uu____0).mu, resM, &tmp0, resM)
        }
    }
}

pub fn exp_vartime(
    k: &[crate::hacl::bignum::bn_mont_ctx_u64],
    aM: &[u64],
    bBits: u32,
    b: &[u64],
    resM: &mut [u64]
)
{
    let len1: u32 = field_get_len(k);
    let uu____0: &crate::hacl::bignum::bn_mont_ctx_u64 = &k[0usize];
    let mut aMc: Vec<u64> = vec![0u64; (*uu____0).len as usize];
    ((&mut aMc)[0usize..(*uu____0).len as usize]).copy_from_slice(
        &aM[0usize..(*uu____0).len as usize]
    );
    if bBits < 200u32
    {
        let mut ctx: Vec<u64> = vec![0u64; len1.wrapping_add(len1) as usize];
        ((&mut ctx)[0usize..len1 as usize]).copy_from_slice(&(&(*uu____0).n)[0usize..len1 as usize]);
        ((&mut ctx)[len1 as usize..len1 as usize + len1 as usize]).copy_from_slice(
            &(&(*uu____0).r2)[0usize..len1 as usize]
        );
        let ctx_n: (&[u64], &[u64]) = (&ctx).split_at(0usize);
        let ctx_r2: (&[u64], &[u64]) = ctx_n.1.split_at(len1 as usize);
        crate::hacl::bignum::bn_from_mont_u64(len1, ctx_r2.0, (*uu____0).mu, ctx_r2.1, resM);
        for i in 0u32..bBits
        {
            let i1: u32 = i.wrapping_div(64u32);
            let j: u32 = i.wrapping_rem(64u32);
            let tmp: u64 = b[i1 as usize];
            let bit: u64 = tmp.wrapping_shr(j) & 1u64;
            if ! (bit == 0u64)
            {
                let ctx_n0: (&[u64], &[u64]) = ctx_r2.1.split_at(0usize - len1 as usize);
                crate::hacl::bignum::bn_mont_mul_u64(
                    len1,
                    ctx_n0.1,
                    (*uu____0).mu,
                    resM,
                    &aMc,
                    resM
                )
            };
            let ctx_n0: (&[u64], &[u64]) = ctx_r2.1.split_at(0usize - len1 as usize);
            crate::hacl::bignum::bn_mont_sqr_u64(len1, ctx_n0.1, (*uu____0).mu, &aMc, &mut aMc)
        }
    }
    else
    {
        let bLen: u32 =
            if bBits == 0u32
            { 1u32 }
            else
            { bBits.wrapping_sub(1u32).wrapping_div(64u32).wrapping_add(1u32) };
        let mut ctx: Vec<u64> = vec![0u64; len1.wrapping_add(len1) as usize];
        ((&mut ctx)[0usize..len1 as usize]).copy_from_slice(&(&(*uu____0).n)[0usize..len1 as usize]);
        ((&mut ctx)[len1 as usize..len1 as usize + len1 as usize]).copy_from_slice(
            &(&(*uu____0).r2)[0usize..len1 as usize]
        );
        let mut table: Vec<u64> = vec![0u64; 16u32.wrapping_mul(len1) as usize];
        let mut tmp: Vec<u64> = vec![0u64; len1 as usize];
        let t0: (&mut [u64], &mut [u64]) = (&mut table).split_at_mut(0usize);
        let t1: (&mut [u64], &mut [u64]) = t0.1.split_at_mut(len1 as usize);
        let ctx_n: (&[u64], &[u64]) = (&ctx).split_at(0usize);
        let ctx_r2: (&[u64], &[u64]) = ctx_n.1.split_at(len1 as usize);
        crate::hacl::bignum::bn_from_mont_u64(len1, ctx_r2.0, (*uu____0).mu, ctx_r2.1, t1.0);
        (t1.1[0usize..len1 as usize]).copy_from_slice(&(&aMc)[0usize..len1 as usize]);
        crate::lowstar::ignore::ignore::<&[u64]>(&table);
        krml::unroll_for!(
            7,
            "i",
            0u32,
            1u32,
            {
                let t11: (&[u64], &[u64]) =
                    (&table).split_at(i.wrapping_add(1u32).wrapping_mul(len1) as usize);
                let ctx_n0: (&[u64], &[u64]) = ctx_r2.1.split_at(0usize - len1 as usize);
                crate::hacl::bignum::bn_mont_sqr_u64(len1, ctx_n0.1, (*uu____0).mu, t11.1, &mut tmp);
                ((&mut table)[2u32.wrapping_mul(i).wrapping_add(2u32).wrapping_mul(len1) as usize..2u32.wrapping_mul(
                    i
                ).wrapping_add(2u32).wrapping_mul(len1)
                as
                usize
                +
                len1 as usize]).copy_from_slice(&(&tmp)[0usize..len1 as usize]);
                let t2: (&[u64], &[u64]) =
                    (&table).split_at(
                        2u32.wrapping_mul(i).wrapping_add(2u32).wrapping_mul(len1) as usize
                    );
                let ctx_n1: (&[u64], &[u64]) = ctx_n0.1.split_at(0usize);
                crate::hacl::bignum::bn_mont_mul_u64(
                    len1,
                    ctx_n1.1,
                    (*uu____0).mu,
                    &aMc,
                    t2.1,
                    &mut tmp
                );
                ((&mut table)[2u32.wrapping_mul(i).wrapping_add(3u32).wrapping_mul(len1) as usize..2u32.wrapping_mul(
                    i
                ).wrapping_add(3u32).wrapping_mul(len1)
                as
                usize
                +
                len1 as usize]).copy_from_slice(&(&tmp)[0usize..len1 as usize])
            }
        );
        if bBits.wrapping_rem(4u32) != 0u32
        {
            let i: u32 = bBits.wrapping_div(4u32).wrapping_mul(4u32);
            let bits_c: u64 = crate::hacl::bignum_base::bn_get_bits_u64(bLen, b, i, 4u32);
            let bits_l32: u32 = bits_c as u32;
            let a_bits_l: (&[u64], &[u64]) =
                (&table).split_at(bits_l32.wrapping_mul(len1) as usize);
            (resM[0usize..len1 as usize]).copy_from_slice(&a_bits_l.1[0usize..len1 as usize])
        }
        else
        {
            let ctx_n0: (&[u64], &[u64]) = ctx_r2.1.split_at(0usize - len1 as usize);
            let ctx_r20: (&[u64], &[u64]) = ctx_n0.1.split_at(len1 as usize);
            crate::hacl::bignum::bn_from_mont_u64(len1, ctx_r20.0, (*uu____0).mu, ctx_r20.1, resM)
        };
        let mut tmp0: Vec<u64> = vec![0u64; len1 as usize];
        for i in 0u32..bBits.wrapping_div(4u32)
        {
            krml::unroll_for!(
                4,
                "_i",
                0u32,
                1u32,
                {
                    let ctx_n0: (&[u64], &[u64]) = ctx_r2.1.split_at(0usize - len1 as usize);
                    crate::hacl::bignum::bn_mont_sqr_u64(len1, ctx_n0.1, (*uu____0).mu, resM, resM)
                }
            );
            let k2: u32 =
                bBits.wrapping_sub(bBits.wrapping_rem(4u32)).wrapping_sub(4u32.wrapping_mul(i)).wrapping_sub(
                    4u32
                );
            let bits_l: u64 = crate::hacl::bignum_base::bn_get_bits_u64(bLen, b, k2, 4u32);
            crate::lowstar::ignore::ignore::<&[u64]>(&table);
            let bits_l32: u32 = bits_l as u32;
            let a_bits_l: (&[u64], &[u64]) =
                (&table).split_at(bits_l32.wrapping_mul(len1) as usize);
            ((&mut tmp0)[0usize..len1 as usize]).copy_from_slice(&a_bits_l.1[0usize..len1 as usize]);
            let ctx_n0: (&[u64], &[u64]) = ctx_r2.1.split_at(0usize - len1 as usize);
            crate::hacl::bignum::bn_mont_mul_u64(len1, ctx_n0.1, (*uu____0).mu, resM, &tmp0, resM)
        }
    }
}

pub fn inverse(k: &[crate::hacl::bignum::bn_mont_ctx_u64], aM: &[u64], aInvM: &mut [u64])
{
    let uu____0: &crate::hacl::bignum::bn_mont_ctx_u64 = &k[0usize];
    let len1: u32 = (*uu____0).len;
    let mut n2: Vec<u64> = vec![0u64; len1 as usize];
    let c0: u64 =
        crate::lib::inttypes_intrinsics::sub_borrow_u64(
            0u64,
            (&(*uu____0).n)[0usize],
            2u64,
            &mut (&mut n2)[0usize..]
        );
    let c: u64 =
        if 1u32 < len1
        {
            let a1: &[u64] = &(&(*uu____0).n)[1usize..];
            let res1: (&mut [u64], &mut [u64]) = (&mut n2).split_at_mut(1usize);
            let mut c: [u64; 1] = [c0; 1usize];
            for i in 0u32..len1.wrapping_sub(1u32).wrapping_div(4u32)
            {
                let t1: u64 = a1[4u32.wrapping_mul(i) as usize];
                let res_i: (&mut [u64], &mut [u64]) =
                    res1.1.split_at_mut(4u32.wrapping_mul(i) as usize);
                (&mut c)[0usize] =
                    crate::lib::inttypes_intrinsics::sub_borrow_u64((&c)[0usize], t1, 0u64, res_i.1);
                ();
                let t10: u64 = a1[4u32.wrapping_mul(i).wrapping_add(1u32) as usize];
                let res_i0: (&mut [u64], &mut [u64]) = res_i.1.split_at_mut(1usize);
                (&mut c)[0usize] =
                    crate::lib::inttypes_intrinsics::sub_borrow_u64(
                        (&c)[0usize],
                        t10,
                        0u64,
                        res_i0.1
                    );
                ();
                let t11: u64 = a1[4u32.wrapping_mul(i).wrapping_add(2u32) as usize];
                let res_i1: (&mut [u64], &mut [u64]) = res_i0.1.split_at_mut(1usize);
                (&mut c)[0usize] =
                    crate::lib::inttypes_intrinsics::sub_borrow_u64(
                        (&c)[0usize],
                        t11,
                        0u64,
                        res_i1.1
                    );
                ();
                let t12: u64 = a1[4u32.wrapping_mul(i).wrapping_add(3u32) as usize];
                let res_i2: (&mut [u64], &mut [u64]) = res_i1.1.split_at_mut(1usize);
                (&mut c)[0usize] =
                    crate::lib::inttypes_intrinsics::sub_borrow_u64(
                        (&c)[0usize],
                        t12,
                        0u64,
                        res_i2.1
                    );
                ();
                ();
                ()
            };
            ();
            for
            i
            in
            len1.wrapping_sub(1u32).wrapping_div(4u32).wrapping_mul(4u32)..len1.wrapping_sub(1u32)
            {
                let t1: u64 = a1[i as usize];
                let res_i: (&mut [u64], &mut [u64]) = res1.1.split_at_mut(i as usize);
                (&mut c)[0usize] =
                    crate::lib::inttypes_intrinsics::sub_borrow_u64((&c)[0usize], t1, 0u64, res_i.1);
                ();
                ()
            };
            let c1: u64 = (&c)[0usize];
            c1
        }
        else
        { c0 };
    crate::lowstar::ignore::ignore::<u64>(c);
    exp_vartime(k, aM, (*uu____0).len.wrapping_mul(64u32), &n2, aInvM)
}
