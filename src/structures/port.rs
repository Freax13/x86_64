//! Traits for accessing I/O ports.

/// A helper trait that implements the read port operation.
///
/// On x86, I/O ports operate on either `u8` (via `inb`/`outb`), `u16` (via `inw`/`outw`),
/// or `u32` (via `inl`/`outl`). Therefore this trait is implemented for exactly these types.
pub trait PortRead: Sized {
    /// Reads a `Self` value from the given port.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the I/O port could have side effects that violate memory
    /// safety.
    unsafe fn read_from_port(port: u16) -> Self;

    /// Reads an slice of `Self` values from the given port.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the I/O port could have side effects that violate memory
    /// safety.
    unsafe fn read_slice_from_port(port: u16, value: &mut [Self]);
}

/// A helper trait that implements the write port operation.
///
/// On x86, I/O ports operate on either `u8` (via `inb`/`outb`), `u16` (via `inw`/`outw`),
/// or `u32` (via `inl`/`outl`). Therefore this trait is implemented for exactly these types.
pub trait PortWrite: Sized {
    /// Writes a `Self` value to the given port.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the I/O port could have side effects that violate memory
    /// safety.
    unsafe fn write_to_port(port: u16, value: Self);

    /// Writes a slice of `Self` values to the given port.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the I/O port could have side effects that violate memory
    /// safety.
    unsafe fn write_slice_to_port(port: u16, value: &[Self]);
}
