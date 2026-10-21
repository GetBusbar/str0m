/// Random values for identifiers (STUN transaction ids, the ICE tie-breaker, SSRCs and the like).
///
/// Each value is drawn from the OS (`getrandom`), a stateless call: no generator state outlives a
/// call, none per thread and none per process, so an `Rtc` keeps no random state outside itself.
pub struct NonCryptographicRng;

/// `N` bytes from the OS.
fn os<const N: usize>() -> [u8; N] {
    let mut b = [0_u8; N];
    NonCryptographicRng::fill(&mut b);
    b
}

impl NonCryptographicRng {
    /// Fill `buf` from the OS.
    ///
    /// # Panics
    ///
    /// When the OS randomness source fails.
    pub fn fill(buf: &mut [u8]) {
        getrandom::fill(buf).expect("the OS randomness source failed");
    }

    #[inline(always)]
    pub fn u8() -> u8 {
        os::<1>()[0]
    }

    #[inline(always)]
    pub fn u16() -> u16 {
        u16::from_ne_bytes(os())
    }

    #[inline(always)]
    pub fn u32() -> u32 {
        u32::from_ne_bytes(os())
    }

    #[inline(always)]
    pub fn u64() -> u64 {
        u64::from_ne_bytes(os())
    }

    /// Uniform in `[0, 1)`: 24 random bits, each value exact in an `f32`.
    #[inline(always)]
    pub fn f32() -> f32 {
        (Self::u32() >> 8) as f32 / (1_u32 << 24) as f32
    }
}
