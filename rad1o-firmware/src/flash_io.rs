use io::{Read, Write, Seek, SeekFrom, Result};
use flash::Flash;

pub struct FlashIO {
    position: u32,
    flash: Flash,
}

impl FlashIO {
    pub fn new(flash: Flash) -> Self {
        FlashIO {
            position: 0,
            flash,
        }
    }
}

impl Read for FlashIO {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.flash.read(self.position, buf);
        self.position += buf.len() as u32;
        Ok(buf.len())
    }
}

impl Write for FlashIO {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // TODO
        Ok(0)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Seek for FlashIO {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        match pos {
            SeekFrom::Start(offset) =>
                self.position = offset as u32,
            SeekFrom::End(inset) =>
                self.position = 512 * 1024 - inset as u32,
            SeekFrom::Current(offset) if offset < 0 =>
                self.position -= (-offset) as u32,
            SeekFrom::Current(offset) =>
                self.position += offset as u32,
        }
        Ok(self.position.into())
    }
}
