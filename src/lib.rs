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
///     SERVER_ADDRESS: String = "0.0.0.0:8080",
///     WORKERS_COUNT: i32 = 32
/// }
///
/// assert_eq!(config::SERVER_ADDRESS.as_str(), "0.0.0.0:8080");
/// assert_eq!(config::WORKERS_COUNT.clone(), 32);
/// ```
/// # Panics
///
/// ```no_compile
/// use env_vars_config::env_vars_config;
///
/// env_vars_config! {
///     // incompatible type
///     ENABLE_REGISTRATION: bool = "false"
/// }
/// ```
#[macro_export]
macro_rules! env_vars_config {
    ( $( $name:ident: $type:tt = $default_value:expr ),* $(,)? ) => {
        pub mod config {
            use std::{
                sync::LazyLock,
                env,
                any::type_name,
            };

            #[inline]
            fn get_variable_type<T>(_: &T) -> &'static str {
                type_name::<T>().split("::").last().unwrap()
            }

            $(
                /// Our environment variable. Lazy-evaluated by default
                pub static $name: LazyLock<$type> = LazyLock::new(|| {
                    let name = stringify!($name);

                    if let Ok(value) = env::var(name) {
                        if let Ok(value) = value.parse::<$type>() {
                            value
                        } else {
                            panic!(
                                "Invalid value type for the variable `{name}`! Expected type `{}`, got `{}`.",
                                stringify!($type),
                                get_variable_type(&value)
                            )
                        }
                    } else {
                        env_vars_config::log::warn!(
                            "Variable `{}` is missing in the env! Using default value `{}`",
                            name,
                            $default_value
                        );
                        <$type>::from($default_value)
                    }
                });
            )*

            /// Inits all environment variables
            pub fn init() {
                $(
                    LazyLock::force(&$name);
                )*
            }
        }
    };
}
