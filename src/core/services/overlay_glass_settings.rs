pub struct OverlayGlassSettings;

impl OverlayGlassSettings {
    pub const DEFAULT_GLASS_CLARITY: i32 = 75;

    pub fn normalize_glass_clarity(clarity: i32) -> i32 {
        clarity.clamp(0, 150)
    }

    pub fn background_tint_opacity_from_clarity(clarity: i32) -> f64 {
        let normalized = Self::normalize_glass_clarity(clarity) as f64;
        // 0 -> 0.85 (solid), 150 -> 0.0 (clear)
        0.85 - (normalized / 150.0) * 0.85
    }

    pub fn blur_radius_from_clarity(clarity: i32) -> f64 {
        let normalized = Self::normalize_glass_clarity(clarity) as f64;
        // 0 -> 0 (solid), 150 -> 48 (max blur)
        (normalized / 150.0) * 48.0
    }

    pub fn content_opacity() -> f64 {
        1.0
    }
}