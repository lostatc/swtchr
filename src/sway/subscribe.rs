use std::sync::mpsc::sync_channel;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;

use eyre::{bail, eyre, WrapErr};
use gtk::glib;
use swayipc::{self, Connection, Event, EventType, WindowChange,ScratchpadState};

use super::queue::WindowQueue;
use super::IconLocator;

fn filter_event(
    event_result: swayipc::Fallible<Event>,
    urgent_first: bool,
    scratch_pad_config: bool,
) -> eyre::Result<Option<WindowEvent>> {
    let event = event_result.wrap_err("Failed reading Sway event result.")?;

    let window_event = match event {
        Event::Window(window_event) => window_event,
        _ => return Ok(None),
    };

    //checking if the window is scratchpad
    let scratch_pad= match window_event.container.scratchpad_state.unwrap() {
                                    //not using scratch pad
                                    ScratchpadState::None => false,
                                    //everything else
                                    _=> true
                                };
    // skipping the wndow if it is scratchpad
    if scratch_pad && !scratch_pad_config {
        return Ok(None);
    }


    match window_event.change {
        WindowChange::New | WindowChange::Focus | WindowChange::Urgent => {
            if window_event.change == WindowChange::Urgent
                && (!urgent_first || !window_event.container.urgent)
            {
                // One of two things has happened:
                // 1. Urgent-first window ordering is turned off.
                // 2. The window urgency changed, but from being urgent to being not-urgent. This
                //    shouldn't affect the order in the window switcher.
                return Ok(None);
            }

            Ok(Some(WindowEvent::Focus(Window::from(
                window_event.container,
            ))))
        }
        WindowChange::Close => Ok(Some(WindowEvent::Close(SwayWindowId(
            window_event.container.id,
        )))),
        _ => Ok(None),
    }
}

#[derive(Debug)]
pub struct WindowSubscription {
    queue: Arc<RwLock<WindowQueue>>,
    errors: mpsc::Receiver<eyre::Report>,
}

impl WindowSubscription {
    pub fn subscribe(urgent_first: bool,scratch_pad_config: bool) -> eyre::Result<WindowSubscription> {
        // We use a rendezvous channel because we don't want the errors piling up in an infinite
        // channel buffer until the next time the user opens the window switcher. The subscription
        // listener thread will block on the first error it encounters, and then that error will be
        // propagated next time the user opens the window switcher.
        let (err_sender, err_receiver) = sync_channel(0);

        let connection = Connection::new().wrap_err("Failed acquiring a Sway IPC connection.")?;
        let subscription = connection
            .subscribe([EventType::Window])
            .wrap_err("Failed opening a Sway window event subscription.")?;

        let sending_queue = Arc::new(RwLock::new(WindowQueue::new()));
        let receiving_queue = Arc::clone(&sending_queue);

        thread::spawn(move || {
            for event_result in subscription {
                if let Some(result) = filter_event(event_result, urgent_first,scratch_pad_config).transpose() {
                    match result {
                        Ok(event) => match sending_queue.write() {
                            Ok(mut queue) => queue.push_event(event),
                            Err(_) => {
                                eprintln!("Lock on window priority queue is poisoned.");
                                break;
                            }
                        },
                        Err(err) => {
                            let is_closed = err_sender.send(err).is_err();

                            if is_closed {
                                eprintln!(
                                    "Cannot send Sway IPC error: Channel closed unexpectedly."
                                );
                                break;
                            }
                        }
                    }
                }
            }
        });

        Ok(Self {
            queue: receiving_queue,
            errors: err_receiver,
        })
    }

    pub fn get_window_list(&self) -> eyre::Result<Vec<Window>> {
        // See if any errors have occurred since we last polled the window list.
        match self.errors.try_recv() {
            Ok(err) => return Err(err),
            // Only fail when the channel is disconnected, not when the channel is empty.
            Err(mpsc::TryRecvError::Disconnected) => {
                bail!("Cannot receive Sway IPC error: Channel closed unexpectedly.");
            }
            _ => {}
        }

        match self.queue.read() {
            Ok(queue) => Ok(queue.sorted_windows()),
            Err(_) => Err(eyre!("Lock on window priority queue is poisoned.")),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, glib::ValueDelegate)]
pub struct SwayWindowId(pub(super) i64);

impl SwayWindowId {
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}

pub enum WindowEvent {
    // A window was focused, created, or marked urgent.
    Focus(Window),

    // A window was closed.
    Close(SwayWindowId),
}

#[derive(Debug, Clone)]
pub struct Window {
    pub id: SwayWindowId,
    pub title: String,
    pub icon_locator: IconLocator,
}

impl From<swayipc::Node> for Window {
    fn from(node: swayipc::Node) -> Self {
        Self {
            id: SwayWindowId(node.id),
            title: node.name.as_ref().cloned().unwrap_or_else(String::new),
            icon_locator: node.into(),
        }
    }
}
