use alloc::vec::Vec;

/// Parses &[&str] into Vec<f32>.
pub fn parse_args(args: &[&str]) -> Result<Vec<f32>, core::num::ParseFloatError> {
    args.iter().map(|arg| arg.parse()).collect()
}
