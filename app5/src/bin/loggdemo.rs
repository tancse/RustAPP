use log::*;

fn main() {
    env_logger::init();

    info!("APP Started");
    debug!("APP Debug Message here");
    warn!("APP warns Here");
    error!("APP Error Message Here");
}