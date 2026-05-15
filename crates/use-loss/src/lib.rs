#![forbid(unsafe_code)]
//! Primitive loss and error helpers for optimization.
//!
//! # Examples
//!
//! ```rust
//! use use_loss::{absolute_error, mean_squared_error, root_mean_squared_error};
//!
//! assert_eq!(absolute_error(4.0, 3.0), 1.0);
//! assert_eq!(mean_squared_error(&[1.0, 2.0], &[1.0, 4.0]).unwrap(), 2.0);
//! assert_eq!(root_mean_squared_error(&[1.0, 2.0], &[1.0, 4.0]).unwrap(), 2.0_f64.sqrt());
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LossError {
    EmptyInput,
    MismatchedLengths,
    NonFiniteInput,
}

pub fn absolute_error(actual: f64, predicted: f64) -> f64 {
    (actual - predicted).abs()
}

pub fn squared_error(actual: f64, predicted: f64) -> f64 {
    let difference = actual - predicted;
    difference * difference
}

pub fn mean_absolute_error(actual: &[f64], predicted: &[f64]) -> Result<f64, LossError> {
    validate_inputs(actual, predicted)?;

    Ok(actual
        .iter()
        .zip(predicted.iter())
        .map(|(actual_value, predicted_value)| absolute_error(*actual_value, *predicted_value))
        .sum::<f64>()
        / actual.len() as f64)
}

pub fn mean_squared_error(actual: &[f64], predicted: &[f64]) -> Result<f64, LossError> {
    validate_inputs(actual, predicted)?;

    Ok(actual
        .iter()
        .zip(predicted.iter())
        .map(|(actual_value, predicted_value)| squared_error(*actual_value, *predicted_value))
        .sum::<f64>()
        / actual.len() as f64)
}

pub fn root_mean_squared_error(actual: &[f64], predicted: &[f64]) -> Result<f64, LossError> {
    Ok(mean_squared_error(actual, predicted)?.sqrt())
}

fn validate_inputs(actual: &[f64], predicted: &[f64]) -> Result<(), LossError> {
    if actual.is_empty() || predicted.is_empty() {
        return Err(LossError::EmptyInput);
    }

    if actual.len() != predicted.len() {
        return Err(LossError::MismatchedLengths);
    }

    if actual
        .iter()
        .chain(predicted.iter())
        .any(|value| !value.is_finite())
    {
        return Err(LossError::NonFiniteInput);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        LossError, absolute_error, mean_absolute_error, mean_squared_error,
        root_mean_squared_error, squared_error,
    };

    fn approx_eq(left: f64, right: f64) {
        assert!((left - right).abs() < 1.0e-10, "left={left}, right={right}");
    }

    #[test]
    fn computes_basic_error_terms() {
        assert_eq!(absolute_error(4.0, 3.0), 1.0);
        assert_eq!(squared_error(4.0, 3.0), 1.0);
    }

    #[test]
    fn computes_common_loss_functions() {
        let actual = [1.0, 2.0, 3.0];
        let predicted = [1.5, 2.5, 2.0];

        approx_eq(mean_absolute_error(&actual, &predicted).unwrap(), 2.0 / 3.0);
        approx_eq(mean_squared_error(&actual, &predicted).unwrap(), 0.5);
        approx_eq(
            root_mean_squared_error(&actual, &predicted).unwrap(),
            0.5_f64.sqrt(),
        );
    }

    #[test]
    fn handles_single_value_inputs() {
        approx_eq(mean_absolute_error(&[3.0], &[2.0]).unwrap(), 1.0);
        approx_eq(mean_squared_error(&[3.0], &[2.0]).unwrap(), 1.0);
        approx_eq(root_mean_squared_error(&[3.0], &[2.0]).unwrap(), 1.0);
    }

    #[test]
    fn rejects_invalid_loss_inputs() {
        assert_eq!(mean_absolute_error(&[], &[]), Err(LossError::EmptyInput));
        assert_eq!(
            mean_squared_error(&[1.0], &[1.0, 2.0]),
            Err(LossError::MismatchedLengths)
        );
        assert_eq!(
            root_mean_squared_error(&[1.0, f64::NAN], &[1.0, 2.0]),
            Err(LossError::NonFiniteInput)
        );
    }
}
