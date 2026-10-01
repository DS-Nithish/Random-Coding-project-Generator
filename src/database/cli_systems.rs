use crate::models::{Difficulty, Domain, ProjectIdea};

pub fn get_projects() -> Vec<ProjectIdea> {
    vec![
        // 1. Beginner CLI 1
        ProjectIdea {
            id: "cli-beg-1".into(),
            title: "Supercharged Todo & Habit CLI with Pomodoro Timer".into(),
            domain: Domain::CliSystems,
            difficulty: Difficulty::Beginner,
            description: "A terminal productivity CLI tool featuring colorized task checklists, priority tags, streak counters, and an interactive ANSI-colored Pomodoro focus timer.".into(),
            requirements: vec![
                "Add, list, complete, edit, and delete tasks with priority flags".into(),
                "Store data in a clean local JSON or SQLite file in ~/.config/todo/".into(),
                "Integrated 25-minute Pomodoro timer with progress bar and terminal bell ding".into(),
                "Filter tasks by due date, tag, or pending status".into(),
            ],
            technologies: vec!["Rust / Go / Python".into(), "ANSI Escape Codes / Colored".into(), "JSON Serialization".into()],
            duration: "5 - 8 hours".into(),
            steps: vec![
                "Step 1: Set up argument parser with subcommands (add, list, done, timer).".into(),
                "Step 2: Implement task struct and JSON file storage persistence.".into(),
                "Step 3: Format output with rich ANSI tables and color-coded status badges.".into(),
                "Step 4: Build countdown Pomodoro timer updating in-place using '\\r' cursor resets.".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "todo_cli.rs".into(),
            starter_code: r#"// Simple CLI Task Model & Display
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    title: String,
    completed: bool,
    priority: String,
}

fn print_tasks(tasks: &[Task]) {
    println!("{}", "=== 📋 TASK LIST ===".bold().cyan());
    for t in tasks {
        let status = if t.completed { "[✓]".green() } else { "[ ]".yellow() };
        let prio = match t.priority.as_str() {
            "high" => "HIGH".red().bold(),
            "med" => "MED".yellow(),
            _ => "LOW".blue(),
        };
        println!("{} #{} [{}] {}", status, t.id, prio, t.title);
    }
}
"#.into(),
            documentation: r#"# Supercharged Todo & Habit CLI

Fast, keyboard-driven productivity manager right in your terminal.
"#.into(),
        },

        // 2. Beginner CLI 2
        ProjectIdea {
            id: "cli-beg-2".into(),
            title: "Blazing Fast Multi-threaded File Duplicate Finder".into(),
            domain: Domain::CliSystems,
            difficulty: Difficulty::Beginner,
            description: "Scans directories recursively, groups candidate files by file size, and computes streaming SHA-256 hashes to detect identical duplicate files with interactive cleanup.".into(),
            requirements: vec![
                "Recursive directory tree traversal skipping symlinks or permission errors".into(),
                "Two-stage filtering: group by byte size first to avoid hashing unique files".into(),
                "Chunked streaming SHA-256 hashing to handle gigabyte-sized files without OOM".into(),
                "Interactive selection prompt to safely delete or symlink duplicate copies".into(),
            ],
            technologies: vec!["Rust (Rayon / Walkdir) or Go".into(), "SHA-256 Hashing".into(), "File I/O Streams".into()],
            duration: "6 - 10 hours".into(),
            steps: vec![
                "Step 1: Walk directory collecting file paths and file metadata lengths into a HashMap.".into(),
                "Step 2: Filter out files with unique sizes.".into(),
                "Step 3: Hash matching candidate files in parallel using worker threads.".into(),
                "Step 4: Display duplicate groups with wasted disk space totals and prompt for action.".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "dup_finder.rs".into(),
            starter_code: r#"// Two-stage Duplicate File Detection Concept
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn group_by_size(paths: Vec<PathBuf>) -> HashMap<u64, Vec<PathBuf>> {
    let mut size_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    for p in paths {
        if let Ok(meta) = fs::metadata(&p) {
            if meta.is_file() {
                size_map.entry(meta.len()).or_default().push(p);
            }
        }
    }
    // Retain only sizes with >1 files
    size_map.retain(|_, files| files.len() > 1);
    size_map
}
"#.into(),
            documentation: r#"# Multi-threaded File Duplicate Finder

Reclaims gigabytes of disk space by swiftly isolating identical binary files.
"#.into(),
        },

        // 3. Beginner CLI 3
        ProjectIdea {
            id: "cli-beg-3".into(),
            title: "Colorized Disk Usage Analyzer (ncdu Clone)".into(),
            domain: Domain::CliSystems,
            difficulty: Difficulty::Beginner,
            description: "Terminal TUI / CLI tool that calculates folder disk consumption, sorts entries from heaviest to lightest, and draws proportional bar graphs.".into(),
            requirements: vec![
                "Traverse folder tree and calculate total recursive size of each subdirectory".into(),
                "Pretty format bytes into Human-Readable units (B, KB, MB, GB)".into(),
                "Render visual ANSI percentage progress bar for each folder entry".into(),
                "Interactive navigation allowing drilling down into heavy folders".into(),
            ],
            technologies: vec!["Rust or Python".into(), "Crossterm / Ratatui".into(), "Filesystem API".into()],
            duration: "6 - 9 hours".into(),
            steps: vec![
                "Step 1: Recursively accumulate file sizes for current directory children.".into(),
                "Step 2: Sort directory entries descending by total size.".into(),
                "Step 3: Render horizontal ASCII bar graph [████████░░░░] 65% for each row.".into(),
                "Step 4: Support keyboard up/down arrows to browse into nested directories.".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "disk_usage.rs".into(),
            starter_code: r#"// Human readable bytes and ASCII bar renderer
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut b = bytes as f64;
    let mut i = 0;
    while b >= 1024.0 && i < UNITS.len() - 1 {
        b /= 1024.0;
        i += 1;
    }
    format!("{:.1} {}", b, UNITS[i])
}

pub fn make_bar(percent: f64, width: usize) -> String {
    let filled = ((percent / 100.0) * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}
"#.into(),
            documentation: r#"# Disk Usage Analyzer CLI

Visualizes storage bottlenecks right from your command line.
"#.into(),
        },

        // 4. Intermediate CLI 1
        ProjectIdea {
            id: "cli-int-1".into(),
            title: "Git from Scratch (Write Your Own MyGit in Rust)".into(),
            domain: Domain::CliSystems,
            difficulty: Difficulty::Intermediate,
            description: "Implement core Git plumbing and porcelain commands: init, hash-object, cat-file, write-tree, commit-tree, log, and checkout by manipulating the .git object database.".into(),
            requirements: vec![
                "Initialize `.mygit/` directory structure: objects/, refs/heads/, HEAD".into(),
                "Read & write Git objects: blobs, trees, and commits with zlib compression & SHA-1".into(),
                "Implement `mygit commit` with author timestamps and parent commit references".into(),
                "Implement `mygit log` traversing parent commit pointers back to genesis commit".into(),
                "Implement `mygit checkout` reconstituting working directory files from tree objects".into(),
            ],
            technologies: vec!["Rust or Go".into(), "SHA-1 Hash".into(), "Zlib / Flate2 Compression".into(), "Filesystem Binary I/O".into()],
            duration: "18 - 30 hours".into(),
            steps: vec![
                "Step 1: Understand Git's object model: header format `'blob <size>\0<content>'` hashed via SHA-1.".into(),
                "Step 2: Implement `hash-object -w` writing zlib compressed content to `.mygit/objects/xx/yy`.".into(),
                "Step 3: Implement `cat-file -p` reading, decompressing, and printing objects.".into(),
                "Step 4: Implement recursive `write-tree` serialization for directory hierarchies.".into(),
                "Step 5: Implement `commit-tree` and update `refs/heads/master` to point to the new hash.".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "git_blob.rs".into(),
            starter_code: r#"// Git Blob Object Creation & SHA-1 Hashing
use std::io::Write;

pub fn create_blob_object(data: &[u8]) -> (String, Vec<u8>) {
    let header = format!("blob {}\0", data.len());
    let mut store = Vec::new();
    store.extend_from_slice(header.as_bytes());
    store.extend_from_slice(data);

    // Compute SHA-1
    // let sha = sha1_hash(&store);
    let mock_sha = format!("{:x}", store.len()); // Placeholder
    (mock_sha, store)
}
"#.into(),
            documentation: r#"# Write Your Own Git in Rust

Master Git internals by rebuilding its content-addressable storage engine from scratch.
"#.into(),
        },

        // 5. Intermediate CLI 2
        ProjectIdea {
            id: "cli-int-2".into(),
            title: "Custom Unix Shell with Pipelines & Redirection".into(),
            domain: Domain::CliSystems,
            difficulty: Difficulty::Intermediate,
            description: "A POSIX-like command shell supporting command execution, job control (Ctrl+C, Ctrl+Z, fg, bg), pipes (|), file redirection (<, >, >>), and environment variables.".into(),
            requirements: vec![
                "Interactive REPL with line-editing, history, and autocomplete".into(),
                "Process execution using `fork` and `execvp` syscalls".into(),
                "Arbitrary pipe chaining (`cat file | grep text | wc -l`) via POSIX `pipe()` and `dup2()`".into(),
                "File redirection (`>`, `>>`, `<`) and background execution (`&`)".into(),
                "Signal handling (SIGINT, SIGTSTP, SIGCHLD) preventing zombie processes".into(),
            ],
            technologies: vec!["C / C++ or Rust (nix crate)".into(), "POSIX Syscalls (fork, exec, pipe, dup2)".into(), "Signal Handling".into()],
            duration: "20 - 32 hours".into(),
            steps: vec![
                "Step 1: Write tokenizer to parse command lines into commands, arguments, pipes, and redirects.".into(),
                "Step 2: Implement built-in commands (cd, pwd, exit, export).".into(),
                "Step 3: Execute external programs using `fork()` and `execvp()`.".into(),
                "Step 4: Wire multi-stage pipelines by dup2'ing stdout of command N to stdin of command N+1.".into(),
                "Step 5: Set up process group IDs and signal handlers for clean job control.".into(),
            ],
            starter_code_language: "c".into(),
            starter_code_filename: "mini_shell.c".into(),
            starter_code: r#"// Basic Fork & Exec Loop in C
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/wait.h>

int main() {
    char input[256];
    while (1) {
        printf("myshell> ");
        if (!fgets(input, sizeof(input), stdin)) break;
        input[strcspn(input, "\n")] = 0; // Strip newline

        if (strcmp(input, "exit") == 0) break;

        pid_t pid = fork();
        if (pid == 0) {
            // Child process
            char *args[] = { input, NULL };
            execvp(args[0], args);
            perror("exec failed");
            exit(1);
        } else {
            // Parent waits
            waitpid(pid, NULL, 0);
        }
    }
    return 0;
}
"#.into(),
            documentation: r#"# Custom Unix Shell

Demystifies operating system process isolation, pipes, and inter-process communication.
"#.into(),
        },

        // 6. Intermediate CLI 3
        ProjectIdea {
            id: "cli-int-3".into(),
            title: "Real-time Terminal System Monitor (htop Clone in Rust)".into(),
            domain: Domain::CliSystems,
            difficulty: Difficulty::Intermediate,
            description: "High-performance TUI system resource monitor reading CPU per-core load, RAM/Swap usage, disk I/O, network bandwidth, and process tree with kill signals.".into(),
            requirements: vec![
                "Parse Linux `/proc` filesystem or macOS `sysctl` / `mach` APIs for metrics".into(),
                "Per-core CPU utilization gauges and sparkline history graphs".into(),
                "Sortable process table (PID, User, CPU%, Mem%, Command, Uptime)".into(),
                "Send signals (SIGTERM, SIGKILL) to targeted processes interactively".into(),
                "Search and filter processes with real-time responsive updates".into(),
            ],
            technologies: vec!["Rust (sysinfo / ratatui / crossterm)".into(), "OS Metrics (/proc, sysctl)".into(), "TUI Terminal Rendering".into()],
            duration: "16 - 25 hours".into(),
            steps: vec![
                "Step 1: Set up Ratatui terminal UI with layout chunks (header, CPU gauges, process table).".into(),
                "Step 2: Collect delta CPU ticks between time intervals to compute % utilization.".into(),
                "Step 3: Read process table and calculate resident set size (RSS) memory.".into(),
                "Step 4: Handle keyboard events (Up/Down to select PID, 'k' to kill, 'f' to filter).".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "sys_mon.rs".into(),
            starter_code: r#"// CPU Percentage calculation from delta ticks
#[derive(Default)]
pub struct CpuTickSample {
    pub user: u64,
    pub system: u64,
    pub idle: u64,
}

pub fn calculate_cpu_usage(prev: &CpuTickSample, curr: &CpuTickSample) -> f32 {
    let prev_total = prev.user + prev.system + prev.idle;
    let curr_total = curr.user + curr.system + curr.idle;
    let total_delta = curr_total.saturating_sub(prev_total) as f32;
    let idle_delta = curr.idle.saturating_sub(prev.idle) as f32;

    if total_delta > 0.0 {
        ((total_delta - idle_delta) / total_delta) * 100.0
    } else {
        0.0
    }
}
"#.into(),
            documentation: r#"# Terminal System Monitor (htop Clone)

Clean TUI system performance dashboard in pure Rust.
"#.into(),
        },

        // 7. Advanced CLI 1
        ProjectIdea {
            id: "cli-adv-1".into(),
            title: "Async High-Throughput HTTP/1.1 & HTTP/2 Server in Rust".into(),
            domain: Domain::CliSystems,
            difficulty: Difficulty::Advanced,
            description: "Custom asynchronous event-driven HTTP server built on raw epoll / kqueue (or Tokio): non-blocking sockets, zero-copy file transfer, HTTP parsing, and connection pooling.".into(),
            requirements: vec![
                "Non-blocking TCP socket listener using OS event multiplexing (epoll on Linux, kqueue on macOS)".into(),
                "HTTP/1.1 request parser supporting chunked transfer encoding and keep-alive connections".into(),
                "Zero-copy file transmission using `sendfile` syscall for static assets".into(),
                "Thread pool / async task scheduler handling 100,000+ concurrent idle connections (C10K problem)".into(),
                "Graceful server shutdown with in-flight request draining".into(),
            ],
            technologies: vec!["Rust (Mio / Tokio or Raw Syscalls)".into(), "epoll / kqueue".into(), "Zero-copy sendfile".into(), "HTTP RFC Standards".into()],
            duration: "35 - 55 hours".into(),
            steps: vec![
                "Step 1: Set up non-blocking TCP socket and register with epoll/kqueue event loop.".into(),
                "Step 2: Buffer incoming bytes and parse HTTP request line and headers using state machine.".into(),
                "Step 3: Route incoming paths to handlers or file responders.".into(),
                "Step 4: Use `sendfile` to stream static media directly from kernel page cache to socket.".into(),
                "Step 5: Benchmark against Nginx with `wrk` and optimize memory allocations.".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "async_http.rs".into(),
            starter_code: r#"// Minimal Non-blocking HTTP Header Parser
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
}

pub fn parse_http_request(raw: &[u8]) -> Option<HttpRequest> {
    let text = std::str::from_utf8(raw).ok()?;
    let mut lines = text.lines();
    let request_line = lines.next()?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next()?.to_string();
    let path = parts.next()?.to_string();

    let mut headers = Vec::new();
    for line in lines {
        if line.is_empty() { break; }
        if let Some((k, v)) = line.split_once(':') {
            headers.push((k.trim().to_string(), v.trim().to_string()));
        }
    }
    Some(HttpRequest { method, path, headers })
}
"#.into(),
            documentation: r#"# Async High-Throughput HTTP Server

Achieves extreme concurrency and throughput by eliminating thread-per-connection overhead.
"#.into(),
        },

        // 8. Advanced CLI 2
        ProjectIdea {
            id: "cli-adv-2".into(),
            title: "Container Runtime from Scratch (Mini Docker in Go/Rust)".into(),
            domain: Domain::CliSystems,
            difficulty: Difficulty::Advanced,
            description: "Linux container isolation runtime: creates isolated containers using Linux namespaces (PID, Mount, Net, UTS, IPC), cgroups v2 resource limits (CPU/Memory), and overlayfs rootfs layering.".into(),
            requirements: vec![
                "Linux `clone()` / `unshare()` syscalls creating isolated namespaces".into(),
                "Process tree isolation (container PID 1) and hostname isolation".into(),
                "Linux Cgroups v2 integration to enforce memory limits (e.g. max 100MB) and CPU quotas".into(),
                "Chroot / Pivot_root filesystem jail with OverlayFS union mount of container images".into(),
                "Veth virtual ethernet pair setup connecting container namespace to host bridge".into(),
            ],
            technologies: vec!["Go or Rust".into(), "Linux Syscalls (clone, unshare, pivot_root)".into(), "Linux Cgroups v2".into(), "OverlayFS & Netlink".into()],
            duration: "40 - 65 hours".into(),
            steps: vec![
                "Step 1: Re-exec binary under new PID and Mount namespaces with `CLONE_NEWPID | CLONE_NEWNS`.".into(),
                "Step 2: Mount proc filesystem inside container jail at `/proc`.".into(),
                "Step 3: Configure Cgroups v2 by creating `/sys/fs/cgroup/container_<id>/memory.max`.".into(),
                "Step 4: Mount base alpine rootfs using `pivot_root`.".into(),
                "Step 5: Run user command inside the isolated container jail.".into(),
            ],
            starter_code_language: "go".into(),
            starter_code_filename: "mini_container.go".into(),
            starter_code: r#"// Container Namespaces isolation skeleton in Go
package main

import (
	"fmt"
	"os"
	"os/exec"
	"syscall"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: mini-docker run <command>")
		return
	}
	switch os.Args[1] {
	case "run":
		parent()
	case "child":
		child()
	}
}

func parent() {
	cmd := exec.Command("/proc/self/exe", append([]string{"child"}, os.Args[2:]...)...)
	cmd.SysProcAttr = &syscall.SysProcAttr{
		Cloneflags: syscall.CLONE_NEWUTS | syscall.CLONE_NEWPID | syscall.CLONE_NEWNS,
	}
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Run()
}

func child() {
	fmt.Printf("Running %v as PID %d inside container\n", os.Args[2:], os.Getpid())
	cmd := exec.Command(os.Args[2], os.Args[3:]...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Run()
}
"#.into(),
            documentation: r#"# Container Runtime from Scratch

Learn how containers really work beneath Docker and Kubernetes.
"#.into(),
        },

        // 9. Advanced CLI 3
        ProjectIdea {
            id: "cli-adv-3".into(),
            title: "Distributed Raft Consensus Key-Value Store in Rust".into(),
            domain: Domain::CliSystems,
            difficulty: Difficulty::Advanced,
            description: "Distributed fault-tolerant KV database implementing the complete Raft consensus protocol: Leader Election, Log Replication, Heartbeats, Membership changes, and Snapshot compaction.".into(),
            requirements: vec![
                "Raft state machine: Follower, Candidate, Leader roles with randomized election timeouts".into(),
                "AppendEntries and RequestVote RPC protocols with majority quorum validation".into(),
                "Persistent state (currentTerm, votedFor, log entries) written to disk before responding".into(),
                "Log compaction via snapshotting to bound disk growth".into(),
                "Survives network partitions and node crashes without data loss or split-brain".into(),
            ],
            technologies: vec!["Rust (Tokio / Tonic gRPC)".into(), "Raft Consensus Algorithm".into(), "Distributed Systems".into()],
            duration: "50 - 75 hours".into(),
            steps: vec![
                "Step 1: Define Raft RPC messages (RequestVoteArgs, RequestVoteReply, AppendEntriesArgs, AppendEntriesReply).".into(),
                "Step 2: Implement randomized election timer (150-300ms) transitioning followers to candidates.".into(),
                "Step 3: Build leader heartbeat broadcast and log replication pipeline.".into(),
                "Step 4: Implement state machine commitment index advancement upon majority acknowledgment.".into(),
                "Step 5: Write automated chaos tests simulating network splits and killed leader nodes.".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "raft_core.rs".into(),
            starter_code: r#"// Raft Node Roles and State
#[derive(Debug, PartialEq, Eq)]
pub enum RaftRole {
    Follower,
    Candidate,
    Leader,
}

pub struct RaftNode {
    pub id: usize,
    pub current_term: u64,
    pub voted_for: Option<usize>,
    pub role: RaftRole,
    pub commit_index: u64,
    pub last_applied: u64,
}

impl RaftNode {
    pub fn handle_heartbeat(&mut self, term: u64, leader_id: usize) -> bool {
        if term >= self.current_term {
            self.current_term = term;
            self.role = RaftRole::Follower;
            self.voted_for = Some(leader_id);
            true
        } else {
            false
        }
    }
}
"#.into(),
            documentation: r#"# Distributed Raft Consensus Key-Value Store

The gold-standard algorithm powering systems like etcd, CockroachDB, and Kafka metadata.
"#.into(),
        },

        // 10. Intermediate CLI 4
        ProjectIdea {
            id: "cli-int-4".into(),
            title: "Dynamic Terminal Stock & Crypto Ticker Dashboard".into(),
            domain: Domain::CliSystems,
            difficulty: Difficulty::Intermediate,
            description: "Terminal stock market and cryptocurrency ticker displaying live price charts, candlestick graphs, volume metrics, and price alert sound notifications.".into(),
            requirements: vec![
                "Fetch live prices and historical candlesticks from public financial APIs".into(),
                "ASCII / Braille candlestick charts rendered directly in terminal cells".into(),
                "Watchlist configuration saved in ~/.config/ticker.toml".into(),
                "Configurable desktop / bell alerts when prices cross custom threshold targets".into(),
            ],
            technologies: vec!["Rust / Go".into(), "Ratatui / Bubbletea".into(), "Braille Canvas Plotting".into()],
            duration: "14 - 20 hours".into(),
            steps: vec![
                "Step 1: Set up asynchronous background polling loop fetching ticker prices.".into(),
                "Step 2: Implement terminal braille character plotting for line and candlestick charts.".into(),
                "Step 3: Color code green (+ gain) and red (- loss) percentage indicators.".into(),
                "Step 4: Trigger terminal sound bell `\x07` when price targets trigger.".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "braille_chart.rs".into(),
            starter_code: r#"// Braille Character Terminal Plotting snippet
pub fn draw_sparkline(values: &[f64]) -> String {
    const BARS: &[char] = &[' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    if values.is_empty() { return String::new(); }
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = if (max - min).abs() < 1e-6 { 1.0 } else { max - min };

    values.iter().map(|&v| {
        let normalized = ((v - min) / range * (BARS.len() - 1) as f64).round() as usize;
        BARS[normalized]
    }).collect()
}
"#.into(),
            documentation: r#"# Terminal Stock & Crypto Ticker Dashboard

Real-time ASCII market charts without leaving your tmux session.
"#.into(),
        },
    ]
}
