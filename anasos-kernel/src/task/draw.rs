use core::{
    pin::Pin,
    task::{Context, Poll},
};

use alloc::sync::Arc;

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use embedded_graphics::pixelcolor::Rgb888;
use futures_util::{task::AtomicWaker, Stream, StreamExt};

use crate::framebuffer::{FramePosition, FRAMEBUFFER};

pub const FRAME_QUEUE_SIZE: usize = 100;


static FRAME_QUEUE: OnceCell<Arc<ArrayQueue<FramePosition>>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub struct FrameStream {
    _private: (),
}

impl FrameStream {
    pub fn new() -> Self {
        FRAME_QUEUE.try_init_once(|| Arc::new(ArrayQueue::new(FRAME_QUEUE_SIZE)))
            .expect("DRAW> FrameStream::new should only be called once");

        FrameStream { _private: () }
    }
}

impl Stream for FrameStream {
    type Item = FramePosition;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<FramePosition>> {
        let queue = FRAME_QUEUE
            .try_get()
            .expect("DRAW> Frame queue not initialized");

        // fast path
        if let Some(frame) = queue.pop() {
            return Poll::Ready(Some(frame));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(frame) => {
                WAKER.take();
                Poll::Ready(Some(frame))
            }
            None => Poll::Pending,
        }
    }
}


pub fn swap_buffers() {
    let mut framebuffer = unsafe { FRAMEBUFFER.lock() };
    if let Some(framebuffer) = framebuffer.as_mut() {
        framebuffer.swap_buffers();
    }
}

pub fn fill_buffer(color: Rgb888) {
    let mut framebuffer = unsafe { FRAMEBUFFER.lock() };
    if let Some(framebuffer) = framebuffer.as_mut() {
        framebuffer.fill(color);
    }
}
