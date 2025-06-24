//! A simple lib for configuring your applications via environment variables

#[doc(hidden)]
pub extern crate log;

/// Creates new mod with specified environment variables
/// # Examples
///
/// ```
/// use env_vars_config::env_vars_config;
///
/// env_vars_config! {
///     JWT_SECRET: String = "secret",
///     WORKERS_COUNT: i32 = 32,
/// }
///
/// assert_eq!(config::JWT_SECRET.as_str(), "secret");
/// assert_eq!(config::WORKERS_COUNT.clone(), 32);
/// ```
///
/// # Panics
///
/// ```no_compile
/// use env_vars_config::env_vars_config;
///
/// env_vars_config! {
///     // incompatible type
///     ENABLE_REGISTRATION: bool = "false",
/// }
/// ```
#[macro_export]
macro_rules! env_vars_config {
    ( $( $name:ident: $type:tt = $default_value:expr ),* $(,)? ) => {
        pub mod config {
            use super::*;

            #[inline]
            fn get_variable_type<T>(_: &T) -> &'static str {
                std::any::type_name::<T>().split("::").last().unwrap()
            }

            #[inline]
            fn missing_in_env_warn(variable_name: &str, default_value: impl std::fmt::Display) {
                $crate::log::warn!(
                    "Variable `{variable_name}` is missing in the env! Using default value `{default_value}`",
                );
            }

            #[inline]
            fn get_variable_value<T: std::str::FromStr>(variable_name: &str, default_value: impl Into<T> + Clone + std::fmt::Display) -> T {
                let (value, is_missing) = if let Ok(value) = std::env::var(variable_name) {
                    let value = value.parse::<T>().unwrap_or_else(|_| panic!(
                        "Invalid value type for the variable `{variable_name}`! Expected type `{}`, got `{}`.",
                        stringify!(T),
                        get_variable_type(&value)
                    ));
                    let is_missing = std::env::var(format!("_{variable_name}_WAS_MISSING"))
                        .unwrap_or("false".to_string())
                        .eq("true");
                    (value, is_missing)
                } else {
                    unsafe {
                        std::env::set_var(format!("_{variable_name}_WAS_MISSING"), "true");
                    }
                    (default_value.clone().into(), true)
                };

                if is_missing {
                    missing_in_env_warn(&variable_name, &default_value);
                }

                value
            }

            $(
                /// Our environment variable. Lazy-evaluated by default
                pub static $name: std::sync::LazyLock<$type> = std::sync::LazyLock::new(|| {
                    get_variable_value(stringify!($name), $default_value)
                });
            )*

            /// Inits all environment variables
            pub fn init() {
                $(
                    std::sync::LazyLock::force(&$name);
                )*
            }

            /// Tries to get every variable value. Usable when you need
            pub fn test_values() {
                $(
                    get_variable_value::<$type>(stringify!($name), $default_value);
                )*
            }

            /// Updates the environment with variable values
            ///
            /// Does `set_env_only` under the hood
            pub fn set_env() {
                $(
                    unsafe {
                        $crate::set_env_only!($name);
                    }
                )*
            }
        }
    };
}

/// Inits config value and sets it in the runtime environment
/// Note: uses `std::env::set_var` under the hood.
/// # Examples
/// ```
/// use env_vars_config::{env_vars_config, set_env_only};
///
/// env_vars_config! {
///     OTEL_SERVICE_NAME: String = "test-service",
/// }
///
/// assert_eq!(config::OTEL_SERVICE_NAME.as_str(), "test-service");
///
/// unsafe {
///     use config::OTEL_SERVICE_NAME;
///     set_env_only!(OTEL_SERVICE_NAME);
/// }
///
/// assert_eq!(std::env::var("OTEL_SERVICE_NAME").unwrap().as_str(), "test-service");
/// ```
#[macro_export]
macro_rules! set_env_only {
    ($($name:ident),*) => {
        $(
            std::env::set_var(stringify!($name), $name.to_string());
        )*
    };
}
