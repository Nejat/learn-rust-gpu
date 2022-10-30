#[cfg(not(target_arch = "wasm32"))]
use learn_rust_gpu::run;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    pollster::block_on(run());
}
