#![forbid(unsafe_code)]
//! Objective direction and objective value helpers.
//!
//! The crate stays intentionally small and focuses on common helpers for
//! choosing better `f64` values without implementing `Ord` over floating-point
//! inputs.
//!
//! # Examples
//!
//! ```rust
//! use use_objective::{best_value, is_better, ObjectiveDirection, ObjectiveValue};
//!
//! assert!(is_better(3.0, 5.0, ObjectiveDirection::Minimize));
//! assert_eq!(best_value(&[1.0, 4.0, 2.0], ObjectiveDirection::Maximize), Some(4.0));
//!
//! let candidate = ObjectiveValue::new(3.0, ObjectiveDirection::Minimize).unwrap();
//! let incumbent = ObjectiveValue::new(4.0, ObjectiveDirection::Minimize).unwrap();
//! assert_eq!(candidate.is_better_than(&incumbent), Some(true));
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectiveDirection {
    Minimize,
    Maximize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectiveValue {
    value: f64,
    direction: ObjectiveDirection,
}

impl ObjectiveValue {
    pub fn new(value: f64, direction: ObjectiveDirection) -> Option<Self> {
        if !value.is_finite() {
            return None;
        }

        Some(Self { value, direction })
    }

    pub fn value(self) -> f64 {
        self.value
    }

    pub fn direction(self) -> ObjectiveDirection {
        self.direction
    }

    pub fn is_better_than(&self, other: &Self) -> Option<bool> {
        if self.direction != other.direction {
            return None;
        }

        Some(is_better(self.value, other.value, self.direction))
    }

    pub fn better(self, other: Self) -> Option<Self> {
        self.is_better_than(&other)
            .map(|self_is_better| if self_is_better { self } else { other })
    }
}

pub fn is_better(candidate: f64, incumbent: f64, direction: ObjectiveDirection) -> bool {
    if !candidate.is_finite() || !incumbent.is_finite() {
        return false;
    }

    match direction {
        ObjectiveDirection::Minimize => candidate < incumbent,
        ObjectiveDirection::Maximize => candidate > incumbent,
    }
}

pub fn best_value(values: &[f64], direction: ObjectiveDirection) -> Option<f64> {
    if values.is_empty() || values.iter().any(|value| !value.is_finite()) {
        return None;
    }

    let mut best = values[0];
    for value in values.iter().copied().skip(1) {
        if is_better(value, best, direction) {
            best = value;
        }
    }

    Some(best)
}

#[cfg(test)]
mod tests {
    use super::{best_value, is_better, ObjectiveDirection, ObjectiveValue};

    #[test]
    fn compares_candidates_by_direction() {
        assert!(is_better(2.0, 5.0, ObjectiveDirection::Minimize));
        assert!(is_better(5.0, 2.0, ObjectiveDirection::Maximize));
        assert!(!is_better(2.0, 2.0, ObjectiveDirection::Maximize));
    }

    #[test]
    fn chooses_best_value() {
        assert_eq!(
            best_value(&[4.0, 2.0, 3.0], ObjectiveDirection::Minimize),
            Some(2.0)
        );
        assert_eq!(
            best_value(&[4.0, 2.0, 3.0], ObjectiveDirection::Maximize),
            Some(4.0)
        );
        assert_eq!(best_value(&[9.0], ObjectiveDirection::Minimize), Some(9.0));
    }

    #[test]
    fn returns_none_for_invalid_values() {
        assert_eq!(best_value(&[], ObjectiveDirection::Minimize), None);
        assert_eq!(
            best_value(&[1.0, f64::NAN], ObjectiveDirection::Maximize),
            None
        );
        assert!(!is_better(f64::INFINITY, 2.0, ObjectiveDirection::Minimize));
    }

    #[test]
    fn objective_value_helpers_stay_explicit() {
        let candidate = ObjectiveValue::new(2.0, ObjectiveDirection::Minimize).unwrap();
        let incumbent = ObjectiveValue::new(5.0, ObjectiveDirection::Minimize).unwrap();
        let other_direction = ObjectiveValue::new(5.0, ObjectiveDirection::Maximize).unwrap();

        assert_eq!(candidate.value(), 2.0);
        assert_eq!(candidate.direction(), ObjectiveDirection::Minimize);
        assert_eq!(candidate.is_better_than(&incumbent), Some(true));
        assert_eq!(candidate.better(incumbent), Some(candidate));
        assert_eq!(candidate.is_better_than(&other_direction), None);
        assert_eq!(
            ObjectiveValue::new(f64::NEG_INFINITY, ObjectiveDirection::Maximize),
            None
        );
    }
}
