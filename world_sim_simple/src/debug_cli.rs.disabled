use std::io::{self, Write};
use std::sync::mpsc::Sender;
use std::thread;
use colored::*;
use crate::debug::{DebugCommand, DebugLevel};

pub struct DebugCLI {
    command_tx: Sender<DebugCommand>,
}

impl DebugCLI {
    pub fn new(command_tx: Sender<DebugCommand>) -> Self {
        Self { command_tx }
    }

    pub fn start(self) {
        thread::spawn(move || {
            self.run_cli();
        });
    }

    fn run_cli(self) {
        println!("{}", "\n=== Debug CLI Started ===".cyan().bold());
        self.print_help();

        loop {
            print!("{} ", ">".green().bold());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            match self.parse_command(input) {
                Some(cmd) => {
                    if let Err(e) = self.command_tx.send(cmd) {
                        eprintln!("{}", format!("Failed to send command: {}", e).red());
                    }
                }
                None => {
                    if input == "help" || input == "h" {
                        self.print_help();
                    } else if input == "quit" || input == "q" {
                        println!("{}", "Exiting debug CLI...".yellow());
                        break;
                    } else {
                        println!("{}", format!("Unknown command: {}", input).red());
                    }
                }
            }
        }
    }

    fn parse_command(&self, input: &str) -> Option<DebugCommand> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "verbosity" | "v" => {
                if parts.len() > 1 {
                    match parts[1] {
                        "error" | "0" => Some(DebugCommand::SetVerbosity(DebugLevel::Error)),
                        "warn" | "1" => Some(DebugCommand::SetVerbosity(DebugLevel::Warn)),
                        "info" | "2" => Some(DebugCommand::SetVerbosity(DebugLevel::Info)),
                        "debug" | "3" => Some(DebugCommand::SetVerbosity(DebugLevel::Debug)),
                        "trace" | "4" => Some(DebugCommand::SetVerbosity(DebugLevel::Trace)),
                        _ => {
                            println!("{}", "Invalid verbosity level. Use: error|warn|info|debug|trace".red());
                            None
                        }
                    }
                } else {
                    println!("{}", "Usage: verbosity <level>".yellow());
                    None
                }
            }
            "grid" | "g" => Some(DebugCommand::ToggleGrid),
            "agents" | "a" => Some(DebugCommand::ToggleAgents),
            "stats" | "s" => Some(DebugCommand::ToggleStats),
            "dump" | "d" => Some(DebugCommand::DumpState),
            "clear" | "c" => Some(DebugCommand::ClearBuffer),
            "pause" | "p" => Some(DebugCommand::Pause),
            "resume" | "r" => Some(DebugCommand::Resume),
            "step" | "n" => Some(DebugCommand::Step),
            _ => None,
        }
    }

    fn print_help(&self) {
        println!("\n{}", "Debug Commands:".cyan().bold());
        println!("  {} - Set verbosity level (error|warn|info|debug|trace)", "verbosity <level>".yellow());
        println!("  {} - Toggle grid display", "grid (g)".yellow());
        println!("  {} - Toggle agents display", "agents (a)".yellow());
        println!("  {} - Toggle stats display", "stats (s)".yellow());
        println!("  {} - Dump current state", "dump (d)".yellow());
        println!("  {} - Clear debug buffer", "clear (c)".yellow());
        println!("  {} - Pause simulation", "pause (p)".yellow());
        println!("  {} - Resume simulation", "resume (r)".yellow());
        println!("  {} - Step one frame", "step (n)".yellow());
        println!("  {} - Show this help", "help (h)".yellow());
        println!("  {} - Exit debug CLI", "quit (q)".yellow());
        println!("\n{}", "Keyboard shortcuts (in game):".cyan().bold());
        println!("  {} - Toggle stats", "F1".yellow());
        println!("  {} - Toggle grid", "F2".yellow());
        println!("  {} - Toggle agents", "F3".yellow());
        println!("  {} - Clear buffer", "F5".yellow());
        println!();
    }
}