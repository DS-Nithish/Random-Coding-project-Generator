use crate::models::{Difficulty, Domain, ProjectIdea};

pub fn get_projects() -> Vec<ProjectIdea> {
    vec![
        // 1. Beginner Game 1
        ProjectIdea {
            id: "game-beg-1".into(),
            title: "Retro Arcade Space Invaders / Asteroids".into(),
            domain: Domain::GameDev,
            difficulty: Difficulty::Beginner,
            description: "Classic 2D space shooter with fluid ship physics, inertia momentum, asteroid splitting on hit, particle explosion sparks, and high score tracking.".into(),
            requirements: vec![
                "Player spaceship controls: rotate, thrust forward with acceleration/decay, and shoot".into(),
                "Screen edge wrapping (toroidal space) for ship and asteroids".into(),
                "Circle-to-circle or Axis-Aligned Bounding Box (AABB) collision detection".into(),
                "Splitting larger asteroids into two smaller faster rocks on bullet impact".into(),
                "Retro particle effect bursts and score HUD".into(),
            ],
            technologies: vec!["HTML5 Canvas / Vanilla JS or Python Pygame".into(), "2D Trigonometry & Vectors".into(), "RequestAnimationFrame".into()],
            duration: "6 - 10 hours".into(),
            steps: vec![
                "Step 1: Set up game canvas loop with fixed time-step update and render cycles.".into(),
                "Step 2: Implement 2D vector class for position, velocity, and angular rotation.".into(),
                "Step 3: Spawn asteroid rocks with random vectors and screen wrapping math.".into(),
                "Step 4: Check bullet-asteroid intersection distance (radius check).".into(),
                "Step 5: Add particle burst emitter for explosions and game over state.".into(),
            ],
            starter_code_language: "html".into(),
            starter_code_filename: "asteroids.html".into(),
            starter_code: r#"<!DOCTYPE html>
<html>
<head>
  <style>
    body { margin: 0; background: #000; overflow: hidden; display: flex; justify-content: center; align-items: center; height: 100vh; font-family: monospace; color: #fff; }
    canvas { border: 2px solid #38bdf8; box-shadow: 0 0 20px #0284c7; }
  </style>
</head>
<body>
  <canvas id="gc" width="800" height="600"></canvas>
  <script>
    const canvas = document.getElementById('gc');
    const ctx = canvas.getContext('2d');
    let ship = { x: 400, y: 300, r: 15, a: 0, rot: 0, thrust: { x: 0, y: 0 } };
    let bullets = [];
    const keys = {};

    window.onkeydown = e => keys[e.code] = true;
    window.onkeyup = e => keys[e.code] = false;

    function loop() {
      // Rotate & Thrust
      if (keys['ArrowLeft'] || keys['KeyA']) ship.a -= 0.07;
      if (keys['ArrowRight'] || keys['KeyD']) ship.a += 0.07;
      if (keys['ArrowUp'] || keys['KeyW']) {
        ship.thrust.x += Math.cos(ship.a) * 0.15;
        ship.thrust.y += Math.sin(ship.a) * 0.15;
      }
      if (keys['Space']) {
        bullets.push({ x: ship.x, y: ship.y, vx: Math.cos(ship.a) * 7, vy: Math.sin(ship.a) * 7, life: 60 });
        keys['Space'] = false; // fire once per tap
      }

      ship.x = (ship.x + ship.thrust.x + canvas.width) % canvas.width;
      ship.y = (ship.y + ship.thrust.y + canvas.height) % canvas.height;
      ship.thrust.x *= 0.98;
      ship.thrust.y *= 0.98;

      // Draw
      ctx.fillStyle = '#05070e';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // Ship
      ctx.strokeStyle = '#38bdf8';
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.moveTo(ship.x + ship.r * Math.cos(ship.a), ship.y + ship.r * Math.sin(ship.a));
      ctx.lineTo(ship.x + ship.r * Math.cos(ship.a + 2.5), ship.y + ship.r * Math.sin(ship.a + 2.5));
      ctx.lineTo(ship.x + ship.r * Math.cos(ship.a - 2.5), ship.y + ship.r * Math.sin(ship.a - 2.5));
      ctx.closePath();
      ctx.stroke();

      // Bullets
      ctx.fillStyle = '#f43f5e';
      bullets.forEach((b, i) => {
        b.x += b.vx; b.y += b.vy; b.life--;
        ctx.fillRect(b.x - 2, b.y - 2, 4, 4);
      });
      bullets = bullets.filter(b => b.life > 0);

      requestAnimationFrame(loop);
    }
    requestAnimationFrame(loop);
  </script>
</body>
</html>"#.into(),
            documentation: r#"# Retro Arcade Space Asteroids

A pure HTML5 canvas game demonstrating inertia mechanics, vector velocity, and bullet management.
"#.into(),
        },

        // 2. Beginner Game 2
        ProjectIdea {
            id: "game-beg-2".into(),
            title: "Grid-Based Roguelike Dungeon Crawler".into(),
            domain: Domain::GameDev,
            difficulty: Difficulty::Beginner,
            description: "Turn-based ASCII / tile dungeon exploration game with procedural room generation, fog-of-war vision field, enemy AI chasing, and inventory loot.".into(),
            requirements: vec![
                "Turn-based movement on a discrete 2D grid matrix".into(),
                "Random dungeon room and corridor carving generator".into(),
                "Field-of-View (FOV) raycasting or shadowcasting reveal system".into(),
                "Enemy monsters that pathfind toward player when spotted".into(),
                "HP bars, combat calculation, and health potion pickups".into(),
            ],
            technologies: vec!["Python (Curses / Pygame) or HTML5 Canvas / Rust".into(), "Shadowcasting FOV".into(), "Bresenham Line Algorithm".into()],
            duration: "8 - 14 hours".into(),
            steps: vec![
                "Step 1: Create 2D tile map structure (0 = wall, 1 = floor).".into(),
                "Step 2: Generate rooms with Random Carving or Binary Space Partitioning.".into(),
                "Step 3: Implement player turn-based inputs and collision detection against walls.".into(),
                "Step 4: Compute visibility shadowcasting to hide unexplored tiles.".into(),
                "Step 5: Add enemy AI turns and attack combat resolution.".into(),
            ],
            starter_code_language: "python".into(),
            starter_code_filename: "roguelike.py".into(),
            starter_code: r#"# Simple Terminal Roguelike in Python
import random

WIDTH, HEIGHT = 40, 15
map_grid = [['#' for _ in range(WIDTH)] for _ in range(HEIGHT)]

# Carve a simple room
for y in range(2, HEIGHT - 2):
    for x in range(2, WIDTH - 2):
        map_grid[y][x] = '.'

player = {'x': 5, 'y': 5, 'hp': 20}
goblin = {'x': 25, 'y': 8, 'hp': 10}

def render():
    for y in range(HEIGHT):
        line = ""
        for x in range(WIDTH):
            if x == player['x'] and y == player['y']: line += '@'
            elif x == goblin['x'] and y == goblin['y']: line += 'g'
            else: line += map_grid[y][x]
        print(line)
    print(f"Player HP: {player['hp']} | Controls: w/a/s/d + Enter (q to quit)")

if __name__ == '__main__':
    while True:
        render()
        cmd = input("Action: ").strip().lower()
        if cmd == 'q': break
        dx, dy = 0, 0
        if cmd == 'w': dy = -1
        elif cmd == 's': dy = 1
        elif cmd == 'a': dx = -1
        elif cmd == 'd': dx = 1
        nx, ny = player['x'] + dx, player['y'] + dy
        if map_grid[ny][nx] == '.':
            player['x'], player['y'] = nx, ny
"#.into(),
            documentation: r#"# Grid-Based Roguelike Dungeon Crawler

Turn-based tactical dungeon crawling engine running anywhere.
"#.into(),
        },

        // 3. Beginner Game 3
        ProjectIdea {
            id: "game-beg-3".into(),
            title: "Physics-Based Marble Balance Labyrinth".into(),
            domain: Domain::GameDev,
            difficulty: Difficulty::Beginner,
            description: "Tilt a wooden maze labyrinth to guide a rolling marble into the goal cup while dodging hole hazards, with realistic friction, restitution, and tilt physics.".into(),
            requirements: vec![
                "Tilt physics simulation driven by arrow keys or device gyroscope".into(),
                "Circle-to-line segment collision detection and bouncy reflection".into(),
                "Hazard holes that suck the marble in if close enough".into(),
                "Timer clock, sound effects, and level progression".into(),
            ],
            technologies: vec!["JavaScript / Canvas or Matter.js / Pygame".into(), "Rigid Body Physics".into(), "DeviceOrientation API".into()],
            duration: "6 - 10 hours".into(),
            steps: vec![
                "Step 1: Define maze wall segments as coordinate line pairs [p1, p2].".into(),
                "Step 2: Apply tilt acceleration vector to marble velocity.".into(),
                "Step 3: Calculate point-to-line segment closest distance and bounce normal.".into(),
                "Step 4: Check hole gravitational suction trigger.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "marble_physics.js".into(),
            starter_code: r#"// Marble Physics Core
class Marble {
  constructor(x, y, radius = 10) {
    this.x = x; this.y = y; this.r = radius;
    this.vx = 0; this.vy = 0;
  }
  update(tiltX, tiltY, friction = 0.985) {
    this.vx += tiltX * 0.5;
    this.vy += tiltY * 0.5;
    this.vx *= friction;
    this.vy *= friction;
    this.x += this.vx;
    this.y += this.vy;
  }
  collideWall(minX, maxX, minY, maxY) {
    if (this.x - this.r < minX) { this.x = minX + this.r; this.vx *= -0.7; }
    if (this.x + this.r > maxX) { this.x = maxX - this.r; this.vx *= -0.7; }
    if (this.y - this.r < minY) { this.y = minY + this.r; this.vy *= -0.7; }
    if (this.y + this.r > maxY) { this.y = maxY - this.r; this.vy *= -0.7; }
  }
}
"#.into(),
            documentation: r#"# Marble Balance Labyrinth

Kinematic simulation of rolling sphere with momentum, drag, and restitution.
"#.into(),
        },

        // 4. Intermediate Game 1
        ProjectIdea {
            id: "game-int-1".into(),
            title: "2D Precision Platformer Engine (Celeste-Style)".into(),
            domain: Domain::GameDev,
            difficulty: Difficulty::Intermediate,
            description: "High-precision platformer with variable jump heights, wall jumps, air dashes, coyote time, jump buffering, and moving platforms.".into(),
            requirements: vec![
                "Responsive platformer physics: jump buffering (input queue), coyote time (edge leeway)".into(),
                "Wall sliding and wall-kick impulse physics".into(),
                "Omni-directional 8-way dash with freeze-frame impact feel (screen shake & hitstop)".into(),
                "Tilemap collision system with one-way drop-through platforms".into(),
                "Collectible strawberry gems and death counter respawn loop".into(),
            ],
            technologies: vec!["Rust (Bevy / Macroquad) or Godot / MonoGame / TS Canvas".into(), "Custom Kinematic Controller".into(), "AABB Separating Axis Theorem".into()],
            duration: "20 - 35 hours".into(),
            steps: vec![
                "Step 1: Implement sub-pixel kinematic movement loop with integer sweeps.".into(),
                "Step 2: Add coyote time (allow jumping ~5 frames after leaving an edge).".into(),
                "Step 3: Implement jump buffering (register jump press up to 5 frames before landing).".into(),
                "Step 4: Build wall-grab, slide friction, and wall-hop impulse angles.".into(),
                "Step 5: Code the 8-directional dash with temporary zero-gravity glide.".into(),
                "Step 6: Add screen shake and juice (dust particles, squish animations).".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "platformer_controller.js".into(),
            starter_code: r#"// Precision Platformer Controller with Coyote Time & Jump Buffering
class PlatformerPlayer {
  constructor(x, y) {
    this.x = x; this.y = y; this.vx = 0; this.vy = 0;
    this.isGrounded = false;
    this.coyoteTimer = 0;
    this.jumpBufferTimer = 0;
    this.canDash = true;
  }

  update(dt, input, collidesAt) {
    // Timers
    if (this.isGrounded) this.coyoteTimer = 0.1; else this.coyoteTimer -= dt;
    if (input.jumpPressed) this.jumpBufferTimer = 0.12; else this.jumpBufferTimer -= dt;

    // Horizontal Movement
    const targetVx = input.horizontalAxis * 180;
    const accel = this.isGrounded ? 1500 : 900;
    this.vx = Math.sign(targetVx - this.vx) * Math.min(Math.abs(targetVx - this.vx), accel * dt) + this.vx;

    // Jump Execution
    if (this.jumpBufferTimer > 0 && this.coyoteTimer > 0) {
      this.vy = -380;
      this.jumpBufferTimer = 0;
      this.coyoteTimer = 0;
    }

    // Variable Jump Cut
    if (!input.jumpHeld && this.vy < -100) {
      this.vy *= 0.5; // Short hop
    }

    // Gravity
    this.vy += 1100 * dt;
    if (this.vy > 500) this.vy = 500; // Terminal velocity
  }
}
"#.into(),
            documentation: r#"# 2D Precision Platformer Engine

Engineered with game feel best practices: coyote time, input buffering, and snappy jump arcs.
"#.into(),
        },

        // 5. Intermediate Game 2
        ProjectIdea {
            id: "game-int-2".into(),
            title: "Multiplayer Tank Battle Arena (WebSocket P2P)".into(),
            domain: Domain::GameDev,
            difficulty: Difficulty::Intermediate,
            description: "Real-time top-down 2D multiplayer tank combat game featuring client-side prediction, server reconciliation, destructible walls, and power-up crates.".into(),
            requirements: vec![
                "Low-latency WebSocket game server syncing tank positions, turret rotations, and projectiles".into(),
                "Client-side prediction and server reconciliation to eliminate perceived network lag".into(),
                "Destructible brick wall obstacles and ricocheting bullets".into(),
                "Multiplayer lobby system with room codes and player leaderboards".into(),
            ],
            technologies: vec!["Node.js / Rust (Tokio) Backend".into(), "WebSocket Protocol".into(), "HTML5 Canvas / PixiJS".into()],
            duration: "25 - 38 hours".into(),
            steps: vec![
                "Step 1: Design binary or compact JSON state sync network protocol.".into(),
                "Step 2: Build tick-based server simulation (e.g. 30 Hz tick rate).".into(),
                "Step 3: Implement client input sequence stamping and local optimistic movement.".into(),
                "Step 4: Reconcile client positions upon receiving authoritative server snapshots.".into(),
                "Step 5: Add bullet ricochet physics off concrete obstacles.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "server_tick.js".into(),
            starter_code: r#"// Server Tick Simulation & State Broadcast Loop
const TICK_RATE = 30; // 30 times per second
const players = new Map();
const bullets = [];

function gameTick() {
  // Update all player tanks based on queued inputs
  for (const [id, player] of players.entries()) {
    player.x += player.vx;
    player.y += player.vy;
    // Turret aim angle
  }

  // Update projectiles
  for (let i = bullets.length - 1; i >= 0; i--) {
    const b = bullets[i];
    b.x += b.vx; b.y += b.vy;
    // Check collisions with tanks & walls...
  }

  // Broadcast snapshot
  const snapshot = {
    t: Date.now(),
    players: Array.from(players.values()),
    bullets
  };
  // wsServer.broadcast(snapshot);
}
setInterval(gameTick, 1000 / TICK_RATE);
"#.into(),
            documentation: r#"# Multiplayer Tank Battle Arena

Demonstrates client-side prediction and authoritative network architectures.
"#.into(),
        },

        // 6. Intermediate Game 3
        ProjectIdea {
            id: "game-int-3".into(),
            title: "Tower Defense: Elemental Hex Strategy".into(),
            domain: Domain::GameDev,
            difficulty: Difficulty::Intermediate,
            description: "Hexagonal grid tower defense game featuring elemental damage combinations (Fire + Water = Steam, Ice + Lightning = Shatter), creeping creep waves, and pathfinding.".into(),
            requirements: vec![
                "Hexagonal coordinate system (Axial / Cube coordinates) and click selection".into(),
                "Dynamic creep pathfinding recalculation when towers block roads".into(),
                "Elemental tower upgrades and damage synergy reaction system".into(),
                "Wave spawner with scaling armor and elemental resistances".into(),
            ],
            technologies: vec!["HTML5 Canvas / TypeScript or Godot".into(), "Hex Grid Math".into(), "Breadth-First Search (BFS)".into()],
            duration: "18 - 28 hours".into(),
            steps: vec![
                "Step 1: Implement hex grid cube coordinates (q, r, s where q+r+s=0).".into(),
                "Step 2: Generate road pathway from spawn portal to base crystal.".into(),
                "Step 3: Build tower placement with range radius preview.".into(),
                "Step 4: Implement elemental status effect stackers (Burn, Freeze, Shock).".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "hex_grid.js".into(),
            starter_code: r#"// Hex Grid Axial to Pixel Coordinates Conversion
const HEX_SIZE = 32;

function axialToPixel(q, r) {
  const x = HEX_SIZE * (Math.sqrt(3) * q + Math.sqrt(3)/2 * r);
  const y = HEX_SIZE * (3/2 * r);
  return { x, y };
}

function pixelToAxial(px, py) {
  const q = (Math.sqrt(3)/3 * px - 1/3 * py) / HEX_SIZE;
  const r = (2/3 * py) / HEX_SIZE;
  return axialRound(q, r);
}

function axialRound(q, r) {
  let s = -q - r;
  let rq = Math.round(q);
  let rr = Math.round(r);
  let rs = Math.round(s);
  const dq = Math.abs(rq - q);
  const dr = Math.abs(rr - r);
  const ds = Math.abs(rs - s);
  if (dq > dr && dq > ds) rq = -rr - rs;
  else if (dr > ds) rr = -rq - rs;
  return { q: rq, r: rr };
}
"#.into(),
            documentation: r#"# Tower Defense: Elemental Hex Strategy

Hexagonal spatial math paired with dynamic pathfinding and elemental combat.
"#.into(),
        },

        // 7. Advanced Game 1
        ProjectIdea {
            id: "game-adv-1".into(),
            title: "Custom 3D Voxel Engine (Minecraft-like in Rust)".into(),
            domain: Domain::GameDev,
            difficulty: Difficulty::Advanced,
            description: "High-performance voxel engine featuring chunk generation, Greedy Meshing algorithm to minimize polygon count, Simplex 3D terrain noise, and block breaking/placing.".into(),
            requirements: vec![
                "Chunk management system (16x16x256 voxel chunks loaded dynamically around player)".into(),
                "Greedy Meshing algorithm reducing block faces into merged quad polygons by >90%".into(),
                "Procedural 3D Perlin/Simplex noise generation for caves, mountains, and biomes".into(),
                "3D Raycasting (Amanatides-Woo algorithm) for precise block selection and voxel destruction".into(),
                "Ambient occlusion lighting computed per voxel vertex".into(),
            ],
            technologies: vec!["Rust + WGPU / OpenGL".into(), "Greedy Meshing Algorithm".into(), "3D Noise Generators".into(), "Fast Voxel Raycast (DDA)".into()],
            duration: "50 - 75 hours".into(),
            steps: vec![
                "Step 1: Set up WGPU graphics pipeline with vertex buffers and camera view projection matrix.".into(),
                "Step 2: Generate 3D density noise values to populate chunk block types.".into(),
                "Step 3: Implement Greedy Meshing to merge adjacent matching faces into single large quads.".into(),
                "Step 4: Implement 3D Digital Differential Analyzer (DDA) raycasting for mouse cursor block targeting.".into(),
                "Step 5: Calculate vertex ambient occlusion based on neighboring corner block occupancy.".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "voxel_dda.rs".into(),
            starter_code: r#"// Fast Voxel Traversal Algorithm (Amanatides & Woo DDA)
pub struct Ray { pub origin: [f32; 3], pub dir: [f32; 3] }

pub fn raycast_voxel(ray: &Ray, max_dist: f32) -> Option<[i32; 3]> {
    let mut x = ray.origin[0].floor() as i32;
    let mut y = ray.origin[1].floor() as i32;
    let mut z = ray.origin[2].floor() as i32;

    let step_x = if ray.dir[0] > 0.0 { 1 } else { -1 };
    let step_y = if ray.dir[1] > 0.0 { 1 } else { -1 };
    let step_z = if ray.dir[2] > 0.0 { 1 } else { -1 };

    let t_delta_x = (1.0 / ray.dir[0]).abs();
    let t_delta_y = (1.0 / ray.dir[1]).abs();
    let t_delta_z = (1.0 / ray.dir[2]).abs();

    let mut t_max_x = ((x as f32 + (if step_x > 0 { 1.0 } else { 0.0 })) - ray.origin[0]) / ray.dir[0];
    let mut t_max_y = ((y as f32 + (if step_y > 0 { 1.0 } else { 0.0 })) - ray.origin[1]) / ray.dir[1];
    let mut t_max_z = ((z as f32 + (if step_z > 0 { 1.0 } else { 0.0 })) - ray.origin[2]) / ray.dir[2];

    while t_max_x.min(t_max_y).min(t_max_z) < max_dist {
        if t_max_x < t_max_y && t_max_x < t_max_z {
            x += step_x;
            t_max_x += t_delta_x;
        } else if t_max_y < t_max_z {
            y += step_y;
            t_max_y += t_delta_y;
        } else {
            z += step_z;
            t_max_z += t_delta_z;
        }
        // Check if voxel at (x, y, z) is solid
    }
    None
}
"#.into(),
            documentation: r#"# Custom 3D Voxel Engine

State-of-the-art voxel terrain meshing and low-overhead 3D traversal in Rust.
"#.into(),
        },

        // 8. Advanced Game 2
        ProjectIdea {
            id: "game-adv-2".into(),
            title: "Raycasting 3D FPS Game Engine (Wolfenstein 3D Style)".into(),
            domain: Domain::GameDev,
            difficulty: Difficulty::Advanced,
            description: "Retro pseudo-3D raycaster written from scratch with DDA wall projection, textured walls, sprite rendering, depth sorting, and door mechanics.".into(),
            requirements: vec![
                "Pure software DDA raycasting rendering 3D perspectives on 2D software buffer".into(),
                "Texture mapping with affine column sampling and wall distance perspective divide".into(),
                "Sprite billboarding with depth-buffer (Z-buffer) sorting for enemies and items".into(),
                "Interactive sliding doors, elevator floors, and minimap HUD overlay".into(),
            ],
            technologies: vec!["C++ / Rust / WebGL Canvas".into(), "Trigonometry & DDA Algorithm".into(), "Software Rasterization".into()],
            duration: "30 - 45 hours".into(),
            steps: vec![
                "Step 1: Implement camera field of view (FOV) ray vectors across screen columns.".into(),
                "Step 2: Cast rays against 2D grid map until wall collision using DDA.".into(),
                "Step 3: Correct fish-eye distortion using cosine angle projection.".into(),
                "Step 4: Draw vertical textured pixel columns proportional to wall distance.".into(),
                "Step 5: Transform enemy sprites into camera space and draw with Z-buffer occlusion.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "raycaster_engine.js".into(),
            starter_code: r#"// Wolfenstein 3D Style Column Raycaster
function renderRaycast(ctx, width, height, player, map) {
  for (let col = 0; col < width; col++) {
    const cameraX = (2 * col / width) - 1;
    const rayDirX = player.dirX + player.planeX * cameraX;
    const rayDirY = player.dirY + player.planeY * cameraX;

    let mapX = Math.floor(player.x);
    let mapY = Math.floor(player.y);
    let perpWallDist = 0;
    
    // DDA stepping...
    // Calculate wall slice height
    const lineHeight = Math.floor(height / (perpWallDist + 0.001));
    const drawStart = Math.max(0, -lineHeight / 2 + height / 2);
    const drawEnd = Math.min(height - 1, lineHeight / 2 + height / 2);

    ctx.fillStyle = '#38bdf8';
    ctx.fillRect(col, drawStart, 1, drawEnd - drawStart);
  }
}
"#.into(),
            documentation: r#"# Raycasting 3D FPS Game Engine

The iconic pseudo-3D engine algorithm that sparked the modern first-person shooter genre.
"#.into(),
        },

        // 9. Advanced Game 3
        ProjectIdea {
            id: "game-adv-3".into(),
            title: "Chess AI Engine with Minimax & Alpha-Beta Pruning".into(),
            domain: Domain::GameDev,
            difficulty: Difficulty::Advanced,
            description: "Chess game engine implementing Bitboards, legal move generation, Minimax search with Alpha-Beta pruning, transposition tables, and Zobrist hashing.".into(),
            requirements: vec![
                "Bitboard board representation (u64 bitmasks for each piece type and color)".into(),
                "Full chess move generator supporting castling, en-passant, and pawn promotions".into(),
                "Minimax search with Alpha-Beta pruning, iterative deepening, and quiescence search".into(),
                "Evaluation function factoring in piece values, positional heatmaps, and pawn structures".into(),
                "UCI (Universal Chess Interface) protocol support to integrate with chess GUIs".into(),
            ],
            technologies: vec!["Rust or C++".into(), "Bitboards (u64)".into(), "Alpha-Beta Pruning".into(), "Zobrist Hashing".into(), "UCI Protocol".into()],
            duration: "40 - 65 hours".into(),
            steps: vec![
                "Step 1: Set up 64-bit bitboards for 12 piece types and occupied squares.".into(),
                "Step 2: Generate pseudo-legal and legal moves (checking for discovered king checks).".into(),
                "Step 3: Implement static position evaluation function with piece-square tables.".into(),
                "Step 4: Build recursive Alpha-Beta search with move ordering (captures first).".into(),
                "Step 5: Integrate Zobrist hashing and Transposition Table cache to avoid redundant node checks.".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "chess_alpha_beta.rs".into(),
            starter_code: r#"// Chess Alpha-Beta Pruning with Evaluation
pub fn alpha_beta(depth: i32, mut alpha: i32, beta: i32, is_maximizing: bool) -> i32 {
    if depth == 0 {
        return evaluate_board();
    }

    let moves = generate_legal_moves();
    if moves.is_empty() {
        return if in_check() { -100_000 + depth } else { 0 }; // Checkmate vs Stalemate
    }

    if is_maximizing {
        let mut max_eval = -999_999;
        for mv in moves {
            make_move(&mv);
            let eval = alpha_beta(depth - 1, alpha, beta, false);
            undo_move(&mv);
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);
            if beta <= alpha { break; } // Beta cutoff
        }
        max_eval
    } else {
        let mut min_eval = 999_999;
        for mv in moves {
            make_move(&mv);
            let eval = alpha_beta(depth - 1, alpha, beta, true);
            undo_move(&mv);
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha { break; } // Alpha cutoff
        }
        min_eval
    }
}
fn evaluate_board() -> i32 { 0 }
fn generate_legal_moves() -> Vec<i32> { vec![] }
fn in_check() -> bool { false }
fn make_move(_mv: &i32) {}
fn undo_move(_mv: &i32) {}
"#.into(),
            documentation: r#"# Chess AI Engine with Alpha-Beta Pruning

High-efficiency board evaluations and tree pruning algorithms.
"#.into(),
        },

        // 10. Intermediate Game 4
        ProjectIdea {
            id: "game-int-4".into(),
            title: "Physics Ragdoll & Soft-body Jelly Simulator".into(),
            domain: Domain::GameDev,
            difficulty: Difficulty::Intermediate,
            description: "Verlet integration physics playground featuring squishy soft-body jellies, swinging ropes, joint constraints, and ragdoll humanoids reacting to gravity and collisions.".into(),
            requirements: vec![
                "Verlet integration particle solver (point masses + distance stick constraints)".into(),
                "Cloth / rope simulation with pin constraints and mouse slicing tear mechanics".into(),
                "Ragdoll humanoid with angular and length joint limits".into(),
                "Iterative constraint relaxation loop ensuring stability under stress".into(),
            ],
            technologies: vec!["HTML5 Canvas / Vanilla JS or Rust".into(), "Verlet Integration".into(), "Constraint Relaxation".into()],
            duration: "14 - 22 hours".into(),
            steps: vec![
                "Step 1: Model Verlet point: x, y, old_x, old_y.".into(),
                "Step 2: Implement distance constraint stick connecting two points.".into(),
                "Step 3: Solve constraints across 5-10 relaxation iterations per frame.".into(),
                "Step 4: Build humanoid stick skeleton with joint limits.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "verlet_physics.js".into(),
            starter_code: r#"// Verlet Integration Point & Stick Constraint
class Point {
  constructor(x, y) {
    this.x = x; this.y = y; this.oldX = x; this.oldY = y; this.pinned = false;
  }
  update(gravity = 0.5) {
    if (this.pinned) return;
    const vx = this.x - this.oldX;
    const vy = this.y - this.oldY;
    this.oldX = this.x;
    this.oldY = this.y;
    this.x += vx;
    this.y += vy + gravity;
  }
}

class Stick {
  constructor(p0, p1, length) {
    this.p0 = p0; this.p1 = p1;
    this.len = length || Math.hypot(p1.x - p0.x, p1.y - p0.y);
  }
  solve() {
    const dx = this.p1.x - this.p0.x;
    const dy = this.p1.y - this.p0.y;
    const dist = Math.hypot(dx, dy);
    const diff = (this.len - dist) / dist;
    const offsetX = dx * diff * 0.5;
    const offsetY = dy * diff * 0.5;
    if (!this.p0.pinned) { this.p0.x -= offsetX; this.p0.y -= offsetY; }
    if (!this.p1.pinned) { this.p1.x += offsetX; this.p1.y += offsetY; }
  }
}
"#.into(),
            documentation: r#"# Verlet Physics Ragdoll Simulator

Simple yet stable physics simulation without expensive velocity computations.
"#.into(),
        },
    ]
}
