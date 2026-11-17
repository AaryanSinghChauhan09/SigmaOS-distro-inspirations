//! SigmaOS-distro-inspirations — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_distro_inspirations_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-distro-inspirations: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-distro-inspirations: ok");
}
