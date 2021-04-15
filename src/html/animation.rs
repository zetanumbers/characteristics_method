use std::{cell::RefCell, rc::Rc};

use super::util::{cancel_animation_frame, performance, request_animation_frame};
use wasm_bindgen::{prelude::Closure, JsCast};

#[derive(Debug, Clone, Copy)]
enum AnimationState {
    Pause { stopped_at: f64 },
    Running { t0: f64, handle: i32 },
}

impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::Pause { stopped_at: 0.0 }
    }
}

pub struct Animation {
    callback: Option<Closure<dyn FnMut(f64)>>,
    state: AnimationState,
}

impl Animation {
    pub fn resume(&mut self) {
        match self.state {
            AnimationState::Pause { stopped_at } => {
                self.state = AnimationState::Running {
                    t0: performance().now() - stopped_at,
                    handle: request_animation_frame(
                        self.callback.as_ref().unwrap().as_ref().unchecked_ref(),
                    ),
                }
            }
            _ => (),
        }
    }

    pub fn pause(&mut self) {
        match self.state {
            AnimationState::Running { t0, handle } => {
                cancel_animation_frame(handle);
                self.state = AnimationState::Pause {
                    stopped_at: performance().now() - t0,
                };
            }
            _ => (),
        }
    }

    pub fn reset(&mut self) {
        match self.state {
            AnimationState::Running { t0: _, handle } => {
                cancel_animation_frame(handle);
            }
            _ => (),
        }
        self.state = AnimationState::Pause { stopped_at: 0.0 };
    }

    /// Returns t0
    fn request(&mut self) -> f64 {
        match self.state {
            AnimationState::Running { t0, ref mut handle } => {
                cancel_animation_frame(*handle);
                *handle = request_animation_frame(
                    self.callback.as_ref().unwrap().as_ref().unchecked_ref(),
                );
                t0
            }
            _ => unreachable!(),
        }
    }

    /// New paused animation
    pub fn new(mut tick: impl FnMut(f64) + 'static) -> Rc<RefCell<Self>> {
        let result = Rc::new(RefCell::new(Animation {
            callback: None,
            state: Default::default(),
        }));

        let handle = Rc::downgrade(&result);
        result.borrow_mut().callback = Some(Closure::wrap(Box::new(move |now| {
            if let Some(handle) = handle.upgrade() {
                tick(now - handle.borrow_mut().request())
            }
        })));
        result
    }
}
