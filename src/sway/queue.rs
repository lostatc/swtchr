use std::cmp;
use std::collections::HashMap;

use super::subscribe::{SwayWindowId, Window, WindowEvent};

#[derive(Debug, Clone)]
struct WindowPriority {
    window: Window,
    priority: u64,
}

#[derive(Debug)]
pub struct WindowQueue {
    map: HashMap<SwayWindowId, WindowPriority>,
    // We're assuming this is big enough to never overflow.
    highest_priority: u64,
}

impl WindowQueue {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            highest_priority: 0,
        }
    }

    pub fn push_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::Focus(window) => {
                self.highest_priority += 1;

                self.map.insert(
                    window.id,
                    WindowPriority {
                        window,
                        priority: self.highest_priority,
                    },
                );
            }
            WindowEvent::Close(node_id) => {
                self.map.remove(&node_id);
            }
        }
    }

    // Return the list of windows in the queue sorted from most recently used to least recently
    // used.
    pub fn sorted_windows(&self) -> Vec<Window> {
        let mut list = self.map.values().cloned().collect::<Vec<_>>();

        list.sort_by_key(|window_priority| cmp::Reverse(window_priority.priority));

        list.into_iter()
            .map(|window_priority| window_priority.window)
            .collect::<Vec<_>>()
    }
}
