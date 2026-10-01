use crate::models::{Difficulty, Domain, ProjectIdea};

pub fn get_projects() -> Vec<ProjectIdea> {
    vec![
        // 1. Beginner Cyber 1
        ProjectIdea {
            id: "sec-beg-1".into(),
            title: "Async Multi-threaded Port Scanner (Mini Nmap in Rust)".into(),
            domain: Domain::Cybersecurity,
            difficulty: Difficulty::Beginner,
            description: "High-speed concurrent TCP port scanner that sweeps IP addresses/hostnames, identifies open ports, banner grabs service names (HTTP, SSH, SMTP), and reports scan durations.".into(),
            requirements: vec![
                "Asynchronous TCP connect scanning over configurable port ranges (e.g. 1-1024 or full 65535)".into(),
                "Configurable concurrency worker limit and timeout thresholds to prevent socket exhaustion".into(),
                "Service banner grabbing reading raw welcoming bytes on open ports".into(),
                "Formatted output table with service names, open ports, and latency milliseconds".into(),
            ],
            technologies: vec!["Rust (Tokio) or Python (asyncio)".into(), "TCP Sockets".into(), "Banner Grabbing".into()],
            duration: "6 - 10 hours".into(),
            steps: vec![
                "Step 1: Resolve target hostname to IPv4/IPv6 address.".into(),
                "Step 2: Spawn asynchronous tasks testing `TcpStream::connect_timeout` against target ports.".into(),
                "Step 3: If port connects successfully, read initial greeting bytes to identify service banner.".into(),
                "Step 4: Output clean colorized results table displaying discovered active services.".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "port_scanner.rs".into(),
            starter_code: r#"// Fast Asynchronous TCP Port Scan Snippet
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use tokio::net::TcpStream;

pub async fn scan_port(addr: SocketAddr, timeout_ms: u64) -> bool {
    tokio::time::timeout(Duration::from_millis(timeout_ms), TcpStream::connect(addr))
        .await
        .map(|res| res.is_ok())
        .unwrap_or(false)
}
"#.into(),
            documentation: r#"# Async Multi-threaded Port Scanner

Fast reconnaissance network scanner with connection timeout guards.
"#.into(),
        },

        // 2. Beginner Cyber 2
        ProjectIdea {
            id: "sec-beg-2".into(),
            title: "Image LSB Steganography Tool (Secret Hide & Reveal)".into(),
            domain: Domain::Cybersecurity,
            difficulty: Difficulty::Beginner,
            description: "CLI security utility that embeds hidden encrypted text messages into the Least Significant Bits (LSB) of lossless PNG image pixels and extracts them without noticeable visual distortion.".into(),
            requirements: vec![
                "Encode binary bitstream into the lowest bit of red, green, and blue color channels".into(),
                "Store message length header or null terminator delimiter to stop decoding".into(),
                "Optional AES password encryption layer prior to LSB embedding".into(),
                "Peak Signal-to-Noise Ratio (PSNR) calculation to measure imperceptibility".into(),
            ],
            technologies: vec!["Python (Pillow) or Rust (image crate)".into(), "Bitwise Operations (AND/OR/SHIFTS)".into(), "Cryptography".into()],
            duration: "5 - 8 hours".into(),
            steps: vec![
                "Step 1: Load image pixel array buffer (RGB values 0-255).".into(),
                "Step 2: Convert secret message string into continuous bit sequence (8 bits per byte).".into(),
                "Step 3: Clear LSB of pixel channel (`pixel & ~1`) and OR with message bit (`pixel | bit`).".into(),
                "Step 4: Implement extraction decoder gathering LSBs until length header reached.".into(),
            ],
            starter_code_language: "python".into(),
            starter_code_filename: "steganography.py".into(),
            starter_code: r#"# Least Significant Bit (LSB) Steganography Encoder
def encode_lsb(byte_val, bit):
    return (byte_val & ~1) | (bit & 1)

def decode_lsb(byte_val):
    return byte_val & 1

# Example embedding bit 1 into pixel color 204 (11001100)
original = 204
encoded = encode_lsb(original, 1)
print(f"Original: {original} -> Encoded: {encoded} (LSB: {decode_lsb(encoded)})")
"#.into(),
            documentation: r#"# Image LSB Steganography Tool

Hides clandestine payload data directly inside raw image color channels.
"#.into(),
        },

        // 3. Beginner Cyber 3
        ProjectIdea {
            id: "sec-beg-3".into(),
            title: "Network Packet Sniffer & Traffic Analyzer".into(),
            domain: Domain::Cybersecurity,
            difficulty: Difficulty::Beginner,
            description: "Captures live network packets using raw sockets / libpcap, dissects Ethernet, IP, TCP, and UDP headers, and prints real-time packet protocol statistics.".into(),
            requirements: vec![
                "Capture live frames on promiscuous network interfaces using libpcap / raw sockets".into(),
                "Dissect packet headers: MAC addresses, IPv4 source/destination, ports, and payload sizes".into(),
                "Filter traffic by protocol (TCP, UDP, ICMP, DNS)".into(),
                "Hexdump visualizer showing ASCII and hexadecimal representation of payloads".into(),
            ],
            technologies: vec!["Python (Scapy / Socket) or Rust (pcap / pnet)".into(), "Raw Sockets".into(), "Packet Parsing".into()],
            duration: "7 - 11 hours".into(),
            steps: vec![
                "Step 1: Bind raw socket or open pcap capture device.".into(),
                "Step 2: Unpack Ethernet frame (14 bytes) extracting EtherType (0x0800 for IPv4).".into(),
                "Step 3: Parse IPv4 header extracting Protocol, TTL, Source IP, and Destination IP.".into(),
                "Step 4: Inspect TCP/UDP ports and format payload bytes into side-by-side hex and ASCII dump.".into(),
            ],
            starter_code_language: "python".into(),
            starter_code_filename: "packet_sniffer.py".into(),
            starter_code: r#"# Basic IPv4 Packet Dissector in Python
import socket
import struct

def parse_ipv4_header(data):
    # IPv4 header is first 20 bytes
    iph = struct.unpack('!BBHHHBBH4s4s', data[:20])
    src_ip = socket.inet_ntoa(iph[8])
    dst_ip = socket.inet_ntoa(iph[9])
    proto = iph[6]
    return src_ip, dst_ip, proto

print("IPv4 Header Unpacker Ready")
"#.into(),
            documentation: r#"# Network Packet Sniffer

Direct inspection of network layer packet headers and hex payloads.
"#.into(),
        },

        // 4. Intermediate Cyber 1
        ProjectIdea {
            id: "sec-int-1".into(),
            title: "Low-Interaction SSH Honeypot (Cowrie-Style in Python/Rust)".into(),
            domain: Domain::Cybersecurity,
            difficulty: Difficulty::Intermediate,
            description: "Deceptive SSH server decoy that traps malicious botnets, simulates a fake Linux shell environment, logs credential brute-force attacks, and records attacker keystrokes.".into(),
            requirements: vec![
                "Custom SSH server accepting connections and capturing username/password combinations".into(),
                "Fake interactive shell with emulated basic commands (ls, pwd, uname -a, cat /etc/passwd, wget)".into(),
                "Session TTY recorder saving attacker keystroke sessions as asciinema / text replays".into(),
                "Malware payload capture intercepting attempted curl/wget file downloads".into(),
                "Real-time JSON alert logging and geolocation IP lookup of attacker origins".into(),
            ],
            technologies: vec!["Python (Paramiko / Twisted) or Rust (Thrussh)".into(), "SSH Protocol".into(), "MaxMind GeoIP".into()],
            duration: "18 - 28 hours".into(),
            steps: vec![
                "Step 1: Set up SSH server listener with custom host key generation.".into(),
                "Step 2: Intercept authentication attempts, log credentials, and always grant access for honey accounts.".into(),
                "Step 3: Implement mock terminal shell responding with realistic fake directory listings.".into(),
                "Step 4: Record every raw typed byte into session replay logs.".into(),
                "Step 5: Alert on downloaded URLs and compute threat actor statistics.".into(),
            ],
            starter_code_language: "python".into(),
            starter_code_filename: "honeypot_auth.py".into(),
            starter_code: r#"# SSH Honeypot Authentication Interceptor (Paramiko Concept)
import paramiko

class HoneySSHInterface(paramiko.ServerInterface):
    def check_auth_password(self, username, password):
        print(f"[ATTACK DETECTED] User: '{username}' | Password: '{password}'")
        # Record to audit log
        return paramiko.AUTH_SUCCESSFUL

    def check_channel_request(self, kind, chanid):
        if kind == 'session': return paramiko.OPEN_SUCCEEDED
        return paramiko.OPEN_FAILED_ADMINISTRATIVELY_PROHIBITED
"#.into(),
            documentation: r#"# Low-Interaction SSH Honeypot

Traps automated worms, scans, and credential stuffers in an isolated decoy sandbox.
"#.into(),
        },

        // 5. Intermediate Cyber 2
        ProjectIdea {
            id: "sec-int-2".into(),
            title: "Encrypted DNS-over-HTTPS (DoH) & Ad-blocking Proxy".into(),
            domain: Domain::Cybersecurity,
            difficulty: Difficulty::Intermediate,
            description: "Local DNS proxy daemon that intercepts traditional plaintext port 53 DNS queries, blocks ads/malware domains via blacklists, and forwards queries via DNS-over-HTTPS (RFC 8484).".into(),
            requirements: vec![
                "UDP socket listener intercepting local port 53 DNS requests".into(),
                "DNS wire-format query parser extracting requested question domains (e.g. example.com)".into(),
                "Domain blocklist engine using Trie or HashSet for instantaneous filtering of millions of ad domains".into(),
                "Upstream forwarding to Cloudflare/Google via DNS-over-HTTPS (HTTP/2 POST with application/dns-message)".into(),
                "Local DNS response caching with TTL expiration to minimize upstream latency".into(),
            ],
            technologies: vec!["Rust or Go".into(), "DNS Protocol (RFC 1035)".into(), "DNS-over-HTTPS (RFC 8484)".into(), "Trie Data Structure".into()],
            duration: "16 - 26 hours".into(),
            steps: vec![
                "Step 1: Listen for incoming UDP datagrams on `127.0.0.1:53`.".into(),
                "Step 2: Parse raw binary DNS wire format packet (Transaction ID, Flags, Questions).".into(),
                "Step 3: Check domain against loaded Pi-hole style blocklists; return `0.0.0.0` if blocked.".into(),
                "Step 4: If clean, forward query payload to `https://cloudflare-dns.com/dns-query` via HTTPS POST.".into(),
                "Step 5: Cache answer payload in memory and reply to local DNS client.".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "doh_proxy.rs".into(),
            starter_code: r#"// DNS Question Domain Extractor
pub fn extract_domain_name(buffer: &[u8], offset: usize) -> Option<(String, usize)> {
    let mut domain = String::new();
    let mut pos = offset;
    loop {
        if pos >= buffer.len() { return None; }
        let len = buffer[pos] as usize;
        if len == 0 { pos += 1; break; }
        pos += 1;
        if pos + len > buffer.len() { return None; }
        let label = std::str::from_utf8(&buffer[pos..pos+len]).ok()?;
        if !domain.is_empty() { domain.push('.'); }
        domain.push_str(label);
        pos += len;
    }
    Some((domain, pos))
}
"#.into(),
            documentation: r#"# DNS-over-HTTPS Ad-blocking Proxy

Guards privacy by defeating ISP DNS snooping and eliminates ads at the network level.
"#.into(),
        },

        // 6. Intermediate Cyber 3
        ProjectIdea {
            id: "sec-int-3".into(),
            title: "Automated Web Vulnerability & Header Security Auditor".into(),
            domain: Domain::Cybersecurity,
            difficulty: Difficulty::Intermediate,
            description: "Security scanning engine that tests web apps for misconfigured security headers (CSP, HSTS, CORS), SSL certificate expiry, open redirect flaws, and exposed .git/.env credentials.".into(),
            requirements: vec![
                "Audit security headers: Content-Security-Policy, Strict-Transport-Security, X-Frame-Options".into(),
                "Evaluate CORS misconfigurations (wildcard `*` paired with credentials `true`)".into(),
                "TLS certificate verification (check expiration days, cipher suite strength, SAN names)".into(),
                "Check sensitive exposure endpoints (/.git/config, /.env, /actuator/health, /wp-config.php.bak)".into(),
                "Generate executive PDF or HTML vulnerability assessment report with remediation advice".into(),
            ],
            technologies: vec!["Python or Rust (Reqwest)".into(), "TLS/SSL Inspection".into(), "HTML/Markdown Report Generator".into()],
            duration: "14 - 22 hours".into(),
            steps: vec![
                "Step 1: Perform HTTP/HTTPS head requests collecting response headers.".into(),
                "Step 2: Score header posture against OWASP Secure Headers Project standards.".into(),
                "Step 3: Connect TLS socket and extract certificate expiration and signature algorithms.".into(),
                "Step 4: Probe known sensitive file paths with status code and signature verification.".into(),
                "Step 5: Compile color-coded remediation grade report (A+ to F).".into(),
            ],
            starter_code_language: "python".into(),
            starter_code_filename: "header_auditor.py".into(),
            starter_code: r#"# Web Security Headers Audit Check
REQUIRED_HEADERS = {
    "Strict-Transport-Security": "Protects against SSL stripping attacks",
    "Content-Security-Policy": "Mitigates XSS and data injection attacks",
    "X-Frame-Options": "Prevents clickjacking framing",
    "X-Content-Type-Options": "Disables MIME-sniffing"
}

def audit_headers(response_headers):
    findings = []
    for h, desc in REQUIRED_HEADERS.items():
        if h.lower() not in [k.lower() for k in response_headers]:
            findings.append({"header": h, "severity": "MEDIUM", "reason": f"Missing: {desc}"})
    return findings
"#.into(),
            documentation: r#"# Web Security Header Auditor

Automated compliance and misconfiguration scanner for modern web applications.
"#.into(),
        },

        // 7. Advanced Cyber 1
        ProjectIdea {
            id: "sec-adv-1".into(),
            title: "TLS 1.3 Handshake Parser & Certificate Verifier from Scratch".into(),
            domain: Domain::Cybersecurity,
            difficulty: Difficulty::Advanced,
            description: "Implement a TLS 1.3 client handshake parser without OpenSSL: ClientHello, ServerHello, key exchange (ECDH X25519), HKDF key derivation, and record layer decryption.".into(),
            requirements: vec![
                "Construct binary TLS 1.3 ClientHello record with cipher suites and key share extensions".into(),
                "Parse ServerHello and extract server's ephemeral public key".into(),
                "Compute shared secret using Diffie-Hellman on Curve25519 (X25519)".into(),
                "Implement HKDF (HMAC-based Extract-and-Expand Key Derivation) to derive client/server traffic keys".into(),
                "Decrypt AES-128-GCM / ChaCha20-Poly1305 encrypted handshake extensions and verify certificates".into(),
            ],
            technologies: vec!["Rust or C".into(), "TLS 1.3 RFC 8446 Specification".into(), "Elliptic Curve Cryptography (X25519)".into(), "HKDF & AES-GCM".into()],
            duration: "45 - 70 hours".into(),
            steps: vec![
                "Step 1: Understand TLS 1.3 record framing: ContentType (22 = Handshake), ProtocolVersion (0x0303).".into(),
                "Step 2: Generate ephemeral X25519 private/public keypair and construct ClientHello payload.".into(),
                "Step 3: Read ServerHello, compute shared ECDH secret, and run HKDF-Extract.".into(),
                "Step 4: Derive handshake traffic keys and initialize AES-GCM cipher.".into(),
                "Step 5: Decrypt EncryptedExtensions and parse X.509 server certificate chain.".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "tls_handshake.rs".into(),
            starter_code: r#"// TLS 1.3 Record Header framing
pub struct TlsRecordHeader {
    pub content_type: u8, // 22 = Handshake, 23 = Application Data
    pub legacy_version: u16, // 0x0303
    pub length: u16,
}

pub fn parse_record_header(buf: &[u8]) -> Option<TlsRecordHeader> {
    if buf.len() < 5 { return None; }
    let content_type = buf[0];
    let legacy_version = u16::from_be_bytes([buf[1], buf[2]]);
    let length = u16::from_be_bytes([buf[3], buf[4]]);
    Some(TlsRecordHeader { content_type, legacy_version, length })
}
"#.into(),
            documentation: r#"# TLS 1.3 Handshake Parser from Scratch

Demystifies modern end-to-end transport encryption protocols.
"#.into(),
        },

        // 8. Advanced Cyber 2
        ProjectIdea {
            id: "sec-adv-2".into(),
            title: "Kernel-Level eBPF Network Firewall & DDoS Mitigator".into(),
            domain: Domain::Cybersecurity,
            difficulty: Difficulty::Advanced,
            description: "Ultra high-speed Linux XDP (eXpress Data Path) firewall using eBPF in Rust/C: filters malicious DDoS packets directly at network card driver level before allocating kernel sk_buffs.".into(),
            requirements: vec![
                "eBPF XDP hook program running in kernel space returning XDP_DROP or XDP_PASS".into(),
                "eBPF BPF_MAP_TYPE_HASH maps syncing blocked IP addresses from user space to kernel in real time".into(),
                "Token bucket rate-limiting algorithm implemented in eBPF byte code".into(),
                "SYN flood, UDP amplification, and DNS reflection detection and drops at 10M+ packets/sec".into(),
                "User-space CLI in Rust (Aya / libbpf-rs) managing firewall rules and inspecting stats".into(),
            ],
            technologies: vec!["Linux eBPF & XDP".into(), "Rust (Aya framework) or C".into(), "High-speed Kernel Networking".into()],
            duration: "40 - 65 hours".into(),
            steps: vec![
                "Step 1: Write XDP C or Rust (Aya) program hooking into network device interface.".into(),
                "Step 2: Parse Ethernet and IPv4 headers using pointer bounds checking required by the BPF verifier.".into(),
                "Step 3: Look up source IP in eBPF BPF_MAP map; if found, execute immediate `XDP_DROP`.".into(),
                "Step 4: Build user-space controller to insert dynamic IP rules without kernel reloads.".into(),
                "Step 5: Benchmark drop performance with packet generator against standard iptables.".into(),
            ],
            starter_code_language: "c".into(),
            starter_code_filename: "xdp_filter.c".into(),
            starter_code: r#"// Minimal eBPF XDP Packet Dropper snippet
#include <linux/bpf.h>
#include <linux/if_ether.h>
#include <linux/ip.h>

SEC("xdp")
int xdp_firewall(struct xdp_md *ctx) {
    void *data = (void *)(long)ctx->data;
    void *data_end = (void *)(long)ctx->data_end;

    struct ethhdr *eth = data;
    if ((void *)(eth + 1) > data_end) return XDP_PASS;
    if (eth->h_proto != __constant_htons(ETH_P_IP)) return XDP_PASS;

    struct iphdr *ip = (void *)(eth + 1);
    if ((void *)(ip + 1) > data_end) return XDP_PASS;

    // eBPF Map lookup for blocked IP...
    // if (is_blocked(ip->saddr)) return XDP_DROP;

    return XDP_PASS;
}
"#.into(),
            documentation: r#"# eBPF Kernel DDoS Mitigator

Sub-microsecond packet filtering directly inside the Linux network driver path.
"#.into(),
        },

        // 9. Advanced Cyber 3
        ProjectIdea {
            id: "sec-adv-3".into(),
            title: "Coverage-Guided Binary Fuzzing Engine (Mini AFL)".into(),
            domain: Domain::Cybersecurity,
            difficulty: Difficulty::Advanced,
            description: "Automated vulnerability hunting fuzzer implementing compiler-based coverage instrumentation (or binary translation), evolutionary mutation algorithms, and crash deduplication.".into(),
            requirements: vec![
                "Process execution monitor capturing target crashes, segfaults (SIGSEGV), and timeouts".into(),
                "Code coverage feedback tracking execution edge transitions via shared memory bitmap (AFL map)".into(),
                "Genetic input mutation engine: bit flips, byte overwrites, integer boundaries, and dictionary splicing".into(),
                "Crash triage & deduplication grouping unique bugs by execution callstack hashes".into(),
                "High-speed fork server eliminating process startup and dynamic link loading overhead".into(),
            ],
            technologies: vec!["Rust / C++".into(), "Coverage Instrumentation".into(), "Syscalls (ptrace / fork)".into(), "Genetic Fuzzing".into()],
            duration: "45 - 70 hours".into(),
            steps: vec![
                "Step 1: Set up shared memory bitmap (64KB) recording edge transitions `prev_loc ^ curr_loc`.".into(),
                "Step 2: Build fork server that spawns mutated input tests via copy-on-write `fork()`.".into(),
                "Step 3: Implement mutation operators (bit flips, arithmetic adds, byte swaps).".into(),
                "Step 4: Check if mutated input triggers previously unseen coverage edges; if so, save to queue.".into(),
                "Step 5: Catch SIGSEGV signals, record crashing payloads, and deduplicate by stack trace.".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "fuzzer_mutator.rs".into(),
            starter_code: r#"// Basic Byte Mutation Operators for Fuzzing
use rand::Rng;

pub fn mutate_payload(data: &mut Vec<u8>) {
    let mut rng = rand::thread_rng();
    if data.is_empty() { return; }
    let choice = rng.gen_range(0..3);
    let idx = rng.gen_range(0..data.len());

    match choice {
        0 => data[idx] ^= 1 << rng.gen_range(0..8), // Bit flip
        1 => data[idx] = rng.gen(),                 // Random byte overwrite
        2 => {
            // Boundary integer insert
            let boundary_vals = [0x00, 0xFF, 0x7F, 0x80];
            data[idx] = boundary_vals[rng.gen_range(0..boundary_vals.len())];
        }
        _ => {}
    }
}
"#.into(),
            documentation: r#"# Coverage-Guided Binary Fuzzing Engine

Discovers critical memory corruption bugs and zero-day vulnerabilities automatically.
"#.into(),
        },

        // 10. Intermediate Cyber 4
        ProjectIdea {
            id: "sec-int-4".into(),
            title: "Layer 7 Reverse Proxy & Web Application Firewall (WAF)".into(),
            domain: Domain::Cybersecurity,
            difficulty: Difficulty::Intermediate,
            description: "High-performance reverse proxy that load balances traffic across backend servers, inspects HTTP bodies for SQL injection (SQLi) and XSS signatures, and enforces rate limits.".into(),
            requirements: vec![
                "Reverse proxy forwarding client HTTP requests to backend cluster with Round-Robin or Least-Connections".into(),
                "WAF regex and heuristic engine inspecting query params, headers, and POST bodies for SQLi and XSS".into(),
                "Sliding-window IP rate limiter blocking brute-force DDoS attempts with 429 Too Many Requests".into(),
                "SSL termination and TLS certificate offloading".into(),
            ],
            technologies: vec!["Rust (Hyper / Axum) or Go".into(), "WAF Heuristic Rules".into(), "Token Bucket Rate Limiting".into()],
            duration: "18 - 28 hours".into(),
            steps: vec![
                "Step 1: Set up HTTP proxy server receiving client requests.".into(),
                "Step 2: Inspect URI paths and bodies with compiled libinjection / regex rules for `' OR 1=1 --` or `<script>`.".into(),
                "Step 3: If malicious pattern detected, terminate connection and return 403 Forbidden with log audit.".into(),
                "Step 4: Forward sanitized requests to upstream backend servers and stream responses back.".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "waf_rules.rs".into(),
            starter_code: r#"// WAF Signature Inspection Pattern
pub fn detect_sqli_or_xss(payload: &str) -> bool {
    let lower = payload.to_lowercase();
    let sqli_patterns = ["' or '", "' or 1=1", "union select", "--", "sleep("];
    let xss_patterns = ["<script", "javascript:", "onerror=", "onload="];

    for pat in sqli_patterns.iter().chain(xss_patterns.iter()) {
        if lower.contains(pat) { return true; }
    }
    false
}
"#.into(),
            documentation: r#"# Layer 7 Reverse Proxy & WAF

Protects web servers from common OWASP Top 10 exploits in real time.
"#.into(),
        },
    ]
}
