use anyhow::Result;
use winapi::um::debugapi;

/// Returns true if the engine is being traced. Currently uses the Windows API's
/// IsDebuggerPresent method.
pub fn check_trace() -> Result<bool> {
	// SAFETY: todo
	let present = unsafe { debugapi::IsDebuggerPresent() };
	Ok(present != 0)
}
