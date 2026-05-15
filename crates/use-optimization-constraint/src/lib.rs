#![forbid(unsafe_code)]
//! Bounds and simple constraint helpers.
//!
//! The crate stays deliberately small and focuses on inclusive numeric bounds
//! over `f64` values.
//!
//! # Examples
//!
//! ```rust
//! use use_optimization_constraint::{satisfies_all, Bounds};
//!
//! let constraints = [
//!     Bounds { min: Some(0.0), max: Some(10.0) },
//!     Bounds { min: Some(2.0), max: None },
//! ];
//!
//! assert!(satisfies_all(4.0, &constraints));
//! assert!(!satisfies_all(1.0, &constraints));
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

impl Bounds {
    pub fn contains(&self, value: f64) -> bool {
        if !self.is_valid() || !value.is_finite() {
            return false;
        }

        if let Some(minimum) = self.min
            && value < minimum
        {
            return false;
        }

        if let Some(maximum) = self.max
            && value > maximum
        {
            return false;
        }

        true
    }

    pub fn clamp(&self, value: f64) -> f64 {
        if !self.is_valid() || !value.is_finite() {
            return value;
        }

        let mut clamped = value;
        if let Some(minimum) = self.min {
            clamped = clamped.max(minimum);
        }
        if let Some(maximum) = self.max {
            clamped = clamped.min(maximum);
        }

        clamped
    }

    pub fn is_valid(&self) -> bool {
        if self.min.is_some_and(|value| !value.is_finite())
            || self.max.is_some_and(|value| !value.is_finite())
        {
            return false;
        }

        match (self.min, self.max) {
            (Some(minimum), Some(maximum)) => minimum <= maximum,
            _ => true,
        }
    }
}

pub fn within(value: f64, min: f64, max: f64) -> bool {
    Bounds {
        min: Some(min),
        max: Some(max),
    }
    .contains(value)
}

pub fn outside(value: f64, min: f64, max: f64) -> bool {
    let bounds = Bounds {
        min: Some(min),
        max: Some(max),
    };

    bounds.is_valid() && value.is_finite() && !bounds.contains(value)
}

pub fn satisfies_all(value: f64, constraints: &[Bounds]) -> bool {
    if !value.is_finite() {
        return false;
    }

    constraints
        .iter()
        .all(|constraint| constraint.contains(value))
}

#[cfg(test)]
mod tests {
    use super::{Bounds, outside, satisfies_all, within};

    #[test]
    fn validates_and_checks_bounds() {
        let bounds = Bounds {
            min: Some(0.0),
            max: Some(10.0),
        };

        assert!(bounds.is_valid());
        assert!(bounds.contains(0.0));
        assert!(bounds.contains(10.0));
        assert!(!bounds.contains(11.0));
        assert_eq!(bounds.clamp(-3.0), 0.0);
        assert_eq!(bounds.clamp(12.0), 10.0);
        assert_eq!(bounds.clamp(6.0), 6.0);
    }

    #[test]
    fn handles_invalid_bounds() {
        let invalid = Bounds {
            min: Some(5.0),
            max: Some(1.0),
        };

        assert!(!invalid.is_valid());
        assert!(!invalid.contains(3.0));
        assert_eq!(invalid.clamp(3.0), 3.0);
        assert!(!within(3.0, 5.0, 1.0));
        assert!(!outside(3.0, 5.0, 1.0));
    }

    #[test]
    fn supports_unbounded_edges() {
        let lower_only = Bounds {
            min: Some(2.0),
            max: None,
        };
        let upper_only = Bounds {
            min: None,
            max: Some(4.0),
        };

        assert!(lower_only.contains(3.0));
        assert!(!lower_only.contains(1.0));
        assert!(upper_only.contains(3.0));
        assert!(!upper_only.contains(5.0));
    }

    #[test]
    fn evaluates_constraint_sets() {
        let constraints = [
            Bounds {
                min: Some(0.0),
                max: Some(10.0),
            },
            Bounds {
                min: Some(2.0),
                max: Some(8.0),
            },
        ];

        assert!(satisfies_all(5.0, &constraints));
        assert!(!satisfies_all(1.0, &constraints));
        assert!(satisfies_all(5.0, &[]));
        assert!(!satisfies_all(f64::NAN, &constraints));
        assert!(within(3.0, 1.0, 5.0));
        assert!(outside(0.0, 1.0, 5.0));
    }
}
