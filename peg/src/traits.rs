pub trait Bounded {
    fn minimum() -> Self;
    fn maximum() -> Self;
}

impl Bounded for char {
    fn minimum() -> Self {
        unsafe { char::from_u32_unchecked(0) }
    }

    fn maximum() -> Self {
        Self::MAX
    }
}
