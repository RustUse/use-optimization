#![forbid(unsafe_code)]
//! Primitive search-space helpers for optimization.
//!
//! `RangeSpace` supports both ascending and descending ranges while requiring a
//! positive, finite step size.
//!
//! # Examples
//!
//! ```rust
//! use use_search_space::{linspace, RangeSpace};
//!
//! let ascending = RangeSpace {
//!     start: 0.0,
//!     end: 2.0,
//!     step: 1.0,
//! };
//! assert_eq!(ascending.values().unwrap(), vec![0.0, 1.0, 2.0]);
//! assert_eq!(linspace(0.0, 1.0, 3).unwrap(), vec![0.0, 0.5, 1.0]);
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchSpaceError {
    NonFiniteInput,
    InvalidStep,
    InvalidPointCount,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeSpace {
    pub start: f64,
    pub end: f64,
    pub step: f64,
}

impl RangeSpace {
    pub fn values(&self) -> Result<Vec<f64>, SearchSpaceError> {
        if !self.start.is_finite() || !self.end.is_finite() || !self.step.is_finite() {
            return Err(SearchSpaceError::NonFiniteInput);
        }

        if self.step <= 0.0 {
            return Err(SearchSpaceError::InvalidStep);
        }

        if self.start == self.end {
            return Ok(vec![self.start]);
        }

        let increasing = self.start < self.end;
        let tolerance = self.step * 1.0e-12;
        let mut values = Vec::new();
        let mut current = self.start;

        loop {
            if increasing {
                if current > self.end + tolerance {
                    break;
                }
            } else if current < self.end - tolerance {
                break;
            }

            let value = if (current - self.end).abs() <= tolerance {
                self.end
            } else {
                current
            };
            values.push(value);

            current = if increasing {
                current + self.step
            } else {
                current - self.step
            };

            if !current.is_finite() {
                return Err(SearchSpaceError::NonFiniteInput);
            }
        }

        Ok(values)
    }
}

pub fn linspace(start: f64, end: f64, points: usize) -> Result<Vec<f64>, SearchSpaceError> {
    if !start.is_finite() || !end.is_finite() {
        return Err(SearchSpaceError::NonFiniteInput);
    }

    if points == 0 {
        return Err(SearchSpaceError::InvalidPointCount);
    }

    if points == 1 {
        return Ok(vec![start]);
    }

    let step = (end - start) / (points - 1) as f64;
    let mut values = Vec::with_capacity(points);
    for index in 0..points {
        if index + 1 == points {
            values.push(end);
        } else {
            values.push(start + step * index as f64);
        }
    }

    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::{linspace, RangeSpace, SearchSpaceError};

    fn approx_eq(left: &[f64], right: &[f64]) {
        assert_eq!(left.len(), right.len());
        for (left_value, right_value) in left.iter().zip(right.iter()) {
            assert!(
                (left_value - right_value).abs() < 1.0e-10,
                "left={left_value}, right={right_value}"
            );
        }
    }

    #[test]
    fn generates_ascending_values() {
        let space = RangeSpace {
            start: 0.0,
            end: 2.0,
            step: 1.0,
        };

        assert_eq!(space.values().unwrap(), vec![0.0, 1.0, 2.0]);
    }

    #[test]
    fn generates_descending_values() {
        let space = RangeSpace {
            start: 3.0,
            end: 0.0,
            step: 1.0,
        };

        assert_eq!(space.values().unwrap(), vec![3.0, 2.0, 1.0, 0.0]);
    }

    #[test]
    fn handles_single_point_ranges() {
        let space = RangeSpace {
            start: 2.0,
            end: 2.0,
            step: 0.5,
        };

        assert_eq!(space.values().unwrap(), vec![2.0]);
    }

    #[test]
    fn rejects_invalid_search_spaces() {
        assert_eq!(
            RangeSpace {
                start: 0.0,
                end: 1.0,
                step: 0.0,
            }
            .values(),
            Err(SearchSpaceError::InvalidStep)
        );
        assert_eq!(
            RangeSpace {
                start: 0.0,
                end: f64::INFINITY,
                step: 1.0,
            }
            .values(),
            Err(SearchSpaceError::NonFiniteInput)
        );
        assert_eq!(
            linspace(0.0, 1.0, 0),
            Err(SearchSpaceError::InvalidPointCount)
        );
    }

    #[test]
    fn builds_linspace_values() {
        approx_eq(
            &linspace(0.0, 1.0, 5).unwrap(),
            &[0.0, 0.25, 0.5, 0.75, 1.0],
        );
        approx_eq(&linspace(3.0, 1.0, 3).unwrap(), &[3.0, 2.0, 1.0]);
        assert_eq!(linspace(9.0, 5.0, 1).unwrap(), vec![9.0]);
    }
}
