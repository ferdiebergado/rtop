use byte_unit::Byte;
use std::{thread, time::Duration};
use sysinfo::{ProcessExt, SystemExt};

pub fn main() {
    const MAX_PROCESSES: u8 = 10;
    const SLEEP_SECS: u64 = 2;

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

    let mut system = sysinfo::System::new_all();

    // First we update all information of our system struct.
    system.refresh_all();

    const NEWLINE: char = 10 as char;
    const TAB: char = 9 as char;

    let mut sysinfo = format!(
        "System name: {}{}{}",
        TAB,
        extract_data(system.get_name()),
        NEWLINE
    );
    sysinfo = format!(
        "{}Kernel version: {}{}",
        sysinfo,
        extract_data(system.get_kernel_version()),
        NEWLINE
    );
    sysinfo = format!(
        "{}OS version:   {}{}{}",
        sysinfo,
        TAB,
        extract_data(system.get_os_version()),
        NEWLINE
    );
    sysinfo = format!(
        "{}Hostname:     {}{}{}",
        sysinfo,
        TAB,
        extract_data(system.get_host_name()),
        NEWLINE
    );

    let total_ram = humanize(system.get_total_memory());
    let total_swap = humanize(system.get_total_swap());

    loop {
        reset_cursor();
        println!("{}", sysinfo);
        // And finally the RAM and SWAP information:
        println!(
            "RAM: {}{}/{}",
            TAB,
            humanize(system.get_used_memory()),
            total_ram
        );
        println!(
            "Swap: {}{}/{}{}",
            TAB,
            humanize(system.get_used_swap()),
            total_swap,
            NEWLINE
        );
        println!(
            "{: <10} {: <13} {: <10} {:}",
            "PID", "CPU %", "MEM", "PROCESS"
        );

        let mut processes: Vec<Process> = vec![];

        for (pid, proc_) in system.get_processes() {
            if proc_.name().len() > 0 {
                let mut cmdstr: String = String::new();
                let space = " ";
                for c in proc_.cmd() {
                    cmdstr += space;
                    cmdstr += c;
                }
                let process: Process =
                    Process::new(*pid, proc_.cpu_usage(), proc_.memory(), cmdstr);
                processes.push(process);
            }
        }

        processes.sort_by(|p1, p2| p1.cpu.partial_cmp(&p2.cpu).unwrap());
        processes.reverse();

        let mut i: u8 = 1;

        for p in &processes {
            println!(
                "{: <10} {:.2}{}{} {: <10} {: <10}",
                p.pid,
                p.cpu,
                TAB,
                TAB,
                humanize(p.mem),
                p.name
            );
            i += 1;
            if i == MAX_PROCESSES {
                break;
            }
        }
        reset_cursor();

        thread::sleep(Duration::from_secs(SLEEP_SECS));
        system.refresh_all();
    }
}

// Convert bytes to human-readable sizes
fn humanize(size: u64) -> String {
    const MB: u64 = 1_024;
    const KB: &str = " kb";

    if size < MB {
        size.to_string() + KB
    } else {
        let byte = Byte::from_str(size.to_string() + KB).unwrap();
        let adjusted = byte.get_appropriate_unit(false);
        adjusted.to_string()
    }
}

// Get data from the system struct
fn extract_data(data: Option<String>) -> String {
    match data {
        Some(data) => data,
        None => "N/A".to_string(),
    }
}

// Position terminal cursor at row 1 column 1
fn reset_cursor() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
