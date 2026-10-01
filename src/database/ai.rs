use crate::models::{Difficulty, Domain, ProjectIdea};

pub fn get_projects() -> Vec<ProjectIdea> {
    vec![
        // 1. Beginner AI 1
        ProjectIdea {
            id: "ai-beg-1".into(),
            title: "Neural Network from Scratch (No PyTorch/TF)".into(),
            domain: Domain::AiMachineLearning,
            difficulty: Difficulty::Beginner,
            description: "Build a multi-layer perceptron (MLP) from scratch using only NumPy: implements matrix dot products, ReLU/Sigmoid activations, forward pass, and backpropagation with gradient descent.".into(),
            requirements: vec![
                "Matrix multiplication implementation for forward propagation".into(),
                "Activation functions (Sigmoid, ReLU) and their mathematical derivatives".into(),
                "Mean Squared Error (MSE) or Binary Cross-Entropy loss computation".into(),
                "Backward pass calculating dW and dB gradients via the chain rule".into(),
                "Training loop capable of learning non-linear XOR logic gate or classifying blobs".into(),
            ],
            technologies: vec!["Python & NumPy".into(), "Linear Algebra & Calculus".into(), "Matplotlib".into()],
            duration: "8 - 12 hours".into(),
            steps: vec![
                "Step 1: Initialize random weight matrices W1, W2 and bias vectors b1, b2 with Xavier/He scaling.".into(),
                "Step 2: Write forward pass: z1 = X.W1 + b1, a1 = relu(z1), z2 = a1.W2 + b2, y_hat = sigmoid(z2).".into(),
                "Step 3: Implement loss function comparing predictions against ground truth labels.".into(),
                "Step 4: Derive analytical gradients using backpropagation chain rule.".into(),
                "Step 5: Update weights: W -= learning_rate * dW; train for 1000 epochs and plot loss curve.".into(),
            ],
            starter_code_language: "python".into(),
            starter_code_filename: "mlp_scratch.py".into(),
            starter_code: r#"# Multi-Layer Perceptron from scratch in Python
import numpy as np

def sigmoid(x): return 1.0 / (1.0 + np.exp(-x))
def sigmoid_derivative(x): return x * (1.0 - x)

# XOR Dataset
X = np.array([[0,0], [0,1], [1,0], [1,1]])
y = np.array([[0], [1], [1], [0]])

np.random.seed(42)
w1 = np.random.uniform(-1, 1, (2, 4))
w2 = np.random.uniform(-1, 1, (4, 1))

# Training loop
lr = 0.5
for epoch in range(10000):
    # Forward Pass
    hidden = sigmoid(np.dot(X, w1))
    output = sigmoid(np.dot(hidden, w2))

    # Loss & Backpropagation
    err = y - output
    d_output = err * sigmoid_derivative(output)
    d_hidden = d_output.dot(w2.T) * sigmoid_derivative(hidden)

    # Weights update
    w2 += hidden.T.dot(d_output) * lr
    w1 += X.T.dot(d_hidden) * lr

print("Trained Predictions:")
for sample, pred in zip(X, output):
    print(f"Input: {sample} -> Output: {pred[0]:.4f} (Target: {pred[0] > 0.5})")
"#.into(),
            documentation: r#"# Neural Network from Scratch

Teaches the core mathematical foundations of deep learning: forward pass, chain rule backprop, and optimization.
"#.into(),
        },

        // 2. Beginner AI 2
        ProjectIdea {
            id: "ai-beg-2".into(),
            title: "Smart Document Q&A with Vector Embeddings (Mini RAG)".into(),
            domain: Domain::AiMachineLearning,
            difficulty: Difficulty::Beginner,
            description: "A local Retrieval-Augmented Generation (RAG) assistant that splits user PDF/text documents into chunks, computes vector embeddings, and answers queries with citations.".into(),
            requirements: vec![
                "Text chunker with sliding window character/token overlap".into(),
                "Vector embedding generation via local sentence-transformers or OpenAI API".into(),
                "Cosine similarity vector search ranking top-K relevant chunks".into(),
                "Prompt template feeding retrieved context into an LLM for factual answers".into(),
            ],
            technologies: vec!["Python".into(), "Sentence-Transformers / Ollama".into(), "NumPy Cosine Similarity".into()],
            duration: "7 - 11 hours".into(),
            steps: vec![
                "Step 1: Read text document and split into 300-word chunks with 50-word overlap.".into(),
                "Step 2: Generate vector embeddings for each chunk and store in memory matrix.".into(),
                "Step 3: When a user query arrives, embed query and calculate cosine similarity: (A . B) / (||A|| * ||B||).".into(),
                "Step 4: Take the top 3 nearest chunks and construct an augmented prompt for the LLM.".into(),
            ],
            starter_code_language: "python".into(),
            starter_code_filename: "mini_rag.py".into(),
            starter_code: r#"# Simple In-Memory Vector Search RAG
import numpy as np

def cosine_similarity(v1, v2):
    return np.dot(v1, v2) / (np.linalg.norm(v1) * np.linalg.norm(v2) + 1e-9)

chunks = [
    "Rust is a systems programming language focused on safety and performance.",
    "Python is renowned for machine learning, data science, and fast prototyping.",
    "WebAssembly enables high-performance client-side binaries running inside web browsers."
]

# Simulated 4-dimensional embeddings
embeddings = np.array([
    [0.9, 0.2, 0.1, 0.8],
    [0.1, 0.9, 0.8, 0.2],
    [0.8, 0.3, 0.2, 0.9]
])

def search(query_vector, top_k=2):
    scores = [cosine_similarity(query_vector, emb) for emb in embeddings]
    ranked = np.argsort(scores)[::-1][:top_k]
    return [(chunks[idx], scores[idx]) for idx in ranked]

print("RAG Matches for query:")
results = search(np.array([0.85, 0.1, 0.2, 0.7]))
for text, score in results:
    print(f"[{score:.3f}] {text}")
"#.into(),
            documentation: r#"# Smart Document Q&A (Mini RAG)

Demystifies retrieval-augmented generation and vector semantic search.
"#.into(),
        },

        // 3. Beginner AI 3
        ProjectIdea {
            id: "ai-beg-3".into(),
            title: "Genetic Algorithm Car Track Navigation".into(),
            domain: Domain::AiMachineLearning,
            difficulty: Difficulty::Beginner,
            description: "Simulation where an autonomous car evolves to navigate an obstacle track through natural selection, genetic crossover, and mutation of neural network steering weights.".into(),
            requirements: vec![
                "Ray sensors projecting forward/diagonal distance to track boundary walls".into(),
                "Small feedforward neural network brain mapping sensor distances to steering/throttle".into(),
                "Fitness function measuring distance traveled along the track without crashing".into(),
                "Genetic evolution loop: tournament selection, weight crossover, and Gaussian mutation".into(),
            ],
            technologies: vec!["Python (Pygame) or HTML5 Canvas".into(), "Genetic Algorithms".into(), "Neuroevolution".into()],
            duration: "8 - 14 hours".into(),
            steps: vec![
                "Step 1: Set up 2D track boundary polygon and raycasting distance sensors.".into(),
                "Step 2: Initialize population of 50 cars with random neural network weights.".into(),
                "Step 3: Run simulation step until all cars crash or time limit hits.".into(),
                "Step 4: Rank cars by fitness score (furthest checkpoint reached).".into(),
                "Step 5: Breed next generation using elite selection and weight mutations.".into(),
            ],
            starter_code_language: "python".into(),
            starter_code_filename: "genetic_evolver.py".into(),
            starter_code: r#"# Genetic Algorithm Mutation and Crossover
import random

def mutate_weights(genome, mutation_rate=0.05, mutation_strength=0.2):
    return [
        w + random.gauss(0, mutation_strength) if random.random() < mutation_rate else w
        for w in genome
    ]

def crossover(parent_a, parent_b):
    split_point = random.randint(0, len(parent_a) - 1)
    child = parent_a[:split_point] + parent_b[split_point:]
    return child

# Example genome evolution step
parent1 = [0.5, -0.2, 0.8, -0.4]
parent2 = [-0.1, 0.9, 0.2, 0.6]
child = mutate_weights(crossover(parent1, parent2))
print("Offspring Genome:", child)
"#.into(),
            documentation: r#"# Genetic Algorithm Navigation

Watch cars learn to drift and corner through procedural generation and natural selection.
"#.into(),
        },

        // 4. Intermediate AI 1
        ProjectIdea {
            id: "ai-int-1".into(),
            title: "Real-time AI Audio Denoiser & Voice Isolator".into(),
            domain: Domain::AiMachineLearning,
            difficulty: Difficulty::Intermediate,
            description: "Deep learning audio processor that removes background noise (fan whir, keyboard clicks, barking) from live microphone streams using Short-Time Fourier Transform (STFT) and UNet mask predictions.".into(),
            requirements: vec![
                "Audio STFT spectrogram computation converting time domain to frequency domain".into(),
                "1D / 2D Convolutional neural network predicting an ideal binary/ratio spectral mask".into(),
                "Inverse STFT (iSTFT) to reconstruct clean audio waveform without phase distortion".into(),
                "Real-time audio buffer streaming with sub-50ms latency".into(),
            ],
            technologies: vec!["Python (PyTorch / Librosa)".into(), "STFT / iSTFT Signal Processing".into(), "ONNX Runtime".into(), "PortAudio / PyAudio".into()],
            duration: "20 - 32 hours".into(),
            steps: vec![
                "Step 1: Convert raw PCM audio samples into magnitude and phase spectrograms via STFT.".into(),
                "Step 2: Train a lightweight UNet or GRU network on paired noisy-clean speech datasets (DNS Challenge).".into(),
                "Step 3: Multiply predicted spectral mask with noisy magnitude to suppress ambient noise.".into(),
                "Step 4: Recombine with original phase and perform iSTFT back to time domain.".into(),
                "Step 5: Export model to ONNX for low-latency live microphone inference.".into(),
            ],
            starter_code_language: "python".into(),
            starter_code_filename: "spectral_mask.py".into(),
            starter_code: r#"# Audio STFT Spectral Masking Pipeline
import numpy as np

def apply_spectral_mask(noisy_stft, mask_predictions):
    """
    noisy_stft: Complex STFT matrix (frequency_bins, time_frames)
    mask_predictions: Float values [0.0, 1.0] representing speech presence ratio
    """
    magnitude = np.abs(noisy_stft)
    phase = np.angle(noisy_stft)

    # Clean magnitude estimation
    clean_mag = magnitude * mask_predictions
    # Reconstruct complex spectrum
    clean_stft = clean_mag * np.exp(1j * phase)
    return clean_stft
"#.into(),
            documentation: r#"# Real-time AI Audio Denoiser

Clean crystal-clear speech extraction using deep spectral filtering.
"#.into(),
        },

        // 5. Intermediate AI 2
        ProjectIdea {
            id: "ai-int-2".into(),
            title: "Autonomous Trading Bot with Reinforcement Learning (PPO)".into(),
            domain: Domain::AiMachineLearning,
            difficulty: Difficulty::Intermediate,
            description: "Algorithmic financial trading agent trained with Proximal Policy Optimization (PPO) reinforcement learning on historical candlestick price data, accounting for transaction fees and slippage.".into(),
            requirements: vec![
                "Custom OpenAI Gym / Farama Gymnasium environment for stock/crypto trading".into(),
                "Observation space: Technical indicators (RSI, MACD, Bollinger Bands, Volume SMA)".into(),
                "Action space: Discrete [Buy, Sell, Hold] or Continuous portfolio allocation weights".into(),
                "Reward function optimizing Sharpe Ratio while penalizing drawdown volatility".into(),
                "Backtesting engine with equity curves and maximum drawdown analysis".into(),
            ],
            technologies: vec!["Python".into(), "Gymnasium / Stable-Baselines3 (PPO)".into(), "Pandas & TA-Lib".into(), "Plotly".into()],
            duration: "22 - 35 hours".into(),
            steps: vec![
                "Step 1: Download OHLCV historical candlestick data and engineer technical indicators.".into(),
                "Step 2: Build custom Gymnasium environment with step(), reset(), and state observations.".into(),
                "Step 3: Formulate reward function balancing portfolio returns against portfolio volatility risk.".into(),
                "Step 4: Train PPO actor-critic network over 500,000 environment steps.".into(),
                "Step 5: Run out-of-sample backtests and calculate benchmark performance vs Buy & Hold.".into(),
            ],
            starter_code_language: "python".into(),
            starter_code_filename: "trading_env.py".into(),
            starter_code: r#"# Simplified Trading Environment Step Method
class SimpleTradingEnv:
    def __init__(self, prices, initial_balance=10000.0):
        self.prices = prices
        self.initial_balance = initial_balance
        self.reset()

    def reset(self):
        self.current_step = 0
        self.balance = self.initial_balance
        self.shares = 0
        return self.prices[self.current_step]

    def step(self, action):
        # 0 = Hold, 1 = Buy, 2 = Sell
        current_price = self.prices[self.current_step]
        if action == 1 and self.balance >= current_price: # Buy
            self.shares += self.balance // current_price
            self.balance %= current_price
        elif action == 2 and self.shares > 0: # Sell
            self.balance += self.shares * current_price
            self.shares = 0

        self.current_step += 1
        done = self.current_step >= len(self.prices) - 1
        portfolio_val = self.balance + self.shares * self.prices[self.current_step]
        reward = portfolio_val - self.initial_balance
        return self.prices[self.current_step], reward, done
"#.into(),
            documentation: r#"# Autonomous Trading Bot with PPO

Reinforcement learning agent trained on macroeconomic signals and technical indicators.
"#.into(),
        },

        // 6. Intermediate AI 3
        ProjectIdea {
            id: "ai-int-3".into(),
            title: "Multi-Agent AI Debate & Consensus Engine".into(),
            domain: Domain::AiMachineLearning,
            difficulty: Difficulty::Intermediate,
            description: "Multi-agent framework where specialized LLM persona agents (Researcher, Critic, Devil's Advocate, Fact-Checker, Judge) debate complex topics and reach verified consensus.".into(),
            requirements: vec![
                "Configurable agent personas with custom system prompts and reasoning objectives".into(),
                "Turn-taking orchestration graph: proposal -> critique -> counter-argument -> arbitration".into(),
                "Fact-checking agent verifying citations against web/search APIs".into(),
                "Final synthesis summary producing agreement points and unresolved nuances".into(),
            ],
            technologies: vec!["Python".into(), "LangGraph / LiteLLM / Instructor".into(), "Pydantic Structured Outputs".into()],
            duration: "16 - 24 hours".into(),
            steps: vec![
                "Step 1: Define structured Pydantic schemas for arguments, critiques, and verdicts.".into(),
                "Step 2: Configure stateful conversation blackboard tracking agent contributions.".into(),
                "Step 3: Implement multi-round debate protocol with maximum rebuttal limits.".into(),
                "Step 4: Integrate scoring judge agent to grade arguments on logical fallacy avoidance.".into(),
            ],
            starter_code_language: "python".into(),
            starter_code_filename: "agent_orchestrator.py".into(),
            starter_code: r#"# Multi-Agent Debate Blackboard Pattern
class DebateSession:
    def __init__(self, topic):
        self.topic = topic
        self.history = []

    def log_argument(self, agent_role, text):
        self.history.append({"role": agent_role, "text": text})

    def get_prompt_context(self):
        context = f"Topic: {self.topic}\n\nTranscript so far:\n"
        for entry in self.history:
            context += f"[{entry['role'].upper()}]: {entry['text']}\n"
        return context

# Simulation
session = DebateSession("Should AI code generators be allowed to commit without human review?")
session.log_argument("proposer", "Autonomous commits accelerate delivery cycles by 10x with test suites.")
session.log_argument("critic", "Test suites cannot catch zero-day logic flaws or hallucinated dependency attacks.")
print(session.get_prompt_context())
"#.into(),
            documentation: r#"# Multi-Agent Debate Engine

Structured dialectic consensus system using cooperating language model personas.
"#.into(),
        },

        // 7. Advanced AI 1
        ProjectIdea {
            id: "ai-adv-1".into(),
            title: "Transformer LLM from Scratch (nanoGPT in PyTorch/Rust)".into(),
            domain: Domain::AiMachineLearning,
            difficulty: Difficulty::Advanced,
            description: "Construct a complete decoder-only Generative Pretrained Transformer from scratch: Multi-Head Self-Attention, RoPE/Learned Positional Embeddings, KV-cache inference, and BPE tokenizer.".into(),
            requirements: vec![
                "Byte-Pair Encoding (BPE) tokenizer training and subword vocabulary encoding".into(),
                "Scaled Dot-Product Multi-Head Attention with causal autoregressive masking".into(),
                "LayerNorm / RMSNorm and SwiGLU / GeLU feed-forward neural layers".into(),
                "KV-Cache optimization for low-latency token-by-token generation during sampling".into(),
                "Pretrain model on Shakespeare text dataset and generate coherent new passages".into(),
            ],
            technologies: vec!["PyTorch or Rust (Burn / Candle)".into(), "Transformer Architecture".into(), "GPU Tensors (CUDA/Metal)".into()],
            duration: "40 - 65 hours".into(),
            steps: vec![
                "Step 1: Train BPE tokenizer on text corpus and create vocabulary lookup tables.".into(),
                "Step 2: Implement Multi-Head Attention module: Q, K, V linear projections and softmax((Q.K^T) / sqrt(d)).".into(),
                "Step 3: Stack N Transformer decoder blocks with residual skip connections and RMSNorm.".into(),
                "Step 4: Train model with AdamW optimizer and cosine learning rate decay scheduler.".into(),
                "Step 5: Implement KV-caching autoregressive text generation with temperature and top-p (nucleus) sampling.".into(),
            ],
            starter_code_language: "python".into(),
            starter_code_filename: "causal_attention.py".into(),
            starter_code: r#"# Scaled Dot-Product Causal Self-Attention in PyTorch
import torch
import torch.nn as nn
import math

class CausalSelfAttention(nn.Module):
    def __init__(self, d_model=256, n_heads=4):
        super().__init__()
        self.d_model = d_model
        self.n_heads = n_heads
        self.head_dim = d_model // n_heads

        self.qkv_proj = nn.Linear(d_model, 3 * d_model)
        self.out_proj = nn.Linear(d_model, d_model)

    def forward(self, x):
        B, T, C = x.size()
        q, k, v = self.qkv_proj(x).chunk(3, dim=-1)
        
        q = q.view(B, T, self.n_heads, self.head_dim).transpose(1, 2)
        k = k.view(B, T, self.n_heads, self.head_dim).transpose(1, 2)
        v = v.view(B, T, self.n_heads, self.head_dim).transpose(1, 2)

        scores = (q @ k.transpose(-2, -1)) / math.sqrt(self.head_dim)
        mask = torch.tril(torch.ones(T, T, device=x.device)).view(1, 1, T, T)
        scores = scores.masked_fill(mask == 0, float('-inf'))
        attn_weights = torch.softmax(scores, dim=-1)

        out = (attn_weights @ v).transpose(1, 2).contiguous().view(B, T, C)
        return self.out_proj(out)
"#.into(),
            documentation: r#"# Transformer LLM from Scratch

Deep dive into the core architecture powering modern generative language models.
"#.into(),
        },

        // 8. Advanced AI 2
        ProjectIdea {
            id: "ai-adv-2".into(),
            title: "Custom Vector Database Engine with HNSW Indexing".into(),
            domain: Domain::AiMachineLearning,
            difficulty: Difficulty::Advanced,
            description: "High-performance vector database built in Rust implementing Hierarchical Navigable Small World (HNSW) graphs, SIMD-accelerated Euclidean/Cosine distance, and write-ahead logging (WAL).".into(),
            requirements: vec![
                "Multi-layer Hierarchical Navigable Small World (HNSW) graph index construction".into(),
                "SIMD (AVX2 / NEON) accelerated vector dot product and cosine distance kernels".into(),
                "Persistent on-disk storage with Write-Ahead Log (WAL) and memory-mapped files (mmap)".into(),
                "Concurrent read/write queries with lock-free skip lists or read-write locks".into(),
                "gRPC / REST API for inserting embeddings and running sub-millisecond approximate nearest neighbor (ANN) searches".into(),
            ],
            technologies: vec!["Rust".into(), "SIMD Intrinsics".into(), "HNSW Graph Algorithm".into(), "Memory Mapping (mmap)".into(), "Tokio / Axum".into()],
            duration: "45 - 70 hours".into(),
            steps: vec![
                "Step 1: Implement SIMD-vectorized cosine distance function in Rust.".into(),
                "Step 2: Build the HNSW multi-layer graph data structure with beam search traversal.".into(),
                "Step 3: Implement heuristic neighborhood edge selection for graph pruning.".into(),
                "Step 4: Add binary serialization and Write-Ahead Logging for crash resilience.".into(),
                "Step 5: Benchmark queries per second (QPS) and recall against Faiss and Milvus.".into(),
            ],
            starter_code_language: "rust".into(),
            starter_code_filename: "simd_distance.rs".into(),
            starter_code: r#"// SIMD-accelerated Vector Dot Product in Rust
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    let mut sum = 0.0;
    let chunks_a = a.chunks_exact(4);
    let chunks_b = b.chunks_exact(4);
    let rem_a = chunks_a.remainder();
    let rem_b = chunks_b.remainder();

    for (ca, cb) in chunks_a.zip(chunks_b) {
        sum += ca[0] * cb[0] + ca[1] * cb[1] + ca[2] * cb[2] + ca[3] * cb[3];
    }
    for (ra, rb) in rem_a.iter().zip(rem_b.iter()) {
        sum += ra * rb;
    }
    sum
}
"#.into(),
            documentation: r#"# Custom Vector Database Engine with HNSW

Blazing-fast approximate nearest neighbor vector indexing engineered in Rust.
"#.into(),
        },

        // 9. Advanced AI 3
        ProjectIdea {
            id: "ai-adv-3".into(),
            title: "Generative Diffusion Model for Image Synthesis".into(),
            domain: Domain::AiMachineLearning,
            difficulty: Difficulty::Advanced,
            description: "Denoising Diffusion Probabilistic Model (DDPM) trained on MNIST/CIFAR-10: forward Markov noise injection, reverse UNet noise prediction, and classifier-free guidance sampling.".into(),
            requirements: vec![
                "Forward diffusion process adding Gaussian noise over T=1000 timesteps with linear/cosine beta schedule".into(),
                "UNet neural network with sinusoidal timestep embeddings and cross-attention".into(),
                "Loss function optimizing variational lower bound (MSE on injected vs predicted noise)".into(),
                "Reverse diffusion sampling loop generating crisp images from pure white noise".into(),
                "Classifier-Free Guidance (CFG) allowing controllable conditional generation".into(),
            ],
            technologies: vec!["Python & PyTorch".into(), "DDPM / DDIM Mathematical Theory".into(), "UNet Architecture".into()],
            duration: "35 - 55 hours".into(),
            steps: vec![
                "Step 1: Formulate variance schedule: betas, alphas, and alpha_hats.".into(),
                "Step 2: Implement q_sample() function: x_t = sqrt(alpha_hat_t) * x_0 + sqrt(1 - alpha_hat_t) * epsilon.".into(),
                "Step 3: Construct UNet with downsampling, bottleneck, and upsampling blocks with skip connections.".into(),
                "Step 4: Train model to predict epsilon given (x_t, t).".into(),
                "Step 5: Write reverse diffusion sampling loop p_sample_loop() and render image generation gif.".into(),
            ],
            starter_code_language: "python".into(),
            starter_code_filename: "ddpm_schedule.py".into(),
            starter_code: r#"# DDPM Forward Diffusion Process Step
import torch

class NoiseScheduler:
    def __init__(self, timesteps=1000, beta_start=1e-4, beta_end=0.02):
        self.timesteps = timesteps
        self.betas = torch.linspace(beta_start, beta_end, timesteps)
        self.alphas = 1.0 - self.betas
        self.alphas_cumprod = torch.cumprod(self.alphas, dim=0)

    def add_noise(self, x_start, noise, t):
        sqrt_alphas_cumprod = torch.sqrt(self.alphas_cumprod[t])[:, None, None, None]
        sqrt_one_minus_alphas = torch.sqrt(1.0 - self.alphas_cumprod[t])[:, None, None, None]
        return sqrt_alphas_cumprod * x_start + sqrt_one_minus_alphas * noise
"#.into(),
            documentation: r#"# Generative Diffusion Model (DDPM)

Learn how modern image generators like Stable Diffusion synthesize imagery from noise.
"#.into(),
        },

        // 10. Intermediate AI 4
        ProjectIdea {
            id: "ai-int-4".into(),
            title: "Voice-Controlled AI Desktop Assistant".into(),
            domain: Domain::AiMachineLearning,
            difficulty: Difficulty::Intermediate,
            description: "Local voice assistant with wake-word detection ('Hey Computer'), fast Whisper speech-to-text, LLM function calling for OS tasks (play music, open apps), and Piper neural TTS voice response.".into(),
            requirements: vec![
                "Continuous microphone audio stream listening for custom wake word".into(),
                "Local Whisper transcription running via whisper.cpp / faster-whisper".into(),
                "Tool calling schema allowing LLM to run system commands, fetch weather, and control volume".into(),
                "Neural Text-to-Speech synthesis with low-latency streaming audio response".into(),
            ],
            technologies: vec!["Python".into(), "faster-whisper".into(), "Piper TTS".into(), "PyAudio".into(), "Ollama / Local LLM".into()],
            duration: "18 - 28 hours".into(),
            steps: vec![
                "Step 1: Set up audio stream with Voice Activity Detection (VAD) using Silero-VAD.".into(),
                "Step 2: Transcribe spoken audio chunks with faster-whisper on CPU/GPU.".into(),
                "Step 3: Define tool definitions (open_application, search_web, set_volume).".into(),
                "Step 4: Execute structured function calls and pipe synthesized TTS response to audio output.".into(),
            ],
            starter_code_language: "python".into(),
            starter_code_filename: "voice_assistant.py".into(),
            starter_code: r#"# Voice Assistant Tool Calling Loop Skeleton
import json

TOOLS = {
    "open_browser": lambda url: print(f"Opening browser to {url}"),
    "get_time": lambda: print("Current time is 15:30")
}

def execute_tool_call(tool_name, arguments_json):
    if tool_name in TOOLS:
        args = json.loads(arguments_json) if isinstance(arguments_json, str) else arguments_json
        return TOOLS[tool_name](**args)
    return f"Unknown tool: {tool_name}"

execute_tool_call("open_browser", {"url": "https://rust-lang.org"})
"#.into(),
            documentation: r#"# Voice-Controlled AI Desktop Assistant

Hands-free local personal assistant with tool execution capabilities.
"#.into(),
        },
    ]
}
