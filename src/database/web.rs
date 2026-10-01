use crate::models::{Difficulty, Domain, ProjectIdea};

pub fn get_projects() -> Vec<ProjectIdea> {
    vec![
        // 1. Beginner Web 1
        ProjectIdea {
            id: "web-beg-1".into(),
            title: "Interactive Markdown Note Keeper".into(),
            domain: Domain::WebDev,
            difficulty: Difficulty::Beginner,
            description: "A sleek in-browser note-taking application featuring live Markdown preview, word counters, local storage persistence, and tag filtering.".into(),
            requirements: vec![
                "Split-pane layout with live markdown parsing".into(),
                "Persist notes across browser sessions using localStorage".into(),
                "Tagging system to organize and filter notes".into(),
                "Export notes as .md or .html files".into(),
                "Dark / Light theme toggle".into(),
            ],
            technologies: vec!["HTML5".into(), "CSS3 (Flexbox/Grid)".into(), "Vanilla JavaScript".into(), "Marked.js".into()],
            duration: "6 - 10 hours".into(),
            steps: vec![
                "Step 1: Set up responsive split-pane UI layout (editor on left, preview on right).".into(),
                "Step 2: Integrate a lightweight markdown parser CDN (e.g. marked.js).".into(),
                "Step 3: Implement real-time input event listeners to re-render preview with debounce.".into(),
                "Step 4: Build localStorage serialization for note CRUD operations and tag indexing.".into(),
                "Step 5: Add download blob functionality to export note files.".into(),
                "Step 6: Polish UI with smooth transitions and keyboard shortcuts (Ctrl+S, Ctrl+B).".into(),
            ],
            starter_code_language: "html".into(),
            starter_code_filename: "index.html".into(),
            starter_code: r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Markdown Note Keeper</title>
  <style>
    * { box-sizing: border-box; margin: 0; padding: 0; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
    body { display: flex; height: 100vh; background: #0f172a; color: #f8fafc; }
    #sidebar { width: 260px; background: #1e293b; padding: 1.5rem; display: flex; flex-direction: column; gap: 1rem; border-right: 1px solid #334155; }
    #editor-pane, #preview-pane { flex: 1; padding: 2rem; overflow-y: auto; }
    #editor-pane textarea { width: 100%; height: 90%; background: #1e293b; color: #e2e8f0; border: 1px solid #334155; border-radius: 8px; padding: 1rem; resize: none; font-family: monospace; font-size: 14px; }
    #preview-pane { background: #0f172a; border-left: 1px solid #334155; }
    button { background: #3b82f6; color: white; border: none; padding: 0.6rem 1rem; border-radius: 6px; cursor: pointer; font-weight: 600; }
    button:hover { background: #2563eb; }
  </style>
</head>
<body>
  <div id="sidebar">
    <h2>📝 NoteKeeper</h2>
    <button onclick="createNewNote()">+ New Note</button>
    <div id="notes-list"></div>
  </div>
  <div id="editor-pane">
    <textarea id="markdown-input" placeholder="Type your markdown here..."></textarea>
  </div>
  <div id="preview-pane">
    <div id="preview-output"></div>
  </div>
  <script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>
  <script>
    const input = document.getElementById('markdown-input');
    const output = document.getElementById('preview-output');
    input.addEventListener('input', () => {
      output.innerHTML = marked.parse(input.value);
      localStorage.setItem('saved_note', input.value);
    });
    input.value = localStorage.getItem('saved_note') || '# Welcome to NoteKeeper\n\nStart typing markdown...';
    output.innerHTML = marked.parse(input.value);
  </script>
</body>
</html>"#.into(),
            documentation: r#"# Interactive Markdown Note Keeper

## Overview
A lightweight, fast, client-side Markdown note taking application with persistent local storage.

## Architecture & Data Flow
1. User types in the `<textarea>` input pane.
2. An input event fires with debounce; marked parses raw markdown to sanitized HTML.
3. The preview pane innerHTML updates synchronously.
4. Auto-save triggers saving state into `localStorage`.

## Setup & Running
1. Save `index.html` locally.
2. Open `index.html` in any modern web browser (Chrome, Firefox, Safari).
3. No build tools or backend server needed!

## Deployment
- Drag and drop into GitHub Pages, Netlify Drop, or Cloudflare Pages.
"#.into(),
        },

        // 2. Beginner Web 2
        ProjectIdea {
            id: "web-beg-2".into(),
            title: "Dynamic Flashcard Quiz Trainer".into(),
            domain: Domain::WebDev,
            difficulty: Difficulty::Beginner,
            description: "Spaced-repetition flashcard web application featuring 3D card-flip animations, custom deck creators, and score tracking.".into(),
            requirements: vec![
                "CSS 3D perspective flip effect upon clicking cards".into(),
                "Card creation form with front/back text and category tags".into(),
                "Quiz mode with 'Know it' and 'Review again' buckets".into(),
                "Progress bar and session statistics summary".into(),
            ],
            technologies: vec!["HTML5".into(), "CSS3 3D Transforms".into(), "JavaScript ES6+".into()],
            duration: "5 - 8 hours".into(),
            steps: vec![
                "Step 1: Construct 3D card flipping CSS with transform-style: preserve-3d and backface-visibility: hidden.".into(),
                "Step 2: Build state manager for decks array and current question pointer.".into(),
                "Step 3: Implement grading algorithm (Know it vs Try again) updating review queue.".into(),
                "Step 4: Add modal dialog to create custom cards and export deck JSON.".into(),
            ],
            starter_code_language: "html".into(),
            starter_code_filename: "flashcards.html".into(),
            starter_code: r#"<!DOCTYPE html>
<html>
<head>
  <style>
    body { background: #111827; color: white; display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; font-family: sans-serif; }
    .card-container { perspective: 1000px; width: 340px; height: 220px; cursor: pointer; }
    .card { width: 100%; height: 100%; transform-style: preserve-3d; transition: transform 0.6s cubic-bezier(0.4, 0, 0.2, 1); position: relative; }
    .card.flipped { transform: rotateY(180deg); }
    .face { position: absolute; width: 100%; height: 100%; backface-visibility: hidden; display: flex; align-items: center; justify-content: center; border-radius: 12px; font-size: 1.25rem; padding: 1.5rem; text-align: center; box-shadow: 0 10px 25px rgba(0,0,0,0.5); }
    .front { background: linear-gradient(135deg, #6366f1, #3b82f6); }
    .back { background: linear-gradient(135deg, #10b981, #059669); transform: rotateY(180deg); }
    .controls { margin-top: 2rem; display: flex; gap: 1rem; }
    button { padding: 0.75rem 1.5rem; border: none; border-radius: 8px; font-weight: bold; cursor: pointer; }
  </style>
</head>
<body>
  <div class="card-container" onclick="document.querySelector('.card').classList.toggle('flipped')">
    <div class="card">
      <div class="face front">What is Closure in JavaScript?</div>
      <div class="face back">A function bundled together with references to its surrounding lexical environment.</div>
    </div>
  </div>
  <div class="controls">
    <button style="background:#ef4444; color:white;" onclick="alert('Added to review queue')">Needs Review</button>
    <button style="background:#10b981; color:white;" onclick="alert('Mastered!')">Got it!</button>
  </div>
</body>
</html>"#.into(),
            documentation: r#"# Dynamic Flashcard Quiz Trainer

## Features
- Hardware-accelerated CSS 3D flip card interactions.
- Study session state management with deck queues.
- Lightweight and easily embeddable into any web page.
"#.into(),
        },

        // 3. Beginner Web 3
        ProjectIdea {
            id: "web-beg-3".into(),
            title: "Real-time Currency & Crypto Converter".into(),
            domain: Domain::WebDev,
            difficulty: Difficulty::Beginner,
            description: "Currency converter that fetches public exchange rates and crypto prices, with historical conversion caching and exchange fee calculation.".into(),
            requirements: vec![
                "Fetch live exchange rates from a public API (e.g. exchangerate-api or CoinGecko)".into(),
                "Support dual-way currency conversions with real-time recalculation".into(),
                "Cache rates in sessionStorage to minimize API rate limit exhaustion".into(),
                "Customizable conversion fee slider and percentage margin display".into(),
            ],
            technologies: vec!["HTML5".into(), "CSS3".into(), "JavaScript (Fetch API, Async/Await)".into()],
            duration: "4 - 7 hours".into(),
            steps: vec![
                "Step 1: Set up responsive form with source currency, target currency, and amount inputs.".into(),
                "Step 2: Connect to a free exchange rates API using async/await.".into(),
                "Step 3: Implement rate caching with timestamp validation (cache for 1 hour).".into(),
                "Step 4: Add visual currency flags and conversion history table.".into(),
            ],
            starter_code_language: "html".into(),
            starter_code_filename: "converter.html".into(),
            starter_code: r#"<!DOCTYPE html>
<html>
<head>
  <style>
    body { font-family: system-ui; background: #090d16; color: #fff; display: grid; place-content: center; height: 100vh; }
    .box { background: #131b2e; padding: 2rem; border-radius: 1rem; border: 1px solid #1e293b; width: 320px; }
    input, select { width: 100%; padding: 0.75rem; margin: 0.5rem 0 1rem; border-radius: 0.5rem; background: #0b0f19; border: 1px solid #334155; color: #fff; box-sizing: border-box; }
    .result { font-size: 1.5rem; color: #38bdf8; font-weight: bold; text-align: center; margin-top: 1rem; }
  </style>
</head>
<body>
  <div class="box">
    <h3>💱 Currency Converter</h3>
    <input id="amount" type="number" value="100" />
    <select id="currency">
      <option value="EUR">USD to EUR (0.92)</option>
      <option value="GBP">USD to GBP (0.78)</option>
      <option value="JPY">USD to JPY (154.5)</option>
    </select>
    <div class="result" id="res">Output: €92.00</div>
  </div>
  <script>
    const rates = { EUR: 0.92, GBP: 0.78, JPY: 154.5 };
    function update() {
      const amt = parseFloat(document.getElementById('amount').value) || 0;
      const cur = document.getElementById('currency').value;
      document.getElementById('res').innerText = (amt * rates[cur]).toFixed(2) + ' ' + cur;
    }
    document.getElementById('amount').addEventListener('input', update);
    document.getElementById('currency').addEventListener('change', update);
  </script>
</body>
</html>"#.into(),
            documentation: r#"# Real-time Currency Converter

Simple single-file currency converter with zero external runtime dependencies.
"#.into(),
        },

        // 4. Intermediate Web 1
        ProjectIdea {
            id: "web-int-1".into(),
            title: "Collaborative Kanban Board with Drag & Drop".into(),
            domain: Domain::WebDev,
            difficulty: Difficulty::Intermediate,
            description: "A Trello-style project management board with drag-and-drop task cards, swimlanes (Todo, In-Progress, Done), subtasks, and board state exports.".into(),
            requirements: vec![
                "HTML5 Drag and Drop API or PointerEvents for fluid reordering".into(),
                "CRUD operations for columns and cards with color-coded priority labels".into(),
                "Filter and search cards by keyword, assignee, and tag".into(),
                "Export / import board configuration as structured JSON".into(),
                "Undo / Redo history tracking for card moves".into(),
            ],
            technologies: vec!["Vanilla JS or TypeScript".into(), "CSS Grid / Flexbox".into(), "HTML5 Drag and Drop API".into(), "LocalStorage".into()],
            duration: "15 - 25 hours".into(),
            steps: vec![
                "Step 1: Model board domain schema (Board -> Columns -> Cards -> Tasks).".into(),
                "Step 2: Build card draggable handlers (dragstart, dragover, drop, dragend).".into(),
                "Step 3: Implement column dropzones with placeholder drop indicators.".into(),
                "Step 4: Create card detail modal with markdown description and checklist.".into(),
                "Step 5: Implement state history stack (Mementos) for undo/redo support.".into(),
                "Step 6: Add JSON backup export and import validation.".into(),
            ],
            starter_code_language: "html".into(),
            starter_code_filename: "kanban.html".into(),
            starter_code: r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Kanban Flow</title>
  <style>
    body { margin: 0; font-family: -apple-system, sans-serif; background: #0f172a; color: #f8fafc; padding: 2rem; }
    .board { display: flex; gap: 1.5rem; align-items: flex-start; }
    .column { background: #1e293b; border-radius: 12px; width: 300px; padding: 1rem; border: 1px solid #334155; }
    .column h3 { margin-top: 0; font-size: 1.1rem; color: #94a3b8; }
    .card-list { min-height: 150px; display: flex; flex-direction: column; gap: 0.75rem; }
    .card { background: #334155; padding: 1rem; border-radius: 8px; cursor: grab; user-select: none; border-left: 4px solid #38bdf8; }
    .card.dragging { opacity: 0.4; }
  </style>
</head>
<body>
  <h1>📋 Project Kanban Board</h1>
  <div class="board">
    <div class="column" ondragover="event.preventDefault()" ondrop="drop(event, this)">
      <h3>To Do</h3>
      <div class="card-list">
        <div class="card" draggable="true" ondragstart="drag(event)" id="c1">Implement Authentication</div>
        <div class="card" draggable="true" ondragstart="drag(event)" id="c2">Design Database Schema</div>
      </div>
    </div>
    <div class="column" ondragover="event.preventDefault()" ondrop="drop(event, this)">
      <h3>In Progress</h3>
      <div class="card-list">
        <div class="card" draggable="true" ondragstart="drag(event)" id="c3">API Endpoint Tests</div>
      </div>
    </div>
    <div class="column" ondragover="event.preventDefault()" ondrop="drop(event, this)">
      <h3>Done</h3>
      <div class="card-list">
        <div class="card" draggable="true" ondragstart="drag(event)" id="c4">Project Setup</div>
      </div>
    </div>
  </div>
  <script>
    function drag(ev) { ev.dataTransfer.setData("text", ev.target.id); ev.target.classList.add('dragging'); }
    document.addEventListener('dragend', (ev) => ev.target.classList.remove('dragging'));
    function drop(ev, col) {
      ev.preventDefault();
      const id = ev.dataTransfer.getData("text");
      const card = document.getElementById(id);
      col.querySelector('.card-list').appendChild(card);
    }
  </script>
</body>
</html>"#.into(),
            documentation: r#"# Collaborative Kanban Board

## Architecture
- Native drag and drop lifecycle management without external heavier libraries.
- State-driven card rendering.
- Extensible to WebSocket backend for multi-user real-time synchronization.
"#.into(),
        },

        // 5. Intermediate Web 2
        ProjectIdea {
            id: "web-int-2".into(),
            title: "WebRTC P2P Video & Screen Sharing Room".into(),
            domain: Domain::WebDev,
            difficulty: Difficulty::Intermediate,
            description: "Peer-to-peer browser video calling app with audio/video toggles, screen sharing capabilities, and peer mesh connectivity.".into(),
            requirements: vec![
                "MediaStream API for camera, microphone, and desktop capture".into(),
                "RTCPeerConnection for encrypted low-latency P2P audio/video delivery".into(),
                "Signaling mechanism via lightweight WebSocket or Firebase channel".into(),
                "Mute/unmute, camera flip, and bandwidth diagnostic stats overlay".into(),
            ],
            technologies: vec!["WebRTC API".into(), "HTML5 Video".into(), "JavaScript (Async/Await)".into(), "WebSocket".into()],
            duration: "20 - 30 hours".into(),
            steps: vec![
                "Step 1: Request media device permissions using navigator.mediaDevices.getUserMedia.".into(),
                "Step 2: Implement local video preview playback.".into(),
                "Step 3: Setup RTCPeerConnection offer/answer exchange lifecycle.".into(),
                "Step 4: Handle ICE candidate discovery and exchange.".into(),
                "Step 5: Integrate screen sharing via getDisplayMedia.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "webrtc_room.js".into(),
            starter_code: r#"// WebRTC Peer Connection Core Logic
const config = { iceServers: [{ urls: 'stun:stun.l.google.com:19302' }] };
let localStream, peerConn;

async function startLocalStream() {
  localStream = await navigator.mediaDevices.getUserMedia({ video: true, audio: true });
  document.getElementById('localVideo').srcObject = localStream;
}

async function createOffer(signalingSend) {
  peerConn = new RTCPeerConnection(config);
  localStream.getTracks().forEach(track => peerConn.addTrack(track, localStream));
  
  peerConn.ontrack = (event) => {
    document.getElementById('remoteVideo').srcObject = event.streams[0];
  };
  
  peerConn.onicecandidate = (event) => {
    if (event.candidate) signalingSend({ type: 'ice', candidate: event.candidate });
  };
  
  const offer = await peerConn.createOffer();
  await peerConn.setLocalDescription(offer);
  signalingSend({ type: 'offer', sdp: offer });
}
"#.into(),
            documentation: r#"# WebRTC P2P Room

Connects two browser tabs directly over STUN/WebRTC for ultra-low latency audio/video.
"#.into(),
        },

        // 6. Intermediate Web 3
        ProjectIdea {
            id: "web-int-3".into(),
            title: "Algorithmic Pathfinding & Maze Visualizer".into(),
            domain: Domain::WebDev,
            difficulty: Difficulty::Intermediate,
            description: "Interactive visualizer for Dijkstra, A*, BFS, and DFS graph traversal algorithms with wall drawing and maze generation.".into(),
            requirements: vec![
                "Interactive grid where cells can be clicked/dragged to draw obstacle walls".into(),
                "Step-by-step animation queue rendering visited nodes and shortest path".into(),
                "Algorithm speed slider, weight node support, and heuristic selection".into(),
                "Maze generators: Recursive Division and Random Walk".into(),
            ],
            technologies: vec!["HTML5 Canvas or CSS Grid".into(), "JavaScript ES6 Classes".into(), "Graph Algorithms".into()],
            duration: "12 - 18 hours".into(),
            steps: vec![
                "Step 1: Build 2D grid matrix state representation.".into(),
                "Step 2: Implement BFS and DFS queue/stack traversal routines.".into(),
                "Step 3: Implement A* with Manhattan distance heuristic priority queue.".into(),
                "Step 4: Create asynchronous delay ticker to animate exploration front.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "pathfinder.js".into(),
            starter_code: r#"// A* Pathfinding Visualizer Core
class GridNode {
  constructor(r, c) {
    this.r = r; this.c = c;
    this.g = Infinity; this.h = 0; this.f = Infinity;
    this.wall = false; this.parent = null;
  }
}

function manhattan(a, b) {
  return Math.abs(a.r - b.r) + Math.abs(a.c - b.c);
}

function solveAStar(grid, start, end) {
  const openSet = [start];
  start.g = 0;
  start.f = manhattan(start, end);
  
  while (openSet.length > 0) {
    openSet.sort((a, b) => a.f - b.f);
    const curr = openSet.shift();
    if (curr === end) return reconstructPath(end);
    
    // Check 4-directional neighbors...
  }
  return [];
}
function reconstructPath(node) {
  const path = [];
  let curr = node;
  while (curr) { path.unshift(curr); curr = curr.parent; }
  return path;
}
"#.into(),
            documentation: r#"# Algorithmic Pathfinding Visualizer

A learning tool to demonstrate graph search algorithms step by step in real time.
"#.into(),
        },

        // 7. Advanced Web 1
        ProjectIdea {
            id: "web-adv-1".into(),
            title: "In-Browser Code Sandbox & AST Analyzer".into(),
            domain: Domain::WebDev,
            difficulty: Difficulty::Advanced,
            description: "High-performance web IDE with Monaco editor, WebAssembly syntax parsing, in-memory virtual filesystem, and sandboxed iframe runtime.".into(),
            requirements: vec![
                "Virtual file tree with multi-tab code editor and unsaved state indicators".into(),
                "Sandboxed WebWorker / iframe execution with hijacked console.log stream".into(),
                "Real-time Abstract Syntax Tree (AST) visualization panel".into(),
                "Support importing external ES modules dynamically via CDN (esm.sh)".into(),
                "Performance profiler measuring memory and frame execution rates".into(),
            ],
            technologies: vec!["TypeScript".into(), "Monaco Editor".into(), "Web Workers".into(), "Acorn / Babel AST".into(), "WebAssembly".into()],
            duration: "40 - 60 hours".into(),
            steps: vec![
                "Step 1: Integrate Monaco Editor or CodeMirror 6 with syntax themes and autocomplete.".into(),
                "Step 2: Build Virtual File System (VFS) in memory supporting folder nesting and imports.".into(),
                "Step 3: Construct sandboxed execution worker with postMessage RPC channel.".into(),
                "Step 4: Implement Babel/Acorn parser to generate and draw AST node tree graph.".into(),
                "Step 5: Bundle and execute multi-file user code using an in-browser bundler.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "sandbox_runner.js".into(),
            starter_code: r#"// Sandboxed In-Memory Execution Engine
class SandboxRunner {
  constructor(containerIframe) {
    this.iframe = containerIframe;
  }

  execute(jsCode) {
    const wrapper = `
      <!DOCTYPE html>
      <html>
      <head>
        <script>
          const _log = console.log;
          console.log = (...args) => {
            window.parent.postMessage({ type: 'CONSOLE_LOG', data: args.map(String).join(' ') }, '*');
            _log.apply(console, args);
          };
          window.onerror = (msg, url, line) => {
            window.parent.postMessage({ type: 'CONSOLE_ERR', data: msg + ' (line ' + line + ')' }, '*');
          };
        </script>
      </head>
      <body>
        <script>
          try {
            ${jsCode}
          } catch(e) {
            console.error(e.message);
          }
        </script>
      </body>
      </html>
    `;
    this.iframe.srcdoc = wrapper;
  }
}
"#.into(),
            documentation: r#"# In-Browser Code Sandbox & AST Analyzer

## Security & Architecture
- Code execution is isolated within sandboxed iframes without `allow-same-origin` or top-level navigation access.
- Console methods are patched to redirect outputs back to the developer HUD.
"#.into(),
        },

        // 8. Advanced Web 2
        ProjectIdea {
            id: "web-adv-2".into(),
            title: "CRDT-Powered Real-time Collaborative Document Editor".into(),
            domain: Domain::WebDev,
            difficulty: Difficulty::Advanced,
            description: "Google Docs-like rich text editor powered by Conflict-free Replicated Data Types (CRDTs), supporting offline editing and peer sync.".into(),
            requirements: vec![
                "CRDT implementation (Yjs or custom RGA/LWW-Element-Set) resolving concurrent edits".into(),
                "Presence indicators showing real-time remote user cursor positions and selections".into(),
                "Offline-first architecture with IndexedDB persistence and auto-reconciliation".into(),
                "Version history with interactive time-travel scrub slider".into(),
            ],
            technologies: vec!["TypeScript".into(), "CRDTs (Yjs / Automerge)".into(), "WebSocket / WebRTC".into(), "IndexedDB".into()],
            duration: "45 - 65 hours".into(),
            steps: vec![
                "Step 1: Understand Lamport timestamps, state vectors, and CRDT character sequences.".into(),
                "Step 2: Bind CRDT text structure to Quill / ProseMirror / Slate editor model.".into(),
                "Step 3: Build WebRTC signaling server for direct P2P state vector diffing.".into(),
                "Step 4: Implement awareness protocol for user cursor markers and colored name tags.".into(),
                "Step 5: Store operation logs in IndexedDB for resilient offline-to-online sync.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "crdt_doc.js".into(),
            starter_code: r#"// Basic Fractional Indexing CRDT Concept
class FractionalCRDTDoc {
  constructor(siteId) {
    this.siteId = siteId;
    this.chars = []; // [{ id: '1.5', val: 'a', siteId: 'node-1' }]
  }

  insert(index, val) {
    const prevPos = index > 0 ? this.chars[index - 1].pos : 0;
    const nextPos = index < this.chars.length ? this.chars[index].pos : prevPos + 2;
    const newPos = (prevPos + nextPos) / 2;
    const item = { pos: newPos, val, site: this.siteId };
    this.chars.splice(index, 0, item);
    return item;
  }

  mergeRemote(item) {
    const idx = this.chars.findIndex(c => c.pos > item.pos);
    if (idx === -1) this.chars.push(item);
    else this.chars.splice(idx, 0, item);
  }

  getText() {
    return this.chars.map(c => c.val).join('');
  }
}
"#.into(),
            documentation: r#"# CRDT Collaborative Document Editor

Mathematical convergence guarantee ensuring no user edits are ever lost even during network partitions.
"#.into(),
        },

        // 9. Advanced Web 3
        ProjectIdea {
            id: "web-adv-3".into(),
            title: "3D Procedural Solar System & Physics Engine".into(),
            domain: Domain::WebDev,
            difficulty: Difficulty::Advanced,
            description: "High-performance Three.js / WebGL interactive 3D universe with N-body Newtonian gravitational simulation and planetary orbit trackers.".into(),
            requirements: vec![
                "Custom GLSL shaders for atmospheric glow, sun solar flares, and planet rings".into(),
                "N-body gravitational simulation computed with Runge-Kutta 4th order integrator".into(),
                "Smooth cinematic camera transitions with target tracking and free-fly controls".into(),
                "Procedural texture generation using Simplex / Perlin noise for planet surfaces".into(),
            ],
            technologies: vec!["Three.js / WebGL".into(), "GLSL Shaders".into(), "Web Workers for Physics".into(), "JavaScript / Math3D".into()],
            duration: "35 - 50 hours".into(),
            steps: vec![
                "Step 1: Set up Three.js perspective scene, lighting, and OrbitControls.".into(),
                "Step 2: Implement Newtonian gravity equations (F = G * m1 * m2 / r^2) between orbital bodies.".into(),
                "Step 3: Offload N-body integrator to Web Worker to keep 60 FPS UI rendering.".into(),
                "Step 4: Write vertex and fragment shaders for planet atmospheric Rayleigh scattering.".into(),
                "Step 5: Add time-scale scrub controls (0.1x to 1000x acceleration).".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "solar_system.js".into(),
            starter_code: r#"// Gravitational Physics Simulation Step (RK4 / Euler)
class CelestialBody {
  constructor(name, mass, x, y, vx, vy) {
    this.name = name; this.mass = mass;
    this.x = x; this.y = y;
    this.vx = vx; this.vy = vy;
  }
}

function updateGravity(bodies, dt, G = 6.674e-11) {
  for (let i = 0; i < bodies.length; i++) {
    let fx = 0, fy = 0;
    for (let j = 0; j < bodies.length; j++) {
      if (i === j) continue;
      const dx = bodies[j].x - bodies[i].x;
      const dy = bodies[j].y - bodies[i].y;
      const dist = Math.sqrt(dx*dx + dy*dy) + 1e-4;
      const force = (G * bodies[i].mass * bodies[j].mass) / (dist * dist);
      fx += force * (dx / dist);
      fy += force * (dy / dist);
    }
    bodies[i].vx += (fx / bodies[i].mass) * dt;
    bodies[i].vy += (fy / bodies[i].mass) * dt;
    bodies[i].x += bodies[i].vx * dt;
    bodies[i].y += bodies[i].vy * dt;
  }
}
"#.into(),
            documentation: r#"# 3D Procedural Solar System

Simulate orbital mechanics and relativistic effects in real time inside the browser.
"#.into(),
        },

        // 10. Intermediate Web 4
        ProjectIdea {
            id: "web-int-4".into(),
            title: "Audio Synthesizer & Beat Sequencer DAW".into(),
            domain: Domain::WebDev,
            difficulty: Difficulty::Intermediate,
            description: "Browser-based digital audio workstation (DAW) with Web Audio API synthesizers, 16-step drum machine, ADSR envelope shaping, and audio export.".into(),
            requirements: vec![
                "Web Audio API AudioContext with oscillator nodes, gain nodes, and biquad filters".into(),
                "16-step grid sequencer with BPM tempo clock and swing control".into(),
                "Interactive ADSR (Attack, Decay, Sustain, Release) envelope visualizer".into(),
                "WAV file audio recorder and download exporter".into(),
            ],
            technologies: vec!["Web Audio API".into(), "HTML5 Canvas".into(), "JavaScript AudioNodes".into()],
            duration: "18 - 26 hours".into(),
            steps: vec![
                "Step 1: Initialize AudioContext and build playable oscillator synth keys.".into(),
                "Step 2: Construct ADSR gain envelope curves on key triggers.".into(),
                "Step 3: Build Web Audio lookahead scheduling timer loop for precision rhythm.".into(),
                "Step 4: Draw live oscilloscope waveform on HTML5 canvas.".into(),
                "Step 5: Record audio stream into downloadable PCM WAV.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "synth_engine.js".into(),
            starter_code: r#"// Web Audio Polyphonic Synth
const audioCtx = new (window.AudioContext || window.webkitAudioContext)();

function playTone(freq, duration = 0.5) {
  const osc = audioCtx.createOscillator();
  const gain = audioCtx.createGain();
  
  osc.type = 'sawtooth';
  osc.frequency.setValueAtTime(freq, audioCtx.currentTime);
  
  // ADSR
  gain.gain.setValueAtTime(0, audioCtx.currentTime);
  gain.gain.linearRampToValueAtTime(0.5, audioCtx.currentTime + 0.05); // Attack
  gain.gain.exponentialRampToValueAtTime(0.001, audioCtx.currentTime + duration); // Decay
  
  osc.connect(gain);
  gain.connect(audioCtx.destination);
  
  osc.start();
  osc.stop(audioCtx.currentTime + duration);
}
"#.into(),
            documentation: r#"# Web Audio Synthesizer

Pure JavaScript synthesizer generating custom frequencies and timbres with no third-party audio libraries.
"#.into(),
        },
    ]
}
