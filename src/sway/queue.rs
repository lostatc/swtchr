use std::collections::HashMap;

use super::subscribe::{SwayNodeId, Window, WindowEvent};

struct WindowPriority {
    window: Window,
    priority: u64,
}

pub struct WindowQueue {
    map: HashMap<SwayNodeId, WindowPriority>,
    highest_priority: u64,
}

impl WindowQueue {
    pub fn push_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::Focus(window) => {
                self.highest_priority += 1;

                self.map
                    .entry(window.id)
                    .and_modify(|window_priority| {
                        window_priority.priority = self.highest_priority;
                    })
                    .or_insert(WindowPriority {
                        window,
                        priority: self.highest_priority,
                    });
            }
            WindowEvent::Close(node_id) => {
                self.map.remove(&node_id);
            }
        }
    }

    pub fn sorted_windows(&self) -> Vec<&Window> {
        let mut list = self.map.values().collect::<Vec<_>>();

        list.sort_by_key(|window_priority| window_priority.priority);

        list.iter()
            .map(|window_priority| &window_priority.window)
            .collect::<Vec<_>>()
    }
}
