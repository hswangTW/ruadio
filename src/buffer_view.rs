//! The buffer view types are used to view and manipulate multi-channel audio data without owning
//! the data. Note that the buffer view types assume that all the channels have the same length.
//! If this is not the case, the behavior is undefined and may lead to panics.

/// A non-owning view into multi-channel audio data
///
/// The `'inner` lifetime is the actual lifetime of the audio data, and the `'outer` lifetime is
/// the lifetime of the buffer view.
#[derive(Debug)]
pub struct BufferView<'outer, 'inner> {
    channels: &'outer [&'inner [f32]],
    num_samples: usize,
}

/// A mutable view into multi-channel audio data
///
/// The `'inner` lifetime is the actual lifetime of the audio data, and the `'outer` lifetime is
/// the lifetime of the buffer view.
#[derive(Debug)]
pub struct BufferViewMut<'outer, 'inner> {
    channels: &'outer mut [&'inner mut [f32]],
    num_samples: usize,
}

impl<'outer, 'inner> BufferView<'outer, 'inner> {
    pub fn new(channels: &'outer [&'inner [f32]]) -> Self {
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

    pub fn channel(&self, index: usize) -> &'inner [f32] {
        self.channels[index]
    }

    pub fn channels(&self) -> &'outer [&'inner [f32]] {
        self.channels
    }

    pub fn to_vec(&self) -> Vec<Vec<f32>> {
        self.channels.iter().map(|ch| ch.to_vec()).collect()
    }
}

impl<'outer, 'inner> BufferViewMut<'outer, 'inner> {
    pub fn new(channels: &'outer mut [&'inner mut [f32]]) -> Self {
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

    pub fn channels_mut(&'outer mut self) -> &'outer mut [&'inner mut [f32]] {
        self.channels
    }

    pub fn to_vec(&self) -> Vec<Vec<f32>> {
        self.channels.iter().map(|ch| ch.to_vec()).collect()
    }
}
