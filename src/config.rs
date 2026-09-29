//! Runtime configuration.
//!
//! Configuration is read from environment variables, following the
//! [twelve-factor](https://12factor.net/config) convention. There is exactly
//! one setting today:
//!
//! | Variable     | Meaning                    | Default          |
//! |--------------|----------------------------|------------------|
//! | `NOTES_ADDR` | Socket address to bind to. | `127.0.0.1:3000` |
//!
//! # Design note
//!
//! The parsing logic lives in a private function that takes the *result of
//! reading the variable* as an argument, instead of reading the process
//! environment itself. That keeps it pure and trivially unit-testable:
//! mutating the environment from tests is unsound in edition 2024
//! (`std::env::set_var` is `unsafe`) and racy across parallel tests anyway.

use std::{
    env,
    net::{AddrParseError, IpAddr, Ipv4Addr, SocketAddr},
};

/// Name of the environment variable holding the bind address.
pub const ADDR_ENV: &str = "NOTES_ADDR";

/// Address used when [`ADDR_ENV`] is not set: loopback only, port 3000.
///
/// Binding to loopback by default is deliberate: a development server should
/// not be reachable from other machines unless you opt in with
/// `NOTES_ADDR=0.0.0.0:3000`.
pub const DEFAULT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 3000);

/// Validated application settings.
///
/// Construct it with [`Config::from_env`] in production code, or
/// [`Config::default`] when you just need sensible values.
///
/// # Examples
///
/// ```
/// use notes_api::config::Config;
///
/// let config = Config::default();
/// assert_eq!(config.addr.port(), 3000);
/// assert!(config.addr.ip().is_loopback());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Socket address the HTTP server binds to.
    pub addr: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Self { addr: DEFAULT_ADDR }
    }
}

/// Reasons why loading a [`Config`] can fail.
///
/// Configuration errors are reported at start-up, before the server accepts
/// any traffic, so failing fast with a precise message is the goal.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// `NOTES_ADDR` was set but is not a valid socket address.
    #[error("NOTES_ADDR must be a socket address such as 127.0.0.1:3000, but got {value:?}")]
    InvalidAddr {
        /// The raw value that failed to parse.
        value: String,
        /// The underlying parse failure.
        #[source]
        source: AddrParseError,
    },

    /// `NOTES_ADDR` was set to bytes that are not valid Unicode.
    #[error("NOTES_ADDR is set but is not valid Unicode")]
    NotUnicode,
}

impl Config {
    /// Loads the configuration from the process environment.
    ///
    /// Unset variables fall back to their documented defaults.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::InvalidAddr`] if `NOTES_ADDR` is set but does not
    /// parse as a [`SocketAddr`], and [`ConfigError::NotUnicode`] if it is set
    /// to a non-Unicode value.
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_env_var(env::var(ADDR_ENV))
    }

    /// Builds a [`Config`] from the outcome of reading [`ADDR_ENV`].
    ///
    /// Separated from [`Config::from_env`] so it can be tested without
    /// touching global process state.
    fn from_env_var(addr: Result<String, env::VarError>) -> Result<Self, ConfigError> {
        let addr = match addr {
            Ok(raw) => raw
                .parse::<SocketAddr>()
                .map_err(|source| ConfigError::InvalidAddr { value: raw, source })?,
            Err(env::VarError::NotPresent) => DEFAULT_ADDR,
            Err(env::VarError::NotUnicode(_)) => return Err(ConfigError::NotUnicode),
        };

        Ok(Self { addr })
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::*;

    #[test]
    fn missing_variable_falls_back_to_default() {
        let config = Config::from_env_var(Err(env::VarError::NotPresent)).unwrap();

        assert_eq!(config, Config::default());
    }

    #[test]
    fn valid_address_is_parsed() {
        let config = Config::from_env_var(Ok("0.0.0.0:8080".to_owned())).unwrap();

        assert_eq!(config.addr, "0.0.0.0:8080".parse::<SocketAddr>().unwrap());
    }

    #[test]
    fn malformed_address_is_rejected_with_the_offending_value() {
        let error = Config::from_env_var(Ok("not-an-address".to_owned())).unwrap_err();

        assert!(matches!(
            error,
            ConfigError::InvalidAddr { ref value, .. } if value == "not-an-address"
        ));
        assert!(error.to_string().contains("NOTES_ADDR"));
    }

    #[test]
    fn non_unicode_value_is_rejected() {
        let error =
            Config::from_env_var(Err(env::VarError::NotUnicode(OsString::from("x")))).unwrap_err();

        assert!(matches!(error, ConfigError::NotUnicode));
    }
}
