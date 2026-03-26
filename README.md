# ttung (RUST-based)

CLI built in rust that include explanation to one who come from ruby.

*disclaimer: tbh after trying, the syntax are pretty wild lol; this good for vibecode-learning trials with AI models and future migration, but for now i might consider back to python or go instead for now.*


## 🚀 Getting Started

### Prerequisites
**Rust 1.85.1**: **2024 Edition** for recent common industry usage.

```bash
# Setup toolchain via mise
mise use -g rust@1.85
```

### Installation
clone the repository and build the binary:

```bash
git clone https://github.com/kwhandy/ttung-rust ttung
cd ttung
```

### Usage
run the tool using `cargo run -- <COMMAND>`

```bash
# 1) string Transformation
# reverse a string (Learning Ownership)
cargo run -- reverse "hello ttung"
# convert to UPPERCASE (Learning Mutability)
cargo run -- upcase "startup speed"

# 2) math Operations
# basic calculator (Learning Types)
cargo run -- calc 10 5
```
