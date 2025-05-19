#[macro_use]
extern crate log;

//     The default is error, to see the other errors, sent the enviroment variable to 
// to warning, info, or debug:
// $ export RUST_LOG=debug

fn main() {
    env_logger::init();
    error!("Error message");
    warn!("Warning message");
    info!("Information message");
    debug!("Debugging message");
}
