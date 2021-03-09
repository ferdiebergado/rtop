use byte_unit::Byte;
use sysinfo::{ProcessExt, SystemExt};
use tokio::time;

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

#[tokio::main]
async fn main() {
    let interval = time::interval(time::Duration::from_secs(2));
    tokio::pin!(interval);

    loop {
        interval.as_mut().tick().await;
        monitor().await;
    }
}

async fn monitor() {
    let mut system = sysinfo::System::new_all();

    // First we update all information of our system struct.
    system.refresh_all();
    
    print!("{}[2J", 27 as char);

    // And finally the RAM and SWAP information:
    println!("total memory: {}", humanize(system.get_total_memory()));
    println!("used memory : {}", humanize(system.get_used_memory()));
    println!("total swap  : {}", humanize(system.get_total_swap()));
    println!("used swap   : {}", humanize(system.get_used_swap()));

    println!("{: <10} {: <10} {: <10} {:}", "PID", "CPU%", "MEM", "NAME");
    let mut processes: Vec<Process> = Vec::new();

    for (pid, proc_) in system.get_processes() {
            if proc_.name().len() > 0 {
                let process: Process =
                    Process::new(*pid, proc_.cpu_usage(), proc_.memory(), proc_.name().to_string());
                processes.push(process);
            }
    }

    processes.sort_by(|p1, p2| p1.cpu.partial_cmp(&p2.cpu).unwrap());
    processes.reverse();

    let mut i: i8 = 0;
    for p in &processes {
        i+=1;
        if i == 10 {
            break;
        }
        println!("{: <10} {: <10} {: <10} {: <10}", p.pid, p.cpu, humanize(p.mem), p.name);
    };

    time::sleep(time::Duration::from_secs(1)).await
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
