use parking_lot::{Condvar, Mutex, MutexGuard};

pub struct Monitor {
    mutex: Mutex<()>,
    cvar: Condvar,
}

pub struct MonitorGuard<'a> {
    guard: MutexGuard<'a, ()>,
    cvar: &'a Condvar,
}

impl Monitor {
    pub fn new() -> Self {
        Monitor {
            mutex: Mutex::new(()),
            cvar: Condvar::new(),
        }
    }

    pub fn enter(&self) -> MonitorGuard {
        let guard = self.mutex.lock();
        MonitorGuard {
            guard,
            cvar: &self.cvar,
        }
    }

    pub fn is_locked(&self) -> bool {
        self.mutex.is_locked()
    }
}

impl<'a> MonitorGuard<'a> {
    pub fn wait(&mut self) {
        self.cvar.wait(&mut self.guard)
    }

    pub fn notify_one(&self) {
        self.cvar.notify_one();
    }

    pub fn notify_all(&self) {
        self.cvar.notify_all();
    }
}
