pub mod sha;
pub mod hasher;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Lang {
    C,
    Asm
}

#[derive(PartialEq, Debug)]
pub enum NoThreads {
    One(u8),
    Two(u8),
    Four(u8),
    Eight(u8),
    Sixteen(u8),
    ThirtyTwo(u8),
    SixtyFour(u8)
}

#[derive(PartialEq, Debug)]
pub enum HasherError {
    BadInputLocation,
    BadOutputLocation,
    AsmLibLoadingError,
    CLibLoadingError,
}
