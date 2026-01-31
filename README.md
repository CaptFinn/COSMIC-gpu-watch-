# GPU Watch - A COSMIC Applet for Real-Time GPU Monitoring

> **⚠️ WORKS ON MY MACHINE DISCLAIMER ⚠️**  
> This applet was built and tested **only** on my specific hardware (Lenovo Legion Y520, Intel i5-7300HQ, NVIDIA GTX 1050 Ti). I make **no guarantees** it will work on your device. Use at your own risk. If it breaks, you get to keep both pieces. 🔧

---

> **100% vibecoded** on a Friday night because existing applets didn't cut it.

A lightweight COSMIC desktop applet that displays real-time GPU temperature and usage directly in your panel. Built for Pop!_OS 24 when no other applet would show my GPU stats.

![GPU Watch in action](https://img.shields.io/badge/status-works%20on%20my%20machine-brightgreen)

## ❄️ What It Does

- **Live GPU monitoring** - Temperature and usage update every 2 seconds
- **Panel integration** - Sits right in your COSMIC top bar
- **Temperature icons** - Visual feedback at a glance (❄️ → 🌡️ → 🔥 → 🚨)
- **Click for popup** - Shows detailed stats and label size adjustment
- **Auto-resize** - Font scales with panel size, or manually adjust via slider
- **Zero bloat** - ~200 lines of Rust, no unnecessary dependencies

## 🎮 Display Format
```
❄️ 36°C 0%
```

- Icon changes based on temperature (cool/warm/hot/critical)
- First number: GPU temperature in Celsius
- Second number: GPU utilization percentage

## ⚙️ Requirements

- **Pop!_OS 24** (COSMIC desktop)
- **NVIDIA GPU** with `nvidia-smi` installed
- **Rust toolchain** (for building from source)

### Check if nvidia-smi works:
```bash
nvidia-smi --query-gpu=temperature.gpu,utilization.gpu --format=csv,noheader,nounits
```

If this returns two numbers (like `36, 0`), you're good to go!

## 🚀 Installation

### Prerequisites
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install just (command runner)
cargo install just

# Install libcosmic development dependencies (Pop!_OS/Ubuntu)
sudo apt install libcosmic-dev
```

### Build & Install
```bash
# Clone the repo
git clone https://github.com/captfinn/gpu-watch.git
cd gpu-watch

# Build and install (includes desktop entry)
sudo just install

# Restart the panel to load the applet
pkill cosmic-panel
```

### Add to Panel

1. Right-click your COSMIC panel
2. Select "Configure Panel" or "Panel Settings"
3. Look for "GPU Watch" in the applet list
4. Add it to your panel

### Uninstall
```bash
sudo just uninstall
```

## 🛠️ Tested Hardware (Literally Just Mine)

| Component | Model |
|-----------|-------|
| **Laptop** | Lenovo Legion Y520 |
| **CPU** | Intel Core i5-7300HQ |
| **GPU** | NVIDIA GTX 1050 Ti (4GB) |
| **OS** | Pop!_OS 24.04 (COSMIC) |

**Your mileage WILL vary.** Seriously.

## ⚠️ Known Limitations

- **NVIDIA only** - Uses `nvidia-smi` for GPU data
- **Not tested on AMD/Intel** - PRs welcome for `radeontop` or `intel_gpu_top` support
- **Pop!_OS 24 specific** - COSMIC applet API is evolving; may not work on other COSMIC implementations
- **Refresh rate hardcoded** - Updates every 2 seconds (change in code if needed)

## 🧠 How It Works

1. Every 2 seconds, calls `nvidia-smi` to query GPU stats
2. Parses the CSV output (temperature, usage, VRAM)
3. Updates the panel widget with formatted data
4. Icons change color based on temperature thresholds

Temperature thresholds:
- `< 50°C` → ❄️ (Cool)
- `50-65°C` → 🌡️ (Warm)
- `65-80°C` → 🔥 (Hot)
- `> 80°C` → 🚨 (Critical)

## 📝 License

MIT License - Use the code however you want. **No warranty. Seriously. None. Zero. Zilch.**

## 🤝 Contributing

PRs welcome! Especially for:
- AMD GPU support (via `radeontop`)
- Intel GPU support (via `intel_gpu_top`)
- Configurable refresh rate
- Testing on hardware that isn't mine

## 🎉 Credits

Built with:
- [libcosmic](https://github.com/pop-os/libcosmic) - COSMIC desktop framework
- [Iced](https://github.com/iced-rs/iced) - GUI framework
- Rust, coffee, and Friday night energy ☕
- A healthy disrespect for the phrase "it can't be done"

## 💬 Why This Exists

Minimon didn't show my GPU. Observatory was broken. I wanted GPU temps in my panel. So I built it.

**That's vibecoding.**

If it works for you, awesome! If it doesn't, well... I told you. 🤷

---

*If this helped you, star the repo! If it broke your system, I accept no responsibility but I'll feel kinda bad about it.* 😎
