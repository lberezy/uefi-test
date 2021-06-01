use crate::table::Header;

pub const EFI_RUNTIME_SERVICES_SIGNATURE: u64 = 0x56524553544e5552;

#[derive(Debug)]
#[repr(C)]
pub struct RuntimeServices {
    /// The table header for the EFI Runtime Services Table. This header
    /// contains the EFI_RUNTIME_SERVICES_SIGNATURE and
    /// EFI_RUNTIME_SERVICES_REVISION values along with the size
    /// of the EFI_RUNTIME_SERVICES structure and a 32-bit CRC to verify that
    /// the contents of the EFI Runtime Services Table are valid.
    header: Header,

    /// Time Services
    time: TimeServices,
    virtual_memory: VirtualMemoryServices,
    variable: VariableServices,
    misc: MiscellaneousServices,
    uefi2: Uefi2Services,
}

#[derive(Debug)]
#[repr(C)]
struct TimeServices {
    /// Returns the current time and date, and the time-keeping capabilities of
    /// the platform.
    get_time: *const usize,
    /// Sets the current local time and date information.
    set_time: *const usize,
    /// Returns the current wakeup alarm clock setting.
    get_wakeup_time: *const usize,
    /// Sets the system wakeup alarm clock time.
    set_wakeup_time: *const usize,
}

#[derive(Debug)]
#[repr(C)]
struct VirtualMemoryServices {
    /// Used by a UEFI OS loader to convert from physical addressing to virtual
    /// addressing.
    set_virtual_address_map: *const usize,
    /// Used by EFI components to convert internal pointers when switching to
    /// virtual addressing.
    convert_pointer: *const usize,
}

#[derive(Debug)]
#[repr(C)]
struct VariableServices {
    /// Returns the value of a variable.
    get_variable: *const usize,
    /// Enumerates the current variable names.
    get_next_variable_name: *const usize,
    /// Sets the value of a variable.
    set_variable: *const usize,
}

#[derive(Debug)]
#[repr(C)]
//// Uefi 2.0 only services
struct MiscellaneousServices {
    //// Returns the next high 32 bits of the platform’s monotonic counter.
    get_next_high_monotonic_count: *const usize,
    //// Resets the entire platform.
    reset_system: *const usize,
}

#[derive(Debug)]
#[repr(C)]
struct Uefi2Services {
    capsule: CapsuleServices,
    misc: MiscUefi2Services,
}

#[derive(Debug)]
#[repr(C)]
struct CapsuleServices {
    //// Passes capsules to the firmware with both virtual and physical mapping.
    update_capsule: *const usize,
    /// Returns if the capsule can be supported via `UpdateCapsule`.
    query_capsule_capabilities: *const usize,
}

#[derive(Debug)]
#[repr(C)]
struct MiscUefi2Services {
    /// Returns information about the EFI variable store.
    query_variable_info: *const usize,
}
