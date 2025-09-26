use env_logger::{Builder, Target};
use log::LevelFilter;
use penumbra::Device;
pub use penumbra::{find_mtk_port, get_mtk_port_connection};
use std::fs::File;
use tokio_serial::SerialPortInfo;

const KAERU: &[u8] = include_bytes!("./fogorow-kaeru.bin");
const DA: &[u8] = include_bytes!("./DA_fogorow.bin");

#[tokio::main]
async fn main() {
    let log_file = File::create("fogorow-unlock.log").expect("Failed to create log file");

    Builder::new()
        .filter_level(LevelFilter::Info)
        .write_style(env_logger::WriteStyle::Always)
        .target(Target::Pipe(Box::new(log_file)))
        .init();

    println!("Fogorow Unlock Tool");
    println!("============================================================");
    println!("Powered by Penumbra (https://github.com/shomykohai/penumbra)");
    println!("SPDX-License-Identifier: AGPL-3.0-or-later");
    println!();
    println!("Custom bootloader (kaeru) by R0rt1z2 ");
    println!("https://github.com/R0rt1z2/kaeru");
    println!("SPDX-License-Identifier: AGPL-3.0-or-later");
    println!("============================================================");

    println!("Searching for MTK port...");
    let mtk_port: SerialPortInfo;
    loop {
        let ports = find_mtk_port();
        if ports.len() > 0 {
            mtk_port = ports[0].clone();
            break;
        }
    }

    println!("Found MTK port: {}", mtk_port.port_name);
    let mut dev = Device::init(mtk_port, DA.to_vec())
        .await
        .expect("Failed to initialize device");

    println!("Device initialized!");
    println!("Unlocking device...");
    dev.set_seccfg_lock_state(penumbra::core::seccfg::LockFlag::Unlock)
        .await
        .expect("Failed to unlock seccfg");

    let mut progress = |_read: usize, _total: usize| {};
    println!("Device unlocked!");

    dev.write_partition("lk_a", KAERU, &mut progress)
        .await
        .expect("Failed to write lk_a partition");
    dev.write_partition("lk_b", KAERU, &mut progress)
        .await
        .expect("Failed to write lk_b partition");

    println!("Wrote Kaeru to lk_a and lk_b partitions");
    println!("All done!");
}
