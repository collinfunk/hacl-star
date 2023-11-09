fn sha224_init4(hash: &mut [crate::lib::intvector_intrinsics::vec128]) -> ()
{
    for i in 0u32..8u32
    {
        let
        os:
        (&mut [crate::lib::intvector_intrinsics::vec128],
        &mut [crate::lib::intvector_intrinsics::vec128])
        =
            hash.split_at_mut(0usize);
        let hi: u32 = (&crate::hacl::hash_sha2::h224)[i as usize];
        let x: crate::lib::intvector_intrinsics::vec128 =
            crate::lib::intvector_intrinsics::vec128_load32(hi);
        os.1[i as usize] = x
    }
}

fn sha256_init4(hash: &mut [crate::lib::intvector_intrinsics::vec128]) -> ()
{
    for i in 0u32..8u32
    {
        let
        os:
        (&mut [crate::lib::intvector_intrinsics::vec128],
        &mut [crate::lib::intvector_intrinsics::vec128])
        =
            hash.split_at_mut(0usize);
        let hi: u32 = (&crate::hacl::hash_sha2::h256)[i as usize];
        let x: crate::lib::intvector_intrinsics::vec128 =
            crate::lib::intvector_intrinsics::vec128_load32(hi);
        os.1[i as usize] = x
    }
}
