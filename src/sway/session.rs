use std::env;

use eyre::bail;

// It's really important that we avoid false negatives here, so we should be as permissive as
// possible in the values we accept for these environment variables. We should not fail if the
// environment variables are unset.
pub fn check_is_sway_session() -> eyre::Result<()> {
    if let Ok(session_type) = env::var("XDG_SESSION_TYPE") {
        let session_type_normalized = session_type.trim().to_lowercase();

        if !session_type_normalized.is_empty() && session_type_normalized != "wayland" {
            bail!("You do not seem to be running in a Wayland session. This tool only supports the Sway window manager. You can run swtchrd with --no-check to override this.");
        }
    }

    let desktop_session_vars = ["DESKTOP_SESSION", "XDG_SESSION_DESKTOP"];

    for session_var in desktop_session_vars {
        if let Ok(session) = env::var(session_var) {
            let session_normalized = session.trim().to_lowercase();

            // The Regolith desktop environment runs the Sway window manager, and is therefore
            // supported, but it uses `DESKTOP_SESSION=regolith-sway`. To try and support cases
            // like this, we check if the string starts or ends with `sway`.
            if !session_normalized.is_empty()
                && !session_normalized.starts_with("sway")
                && !session_normalized.ends_with("sway")
            {
                bail!("You do not seem to be running in a Sway session. This tool only supports the Sway window manager. You can run swtchrd with --no-check to override this.");
            }
        }
    }

    Ok(())
}
