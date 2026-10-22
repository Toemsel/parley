// Foldaria (B099): a renderer's own glyph advances in place of the font's natural ones.
//
// Slint's software renderer draws glyphs with DirectWrite GDI-classic rendering, which fits each
// glyph to a whole-pixel GDI width. The layout has to space glyphs by those same widths, or the
// drawn text drifts from the caret, the selection and the elision computed from the layout.

use crate::FontData;

/// For a glyph of `font` at `font_size` (with the run's `synthesis`): its advance as the renderer
/// draws it and its natural advance at `font_size`, both in layout units; `None` keeps the font's.
pub type GlyphAdvanceHook =
    fn(font: &FontData, font_size: f32, synthesis: &fontique::Synthesis, glyph_id: u16) -> Option<(f32, f32)>;

#[cfg(feature = "std")]
static HOOK: std::sync::OnceLock<GlyphAdvanceHook> = std::sync::OnceLock::new();

/// Sets the hook used by every layout shaped from now on (the first one set stays; needs `std`).
pub fn set_glyph_advance_hook(hook: GlyphAdvanceHook) {
    #[cfg(feature = "std")]
    HOOK.set(hook).ok();
    #[cfg(not(feature = "std"))]
    let _ = hook;
}

pub(crate) fn glyph_advance_hook() -> Option<GlyphAdvanceHook> {
    #[cfg(feature = "std")]
    return HOOK.get().copied();
    #[cfg(not(feature = "std"))]
    return None;
}
