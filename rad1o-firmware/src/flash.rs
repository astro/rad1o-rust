use target::spifi::cmd::{DOUTW, FIELDFORMW, FRAMEFORMW};
use hal::spifi::FlashInterface;
use flash_io::FlashIO;

const OP_READ_S2: u8 = 0x35;
const OP_QREAD: u8 = 0xEB;
const FLASH_OFFSET: u32 = 512 * 1024;

pub struct Flash {
    pub iface: FlashInterface,
}

impl Flash {
    pub fn new(iface: FlashInterface) -> Self {
        Flash { iface }
    }

    pub fn io(self) -> FlashIO {
        FlashIO::new(self)
    }

    pub fn read(&self, addr: u32, buf: &mut [u8]) {
        self.iface.set_addr(FLASH_OFFSET + addr);
        self.iface.command(
            OP_QREAD, buf, DOUTW::INPUT_FROM_SERIAL_FL,
            FIELDFORMW::SERIAL_OPCODE, 3, FRAMEFORMW::OPCODE_THREE_BYTES
        );
    }

    pub fn status2(&self) -> u8 {
        let mut buf = [0u8];
        self.iface.command(
            OP_READ_S2, &mut buf, DOUTW::INPUT_FROM_SERIAL_FL,
            FIELDFORMW::SERIAL_OPCODE, 0, FRAMEFORMW::OPCODE
        );
        buf[0]
    }
}
