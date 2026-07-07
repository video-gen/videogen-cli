pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An RGB color. Each channel is an integer from 0 to 255.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowRgbColor {
    #[serde(default)]
    pub red: i64,
    #[serde(default)]
    pub green: i64,
    #[serde(default)]
    pub blue: i64,
}

impl WorkflowRgbColor {
    pub fn builder() -> WorkflowRgbColorBuilder {
        <WorkflowRgbColorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowRgbColorBuilder {
    red: Option<i64>,
    green: Option<i64>,
    blue: Option<i64>,
}

impl WorkflowRgbColorBuilder {
    pub fn red(mut self, value: i64) -> Self {
        self.red = Some(value);
        self
    }

    pub fn green(mut self, value: i64) -> Self {
        self.green = Some(value);
        self
    }

    pub fn blue(mut self, value: i64) -> Self {
        self.blue = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowRgbColor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`red`](WorkflowRgbColorBuilder::red)
    /// - [`green`](WorkflowRgbColorBuilder::green)
    /// - [`blue`](WorkflowRgbColorBuilder::blue)
    pub fn build(self) -> Result<WorkflowRgbColor, BuildError> {
        Ok(WorkflowRgbColor {
            red: self.red.ok_or_else(|| BuildError::missing_field("red"))?,
            green: self.green.ok_or_else(|| BuildError::missing_field("green"))?,
            blue: self.blue.ok_or_else(|| BuildError::missing_field("blue"))?,
        })
    }
}
