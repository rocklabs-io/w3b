pub struct HexNumeric<T>(T);

impl<T> HexNumeric<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self(inner)
    }

    #[inline]
    pub fn inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for HexNumeric<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self(value)
    }
}

macro_rules! impl_num {
    ($num:ident) => {
        impl $crate::serde::Serialize for HexNumeric<$num> {
            #[inline]
            fn serialize<S: $crate::serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                $crate::core::hex::serialize(self.0.to_be_bytes().as_ref(), serializer)
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for HexNumeric<$num> {
            #[inline]
            fn deserialize<D: $crate::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                let mut bytes = [0; ::std::mem::size_of::<$num>()];
                $crate::core::hex::deserialize_expanded(&mut bytes, deserializer)?;
                Ok(Self($num::from_be_bytes(bytes)))
            }
        }
    };
}

impl_num!(i8);
impl_num!(i16);
impl_num!(i32);
impl_num!(i64);

impl_num!(u8);
impl_num!(u16);
impl_num!(u32);
impl_num!(u64);

#[cfg(has_i128)]
const _I128_IMPLS: () = {
    impl_num!(i128);
    impl_num!(u128);
};