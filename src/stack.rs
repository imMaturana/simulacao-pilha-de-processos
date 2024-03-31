use crate::process::Process;

pub struct Stack {
    processes: Vec<Process>,
    next_pid: i32
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            processes: Vec::new(),
            next_pid: 1
        }
    }

    pub fn add(&mut self, process: Process) {
        let process = process.set_pid(self.next_pid);
        self.processes.push(process);
        self.next_pid += 1;
    }

    pub fn remove_pid(&mut self, pid: i32) -> Process {
        let idx = self.processes.iter()
            .position(|process| process.get_pid() == pid)
            .unwrap();

        self.processes.remove(idx)
    }

    pub fn run_processes(&mut self) -> Vec<Process> {
        let len = self.processes.len();
        let mut removed_procs: Vec<Process> = Vec::new();

        for i in (0..len).rev() {
            let p = &self.processes[i];
            p.run();
            removed_procs.push(self.remove_pid(p.get_pid()));
        }

        removed_procs
    }
}