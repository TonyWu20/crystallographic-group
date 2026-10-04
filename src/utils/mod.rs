use crate::hall_symbols::SEITZ_TRANSLATE_BASE_NUMBER;

/// Get the positive residue of an `i32` value against
/// `SEITZ_TRANSLATE_BASE_NUMBER` (12).
pub(crate) fn positive_mod_stbn_i32(val: i32) -> i32 {
    if val < 0 {
        val % SEITZ_TRANSLATE_BASE_NUMBER + SEITZ_TRANSLATE_BASE_NUMBER
    } else {
        val % SEITZ_TRANSLATE_BASE_NUMBER
    }
}
