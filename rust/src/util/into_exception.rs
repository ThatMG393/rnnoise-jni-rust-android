use std::fmt::Display;
use crate::util::exception::JavaException;

pub trait ErrIntoException<T> {

    fn err_into_denoise_exception(self, message: String) -> Result<T, JavaException>;
}

impl<T, E: Display> ErrIntoException<T> for Result<T, E> {
    fn err_into_denoise_exception(self, message: String) -> Result<T, JavaException> {
        self.map_err(|error| JavaException::new_denoise(
            format!("{}: {}", message, error)
        ))
    }
}
