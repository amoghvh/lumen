# 🛡️ Lumen - Security Telemetry Engine

[![Rust](https://img.shields.io/badge/rust-1.95%2B-orange)](https://www.rust-lang.org/)
[![Tokio](https://img.shields.io/badge/tokio-1.52-red)](https://tokio.rs/)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

**High-performance, memory-safe log processor that detects security threats in real-time.**

## ✨ Features

- ⚡ **10,000+ concurrent connections** via Tokio async runtime
- 🔍 **Real-time threat detection** (SQL injection, XSS, PII)
- 🛡️ **Memory safety** with zero unsafe code
- 📊 **Backpressure** via bounded MPSC channels
- 🧪 **Comprehensive tests** (5+ unit tests)
- 🐳 **Docker ready**
- 📝 **Structured logging** with tracing

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/amoghvh/lumen
cd lumen

# Run in development mode
cargo run

# Run in production mode (optimized)
cargo run --release
