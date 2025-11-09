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
///     WORKER_COUNT: i32 = 32,
/// }
///
/// assert_eq!(config::JWT_SECRET.as_str(), "secret");
/// assert_eq!(config::WORKER_COUNT.clone(), 32);
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
            fn variable_exists(variable_name: &str) -> bool {
                std::env::var(variable_name).is_ok() || !std::env::var(format!("_{variable_name}_WAS_MISSING"))
                        .unwrap_or("false".to_string())
                        .eq("true")
            }

            #[inline]
            fn warn_if_env_var_is_missing(variable_name: &str, default_value: impl std::fmt::Display) -> bool {
                let variable_exists = variable_exists(variable_name);
                if !variable_exists {
                    $crate::log::warn!(
                        "Variable `{variable_name}` is missing in the env! Using default value `{default_value}`",
                    );
                }
                variable_exists
            }

            #[inline]
            fn get_variable_value<T: std::str::FromStr>(variable_name: &str, default_value: impl Into<T> + Clone + std::fmt::Display) -> T {
                let value = if let Ok(value) = std::env::var(variable_name) {
                    let value = value.parse::<T>().unwrap_or_else(|_| panic!(
                        "Invalid value type for the variable `{variable_name}`! Expected type `{}`, got `{}`.",
                        stringify!(T),
                        get_variable_type(&value)
                    ));
                    value
                } else {
                    unsafe {
                        std::env::set_var(format!("_{variable_name}_WAS_MISSING"), "true");
                    }
                    default_value.clone().into()
                };

                warn_if_env_var_is_missing(variable_name, default_value);

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

            /// Checks whether all variables are set or not
            pub fn check_values() -> bool {
                vec![$(
                    warn_if_env_var_is_missing(stringify!($name), $default_value)
                ),*]
                    .into_iter()
                    .all(|elem| elem)
            }

            /// Updates the environment with variable values
            ///
            /// Uses `set_env_only` under the hood
            pub fn set_env() {
                unsafe {
                    $(
                        $crate::set_env_only!($name);
                    )*
                }
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
