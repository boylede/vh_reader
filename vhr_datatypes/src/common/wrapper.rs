use vhr_serde::de::VHDeserializer;
use vhr_serde::ser::VHSerializer;


use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(from = "WrapperArray", into="WrapperArray")]
pub struct Wrapper<T> where T: Serialize + Clone{
    // pub length: u32,
    pub inner: T,
}

impl<T> Wrapper<T> where T: Serialize + Clone {
    pub fn from_inner(inner: T) -> Wrapper<T>
    where
        T: Serialize,
    {
        Wrapper { inner }

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

impl<'de, T> From<WrapperArray> for Wrapper<T> where T: Serialize + Deserialize<'de> + Clone {
    fn from(wrapper: WrapperArray) -> Wrapper<T> {
        // let length = wrapper.inner.len() as u32;
        let mut deserializer = VHDeserializer::from_owned(wrapper.inner, ());
        let inner = <T as Deserialize>::deserialize(&mut deserializer).unwrap();
        Wrapper { inner }
    }
}

impl<'de, T> From<Wrapper<T>> for WrapperArray  where T: Serialize + Clone {
    fn from(wrapper: Wrapper<T>) -> WrapperArray {
        let inner  = { 
            let mut serializer = VHSerializer::new();
            <T as Serialize>::serialize(&wrapper.inner, &mut serializer).unwrap();
            serializer.to_inner()
        };
        WrapperArray {
            inner,
        }
    }
}
