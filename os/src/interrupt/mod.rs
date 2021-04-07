//! 中断模块
//!
//!

mod handler;
mod context;
mod timer;

/// 初始化中断相关的⼦模块
///
/// - [`handler::init`]
/// - [`timer::init`]
pub fn init() {
	handler::init();
	timer::init();
	println!("Mod interrupt is initialized by Lingfeng Zhu.");
}
