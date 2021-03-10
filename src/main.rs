use byte_unit::Byte;
use std::{thread, time::Duration};
use sysinfo::{ProcessExt, SystemExt};

#[derive(PartialEq, PartialOrd)]
struct Process {
    pid: i32,
    cpu: f32,
    mem: u64,
    name: String,
}

impl Process {
    pub fn new(pid: i32, cpu: f32, mem: u64, name: String) -> Self {
        Self {
            pid,
            cpu,
            mem,
            name,
        }
    }
}

fn main() {
    let mut system = sysinfo::System::new_all();
    // system.refresh_all();
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    // // Display system information:
    // println!("System name:             {:?}", system.get_name());
    // println!("System kernel version:   {:?}", system.get_kernel_version());
    // println!("System OS version:       {:?}", system.get_os_version());
    // println!("System host name:        {:?}", system.get_host_name());
    loop {
        // First we update all information of our system struct.
        system.refresh_all();

        // Position terminal cursor at row 1 column 1
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        // And finally the RAM and SWAP information:
        println!("total memory: {}", humanize(system.get_total_memory()));
        println!("used memory : {}", humanize(system.get_used_memory()));
        println!("total swap  : {}", humanize(system.get_total_swap()));
        println!("used swap   : {}", humanize(system.get_used_swap()));
        println!();
        println!("{: <10} {: <10} {: <10} {:}", "PID", "CPU%", "MEM", "NAME");

        let mut processes: Vec<Process> = vec![];

        for (pid, proc_) in system.get_processes() {
            if proc_.name().len() > 0 {
                let process: Process = Process::new(
                    *pid,
                    proc_.cpu_usage(),
                    proc_.memory(),
                    proc_.name().to_string(),
                );
                processes.push(process);
            }
        }

        processes.sort_by(|p1, p2| p1.cpu.partial_cmp(&p2.cpu).unwrap());
        processes.reverse();

        let mut i: u8 = 0;
        const MAX_PROCESSES: u8 = 25;

        for p in &processes {
            i += 1;
            if i == MAX_PROCESSES {
                break;
            }
            println!(
                "{: <10} {:.2} {: <10} {: <10}",
                p.pid,
                p.cpu,
                humanize(p.mem),
                p.name
            );
        }
        thread::sleep(Duration::from_secs(2));
    }
}

fn humanize(size: u64) -> String {
    if size < 1_024 {
        size.to_string() + "KB"
    } else {
        let byte = Byte::from_str(size.to_string() + "KB").unwrap();
        let adjusted = byte.get_appropriate_unit(false);
        adjusted.to_string()
    }
}
