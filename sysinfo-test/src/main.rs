use std::time::{Duration, Instant};
use sysinfo::{ProcessExt, SystemExt};

fn main() {
    build_some_threads();

    let mut last_loop = Instant::now();
    loop {
        if last_loop.elapsed() > Duration::from_secs(1) {
            last_loop = Instant::now();
        } else {
            std::thread::sleep(Duration::from_micros(100));
            continue;
        }
        println!("----------");
        let mut system = sysinfo::System::new_all();

        // First we update all information of our system struct.
        system.refresh_all();

        // Now let's print every process' id and name:
        // for (pid, proc_) in system.get_processes() {
        //     println!("{}:{} => status: {:?}", pid, proc_.name(), proc_.status());
        // }
        //
        // // Then let's print the temperature of the different components:
        // for component in system.get_components() {
        //     println!("{:?}", component);
        // }
        //
        // // And then all disks' information:
        // for disk in system.get_disks() {
        //     println!("{:?}", disk);
        // }

        // And finally the RAM and SWAP information:
        println!("total memory: {} KB", system.get_total_memory());
        println!("used memory : {} KB", system.get_used_memory());
        println!("total swap  : {} KB", system.get_total_swap());
        println!("used swap   : {} KB", system.get_used_swap());

        // Display system information:
        println!("System name:             {:?}", system.get_name());
        println!("System kernel version:   {:?}", system.get_kernel_version());
        println!("System OS version:       {:?}", system.get_os_version());
        println!("System host name:        {:?}", system.get_host_name());

        let process_id = std::process::id() as i32;
        let cpu_usage = system.get_process(process_id).unwrap().cpu_usage();
        println!("thread {:?},cpu usage {:?}", process_id, cpu_usage);
    }
}

fn build_some_threads() {
    for _ in 0..5 {
        std::thread::spawn(|| loop {
            let _ = (0..9_000).into_iter().sum::<i128>();
        });
    }
}
