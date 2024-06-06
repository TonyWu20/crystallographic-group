use crate::hall_symbols::SEITZ_TRANSLATE_BASE_NUMBER;

/// Get the positive mod against `SEITZ_TRANSLATE_BASE_NUMBER` (12)
/// for i32
pub(crate) fn positive_mod_stbn_i32(val: i32) -> i32 {
    if val < 0 {
        val % SEITZ_TRANSLATE_BASE_NUMBER + SEITZ_TRANSLATE_BASE_NUMBER
    } else {
        val % SEITZ_TRANSLATE_BASE_NUMBER
    }
}

/// Get the positive mod against `SEITZ_TRANSLATE_BASE_NUMBER` (12)
/// for f64
pub(crate) fn positive_mod_stbn_f64(val: f64) -> f64 {
    if val < 0_f64 {
        val % SEITZ_TRANSLATE_BASE_NUMBER as f64 + SEITZ_TRANSLATE_BASE_NUMBER as f64
    } else {
        val % SEITZ_TRANSLATE_BASE_NUMBER as f64
    }
}
