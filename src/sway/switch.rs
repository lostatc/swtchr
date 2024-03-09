use eyre::WrapErr;
use swayipc::Connection;

use super::subscribe::SwayWindowId;

pub fn switch_window(id: SwayWindowId) -> eyre::Result<()> {
    let mut connection = Connection::new().wrap_err("failed acquiring a Sway IPC connection")?;

    connection
        .run_command(format!("[con_id=\"{}\"] focus", id.0))
        .wrap_err("failed running Sway window switch command over IPC API")?
        .into_iter()
        .collect::<Result<_, _>>()
        .wrap_err("failed running Sway window switch command over IPC API")?;

    Ok(())
}
