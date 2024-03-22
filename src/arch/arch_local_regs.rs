pub const ARM_LOCAL_BASE: u32 = 0x4000_0000;

// ARM Local register for PI see <BCM2711 ARM Peripherals> 6.5.2
pub const ARM_CONTROL: u32 = ARM_LOCAL_BASE + 0x0;
pub const CORE_IRQ_CONTROL: u32 = ARM_LOCAL_BASE + 0xc;
pub const PMU_CONTROL_SET: u32 = ARM_LOCAL_BASE + 0x10;
pub const PMU_CONTROL_CLR: u32 = ARM_LOCAL_BASE + 0x14;
pub const PERI_IRQ_ROUTE0: u32 = ARM_LOCAL_BASE + 0x24;
pub const AXI_QUIET_TIME: u32 = ARM_LOCAL_BASE + 0x30;
pub const LOCAL_TIMER_CONTROL: u32 = ARM_LOCAL_BASE + 0x34;
pub const LOCAL_TIMER_IRQ: u32 = ARM_LOCAL_BASE + 0x38;

pub const TIMER_CNTRL0: u32 = ARM_LOCAL_BASE + 0x40;
pub const TIMER_CNTRL1: u32 = ARM_LOCAL_BASE + 0x44;
pub const TIMER_CNTRL2: u32 = ARM_LOCAL_BASE + 0x48;
pub const TIMER_CNTRL3: u32 = ARM_LOCAL_BASE + 0x4c;
// Secure Physical Timer Event for IRQ
pub const CNT_PS_IRQ: u32 = 1 << 0;
// Nonsecure Physical Timer Event for IRQ
pub const CNT_PNS_IRQ: u32 = 1 << 1;
// Hypervisor Physical Timer Event for IRQ
pub const CNT_HP_IRQ: u32 = 1 << 2;
// Virtual Timer Event for IRQ
pub const CNT_V_IRQ: u32 = 1 << 3;
// Secure Physical Timer Event for FIQ
pub const CNT_PS_IRQ_FIQ: u32 = 1 << 4;
// Nonsecure Physical Timer Event for FIQ
pub const CNT_PNS_IRQ_FIQ: u32 = 1 << 5;
// Hypervisor Physical Timer Event for FIQ
pub const CNT_HP_IRQ_FIQ: u32 = 1 << 6;
// Virtual Timer Event for FIQ
pub const CNT_V_IRQ_FIQ: u32 = 1 << 7;

pub const ARM_LOCAL_IRQ_SOURCE0: u32 = ARM_LOCAL_BASE + 0x60;
pub const ARM_LOCAL_IRQ_SOURCE1: u32 = ARM_LOCAL_BASE + 0x64;
pub const ARM_LOCAL_IRQ_SOURCE2: u32 = ARM_LOCAL_BASE + 0x68;
pub const ARM_LOCAL_IRQ_SOURCE3: u32 = ARM_LOCAL_BASE + 0x6c;
pub const MAILBOX_IRQ_SHIFT: u32 = 4;
pub const CORE_IRQ: u32 = 1 << 8;
pub const PMU_IRQ: u32 = 1 << 9;
pub const AXI_QUIET: u32 = 1 << 10;
pub const TIMER_IRQ: u32 = 1 << 11;
pub const AXI_IRQ: u32 = 1 << 30;

pub const ARM_LOCAL_FRQ_SOURCE0: u32 = ARM_LOCAL_BASE + 0x70;
pub const ARM_LOCAL_FRQ_SOURCE1: u32 = ARM_LOCAL_BASE + 0x74;
pub const ARM_LOCAL_FRQ_SOURCE2: u32 = ARM_LOCAL_BASE + 0x78;
pub const ARM_LOCAL_FRQ_SOURCE3: u32 = ARM_LOCAL_BASE + 0x7c;
