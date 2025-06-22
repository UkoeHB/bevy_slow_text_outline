use bevy::prelude::*;
use bevy::render::RenderApp;
use bevy::ui::RenderUiSystem;

use crate::text_outline_rendering::{extract_text_outlines, TextOutlineMaxWidth};

//-------------------------------------------------------------------------------------------------------------------

pub struct SlowTextOutlinePlugin
{
    pub max_width: u16,
}

impl Plugin for SlowTextOutlinePlugin
{
    fn build(&self, app: &mut App)
    {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .insert_resource(TextOutlineMaxWidth { max_width: self.max_width })
            .add_systems(
                ExtractSchedule,
                // Outlines last so they render above shadows.
                extract_text_outlines
                    .after(bevy::ui::extract_text_shadows)
                    .in_set(RenderUiSystem::ExtractTextShadows),
            );
    }
}

impl Default for SlowTextOutlinePlugin
{
    fn default() -> Self
    {
        Self { max_width: 8u16 }
    }
}

//-------------------------------------------------------------------------------------------------------------------
