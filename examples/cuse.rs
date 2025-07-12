use fuser::Filesystem;
use log::info;

struct CuseDevice;

impl Filesystem for CuseDevice {
    fn cuse_init(
        &mut self,
        _req: &fuser::Request<'_>,
        reply: fuser::ReplyCuseInit,
    ) -> Result<(), libc::c_int> {
        let name = "fuser_cuse_example";
        let major = 51;
        let minor = 0;
        info!("Initializing CUSE at /dev/{}", name);
        let config = fuser::CuseConfig::new(name, major, minor);
        reply.reply(config);
        Ok(())
    }
}

fn main() {
    env_logger::init();
    let _guard = fuser::cuse(CuseDevice).unwrap_or_else(|e| {
        eprintln!("Failed to start CUSE device: {}. Try run this example as priviledged user", e);
        std::process::exit(1);
    });
    println!("Done");
}
