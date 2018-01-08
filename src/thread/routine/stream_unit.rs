use sync::spsc::unit::{channel, Receiver, SendError};
use thread::prelude::*;

/// A stream of results from another thread.
///
/// This stream can be created by the instance of [`Thread`].
///
/// [`Thread`]: ../trait.Thread.html
#[must_use]
pub struct RoutineStreamUnit<E> {
  rx: Receiver<E>,
}

impl<E> RoutineStreamUnit<E> {
  pub(crate) fn new<T, G, O>(thread: &T, mut generator: G, overflow: O) -> Self
  where
    T: Thread,
    G: Generator<Yield = Option<()>, Return = Result<Option<()>, E>>,
    O: Fn() -> Result<(), E>,
    G: Send + 'static,
    E: Send + 'static,
    O: Send + 'static,
  {
    let (mut tx, rx) = channel();
    thread.routine(move || loop {
      if tx.is_canceled() {
        break;
      }
      match generator.resume() {
        Yielded(None) => {}
        Yielded(Some(())) => match tx.send() {
          Ok(()) => {}
          Err(SendError::Canceled) => {
            break;
          }
          Err(SendError::Overflow) => match overflow() {
            Ok(()) => {}
            Err(err) => {
              tx.send_err(err).ok();
              break;
            }
          },
        },
        Complete(Ok(None)) => {
          break;
        }
        Complete(Ok(Some(()))) => {
          tx.send().ok();
          break;
        }
        Complete(Err(err)) => {
          tx.send_err(err).ok();
          break;
        }
      }
      yield;
    });
    Self { rx }
  }

  /// Gracefully close this stream, preventing sending any future messages.
  #[inline(always)]
  pub fn close(&mut self) {
    self.rx.close()
  }
}

impl<E> Stream for RoutineStreamUnit<E> {
  type Item = ();
  type Error = E;

  #[inline(always)]
  fn poll(&mut self) -> Poll<Option<()>, E> {
    self.rx.poll()
  }
}