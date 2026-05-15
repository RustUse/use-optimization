#![forbid(unsafe_code)]
//! Primitive one-dimensional and two-dimensional grid search helpers.
//!
//! # Examples
//!
//! ```rust
//! use use_grid_search::{grid_search_1d, grid_search_2d};
//! use use_objective::ObjectiveDirection;
//!
//! let one_dimensional = grid_search_1d(
//!     &[0.0, 1.0, 2.0, 3.0],
//!     |value| -(value - 2.0) * (value - 2.0),
//!     ObjectiveDirection::Maximize,
//! )
//! .unwrap();
//! assert_eq!(one_dimensional.best_value, 2.0);
//!
//! let two_dimensional = grid_search_2d(
//!     &[0.0, 1.0, 2.0],
//!     &[-2.0, -1.0, 0.0],
//!     |x, y| -((x - 1.0) * (x - 1.0) + (y + 1.0) * (y + 1.0)),
//!     ObjectiveDirection::Maximize,
//! )
//! .unwrap();
//! assert_eq!((two_dimensional.best_x, two_dimensional.best_y), (1.0, -1.0));
//! ```

use use_objective::{ObjectiveDirection, is_better};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridSearchResult1D {
    pub best_value: f64,
    pub best_score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridSearchResult2D {
    pub best_x: f64,
    pub best_y: f64,
    pub best_score: f64,
}

pub fn grid_search_1d<F>(
    values: &[f64],
    objective: F,
    direction: ObjectiveDirection,
) -> Option<GridSearchResult1D>
where
    F: Fn(f64) -> f64,
{
    if values.is_empty() || values.iter().any(|value| !value.is_finite()) {
        return None;
    }

    let mut best: Option<GridSearchResult1D> = None;
    for value in values.iter().copied() {
        let score = objective(value);
        if !score.is_finite() {
            return None;
        }

        let candidate = GridSearchResult1D {
            best_value: value,
            best_score: score,
        };

        if best.is_none_or(|current| is_better(candidate.best_score, current.best_score, direction))
        {
            best = Some(candidate);
        }
    }

    best
}

pub fn grid_search_2d<F>(
    x_values: &[f64],
    y_values: &[f64],
    objective: F,
    direction: ObjectiveDirection,
) -> Option<GridSearchResult2D>
where
    F: Fn(f64, f64) -> f64,
{
    if x_values.is_empty()
        || y_values.is_empty()
        || x_values
            .iter()
            .chain(y_values.iter())
            .any(|value| !value.is_finite())
    {
        return None;
    }

    let mut best: Option<GridSearchResult2D> = None;
    for x_value in x_values.iter().copied() {
        for y_value in y_values.iter().copied() {
            let score = objective(x_value, y_value);
            if !score.is_finite() {
                return None;
            }

            let candidate = GridSearchResult2D {
                best_x: x_value,
                best_y: y_value,
                best_score: score,
            };

            if best.is_none_or(|current| {
                is_better(candidate.best_score, current.best_score, direction)
            }) {
                best = Some(candidate);
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::{grid_search_1d, grid_search_2d};
    use use_objective::ObjectiveDirection;

    #[test]
    fn finds_best_one_dimensional_candidate() {
        let result = grid_search_1d(
            &[0.0, 1.0, 2.0, 3.0],
            |value| -(value - 2.0) * (value - 2.0),
            ObjectiveDirection::Maximize,
        )
        .unwrap();

        assert_eq!(result.best_value, 2.0);
        assert_eq!(result.best_score, 0.0);
    }

    #[test]
    fn finds_best_two_dimensional_candidate() {
        let result = grid_search_2d(
            &[0.0, 1.0, 2.0],
            &[-2.0, -1.0, 0.0],
            |x, y| -((x - 1.0) * (x - 1.0) + (y + 1.0) * (y + 1.0)),
            ObjectiveDirection::Maximize,
        )
        .unwrap();

        assert_eq!(result.best_x, 1.0);
        assert_eq!(result.best_y, -1.0);
        assert_eq!(result.best_score, 0.0);
    }

    #[test]
    fn supports_minimization() {
        let result = grid_search_1d(
            &[0.0, 1.0, 2.0, 3.0],
            |value| (value - 1.0) * (value - 1.0),
            ObjectiveDirection::Minimize,
        )
        .unwrap();

        assert_eq!(result.best_value, 1.0);
    }

    #[test]
    fn returns_none_for_invalid_inputs() {
        assert_eq!(
            grid_search_1d(&[], |value| value, ObjectiveDirection::Maximize),
            None
        );
        assert_eq!(
            grid_search_1d(
                &[1.0, f64::NAN],
                |value| value,
                ObjectiveDirection::Maximize
            ),
            None
        );
        assert_eq!(
            grid_search_2d(
                &[1.0],
                &[2.0],
                |_x, _y| f64::NAN,
                ObjectiveDirection::Maximize,
            ),
            None
        );
    }
}
