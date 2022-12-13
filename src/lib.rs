use std::sync::{Mutex, Arc};
use std::time::{Instant, Duration};


/// Shortcut for timing a section of code and adding it to the given ['OneList<OneDuration>']
#[macro_export]
macro_rules! onestop {
    (($stopwatch:ident, $name:expr) => $code:block) => {
        {
            //Wasteful but whatever (the string from)
            let mut dur = OneDuration::new(String::from($name));
            $code
            dur.finish();
            $stopwatch.add(dur);
        }
    };
}

/// Represents a single duration you have tracked and named, usually to 
/// time code segments. Light wrapper around ['std::time::Instant'].
/// Spawning with [`Self::new()`] immediately starts tracking a duration, then you [`Self::finish()`]
/// it when done. You would then normally add it to a [`OneList<T>`]
#[derive(Debug, Clone, PartialEq)]
pub struct OneDuration
{
    pub name: String,
    pub clock: Option<Instant>,
    pub duration: Duration
}

impl OneDuration {
    /// Create a named duration from an existing ['std::time::Duration`]
    pub fn from_duration(duration: Duration, name: String) -> Self {
        Self {
            name,
            duration,
            clock: None
        }
    }

    /// Create AND START the clock, which you can finish with `finish()`
    pub fn new(name: String) -> Self {
        Self {
            name,
            duration: Duration::from_secs(0),
            clock: Some(Instant::now())
        }
    }

    /// Complete the given timer. Returns if the duration was updated or not (if you
    /// have no timer set, this won't update anything)
    pub fn finish(&mut self) -> bool {
        if let Some(clock) = self.clock {
            self.duration = clock.elapsed();
            true
        }
        else {
            false
        }
    }
}

/// A threadsafe list to be shared between contexts or threads. Inexpensive clones,
/// all clones point to the same list. Combine with [`OneDuration`] for aggregating
/// code timings
#[derive(Debug, Clone)]
pub struct OneList<T> where T : Send + Sync + Clone {
    //Can do auto-clone because it's just arc?
    pub items: Arc<Mutex<Vec<T>>>
}

impl<T> OneList<T> where T : Send + Sync + Clone {
    /// Create a NEW list, no copies yet
    pub fn new() -> Self {
        Self { 
            items: Arc::new(Mutex::new(Vec::new()))
        }
    }

    /// Add an item in a thread-safe manner.
    pub fn add(&mut self, item: T) {
        let mut items = self.items.lock().unwrap();
        items.push(item)
    }

    /// Get a FULL COPY of all existing items saved in this instance. All items are cloned,
    /// use this only as needed
    pub fn list_copy(&self) -> Vec<T> {
        let items = self.items.lock().unwrap();
        items.iter().map(|p| p.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_updates() {
        let mut profiler = OneList::<OneDuration>::new();
        let mut other = profiler.clone();
        profiler.add(OneDuration::from_duration(Duration::from_micros(10), String::from("hello")));
        other.add(OneDuration::from_duration(Duration::from_nanos(99), String::from("wow")));

        let vec1 = profiler.list_copy();
        let vec2 = other.list_copy();

        assert_eq!(vec1.len(), vec2.len());
        assert_eq!(vec1.len(), 2);
        assert_eq!(vec1.get(0), vec2.get(0));
        assert_eq!(vec1.get(1), vec2.get(1));
    }

    #[test]
    fn macro_works() {
        let mut list = OneList::<OneDuration>::new();
        let mut count = 0;
        onestop!((list, "wow") => {
            count += 1;
        });
        assert_eq!(count, 1);
        let durs = list.list_copy();
        assert_eq!(1, durs.len());
        assert_eq!("wow", &durs.get(0).unwrap().name);
    }

}
