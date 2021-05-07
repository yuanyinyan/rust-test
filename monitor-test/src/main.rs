use std::time::Duration;
use std::time::Instant;

use perf_monitor::cpu::processor_numbers;
use perf_monitor::cpu::ProcessStat;
use perf_monitor::cpu::ThreadStat;
// use perf_monitor::fd::fd_count_cur;
// use perf_monitor::io::get_process_io_stats;
// use perf_monitor::mem::get_process_memory_info;
use std::process::exit;
use sysinfo::{ProcessExt, SystemExt};

fn main() {
    build_some_threads();

    // cpu
    let _core_num = processor_numbers().unwrap();
    let mut stat_p = ProcessStat::cur().unwrap();
    let mut stat_t = ThreadStat::cur().unwrap();

    let mut system = sysinfo::System::new();

    let mut count = 0;
    let mut last_loop = Instant::now();
    loop {
        if last_loop.elapsed() > Duration::from_secs(1) {
            last_loop = Instant::now();
            count = count + 1;
            if count > 100 {
                exit(0);
            }
        } else {
            std::thread::sleep(Duration::from_micros(100));
            continue;
        }
        println!("----------");

        // cpu
        let _ = (0..1_000).into_iter().sum::<i128>();

        let usage_p = stat_p.cpu().unwrap() * 100f64;
        let _usage_t = stat_t.cpu().unwrap() * 100f64;

        let process_id = std::process::id() as i32;
        system.refresh_process(process_id);
        // system.refresh_cpu();

        let cpu_usage = system.get_process(process_id).unwrap().cpu_usage();

        println!(
            "cpu usage: [perf_monitor, sysinfo] = [{:.2}%, {:.2}%]",
            usage_p, cpu_usage
        );

        // println!(
        //     "[CPU] core Number: {}, process usage: {:.2}%, current thread usage: {:.2}%",
        //     core_num, usage_p, usage_t,
        // );
        //
        // // mem
        // let mem_info = get_process_memory_info().unwrap();
        //
        // println!(
        //     "[Memory] memory used: {} bytes, virtural memory used: {} bytes ",
        //     mem_info.resident_set_size, mem_info.virtual_memory_size
        // );
        //
        // // fd
        // let fd_num = fd_count_cur().unwrap();
        //
        // println!("[FD] fd number: {}", fd_num);
        //
        // // io
        // let io_stat = get_process_io_stats().unwrap();
        //
        // println!(
        //     "[IO] io-in: {} bytes, io-out: {} bytes",
        //     io_stat.read_bytes, io_stat.write_bytes
        // );
    }
}

fn build_some_threads() {
    for _ in 0..5 {
        std::thread::spawn(|| loop {
            let _ = (0..9_000).into_iter().sum::<i128>();
        });
    }
}
