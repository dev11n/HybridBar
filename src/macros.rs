#[macro_export]
/// Logs a [HYBRID] [DEBUG] formatted message to stdout.
macro_rules! log {
    ($msg:expr) => {
        if $crate::utils::environment::try_get_var("HYBRID_LOG", "0") == "1" {
            println!("[LOG]: {}", $msg)
        }
    };
}
#[macro_export]
/// Executes a bash command and outputs it to `result`.
macro_rules! execute {
    ($cmd:expr) => {{
        let mut result = unsafe {
            String::from_utf8_unchecked(
                std::process::Command::new($crate::constants::PROC_TARGET)
                    .args(["-c", $cmd])
                    .output()
                    .unwrap()
                    .stdout,
            )
        };

        // Remove the last character as its a new line.
        result.pop();

        result
    }};
}

#[macro_export]
/// Gets a value from the config.
macro_rules! conf {
    ($root:expr, $key:expr, $is_string:expr, $with_custom_variables:expr) => {
        $crate::config::try_get($root, $key, $is_string, $with_custom_variables)
    };
    ($root:expr, $key:expr, $default:expr) => {
        if let Some(res) = conf!($root, $key, true, false).string {
            res == "true"
        } else {
            $default
        }
    };
}

#[macro_export]
/// Checks if the specified feature is active.
macro_rules! is_feature_active {
    ($tag:expr) => {
        $crate::config::get_config()[$crate::constants::HYBRID_ROOT_JSON]
            [$crate::constants::HYBRID_F_ROOT_JSON]
            .contains($tag)
    };
}

#[macro_export]
/// Restarts the given `Revealer` and plays the given animation after the `after` closure has
/// finished.
macro_rules! restart_revealer {
    ($revealer:expr, $after:expr, $anim:expr, $speed:expr) => {
        if $anim == RevealerTransitionType::None {
            // No transition, skip full restart and instead just call directly.
            $after();
        } else {
            $revealer.set_transition_duration(0);
            $revealer.set_reveal_child(false);
            $revealer.set_transition_type(RevealerTransitionType::None);
            $after();
            $revealer.set_transition_duration($speed);
            $revealer.set_transition_type($anim);
            $revealer.set_reveal_child(true);
        }
    };
}
