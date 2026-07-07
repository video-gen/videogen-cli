pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An avatar presenter available for video generation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AvatarPresenter {
    /// Presenter id (e.g. `vg_pres_...`). Pass as `avatarPresenterId` to `POST /v1/tools/generate-avatar`.
    #[serde(rename = "avatarPresenterId")]
    #[serde(default)]
    pub avatar_presenter_id: String,
    /// Presenter gender.
    #[serde(rename = "displayableGender")]
    pub displayable_gender: AvatarPresenterDisplayableGender,
    /// Still image of the presenter.
    #[serde(rename = "imageUrl")]
    #[serde(default)]
    pub image_url: String,
    /// Thumbnail image of the presenter.
    #[serde(rename = "thumbnailUrl")]
    #[serde(default)]
    pub thumbnail_url: String,
    /// Short preview clip of the presenter speaking.
    #[serde(rename = "previewVideoUrl")]
    #[serde(default)]
    pub preview_video_url: String,
}

impl AvatarPresenter {
    pub fn builder() -> AvatarPresenterBuilder {
        <AvatarPresenterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AvatarPresenterBuilder {
    avatar_presenter_id: Option<String>,
    displayable_gender: Option<AvatarPresenterDisplayableGender>,
    image_url: Option<String>,
    thumbnail_url: Option<String>,
    preview_video_url: Option<String>,
}

impl AvatarPresenterBuilder {
    pub fn avatar_presenter_id(mut self, value: impl Into<String>) -> Self {
        self.avatar_presenter_id = Some(value.into());
        self
    }

    pub fn displayable_gender(mut self, value: AvatarPresenterDisplayableGender) -> Self {
        self.displayable_gender = Some(value);
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn thumbnail_url(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_url = Some(value.into());
        self
    }

    pub fn preview_video_url(mut self, value: impl Into<String>) -> Self {
        self.preview_video_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AvatarPresenter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`avatar_presenter_id`](AvatarPresenterBuilder::avatar_presenter_id)
    /// - [`displayable_gender`](AvatarPresenterBuilder::displayable_gender)
    /// - [`image_url`](AvatarPresenterBuilder::image_url)
    /// - [`thumbnail_url`](AvatarPresenterBuilder::thumbnail_url)
    /// - [`preview_video_url`](AvatarPresenterBuilder::preview_video_url)
    pub fn build(self) -> Result<AvatarPresenter, BuildError> {
        Ok(AvatarPresenter {
            avatar_presenter_id: self.avatar_presenter_id.ok_or_else(|| BuildError::missing_field("avatar_presenter_id"))?,
            displayable_gender: self.displayable_gender.ok_or_else(|| BuildError::missing_field("displayable_gender"))?,
            image_url: self.image_url.ok_or_else(|| BuildError::missing_field("image_url"))?,
            thumbnail_url: self.thumbnail_url.ok_or_else(|| BuildError::missing_field("thumbnail_url"))?,
            preview_video_url: self.preview_video_url.ok_or_else(|| BuildError::missing_field("preview_video_url"))?,
        })
    }
}
