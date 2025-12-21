use bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        const FIRST_FRAME = 1 << 0;
        const LAST_FRAME = 1 << 1;
        const CONTROL_MESSAGE = 1 << 2;
        const ENCRYPTED = 1 << 3;
    }
}