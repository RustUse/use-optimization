#![forbid(unsafe_code)]
//! Minimal one-dimensional local search helpers.
//!
//! The initial implementation performs hill-climbing style search by checking a
//! left and right neighbor around the current value and moving only when a
//! strictly better score is found.
//!
//! # Examples
//!
//! ```rust
//! use use_local_search::{local_search_1d, LocalSearchConfig};
//! use use_objective::ObjectiveDirection;
//!
//! let result = local_search_1d(
//!     LocalSearchConfig {
//!         initial_value: 0.0,
//!         step_size: 1.0,
//!         max_iterations: 8,
//!         direction: ObjectiveDirection::Maximize,
//!     },
//!     |value| -(value - 3.0) * (value - 3.0),
//! )
//! .unwrap();
//!
//! assert_eq!(result.best_value, 3.0);
//! assert_eq!(result.best_score, 0.0);
//! ```

use use_objective::{ObjectiveDirection, is_better};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocalSearchConfig {
    pub initial_value: f64,
    pub step_size: f64,
    pub max_iterations: usize,
    pub direction: ObjectiveDirection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocalSearchResult {
    pub best_value: f64,
    pub best_score: f64,
    pub iterations: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalSearchError {
    InvalidInitialValue,
    InvalidStepSize,
    InvalidIterationCount,
    NonFiniteScore,
}

pub fn local_search_1d<F>(
    config: LocalSearchConfig,
    objective: F,
) -> Result<LocalSearchResult, LocalSearchError>
where
    F: Fn(f64) -> f64,
{
    if !config.initial_value.is_finite() {
        return Err(LocalSearchError::InvalidInitialValue);
    }

    if !config.step_size.is_finite() || config.step_size <= 0.0 {
        return Err(LocalSearchError::InvalidStepSize);
    }

    if config.max_iterations == 0 {
        return Err(LocalSearchError::InvalidIterationCount);
    }

    let mut current_value = config.initial_value;
    let mut current_score = objective(current_value);
    if !current_score.is_finite() {
        return Err(LocalSearchError::NonFiniteScore);
    }

    let mut iterations = 0;
    while iterations < config.max_iterations {
        iterations += 1;

        let left_value = current_value - config.step_size;
        let right_value = current_value + config.step_size;
        if !left_value.is_finite() || !right_value.is_finite() {
            break;
        }

        let left_score = objective(left_value);
        let right_score = objective(right_value);
        if !left_score.is_finite() || !right_score.is_finite() {
            return Err(LocalSearchError::NonFiniteScore);
        }

        let mut next_value = current_value;
        let mut next_score = current_score;

        if is_better(left_score, next_score, config.direction) {
            next_value = left_value;
            next_score = left_score;
        }

        if is_better(right_score, next_score, config.direction) {
            next_value = right_value;
            next_score = right_score;
        }

        if next_value == current_value {
            break;
        }

        current_value = next_value;
        current_score = next_score;
    }

    Ok(LocalSearchResult {
        best_value: current_value,
        best_score: current_score,
        iterations,
    })
}

#[cfg(test)]
mod tests {
    use super::{LocalSearchConfig, LocalSearchError, local_search_1d};
    use use_objective::ObjectiveDirection;

    #[test]
    fn climbs_toward_better_scores_for_maximization() {
        let result = local_search_1d(
            LocalSearchConfig {
                initial_value: 0.0,
                step_size: 1.0,
                max_iterations: 10,
                direction: ObjectiveDirection::Maximize,
            },
            |value| -(value - 3.0) * (value - 3.0),
        )
        .unwrap();

        assert_eq!(result.best_value, 3.0);
        assert_eq!(result.best_score, 0.0);
        assert!(result.iterations <= 10);
    }

    #[test]
    fn climbs_toward_better_scores_for_minimization() {
        let result = local_search_1d(
            LocalSearchConfig {
                initial_value: 5.0,
                step_size: 1.0,
                max_iterations: 10,
                direction: ObjectiveDirection::Minimize,
            },
            |value| (value - 2.0) * (value - 2.0),
        )
        .unwrap();

        assert_eq!(result.best_value, 2.0);
        assert_eq!(result.best_score, 0.0);
    }

    #[test]
    fn rejects_invalid_configuration() {
        assert_eq!(
            local_search_1d(
                LocalSearchConfig {
                    initial_value: f64::NAN,
                    step_size: 1.0,
                    max_iterations: 10,
                    direction: ObjectiveDirection::Maximize,
                },
                |value| value,
            ),
            Err(LocalSearchError::InvalidInitialValue)
        );
        assert_eq!(
            local_search_1d(
                LocalSearchConfig {
                    initial_value: 0.0,
                    step_size: 0.0,
                    max_iterations: 10,
                    direction: ObjectiveDirection::Maximize,
                },
                |value| value,
            ),
            Err(LocalSearchError::InvalidStepSize)
        );
        assert_eq!(
            local_search_1d(
                LocalSearchConfig {
                    initial_value: 0.0,
                    step_size: 1.0,
                    max_iterations: 0,
                    direction: ObjectiveDirection::Maximize,
                },
                |value| value,
            ),
            Err(LocalSearchError::InvalidIterationCount)
        );
    }

    #[test]
    fn rejects_non_finite_objective_scores() {
        assert_eq!(
            local_search_1d(
                LocalSearchConfig {
                    initial_value: 0.0,
                    step_size: 1.0,
                    max_iterations: 10,
                    direction: ObjectiveDirection::Maximize,
                },
                |_value| f64::NAN,
            ),
            Err(LocalSearchError::NonFiniteScore)
        );
    }
}
