use vhr_serde::de::VHDeserializer;
use vhr_serde::ser::VHSerializer;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(from = "WrapperArray", into = "WrapperArray")]
pub struct Wrapper<T: Wrapped> {
    pub inner: T,
}

impl<T> Wrapper<T>
where
    T: Wrapped,
{
    pub fn wrap(inner: T) -> Wrapper<T> {
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

impl<'de, T> From<WrapperArray> for Wrapper<T>
where
    T: Wrapped + Deserialize<'de>,
{
    fn from(wrapper: WrapperArray) -> Wrapper<T> {
        // // let length = wrapper.inner.len() as u32;
        // let mut deserializer = VHDeserializer::from_owned(wrapper.inner, ());
        // let inner = <T as Deserialize>::deserialize(&mut deserializer).unwrap();
        // Wrapper { inner }
        <T as Wrapped>::strip(wrapper)
    }
}

impl<T> From<Wrapper<T>> for WrapperArray
where
    T: Wrapped,
{
    fn from(wrapper: Wrapper<T>) -> WrapperArray {
        // let inner  = {
        //     let mut serializer = VHSerializer::new();
        //     <T as Serialize>::serialize(&wrapper.inner, &mut serializer).unwrap();
        //     serializer.to_inner()
        // };
        // WrapperArray {
        //     inner,
        // }
        <T as Wrapped>::wrap(wrapper)
    }
}

/// trait to assist with unwrapping wrapped elements
/// normally this could be auto-implemented for traits that implement deserialize, but
/// everything i'm doing implements deserialize so rust doesn't know what to do
/// plus sometimes you need to tweak some values when converting,
/// so this trait gives you that control.
/// a normal implementation would look like:
/// ````
/// type MyType = ();
/// impl Wrapped for MyType {
///     fn strip(wrapper: WrapperArray) -> Wrapper<MyType> {
///         let mut deserializer = VHDeserializer::from_owned(wrapper.inner, ());
///         let inner = <MyType as Deserialize>::deserialize(&mut deserializer).unwrap();
///         Wrapper { inner }
///     }
///     fn wrap(item: Wrapper<MyType>) -> WrapperArray {
///         let inner = {
///             let mut serializer = VHSerializer::new();
///             <MyType as Serialize>::serialize(&item.inner, &mut serializer).unwrap();
///             serializer.to_inner()
///         };
///         WrapperArray { inner }
///     }
/// }
/// ````
pub trait Wrapped: Clone + Serialize {
    fn strip(wrapper: WrapperArray) -> Wrapper<Self>;
    fn wrap(item: Wrapper<Self>) -> WrapperArray;
}
