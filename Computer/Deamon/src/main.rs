
extern crate deamon;
extern crate nvapi_sys;

use deamon::*;

fn main() {
    let all_gpus = get_gpus();

    loop {
        print!("{}", "\n".repeat(5));

        for (i, gpu) in all_gpus.iter().enumerate() {
            let gpu_temp = get_gpu_temp(&gpu);
            println!("GPU {} temperature: {} degrees", i, gpu_temp);
        }

        let cpu_usage = get_cpu_usage() * 100.0;
        println!("CPU usage: {}%", cpu_usage as u32);
        

        let ram_usage = get_ram_usage();
        println!("RAM usage: {} MB", ram_usage);

        let disk_usage = get_disk_usage();
        println!("Disk usage: {} MB", disk_usage);

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}