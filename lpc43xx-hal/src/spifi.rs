//! SPI Flash Interface

use core::mem::transmute;
use vcell::VolatileCell;
use target::SPIFI;
use target::spifi;
use target::spifi::cmd;

pub struct FlashInterface {
    spifi: SPIFI,
}

impl FlashInterface {
    pub fn new(spifi: SPIFI) -> Self {
        FlashInterface { spifi }
    }

    fn wait(&self) {
        while self.spifi.stat.read()
            .cmd().bit_is_set() {}
    }

    pub fn set_addr(&self, addr: u32) {
        self.spifi.addr.write(|w| unsafe {
            w.address().bits(addr)
        })
    }

    pub fn command(
        &self,
        opcode: u8, data: &mut [u8], dout: cmd::DOUTW, fieldform: cmd::FIELDFORMW, intlen: u8, frameform: cmd::FRAMEFORMW
    ) {
        let datalen = data.len() as u16;
        self.spifi.cmd.write(
            |w| unsafe {
                w
                    .datalen().bits(datalen)
                    .dout().variant(dout)
                    .fieldform().variant(fieldform)
                    .intlen().bits(intlen)
                    .frameform().variant(frameform)
                    .opcode().bits(opcode)
            }
        );
        let data_byte = unsafe {
            transmute::<&spifi::DATA, &VolatileCell<u8>>(&self.spifi.data)
        };
        for d in data.iter_mut() {
            *d = data_byte.get();
        }

        self.wait();
    }
}
