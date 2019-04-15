/// You can read and write from a raw pointer (*mut T) using read_volatile() and write_volatile()
/// example:
/// 
/// const A: *mut u32 = 0x12 as *mut u32
/// const B: *mut u32 = 0x34 as *mut u32
/// B.write_volatile(A.read_volatile());

/// Perhiperal base address:
const MMIO_BASE: u32 = 0x3F00_0000;
const FSEL0: u32 = MMIO_BASE + 0x0020_0000;
const FSEL1: u32 = MMIO_BASE + 0x0020_0004;
const FSEL2: u32 = MMIO_BASE + 0x0020_0008;
const FSEL3: u32 = MMIO_BASE + 0x0020_000C;
const FSEL4: u32 = MMIO_BASE + 0x0020_0010;
const FSEL5: u32 = MMIO_BASE + 0x0020_0014;
const SET0: u32 = MMIO_BASE + 0x0020_001C;
const SET1: u32 = MMIO_BASE + 0x0020_0020;
const CLR0: u32 = MMIO_BASE + 0x0020_0028;
const LEV0: u32 = MMIO_BASE + 0x0020_0034;
const LEV1: u32 = MMIO_BASE + 0x0020_0038;
const EDS0: u32 = MMIO_BASE + 0x0020_0040;
const EDS1: u32 = MMIO_BASE + 0x0020_0044;
const HEN0: u32 = MMIO_BASE + 0x0020_0064;
const HEN1: u32 = MMIO_BASE + 0x0020_0068;
const PUD: u32 = MMIO_BASE + 0x0020_0094;
const PUDCLK0: u32 = MMIO_BASE + 0x0020_0098;
const PUDCLK1: u32 = MMIO_BASE + 0x0020_009C;

///					   				  9	  8	  7   6   5   4   3   2   1   0
const GPIFSEL1_REGISTER_0: u32 = 0b00_000_000_000_000_000_000_000_000_000_000
///					   				  19  18  17  16  15  14  13  12  11  10
const GPIFSEL1_REGISTER_1: u32 = 0b00_000_000_000_000_000_000_000_000_000_000


enum GPIOPINS {
	P0,	P1,	P2, P3,	P4, P5,	P6,	P7, P8,	P9,
	P10, P11, P12, P13,	P14, P15, P16, P17, P18, P19,
	P20, P21, P22, P23,	P24, P25, P26, P27, P28, P29,
}


enum GPIOFunction {
	Input,
	Output
}

enum GPIO {
	FSelect(usize, GPIOFunction), // GP Function select 0-5, with a 32-bit value, R/W
}