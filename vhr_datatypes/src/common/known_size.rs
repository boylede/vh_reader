pub trait KnownSize {
    fn count_bytes(&self) -> usize;
}

impl<T> KnownSize for Vec<T>
where
    T: KnownSize,
{
    fn count_bytes(&self) -> usize {
        self.iter()
            .map(|i| <T as KnownSize>::count_bytes(i))
            .sum::<usize>()
            + 4
    }
}

impl<T> KnownSize for Option<T>
where
    T: KnownSize,
{
    fn count_bytes(&self) -> usize {
        match self {
            Some(i) => 1 + <T as KnownSize>::count_bytes(i),
            None => 1,
        }
    }
}

impl KnownSize for String {
    fn count_bytes(&self) -> usize {
        // todo: longer than 128 strings
        1 + self.len()
    }
}

impl KnownSize for u32 {
    fn count_bytes(&self) -> usize {
        4
    }
}

impl KnownSize for u8 {
    fn count_bytes(&self) -> usize {
        1
    }
}

impl KnownSize for f32 {
    fn count_bytes(&self) -> usize {
        4
    }
}

impl KnownSize for bool {
    fn count_bytes(&self) -> usize {
        1
    }
}

impl<T, E> KnownSize for (T, E)
where
    T: KnownSize,
    E: KnownSize,
{
    fn count_bytes(&self) -> usize {
        <T as KnownSize>::count_bytes(&self.0) + <E as KnownSize>::count_bytes(&self.1)
    }
}

impl<T, E, F> KnownSize for (T, E, F)
where
    T: KnownSize,
    E: KnownSize,
    F: KnownSize,
{
    fn count_bytes(&self) -> usize {
        <T as KnownSize>::count_bytes(&self.0)
            + <E as KnownSize>::count_bytes(&self.1)
            + <F as KnownSize>::count_bytes(&self.2)
    }
}

impl<T, E, F, G> KnownSize for (T, E, F, G)
where
    T: KnownSize,
    E: KnownSize,
    F: KnownSize,
    G: KnownSize,
{
    fn count_bytes(&self) -> usize {
        <T as KnownSize>::count_bytes(&self.0)
            + <E as KnownSize>::count_bytes(&self.1)
            + <F as KnownSize>::count_bytes(&self.2)
            + <G as KnownSize>::count_bytes(&self.3)
    }
}
