#![allow(dead_code)] // TODO: remove this at some point
use std::collections::HashMap;

// Lambda prototype: (EventName: String) -> ()
type EventCallback<'a> = Box<dyn FnMut(String) -> () + 'a>;

pub struct EventEngine {
    event: HashMap<String, EventCallback<'static>>,
}

impl EventEngine {
    pub fn new() -> EventEngine {
        return EventEngine {
            event: HashMap::new(),
        };
    }

    pub fn on<F: 'static>(&mut self, id: String, cb: F)
    where
        F: FnMut(String),
    {
        self.event.insert(id, Box::new(cb));
    }

    pub fn on_raw<F: 'static>(&mut self, id: &str, cb: F)
    where
        F: FnMut(String),
    {
        self.on(id.to_string(), cb);
    }

    pub fn unon(&mut self, id: String) {
        self.event.remove(&id);
    }

    pub fn unon_raw(&mut self, id: &str) {
        self.unon(id.to_string());
    }

    pub fn is_set(&mut self, id: String) -> bool {
        self.event.contains_key(&id)
    }

    pub fn is_set_raw(&mut self, id: &str) -> bool {
        self.is_set(id.to_string())
    }

    pub fn emit(&mut self, id: String) {
        match self.event.get_mut(&id) {
            Some(cb) => cb(id),
            None => println!("Event not set"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_use() {
        let mut x1 = EventEngine::new();
        let t = true;
        let cb = move |s: String| {
            println!("{}", t);
            assert_eq!(s, "test");
        };

        x1.on_raw("test", cb);
        x1.emit("test".to_string());
    }

    #[test]
    fn unon() {
        let mut x1 = EventEngine::new();
        let cb = move |s: String| {
            assert_eq!(s, "test");
        };

        x1.on_raw("test", cb);
        assert!(x1.is_set_raw("test") == true);
        x1.unon_raw("test");
        assert!(x1.is_set_raw("test") == false);
    }
}
