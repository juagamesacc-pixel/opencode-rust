//! Rust port of `src/renderer/initialization.ts` (opencode v1.18.30).
//!
//! The source reads a Solid `createResource` accessor (a callable with
//! `.error`/`.loading` props). The port takes the same three pieces
//! explicitly: a value thunk (never invoked while `loading`, mirroring the
//! source's short-circuit), the error slot (`None` = no error; any
//! `Some`, including falsy strings, throws — mirroring `!== undefined`),
//! and the loading flag.
//!
//! Original file: `packages/desktop/src/renderer/initialization.ts`

pub const REMOTE_INIT_PREFIX: &str = "Error invoking remote method 'await-initialization': Error: ";

pub struct InitializationFailure {
    pub message: String,
    pub stack: Option<String>,
    /// Mirrors the `localServerStartup: true` marker defined via
    /// `Object.defineProperty`.
    pub local_server_startup: bool,
}

pub fn mark_local_server_startup(message: &str, stack: Option<&str>) -> InitializationFailure {
    let mut message = message.to_string();
    let mut stack = stack.map(str::to_string);
    if let Some(stripped) = message.strip_prefix(REMOTE_INIT_PREFIX).map(str::to_string) {
        let previous = message.clone();
        message = stripped;
        if let Some(previous_stack) = stack.take() {
            stack = Some(previous_stack.replace(
                &format!("Error: {}", previous),
                &format!("Error: {}", message),
            ));
        }
    }
    InitializationFailure {
        message,
        stack,
        local_server_startup: true,
    }
}

pub fn initialization_data<A>(
    value: impl FnOnce() -> Option<A>,
    error: Option<(String, Option<String>)>,
) -> Result<Option<A>, InitializationFailure> {
    if let Some((message, stack)) = error {
        return Err(mark_local_server_startup(&message, stack.as_deref()));
    }
    Ok(value())
}

pub fn initialization_ready<A>(
    value: impl FnOnce() -> Option<A>,
    error: Option<(String, Option<String>)>,
    loading: bool,
) -> Result<bool, InitializationFailure> {
    if loading {
        return Ok(false);
    }
    initialization_data(value, error)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    // Mirrors `src/renderer/initialization.test.ts`
    // (`describe("desktop renderer initialization")`). Object identity
    // (`toBe(error)`) has no Rust equivalent; the message/flag assertions
    // carry the same intent.
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn throws_the_original_initialization_error_before_rendering_server_providers() {
        let failure =
            initialization_data::<()>(|| None, Some(("sidecar startup failed".to_string(), None)))
                .expect_err("initialization fails");
        assert_eq!(failure.message, "sidecar startup failed");
        assert!(failure.local_server_startup);
    }

    #[test]
    fn removes_electrons_remote_invocation_wrapper_from_startup_errors() {
        let failure = initialization_data::<()>(
            || None,
            Some((
                "Error invoking remote method 'await-initialization': Error: Cannot migrate session_message projections"
                    .to_string(),
                None,
            )),
        )
        .expect_err("initialization fails");
        assert_eq!(
            failure.message,
            "Cannot migrate session_message projections"
        );
    }

    #[test]
    fn returns_initialized_sidecar_data() {
        let data = initialization_data(
            || {
                Some((
                    "http://127.0.0.1:1234".to_string(),
                    "opencode".to_string(),
                    "secret".to_string(),
                ))
            },
            None,
        )
        .expect("initialization succeeds");
        assert_eq!(
            data,
            Some((
                "http://127.0.0.1:1234".to_string(),
                "opencode".to_string(),
                "secret".to_string()
            ))
        );
    }

    #[test]
    fn does_not_discard_falsy_initialization_errors() {
        let failure = initialization_data::<()>(|| None, Some((String::new(), None)))
            .expect_err("empty message still fails");
        assert_eq!(failure.message, "");
        assert!(failure.local_server_startup);
    }

    #[test]
    fn checks_initialization_errors_before_rendering_server_providers() {
        let result = initialization_ready::<()>(
            || None,
            Some(("sidecar startup failed".to_string(), None)),
            false,
        );
        let failure = result.expect_err("initialization fails");
        assert_eq!(failure.message, "sidecar startup failed");
    }

    #[test]
    fn waits_for_pending_initialization_without_reading_it() {
        let reads = Rc::new(Cell::new(0));
        let probe = Rc::clone(&reads);
        let ready = initialization_ready(
            || {
                probe.set(probe.get() + 1);
                None::<()>
            },
            None,
            true,
        )
        .expect("loading never fails");
        assert!(!ready);
        assert_eq!(reads.get(), 0);
    }
}
