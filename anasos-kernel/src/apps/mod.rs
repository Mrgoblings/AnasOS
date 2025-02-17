use core::{
    fmt::Debug,
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Poll},
};

use alloc::{boxed::Box, vec::Vec};
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use embedded_graphics::prelude::RgbColor;
use futures_util::{
    stream::{Stream, StreamExt},
    task::AtomicWaker,
};

use crate::{
    println,
    task::draw::{fill_buffer, swap_buffers},
};

pub static APPS_UPDATE_WAKER: AtomicWaker = AtomicWaker::new();
pub static APPS_HAS_UPDATES: AtomicBool = AtomicBool::new(true);

pub mod terminal;

pub static APPS_QUEUE: OnceCell<ArrayQueue<Box<(dyn App + 'static)>>> = OnceCell::uninit();
pub static APPS_SCANNCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

pub struct AppList {
    app_list: Vec<Box<dyn App>>,
    active_app: usize,
}

#[allow(dead_code)]
impl AppList {
    pub fn new() -> Self {
        APPS_SCANNCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("APPLIST> scancode queue init failed");

        AppList {
            app_list: Vec::new(),
            active_app: 0,
        }
    }

    pub fn push(&mut self, app: Box<dyn App>) {
        self.app_list.push(app);
    }

    // TODO implement remove method
    pub fn remove(&mut self, _index: usize) {
        unimplemented!()
    }

    fn init_all(&mut self) {
        for app in &mut self.app_list {
            app.init();
        }
    }

    // lifecycle methods
    fn draw_active(&mut self) {
        println!(
            "APPLIST> Drawing active app: {}",
            self.app_list[self.active_app].name()
        );
        unsafe { self.app_list[self.active_app].draw() };
    }

    fn update_all(&mut self) {
        println!("APPLIST> Updating all apps");
        for app in &mut self.app_list {
            app.update();
        }
    }

    pub fn single_cycle(&mut self) {
        if self.app_list.is_empty() {
            return;
        }

        self.update_all();
        self.draw_active();
    }

    // input handlers
    pub fn handle_scancodes(&self) {
        let scancode_queue = APPS_SCANNCODE_QUEUE.try_get();
        if scancode_queue.is_err() {
            return;
        }

        let scancode_queue = scancode_queue.unwrap();
        while let Some(scancode) = scancode_queue.pop() {
            let _ = self.app_list[self.active_app].scancode_push(scancode);
        }
    }

    pub fn handle_app_queue(&mut self) {
        let app_queue = APPS_QUEUE.try_get();
        if app_queue.is_err() {
            return;
        }

        let app_queue = app_queue.unwrap();
        while let Some(app) = app_queue.pop() {
            app.init(); // NOTE: this may not be the right place to call init
            self.push(app);
        }

        self.change_app(self.app_list.len() - 1);
    }

    // active-app manipulation
    pub fn next_app(&mut self) {
        self.active_app = (self.active_app + 1) % self.app_list.len();
    }

    pub fn prev_app(&mut self) {
        self.active_app = (self.active_app + self.app_list.len() - 1) % self.app_list.len();
    }

    pub fn change_app(&mut self, index: usize) {
        self.active_app = index % self.app_list.len();
    }
}

impl Stream for AppList {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<()>> {
        // Register the current waker
        APPS_UPDATE_WAKER.register(cx.waker());

        // Check if there are updates
        if APPS_HAS_UPDATES.load(Ordering::Relaxed) {
            // Reset the update flag
            // TODO too many wakeups are happening when commented out. executor filled in first 10 sek
            if let Ok(apps_queue) = APPS_QUEUE.try_get() {
                if let Ok(apps_scancode_queue) = APPS_SCANNCODE_QUEUE.try_get() {
                    if apps_queue.is_empty() && !apps_scancode_queue.is_empty() {
                        APPS_HAS_UPDATES.store(false, Ordering::Relaxed);
                    }
                }
            }

            Poll::Ready(Some(()))
        } else {
            Poll::Pending
        }
    }
}

pub async fn apps_lifecycle() {
    println!("APPLIST> Starting apps lifecycle");
    let mut apps_list = AppList::new();

    loop {
        println!("APPLIST> Apps lifecycle loop");
        apps_list.handle_scancodes();
        println!("APPLIST> Scancodes handled");
        apps_list.handle_app_queue();
        println!("APPLIST> App queue handled");

        apps_list.next().await;
        
        println!("APPLIST> Next cycle");
        apps_list.single_cycle();
        println!("APPLIST> Single cycle");

        //swap buffers
        swap_buffers();
        fill_buffer(RgbColor::BLACK);
    }
}

pub fn add_app(app: Box<dyn App>) {
    println!("APPLIST> Adding app: {}", app.name());
    let app_queue = APPS_QUEUE
        .try_get_or_init(|| ArrayQueue::new(10))
        .expect("APPLIST> app queue uninitialized");
    let _ = app_queue.push(app);
    APPS_HAS_UPDATES.store(true, Ordering::Relaxed);
    println!("APPLIST> App added");
}

pub fn add_scancode(scancode: u8) {
    let scancode_queue = APPS_SCANNCODE_QUEUE
        .try_get_or_init(|| ArrayQueue::new(100))
        .expect("APPLIST> scancode queue uninitialized");
    let _ = scancode_queue.push(scancode);
    APPS_HAS_UPDATES.store(true, Ordering::Relaxed);
}

pub trait App: Send + Sync {
    //getters methods
    fn name(&self) -> &'static str;
    fn priority(&self) -> u8;
    fn title(&self) -> &'static str;

    // input methods
    fn scancode_push(&self, scancode: u8) -> Result<(), ()>;

    // lifecycle methods
    fn init(&self);
    unsafe fn draw(&mut self);
    fn update(&mut self);
}
