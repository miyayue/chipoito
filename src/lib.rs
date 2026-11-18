mod display;
mod keyboard;
mod memory;
mod registers;

pub struct Chipoito {
    pc: u16,
    sp: u8,
    regs: registers::Regs,
    mem: memory::Mem,
    keyboard: (),
    display: (),
}
