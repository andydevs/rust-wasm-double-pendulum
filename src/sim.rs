use crate::jsanim::{FrameCtx, WindowCtx};

#[allow(dead_code)]
pub struct RenderCtx<'s> {
    pub window: &'s WindowCtx,
    pub frame: &'s FrameCtx,
}

#[allow(dead_code)]
pub struct UpdateCtx<'s> {
    pub frame: &'s FrameCtx,
}

pub trait Simulation {
    fn render(&self, render: &RenderCtx);

    fn update(&mut self, update: &UpdateCtx);
}
