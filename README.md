# dirpulse

A CLI tool that analyzes directory contents and reports statistics about files, including size distribution, file types, and age.

## Features

- Total file and directory counts with size summaries
- Top N largest files
- File extension breakdown
- File age distribution (fresh, aging, stale)

## Installation

### From crates.io

```bash
cargo install dirpulse
```

### From source

```bash
git clone https://github.com/bradleyd/dirpulse
cd dirpulse
cargo install --path .
```

## Usage

```bash
# Analyze current directory
dirpulse .

# Analyze a specific directory
dirpulse /path/to/directory

# Show top 20 largest files (default is 10)
dirpulse /path/to/directory -n 20
```

## Example Output

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│ dirpulse · /home/user/projects                                                           │
├──────────────────────────────────────────────────────────────────────────────────────────┤
│ 🔴 1.20 GB Stale, 📦 Largest: video.mp4 (800.00 MB), 📊 Top 10 = 45.20%                   │
└──────────────────────────────────────────────────────────────────────────────────────────┘

📁 1284 files · 89 directories · 4.50 GB total

── Top 10 Largest ────────────────────────────────────────────────────────────
1          800 MB    video.mp4
2          450 MB    backup.tar.gz
...

── Top 10 Extension ──────────────────────────────────────────────────────────
.rs                       342           12.4 MB
.json                     128            2.1 MB
...

── File Age ──────────────────────────────────────────────────────────────────
🟢 Fresh (< 30 days)        234 files      1.20 GB
🟡 Aging (30 days - 6 mo)   450 files      2.10 GB
🔴 Stale (> 6 months)       600 files      1.20 GB
```

## License

MIT
