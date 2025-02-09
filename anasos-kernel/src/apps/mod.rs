use core::{future::Future, pin::Pin, task::{Context, Poll}};

use conquer_once::spin::OnceCell;
use alloc::{boxed::Box, vec::{Vec}};
use crossbeam_queue::ArrayQueue;
use futures_util::{task::AtomicWaker, Stream};

use crate::println;
use spin::Mutex;

pub mod terminal;


pub static APPS: OnceCell<Mutex<Option<AppList>>> = OnceCell::uninit();

pub trait App: Send + Sync {
    fn name(&self) -> &'static str;
    fn priority(&self) -> u8;
    fn init(&self);
    unsafe fn draw(&self);
    fn load(&mut self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
    fn add_key_input(&self, scancode: u8);
}

pub struct KeyStream {
    scancode_queue: ArrayQueue<u8>,
    waker: AtomicWaker,
}

impl KeyStream {
    pub fn new() -> Self {
        KeyStream {
            scancode_queue: ArrayQueue::new(100),
            waker: AtomicWaker::new(),
        }
    }

    pub fn add_scancode(&self, scancode: u8) {
        println!("add_scancode: scancode: {}", scancode);
        if let Err(_) = self.scancode_queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            self.waker.wake();
        }
    }
}

impl Stream for KeyStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        println!("KeyStream::poll_next");
        let queue = &self.scancode_queue;

        // fast path
        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        self.waker.register(&cx.waker());
        match queue.pop() {
            Some(scancode) => {
                self.waker.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }

}

pub struct AppList {
    apps: Vec<Box<dyn App>>,
    active_app: usize,
}

impl AppList {
    pub fn new() -> Self {
        APPS.try_init_once(|| Mutex::new(Some(AppList { apps: Vec::new(), active_app: 0 }))).expect("AppList::new should only be called once");
        AppList { apps: Vec::new(), active_app: 0}
    }

    pub fn add_app(&mut self, app: Box<dyn App>) {
        self.apps.push(app);
    }

    pub fn draw_all(&mut self) {
        self.apps.sort_by(|a, b| a.priority().cmp(&b.priority()));

        for app in &self.apps {
            unsafe {
                app.draw();
            }
        }
    }

    pub fn init_all(&mut self) {
        for app in &self.apps {
            app.init();
        }
    }

    pub async fn load_all(&mut self) {
        for app in &mut self.apps {
            app.load().await;
        }
    }

    pub fn add_key_input(&self, scancode: u8) {
        self.apps[self.active_app].add_key_input(scancode);
    }

    pub fn next_app(&mut self) {
        self.active_app = (self.active_app + 1) % self.apps.len();
    }

    pub fn prev_app(&mut self) {
        self.active_app = (self.active_app + self.apps.len() - 1) % self.apps.len();
    }

    pub fn change_app(&mut self, index: usize) {
        self.active_app = index % self.apps.len();
    }

    pub fn active_app(&self) -> &Box<dyn App> {
        &self.apps[self.active_app]
    }
}


pub async fn load_all_apps() {
    APPS.try_get().unwrap().lock().as_mut().unwrap().load_all().await;
}