use bevy::prelude::*;
use bevy::render::sync_world::TemporaryRenderEntity;
use bevy::render::Extract;
use bevy::sprite::{
    Anchor, ExtractedSlice, ExtractedSlices, ExtractedSprite, ExtractedSpriteKind, ExtractedSprites,
};
use bevy::text::{PositionedGlyph, TextBounds, TextLayoutInfo};
use bevy::ui::{ExtractedGlyph, ExtractedUiItem, ExtractedUiNode, ExtractedUiNodes, UiCameraMap};
use bevy::window::PrimaryWindow;

use crate::prelude::TextOutline;

//-------------------------------------------------------------------------------------------------------------------

fn spawn_text_outline_shadows<G>(
    start: &mut usize,
    scale_factor: f32,
    max_width: u16,
    outline: &TextOutline,
    text_layout_info: &TextLayoutInfo,
    texture_atlases: &Assets<TextureAtlasLayout>,
    aa_cache: &mut Vec<G>,
    make_glyph: impl Fn(Vec2, Vec2, Rect) -> G,
    mut add_glyph: impl FnMut(G),
    mut add_batch: impl FnMut(Rect, LinearRgba, AssetId<Image>, usize, usize),
)
{
    let preclamped_width = (outline.width * scale_factor).ceil() as i32;
    let width = preclamped_width.min(max_width as i32);
    let width_pow2 = width.pow(2);
    let aa_factor = outline.anti_aliasing.unwrap_or(1.0);
    let color: LinearRgba = outline.color.into();
    let mut aa_color = color;
    aa_color.alpha *= aa_factor;
    let mut len = 0;

    for (i, PositionedGlyph { position, atlas_info, .. }) in text_layout_info.glyphs.iter().enumerate() {
        let rect = texture_atlases
            .get(&atlas_info.texture_atlas)
            .unwrap()
            .textures[atlas_info.location.glyph_index]
            .as_rect();

        for offset_x in -width..=width {
            // Adjust height to follow a radial pattern.
            let height = ((width_pow2 - offset_x.pow(2)).abs() as f32).sqrt().ceil() as i32;

            for offset_y in -height..=height {
                if offset_x == 0 && offset_y == 0 {
                    continue;
                }

                let offset = Vec2 { x: offset_x as f32, y: offset_y as f32 };

                let glyph = (make_glyph)(offset, *position, rect);

                if aa_factor != 1.0 && offset_y.abs() == height {
                    aa_cache.push(glyph);
                } else {
                    (add_glyph)(glyph);
                    len += 1;
                }
            } // y offset
        } // x offset

        if text_layout_info
            .glyphs
            .get(i + 1)
            .is_none_or(|info| info.atlas_info.texture != atlas_info.texture)
        {
            if len > 0 {
                (add_batch)(rect, color, atlas_info.texture.id(), *start, len);
                *start += len;
                len = 0;
            }

            let aa_len = aa_cache.len();
            for aa_glyph in aa_cache.drain(..) {
                (add_glyph)(aa_glyph);
            }

            if aa_len > 0 {
                (add_batch)(rect, aa_color, atlas_info.texture.id(), *start, aa_len);
                *start += aa_len;
            }
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource)]
pub struct TextOutlineMaxWidth
{
    pub(crate) max_width: u16,
}

//-------------------------------------------------------------------------------------------------------------------

pub fn extract_ui_text_outlines(
    mut aa_glyph_cache: Local<Vec<ExtractedGlyph>>,
    mut commands: Commands,
    max_width: Res<TextOutlineMaxWidth>,
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    texture_atlases: Extract<Res<Assets<TextureAtlasLayout>>>,
    uinode_query: Extract<
        Query<(
            Entity,
            &ComputedNode,
            &ComputedNodeTarget,
            &GlobalTransform,
            &InheritedVisibility,
            Option<&CalculatedClip>,
            &TextLayoutInfo,
            &TextOutline,
        )>,
    >,
    camera_map: Extract<UiCameraMap>,
)
{
    aa_glyph_cache.clear();
    let max_width = max_width.max_width;
    let mut start = extracted_uinodes.glyphs.len();
    let ExtractedUiNodes { glyphs, uinodes, .. } = &mut *extracted_uinodes;

    let mut camera_mapper = camera_map.get_mapper();
    for (entity, uinode, target, global_transform, inherited_visibility, clip, text_layout_info, outline) in
        &uinode_query
    {
        // Skip if not visible or if size is set to zero (e.g. when a parent is set to `Display::None`)
        if !inherited_visibility.get() || uinode.is_empty() || outline.width == 0.0 {
            continue;
        }

        let Some(extracted_camera_entity) = camera_mapper.map(target) else {
            continue;
        };

        spawn_text_outline_shadows::<ExtractedGlyph>(
            &mut start,
            1.0 / uinode.inverse_scale_factor(),
            max_width,
            outline,
            text_layout_info,
            &texture_atlases,
            &mut aa_glyph_cache,
            |offset, position, rect| {
                let transform =
                    global_transform.affine() * Mat4::from_translation((-0.5 * uinode.size() + offset).extend(0.));
                ExtractedGlyph {
                    transform: transform * Mat4::from_translation(position.extend(0.)),
                    rect,
                }
            },
            |glyph| {
                glyphs.push(glyph);
            },
            |rect, color, image, start, len| {
                uinodes.push(ExtractedUiNode {
                    render_entity: commands.spawn(TemporaryRenderEntity).id(),
                    stack_index: uinode.stack_index,
                    color,
                    image,
                    clip: clip.map(|clip| clip.clip),
                    extracted_camera_entity,
                    rect,
                    item: ExtractedUiItem::Glyphs { range: start..(start + len) },
                    main_entity: entity.into(),
                });
            },
        );
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub fn extract_2d_text_outlines(
    mut aa_slice_cache: Local<Vec<ExtractedSlice>>,
    mut commands: Commands,
    max_width: Res<TextOutlineMaxWidth>,
    mut extracted_sprites: ResMut<ExtractedSprites>,
    mut extracted_slices: ResMut<ExtractedSlices>,
    texture_atlases: Extract<Res<Assets<TextureAtlasLayout>>>,
    windows: Extract<Query<&Window, With<PrimaryWindow>>>,
    text2d_query: Extract<
        Query<(
            Entity,
            &ViewVisibility,
            &TextLayoutInfo,
            &TextBounds,
            &Anchor,
            &GlobalTransform,
            &TextOutline,
        )>,
    >,
)
{
    aa_slice_cache.clear();
    let max_width = max_width.max_width;
    let mut start = extracted_slices.slices.len();

    // TODO: Support window-independent scaling: https://github.com/bevyengine/bevy/issues/5621
    let scale_factor = windows
        .single()
        .map(|window| window.resolution.scale_factor())
        .unwrap_or(1.0);
    let scaling = GlobalTransform::from_scale(Vec2::splat(scale_factor.recip()).extend(1.));

    for (main_entity, view_visibility, text_layout_info, text_bounds, anchor, global_transform, outline) in
        text2d_query.iter()
    {
        if !view_visibility.get() || outline.width == 0.0 {
            continue;
        }

        let size = Vec2::new(
            text_bounds.width.unwrap_or(text_layout_info.size.x),
            text_bounds.height.unwrap_or(text_layout_info.size.y),
        );

        let top_left = (Anchor::TopLeft.as_vec() - anchor.as_vec()) * size;
        let transform = *global_transform * GlobalTransform::from_translation(top_left.extend(0.)) * scaling;

        spawn_text_outline_shadows::<ExtractedSlice>(
            &mut start,
            scale_factor,
            max_width,
            outline,
            text_layout_info,
            &texture_atlases,
            &mut aa_slice_cache,
            |offset, position, rect| {
                // TODO: change to -position.y in bevy v0.17 ?
                ExtractedSlice {
                    offset: Vec2::new(position.x, position.y - 2. * size.y) + offset,
                    rect,
                    size: rect.size(),
                }
            },
            |glyph| {
                extracted_slices.slices.push(glyph);
            },
            |_rect, color, image, start, len| {
                let render_entity = commands.spawn(TemporaryRenderEntity).id();
                extracted_sprites.sprites.push(ExtractedSprite {
                    main_entity,
                    render_entity,
                    transform,
                    color,
                    image_handle_id: image,
                    flip_x: false,
                    flip_y: false,
                    kind: ExtractedSpriteKind::Slices { indices: start..(start + len) },
                });
            },
        );
    }
}

//-------------------------------------------------------------------------------------------------------------------
