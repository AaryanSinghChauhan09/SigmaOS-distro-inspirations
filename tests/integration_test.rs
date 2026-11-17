//! Integration tests for SigmaOS-distro-inspirations.

use sigmaos_distro_inspirations_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
