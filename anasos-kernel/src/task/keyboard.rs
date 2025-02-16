use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

use core::{
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::stream::{Stream, StreamExt};
use futures_util::task::AtomicWaker;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

use crate::{apps::{self, APPS_SCANNCODE_QUEUE}, print, println};

// static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

static WAKER: AtomicWaker = AtomicWaker::new();

// pub async fn load_keypresses() {
//     let mut scancodes = ScancodeStream::new();
//     let mut keyboard = Keyboard::new(
//         ScancodeSet1::new(),
//         layouts::Us104Key,
//         HandleControl::Ignore,
//     );

//     while let Some(scancode) = scancodes.next().await {
//         apps::add_scancode(scancode);

//         // debug print
//         if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
//             if let Some(key) = keyboard.process_keyevent(key_event) {
//                 match key {
//                     DecodedKey::Unicode(character) => print!("{}", character),
//                     DecodedKey::RawKey(key) => print!("{:?}", key),
//                 }
//             }
//         }
//     }
// }

/// Called by the keyboard interrupt handler
///
/// Must not block or allocate.
pub(crate) fn add_scancode(scancode: u8) {
    apps::add_scancode(scancode);
}

// pub struct ScancodeStream {
//     _private: (),
// }

// impl ScancodeStream {
//     pub fn new() -> Self {
//         SCANCODE_QUEUE
//             .try_init_once(|| ArrayQueue::new(100))
//             .expect("ScancodeStream::new should only be called once");
//         ScancodeStream { _private: () }
//     }
// }

// impl Stream for ScancodeStream {
//     type Item = u8;

//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
//         let queue = SCANCODE_QUEUE
//             .try_get()
//             .expect("Scancode queue not initialized");

//         // fast path
//         if let Some(scancode) = queue.pop() {
//             return Poll::Ready(Some(scancode));
//         }

//         WAKER.register(&cx.waker());
//         match queue.pop() {
//             Some(scancode) => {
//                 WAKER.take();
//                 Poll::Ready(Some(scancode))
//             }
//             None => Poll::Pending,
//         }
//     }
// }
