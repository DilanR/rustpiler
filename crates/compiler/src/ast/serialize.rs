use serde::{Serialize, ser::SerializeStruct};

use crate::{ast::Spanned, error::ErrRange};

impl<T> Serialize for Spanned<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Spanned", 2)?;

        state.serialize_field("node", &self.node)?;

        let range = ErrRange::from(self.span);

        state.serialize_field("span", &range)?;

        state.end()
    }
}
