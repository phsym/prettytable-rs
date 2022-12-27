//! This modules contains traits and implementations to work within Evcxr

use super::utils::StringWriter;
use super::AsTableSlice;
use std::io::Write;

/// Evcxr specific output trait
pub trait EvcxrDisplay {
    /// Print self in one or multiple Evcxr compatile types.
    fn evcxr_display(&self);
}

impl<T> EvcxrDisplay for T
where
    T: AsTableSlice,
{
    fn evcxr_display(&self) {
        let mut writer = StringWriter::new();
        // Plain Text
        let _ = writer.write_all(b"EVCXR_BEGIN_CONTENT text/plain\n");
        let _ = self.as_slice().print(&mut writer);
        let _ = writer.write_all(b"\nEVCXR_END_CONTENT\n");

        // Html
        let _ = writer.write_all(b"EVCXR_BEGIN_CONTENT text/html\n");
        let _ = self.as_slice().print_html(&mut writer);
        let _ = writer.write_all(b"\nEVCXR_END_CONTENT\n");
        println!("{}", writer.as_string());
    }
}
