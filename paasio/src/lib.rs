use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    reads: usize,
    bytes: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            reads: 0,
            bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.wrapped.read(buf)?;

        self.reads += 1;
        self.bytes += bytes;

        Ok(bytes)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    writes: usize,
    bytes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            writes: 0,
            bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = self.wrapped.write(buf)?;

        self.writes += 1;
        self.bytes += bytes;

        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}