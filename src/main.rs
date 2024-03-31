mod process;
mod stack;

use std::{
    io::{self, Write},
    time::Duration
};
use stack::Stack;
use process::Process;

fn main() {
    let mut stack = Stack::new();

    loop {
        let op = read_input("Deseja adicionar um processo? [S/N] ")
            .trim().to_uppercase();

        if op.as_str() == "S" {
            let mem_size: usize = read_input("Digite o tamanho da memória: ").trim().parse().unwrap();
            let secs: u64 = read_input("Digite o tempo de execução: ").trim().parse().unwrap();
            let exec_time: Duration = Duration::new(secs, 0);

            if secs <= 30 || secs >= 90 {
                println!("Tempo de execução deve estar entre 30 a 90 segundos. Tente novamente.");
                continue
            }

            let process = Process::new(mem_size, exec_time);
            stack.add(process);
        } else {
            break
        }
    }

    println!("\nRodando processos..");
    let removed_procs = stack.run_processes();

    description(removed_procs);
}

fn description(removed_procs: Vec<Process>) {
    let mut total_mem_size: usize = 0;
    let mut total_exec_time = Duration::new(0, 0);

    println!("\nResumo dos processos:");
    for p in removed_procs {
        println!("Processo PID - {}.", p.get_pid());

        total_mem_size += p.get_mem_size();
        total_exec_time += p.get_exec_time();
    }

    println!("---------");
    println!("Consumo total de memória: {}", total_mem_size);
    println!("Tempo total de execução: {}s", total_exec_time.as_secs());
} 

fn read_input(msg: &str) -> String {
    let mut input = String::new();

    print!("{}", msg);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler entrada.");

    input
}
