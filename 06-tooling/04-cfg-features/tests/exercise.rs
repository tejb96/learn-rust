// DO NOT EDIT — implement the solution in src/lib.rs

use cfg_features::{bonus_value, feature_label};

#[test]
fn default_feature_label() {
    assert_eq!(feature_label(), "default");
}

#[test]
fn default_bonus_zero() {
    assert_eq!(bonus_value(), 0);
}

#[cfg(feature = "enabled")]
mod enabled_tests {
    use super::*;

    #[test]
    fn enabled_feature_label() {
        assert_eq!(feature_label(), "enabled");
    }

    #[test]
    fn enabled_bonus() {
        assert_eq!(bonus_value(), 42);
    }
}
