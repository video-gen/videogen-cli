pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Aspect ratio as a width:height pair (e.g. 16 and 9 for 16:9). Not pixel dimensions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AspectRatio {
    #[serde(default)]
    pub width: i64,
    #[serde(default)]
    pub height: i64,
}

impl AspectRatio {
    pub fn builder() -> AspectRatioBuilder {
        <AspectRatioBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AspectRatioBuilder {
    width: Option<i64>,
    height: Option<i64>,
}

impl AspectRatioBuilder {
    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AspectRatio`].
    /// This method will fail if any of the following fields are not set:
    /// - [`width`](AspectRatioBuilder::width)
    /// - [`height`](AspectRatioBuilder::height)
    pub fn build(self) -> Result<AspectRatio, BuildError> {
        Ok(AspectRatio {
            width: self.width.ok_or_else(|| BuildError::missing_field("width"))?,
            height: self.height.ok_or_else(|| BuildError::missing_field("height"))?,
        })
    }
}
