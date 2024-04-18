use cosmic_text::{CacheKey, CacheKeyFlags, LayoutGlyph, LayoutRun};

use itertools::Itertools;
use massive_geometry::Color;
use massive_shapes::{GlyphRun, GlyphRunMetrics, PositionedGlyph};

const RENDER_SUBPIXEL: bool = false;

/// Converts a cosmic_text `LayoutRun` into one or more `GlyphRun`s.
///
/// We split `LayoutRun`s if they contain different metadata which points to a color.
pub fn to_colored_glyph_runs(run: &LayoutRun, line_height: f32, colors: &[Color]) -> Vec<GlyphRun> {
    let metrics = metrics(run, line_height);

    run.glyphs
        .iter()
        .group_by(|r| r.metadata)
        .into_iter()
        .map(|(metadata, run)| {
            let positioned = run.map(position_glyph);
            GlyphRun::new(metrics, colors[metadata], positioned.collect())
        })
        .collect()
}

pub fn to_glyph_run(run: &LayoutRun, line_height: f32) -> GlyphRun {
    let metrics = metrics(run, line_height);
    let positioned = position_glyphs(run.glyphs);
    GlyphRun::new(metrics, Color::BLACK, positioned)
}

fn metrics(run: &LayoutRun, line_height: f32) -> GlyphRunMetrics {
    let max_ascent = run.line_y - run.line_top;

    GlyphRunMetrics {
        max_ascent: max_ascent.ceil() as _,
        max_descent: (line_height - max_ascent).ceil() as _,
        width: run.line_w.ceil() as u32,
    }
}

/// Position individual `LayoutGlyph` from a `LayoutRun`.
pub fn position_glyphs(glyphs: &[LayoutGlyph]) -> Vec<PositionedGlyph> {
    glyphs.iter().map(position_glyph).collect()
}

fn position_glyph(glyph: &LayoutGlyph) -> PositionedGlyph {
    let fractional_pos = if RENDER_SUBPIXEL {
        (glyph.x, glyph.y)
    } else {
        (glyph.x.round(), glyph.y.round())
    };

    let (ck, x, y) = CacheKey::new(
        glyph.font_id,
        glyph.glyph_id,
        glyph.font_size,
        fractional_pos,
        CacheKeyFlags::empty(),
    );
    // Note: hitbox with is fractional, but does not change with / without subpixel
    // rendering.
    PositionedGlyph::new(ck, (x, y), glyph.w)
}
