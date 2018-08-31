macro_rules! wrapper_serde_decl {
	($type:tt) => {
		impl ::serde::Serialize for $type {
			fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
			where
				S: ::serde::Serializer,
			{
				self.0.serialize(ser)
			}
		}

		impl<'de> ::serde::Deserialize<'de> for $type {
			fn deserialize<D>(de: D) -> Result<Self, D::Error>
			where
				D: ::serde::Deserializer<'de>,
			{
				Ok($type(::serde::Deserialize::deserialize(de)?))
			}
		}
	};
}