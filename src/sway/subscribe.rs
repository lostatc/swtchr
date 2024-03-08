use eyre::WrapErr;
use std::sync::mpsc;
use std::thread;
use swayipc::{self, Connection, Event, EventType, WindowChange};

fn filter_event(event_result: swayipc::Fallible<Event>) -> eyre::Result<Option<WindowEvent>> {
    let event = event_result.wrap_err("failed reading Sway event result")?;

    let window_event = match event {
        Event::Window(window_event) => window_event,
        _ => return Ok(None),
    };

    match window_event.change {
        WindowChange::New | WindowChange::Focus => {
            match Window::from_node(window_event.container) {
                Some(sway_window) => Ok(Some(WindowEvent::FocusOrNew(sway_window))),
                None => Ok(None),
            }
        }
        WindowChange::Close => Ok(Some(WindowEvent::Close(window_event.container.id))),
        _ => Ok(None),
    }
}

fn subscribe_focus_events() -> eyre::Result<mpsc::Receiver<eyre::Result<WindowEvent>>> {
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

pub type SwayNodeId = i64;

pub enum WindowEvent {
    FocusOrNew(Window),
    Close(SwayNodeId),
}

#[derive(Debug)]
pub struct Window {
    pub id: SwayNodeId,
    pub urgent: bool,
    pub window_title: String,
    pub app_id: String,
}

impl Window {
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
