use eyre::WrapErr;
use std::sync::mpsc;
use std::thread;
use swayipc::{self, Connection, Event, EventType, WindowChange};

fn filter_event(event_result: swayipc::Fallible<Event>) -> eyre::Result<Option<SwayWindowEvent>> {
    let event = event_result.wrap_err("failed reading Sway event result")?;

    let window_event = match event {
        Event::Window(window_event) => window_event,
        _ => return Ok(None),
    };

    match window_event.change {
        WindowChange::New | WindowChange::Focus => {
            match SwayWindow::from_node(window_event.container) {
                Some(sway_window) => Ok(Some(SwayWindowEvent::Focus(sway_window))),
                None => Ok(None),
            }
        }
        WindowChange::Close => Ok(Some(SwayWindowEvent::Close(window_event.container.id))),
        _ => Ok(None),
    }
}

fn subscribe_focus_events() -> eyre::Result<mpsc::Receiver<eyre::Result<SwayWindowEvent>>> {
    let (sender, receiver) = mpsc::channel();

    let connection = Connection::new().wrap_err("failed acquiring a Sway IPC connection")?;
    let subscription = connection
        .subscribe([EventType::Window])
        .wrap_err("failed opening a Sway window event subscription")?;

    thread::spawn(move || -> eyre::Result<()> {
        for event_result in subscription {
            if let Some(result) = filter_event(event_result).transpose() {
                sender
                    .send(result)
                    .expect("failed sending Sway window event result to channel");
            } else {
                continue;
            }
        }

        Ok(())
    });

    Ok(receiver)
}

type SwayNodeId = i64;

pub enum SwayWindowEvent {
    Focus(SwayWindow),
    Close(SwayNodeId),
}

#[derive(Debug)]
pub struct SwayWindow {
    id: SwayNodeId,
    urgent: bool,
    window_title: String,
    app_id: String,
}

impl SwayWindow {
    // Returns `None` if the node is not a view.
    fn from_node(node: swayipc::Node) -> Option<Self> {
        if let (Some(name), Some(app_id)) = (node.name, node.app_id) {
            Some(Self {
                id: node.id,
                urgent: node.urgent,
                window_title: name,
                app_id,
            })
        } else {
            None
        }
    }
}
