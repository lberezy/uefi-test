use crate::protocol;
use crate::runtime::RuntimeServices;
use crate::types::Handle;

#[derive(Debug)]
#[repr(C)]
pub struct Header {
    ///A 64-bit signature that identifies the type of table that follows.
    /// Unique signatures have been generated for the EFI System Table, the
    /// EFI Boot Services Table, and the EFI Runtime Services Table.
    pub signature: u64,
    ///The revision of the EFI Specification to which this table conforms. The
    /// upper 16 bits of this field contain the major revision value, and
    /// the lower 16 bits contain the minor revision value. The minor
    /// revision values are binary coded decimals and are limited to the
    /// range of 00..99.When printed or displayed UEFI spec revision is referred
    /// as (Major revision).(Minor revision upper decimal).(Minor revision
    /// lower decimal) or (Major revision).(Minor revision upper decimal) in
    /// case Minor revision lower decimal is set to 0. For example:A
    /// specification with the revision value ((2<<16) | (30)) would be referred
    /// as 2.3;A specification with the revision value ((2<<16) | (31))
    /// would be referred as 2.3.1
    pub revision: u32,
    /// The size, in bytes, of the entire table including the EFI_TABLE_HEADER.
    pub size: u32,
    ///The 32-bit CRC for the entire table. This value is computed by setting
    /// this field to 0, and computing the 32-bit CRC for HeaderSize bytes.
    pub crc32: u32,
    /// Reserved field that must be set to 0.
    pub reserved: u32,
}

const EFI_SYSTEM_TABLE_SIGNATURE: u64 = 0x5453595320494249;

#[derive(Debug)]
#[repr(C)]
pub struct SystemTable {
    /// The table header for the EFI System Table. This header contains the
    /// EFI_SYSTEM_TABLE_SIGNATURE and EFI_SYSTEM_TABLE_REVISION values along
    /// with the size of the EFI_SYSTEM_TABLE structure and a 32-bit CRC to
    /// verify that the contents of the EFI System Table are valid.
    header: Header,
    /// A pointer to a null terminated string that identifies the vendor that
    /// produces the system firmware for the platform.
    vendor: *const u16,
    /// A firmware vendor specific value that identifies the revision of the
    /// system firmware for the platform.
    revision: u32,
    /// The handle for the active console input device. This handle must support
    /// EFI_SIMPLE_TEXT_INPUT_PROTOCOL and EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL. If
    /// there is no active console, these protocols must still be present.
    console_in_handle: Handle,
    /// A pointer to the EFI_SIMPLE_TEXT_INPUT_PROTOCOL interface that is
    /// associated with ConsoleInHandle.
    console_in: protocol::SimpleTextOutput,
    /// The handle for the active console output device. This handle must
    /// support the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL. If there is no active
    /// console, this protocol must still be present.
    console_out_handle: Handle,
    /// A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is
    /// associated with ConsoleOutHandle.
    console_out: protocol::SimpleTextOutput,
    /// The handle for the active standard error console device. This handle
    /// must support the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL. If there is no active
    /// console, this protocol must still be present.
    console_error_handle: Handle,
    /// A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is
    /// associated with StandardErrorHandle.
    console_error: protocol::SimpleTextOutput,
    /// A pointer to the EFI Runtime Services Table. See Section4.5.
    runtime_services: *const RuntimeServices,
    /// A pointer to the EFI Boot Services Table. See Section 4.4.
    boot_services: *const usize,
    /// The number of system configuration tables in the buffer
    /// `configuration_table`.
    number_table_entries: usize,
    /// A pointer to the system configuration tables. The number of entries in
    /// the table is `number_table_entries`.
    configuration_table: *const usize,
}

impl SystemTable {}

#[derive(Debug)]
#[repr(C)]
pub struct EfiGuid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 4],
}
