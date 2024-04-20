/// Unpacks a `Result` value, returning the contained value if `Ok` or terminating the function and
/// responding with an internal server error if `Err`.
/// # Parameters
/// - `$expr`: The `Result` expression to unpack.
/// - `$error_message`: The message to return in the JSON response if an error occurs.
#[macro_export]
macro_rules! unpack_result {
    ($expr:expr, $error_message:expr) => {
        match $expr {
            Ok(value) => value,
            Err(_) => {
                return Json((StatusCode::INTERNAL_SERVER_ERROR, $error_message)).into_response()
            }
        }
    };
}

/// Unpacks an `Option` value, returning the contained value if `Some` or terminating the function
/// and responding with a specified status code and message if `None`.
/// # Parameters
/// - `$option`: The `Option` expression to unpack.
/// - `$status_code`: The `StatusCode` to use in the JSON response if the value is `None`.
/// - `$status_message`: The message to return in the JSON response if the value is `None`.
#[macro_export]
macro_rules! unpack_option {
    ($option:expr, $status_code:expr, $status_message:expr) => {
        match $option {
            Some(value) => value,
            None => return Json(($status_code, $status_message)).into_response(),
        }
    };
}

/// Unpacks a `Result<Option<_>>` value, returning the contained value if `Ok(Some(_))`, or terminating
/// the function and responding with specified status codes and messages depending on the error.
/// # Parameters
/// - `$expr`: The `Result<Option<_>>` expression to unpack.
/// - `$status_code`: The `StatusCode` for `None` outcomes in the `Result`.
/// - `$status_message`: The message for `None` outcomes in the `Result`.
/// - `$error_message`: The error message for `Err` outcomes.
#[macro_export]
macro_rules! unpack_result_option {
    ($expr:expr, $status_code:expr, $status_message:expr, $error_message:expr) => {
        match $expr {
            Ok(Some(value)) => value,
            Ok(None) => return Json(($status_code, $status_message)).into_response(),
            Err(_) => {
                return Json((StatusCode::INTERNAL_SERVER_ERROR, $error_message)).into_response()
            }
        }
    };
}
