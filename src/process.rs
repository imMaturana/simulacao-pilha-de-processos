use std::{
    thread,
    time::Duration
};
use indicatif::{ProgressBar, ProgressStyle};

pub struct Process {
    id: i32,
    mem_size: usize,
    exec_time: Duration
}

impl Process {
    pub fn new(mem_size: usize, exec_time: Duration) -> Self {
        Process {
            id: 0,
            mem_size,
            exec_time,
        }
    }

    pub fn set_pid(&self, id: i32) -> Self {
        Process {
            id,
            mem_size: self.mem_size,
            exec_time: self.exec_time,
        }
    }

    pub fn get_pid(&self) -> i32 {
        self.id
    }

    pub fn get_mem_size(&self) -> usize {
        self.mem_size
    }

    pub fn get_exec_time(&self) -> Duration {
        self.exec_time
    }

    pub fn run(&self) {
        let secs = self.exec_time.as_secs();
        let bar = ProgressBar::new(secs);

        bar.set_style(ProgressStyle::with_template("{bar:90.}").unwrap().progress_chars("##-"));

        println!("Rodando processo {}.", self.get_pid());

        for _ in 0..secs {
            thread::sleep(Duration::new(1, 0));
            bar.inc(1);
        }

        println!("Processo {} finalizado.", self.get_pid());
    }
}