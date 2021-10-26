use vhr_serde::de::VHDeserializer;

use super::known_size::KnownSize;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(from = "WrapperArray")]
pub struct Wrapper<T> where T: FromWrapper {
    pub length: u32,
    pub inner: T,
}

impl<T> Wrapper<T> where T: FromWrapper {
    pub fn from_inner(inner: T) -> Wrapper<T>
    where
        T: KnownSize,
    {
        let length = <T as KnownSize>::count_bytes(&inner) as u32;
        Wrapper { length, inner }
    }
}

impl<T> KnownSize for Wrapper<T>
where
    T: KnownSize + FromWrapper,
{
    fn count_bytes(&self) -> usize {
        <T as KnownSize>::count_bytes(&self.inner) + 4
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct WrapperArray {
    pub inner: Vec<u8>,
}

impl WrapperArray {
    pub fn get_vhrd<O>(&self, options: O) -> VHDeserializer<O> {
        VHDeserializer::from_bytes_options(&self.inner, options)
    }
}

impl<T> From<WrapperArray> for Wrapper<T> where T: FromWrapper {
    fn from(wrapper: WrapperArray) -> Wrapper<T> {
        let length = wrapper.inner.len() as u32;
        let inner = FromWrapper::from(wrapper);
        Wrapper { length, inner }
    }
}

pub trait FromWrapper {
    fn from(wrapper: WrapperArray) -> Self;
}
