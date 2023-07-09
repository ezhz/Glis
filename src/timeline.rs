
use std::
{
    time::Instant,
    num::NonZeroU32
};

// ------------------------------------------------------------

pub type Frame = u32;

// ------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct FPS(pub NonZeroU32);

impl Default for FPS
{
    fn default() -> Self
    {
        Self(unsafe{NonZeroU32::new_unchecked(60)})
    }
}

// ------------------------------------------------------------

#[derive(Clone, Copy)]
pub enum FrameRange
{
    Bounded(NonZeroU32),
    Endless
}

impl Default for FrameRange
{
    fn default() -> Self
    {
        Self::Endless
    }
}

// ------------------------------------------------------------

#[derive(Clone)]
pub struct Timeline
{
    fps: u32,
    frame: Frame,
    range: FrameRange,
    onset: Option<Instant>
}

impl Default for Timeline
{
    fn default() -> Self
    {
        Self::new
        (
            FPS::default(),
            FrameRange::default()
        )
    }
}

impl Timeline
{
    pub fn new(FPS(fps): FPS, range: FrameRange) -> Self
    {
        let fps = fps.get();
        Self{fps, frame: 0, range, onset: None}
    }

    #[allow(unused)]
    pub fn fps(&self) -> u32
    {
        self.fps
    }

    #[allow(unused)]
    pub fn range(&self) -> Option<u32>
    {
        if let FrameRange::Bounded(end) = self.range
        {
            return Some(end.get())
        }
        None
    }

    pub fn time(&self) -> f32
    {
        self.frame as f32 / self.fps as f32
    }
}

impl Iterator for Timeline
{
    type Item = Frame;
    fn next(&mut self) -> Option<Self::Item>
    {
        match &mut self.onset
        {
            Some(ref mut onset) =>
            {
                let mut frame: Frame =
                (
                    self.fps as f32 * onset.elapsed()
                        .as_secs_f32()
                ) as _;
                if let FrameRange::Bounded(end) = self.range
                {
                    frame %= end.get()
                }
                match frame == self.frame
                {
                    true => None,
                    false =>
                    {
                        self.frame = frame;
                        if frame == 0
                        {
                            *onset = Instant::now()
                        }
                        Some(frame)
                    }
                }
            }
            None =>
            {
                self.onset = Some(Instant::now());
                Some(0)
            }
        }
    }
}

