use bevy::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

/// Component for adding an outline around text.
///
/// Does *not* interfere with existing [`TextShadow`] components on the entity. The
/// outline will be 'above' any shadows.
#[derive(Component, Reflect, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct TextOutline
{
    /// Width of the outline in pixels.
    ///
    /// The actual width of the shadow will be adjusted by scaling factors. For example, Retina display Macs have
    /// a scaling factor of `2`. If your width is `2``, scaling factor is `2``, and `UiScale` is `2`, then
    /// the actual scaling-adjusted width will be `8`.
    ///
    /// Scaling-adjusted width is currently capped at `8` pixels to avoid catastrophic performance degredation.
    pub width: f32,
    /// Defaults to [`Color::BLACK`].
    #[reflect(default = "TextOutline::default_color")]
    pub color: Color,
    /// Multiplied to the alpha of the outermost layer of shadows that make up the outline. The effect is to
    /// soften corners and arcs while leaving straight edges dark.
    ///
    /// Defaults to no anti-aliasing.
    #[reflect(default)]
    pub anti_aliasing: Option<f32>,
}

impl TextOutline
{
    fn default_color() -> Color
    {
        Color::BLACK
    }
}

impl Default for TextOutline
{
    fn default() -> Self
    {
        Self {
            width: 0.0,
            color: Self::default_color(),
            anti_aliasing: None,
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------
