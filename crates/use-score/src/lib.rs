#![forbid(unsafe_code)]
//! Primitive scoring and ranking helpers for optimization.
//!
//! # Examples
//!
//! ```rust
//! use use_score::{normalize_min_max, rank_descending, weighted_sum};
//!
//! assert_eq!(weighted_sum(&[3.0, 4.0], &[0.25, 0.75]).unwrap(), 3.75);
//! assert_eq!(normalize_min_max(&[2.0, 4.0, 6.0]).unwrap(), vec![0.0, 0.5, 1.0]);
//! assert_eq!(rank_descending(&[1.0, 3.0, 2.0]), vec![1, 2, 0]);
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScoreError {
    EmptyInput,
    MismatchedLengths,
    NonFiniteInput,
}

pub fn weighted_sum(values: &[f64], weights: &[f64]) -> Result<f64, ScoreError> {
    if values.is_empty() || weights.is_empty() {
        return Err(ScoreError::EmptyInput);
    }

    if values.len() != weights.len() {
        return Err(ScoreError::MismatchedLengths);
    }

    if values.iter().any(|value| !value.is_finite())
        || weights.iter().any(|weight| !weight.is_finite())
    {
        return Err(ScoreError::NonFiniteInput);
    }

    Ok(values
        .iter()
        .zip(weights.iter())
        .map(|(value, weight)| value * weight)
        .sum())
}

pub fn normalize_min_max(values: &[f64]) -> Option<Vec<f64>> {
    if values.is_empty() || values.iter().any(|value| !value.is_finite()) {
        return None;
    }

    let minimum = values.iter().copied().min_by(f64::total_cmp)?;
    let maximum = values.iter().copied().max_by(f64::total_cmp)?;
    let range = maximum - minimum;

    if range == 0.0 {
        return Some(vec![0.0; values.len()]);
    }

    Some(
        values
            .iter()
            .map(|value| (*value - minimum) / range)
            .collect(),
    )
}

pub fn rank_descending(values: &[f64]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..values.len()).collect();
    indices.sort_by(|left, right| {
        values[*right]
            .total_cmp(&values[*left])
            .then_with(|| left.cmp(right))
    });
    indices
}

pub fn rank_ascending(values: &[f64]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..values.len()).collect();
    indices.sort_by(|left, right| {
        values[*left]
            .total_cmp(&values[*right])
            .then_with(|| left.cmp(right))
    });
    indices
}

pub fn penalize(score: f64, penalty: f64) -> f64 {
    score - penalty
}

pub fn reward(score: f64, reward_value: f64) -> f64 {
    score + reward_value
}

#[cfg(test)]
mod tests {
    use super::{
        ScoreError, normalize_min_max, penalize, rank_ascending, rank_descending, reward,
        weighted_sum,
    };

    #[test]
    fn computes_weighted_scores() {
        assert_eq!(weighted_sum(&[3.0, 4.0], &[0.25, 0.75]).unwrap(), 3.75);
        assert_eq!(weighted_sum(&[9.0], &[2.0]).unwrap(), 18.0);
    }

    #[test]
    fn rejects_invalid_weighted_score_inputs() {
        assert_eq!(weighted_sum(&[], &[]), Err(ScoreError::EmptyInput));
        assert_eq!(
            weighted_sum(&[1.0, 2.0], &[1.0]),
            Err(ScoreError::MismatchedLengths)
        );
        assert_eq!(
            weighted_sum(&[1.0, f64::NAN], &[0.5, 0.5]),
            Err(ScoreError::NonFiniteInput)
        );
    }

    #[test]
    fn normalizes_values_with_min_max_scaling() {
        assert_eq!(
            normalize_min_max(&[2.0, 4.0, 6.0]),
            Some(vec![0.0, 0.5, 1.0])
        );
        assert_eq!(normalize_min_max(&[5.0]), Some(vec![0.0]));
        assert_eq!(normalize_min_max(&[3.0, 3.0]), Some(vec![0.0, 0.0]));
        assert_eq!(normalize_min_max(&[]), None);
        assert_eq!(normalize_min_max(&[1.0, f64::INFINITY]), None);
    }

    #[test]
    fn ranks_values_in_both_directions() {
        assert_eq!(rank_descending(&[1.0, 3.0, 2.0]), vec![1, 2, 0]);
        assert_eq!(rank_ascending(&[1.0, 3.0, 2.0]), vec![0, 2, 1]);
    }

    #[test]
    fn adjusts_scores_with_penalties_and_rewards() {
        assert_eq!(penalize(10.0, 1.5), 8.5);
        assert_eq!(reward(10.0, 1.5), 11.5);
    }
}
