use std::sync::mpsc::sync_channel;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;

use eyre::{bail, eyre, WrapErr};
use swayipc::{self, Connection, Event, EventType, WindowChange};

use super::queue::WindowQueue;

fn filter_event(
    event_result: swayipc::Fallible<Event>,
    urgent_first: bool,
) -> eyre::Result<Option<WindowEvent>> {
    let event = event_result.wrap_err("failed reading Sway event result")?;

    let window_event = match event {
        Event::Window(window_event) => window_event,
        _ => return Ok(None),
    };

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

            match Window::from_node(window_event.container) {
                Some(sway_window) => Ok(Some(WindowEvent::Focus(sway_window))),
                None => Ok(None),
            }
        }
        WindowChange::Close => Ok(Some(WindowEvent::Close(window_event.container.id))),
        _ => Ok(None),
    }
}

#[derive(Debug)]
pub struct WindowSubscription {
    queue: Arc<RwLock<WindowQueue>>,
    errors: mpsc::Receiver<eyre::Report>,
}

impl WindowSubscription {
    pub fn subscribe(urgent_first: bool) -> eyre::Result<WindowSubscription> {
        // We use a rendezvous channel because we don't want the errors piling up in an infinite
        // channel buffer until the next time the user opens the window switcher. The subscription
        // listener thread will block on the first error it encounters, and then that error will be
        // propagated next time the user opens the window switcher.
        let (err_sender, err_receiver) = sync_channel(0);

        let connection = Connection::new().wrap_err("failed acquiring a Sway IPC connection")?;
        let subscription = connection
            .subscribe([EventType::Window])
            .wrap_err("failed opening a Sway window event subscription")?;

        let sending_queue = Arc::new(RwLock::new(WindowQueue::new()));
        let receiving_queue = Arc::clone(&sending_queue);

        thread::spawn(move || {
            for event_result in subscription {
                if let Some(result) = filter_event(event_result, urgent_first).transpose() {
                    match result {
                        Ok(event) => {
                            match sending_queue.write() {
                                Ok(mut queue) => queue.push_event(event),
                                // Lock is poisoned.
                                Err(_) => break,
                            }
                        }
                        Err(err) => {
                            let is_closed = err_sender.send(err).is_err();

                            if is_closed {
                                break;
                            }
                        }
                    }
                } else {
                    continue;
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
                bail!("window priority queue errors channel closed unexpectedly");
            }
            _ => {}
        }

        match self.queue.read() {
            Ok(queue) => Ok(queue.sorted_windows()),
            Err(_) => Err(eyre!("lock on window priority queue is poisoned")),
        }
    }
}

pub type SwayNodeId = i64;

pub enum WindowEvent {
    // A window was focused, created, or marked urgent.
    Focus(Window),

    // A window was closed.
    Close(SwayNodeId),
}

#[derive(Debug, Clone)]
pub struct Window {
    pub id: SwayNodeId,
    pub window_title: String,
    pub app_id: String,
}

impl Window {
    // Returns `None` if the node is not a view.
    fn from_node(node: swayipc::Node) -> Option<Self> {
        if let (Some(name), Some(app_id)) = (node.name, node.app_id) {
            Some(Self {
                id: node.id,
                window_title: name,
                app_id,
            })
        } else {
            None
        }
    }
}
