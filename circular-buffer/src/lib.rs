pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    capacity: usize
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: Vec::with_capacity(capacity),
            capacity: capacity
        }
    }

    pub fn write(&mut self, item: T) -> Result<(), Error> {
        if self.buffer.len() == self.capacity {
            return Err(Error::FullBuffer)
        }

        Ok(self.buffer.push(item))
    }

    pub fn overwrite(&mut self, item: T) {
        if self.buffer.len() == self.capacity {
            self.buffer.remove(0);
        }

        self.buffer.push(item);
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer.len() == 0 {
            return Err(Error::EmptyBuffer)
        }

        Ok(self.buffer.remove(0))
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}
