pub struct Mem {
    hex_sprites: [[u8; 5]; 10],
    stack: [u16; 16],
    ram: [u8; 4096 - 512],
}
