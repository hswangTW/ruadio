//! The buffer view types are used to view and manipulate multi-channel audio data without owning
//! the data. Note that the buffer view types assume that all the channels have the same length.
//! If this is not the case, the behavior is undefined and may lead to panics.

/// A non-owning view into multi-channel audio data
#[derive(Debug)]
pub struct BufferView<'a> {
    channels: &'a [&'a [f32]],
    num_samples: usize,
}

/// A mutable view into multi-channel audio data
#[derive(Debug)]
pub struct BufferViewMut<'a> {
    channels: &'a mut [&'a mut [f32]],
    num_samples: usize,
}

impl<'a> BufferView<'a> {
    pub fn new(channels: &'a [&'a [f32]]) -> Self {
        Self {
            channels,
            num_samples: channels.get(0).map_or(0, |ch| ch.len()),
        }
    }

    pub fn num_channels(&self) -> usize {
        self.channels.len()
    }

    pub fn num_samples(&self) -> usize {
        self.num_samples
    }

    pub fn channel(&self, index: usize) -> &'a [f32] {
        self.channels[index]
    }

    pub fn channels(&self) -> &'a [&'a [f32]] {
        self.channels
    }

    pub fn to_vec(&self) -> Vec<Vec<f32>> {
        self.channels.iter().map(|ch| ch.to_vec()).collect()
    }
}

impl<'a> BufferViewMut<'a> {
    pub fn new(channels: &'a mut [&'a mut [f32]]) -> Self {
        let num_samples = if let Some(ch) = channels.first() {
            ch.len()
        } else {
            0
        };

        Self {
            channels,
            num_samples,
        }
    }

    pub fn num_channels(&self) -> usize {
        self.channels.len()
    }

    pub fn num_samples(&self) -> usize {
        self.num_samples
    }

    pub fn channel_mut(&mut self, index: usize) -> &mut [f32] {
        self.channels[index]
    }

    pub fn channels_mut(&'a mut self) -> &'a mut [&'a mut [f32]] {
        self.channels
    }

    pub fn to_vec(&self) -> Vec<Vec<f32>> {
        self.channels.iter().map(|ch| ch.to_vec()).collect()
    }
}
