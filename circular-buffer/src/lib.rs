pub struct CircularBuffer<T>(Vec<T>);

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer(Vec::with_capacity(capacity))
    }

    pub fn write(&mut self, item: T) -> Result<(), Error> {
        if self.0.len() == self.0.capacity() {
            return Err(Error::FullBuffer)
        }

        Ok(self.0.push(item))
    }

    pub fn overwrite(&mut self, item: T) {
        if self.0.len() == self.0.capacity() {
            self.0.remove(0);
        }

        self.0.push(item);
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.0.len() == 0 {
            return Err(Error::EmptyBuffer)
        }

        Ok(self.0.remove(0))
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}
