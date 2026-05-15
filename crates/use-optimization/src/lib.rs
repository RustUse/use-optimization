#![forbid(unsafe_code)]
//! Thin facade for the `use-optimization` workspace.
//!
//! The crate reexports the focused optimization crates directly so consumers can
//! opt into one dependency while still using the smaller APIs.
//!
//! # Examples
//!
//! ```rust
//! use use_optimization::*;
//!
//! let direction = ObjectiveDirection::Maximize;
//! let result = grid_search_1d(&[0.0, 1.0, 2.0, 3.0], |x| -(x - 2.0) * (x - 2.0), direction)
//!     .expect("grid search should find a best result");
//!
//! assert_eq!(result.best_value, 2.0);
//! assert_eq!(best_value(&[1.0, 3.0, 2.0], direction), Some(3.0));
//! assert!(Bounds { min: Some(0.0), max: Some(10.0) }.contains(4.0));
//! ```

pub use use_constraint;
pub use use_constraint::*;
pub use use_grid_search;
pub use use_grid_search::*;
pub use use_local_search;
pub use use_local_search::*;
pub use use_loss;
pub use use_loss::*;
pub use use_objective;
pub use use_objective::*;
pub use use_score;
pub use use_score::*;
pub use use_search_space;
pub use use_search_space::*;

#[cfg(test)]
mod tests {
    use super::{
        absolute_error, best_value, grid_search_1d, local_search_1d, normalize_min_max, Bounds,
        LocalSearchConfig, ObjectiveDirection, RangeSpace,
    };

    #[test]
    fn facade_reexports_workspace_apis() {
        assert_eq!(
            best_value(&[2.0, 6.0, 4.0], ObjectiveDirection::Maximize),
            Some(6.0)
        );
        assert!(Bounds {
            min: Some(0.0),
            max: Some(5.0)
        }
        .contains(3.0));
        assert_eq!(absolute_error(3.0, 1.5), 1.5);
        assert_eq!(
            normalize_min_max(&[2.0, 4.0, 6.0]),
            Some(vec![0.0, 0.5, 1.0])
        );
        assert_eq!(
            RangeSpace {
                start: 0.0,
                end: 2.0,
                step: 1.0
            }
            .values()
            .unwrap(),
            vec![0.0, 1.0, 2.0]
        );

        let grid = grid_search_1d(
            &[0.0, 1.0, 2.0, 3.0],
            |value| -(value - 2.0) * (value - 2.0),
            ObjectiveDirection::Maximize,
        )
        .expect("grid search should succeed");
        assert_eq!(grid.best_value, 2.0);

        let local = local_search_1d(
            LocalSearchConfig {
                initial_value: 0.0,
                step_size: 1.0,
                max_iterations: 8,
                direction: ObjectiveDirection::Maximize,
            },
            |value| -(value - 2.0) * (value - 2.0),
        )
        .expect("local search should succeed");
        assert_eq!(local.best_value, 2.0);
    }
}
