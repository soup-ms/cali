# Cali - Nutrition Tracker CLI

A simple, colorful command-line tool for tracking daily nutrition metrics.

## Features

- **Quick Calorie Logging**: `cali 150` logs 150 calories
- **Track Multiple Nutrition Metrics**:
  - Calories (green)
  - Water in fluid ounces (fl oz) (blue)
  - Protein in grams (yellow)
  - Carbs in grams (purple)
  - Fat in grams (red)
- **Daily Summaries**: View nutrition totals for today or any specific date
- **History View**: See all your historical nutrition data
- **Reset Function**: Clear today's data if needed
- **Colorful Output**: Each nutrition type has its own distinct color
- **Data Persistence**: All data is stored in JSON format in the `data` directory

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/cali.git
cd cali

# Build the project
cargo build --release

# Optional: Add to your PATH
cp target/release/cali /usr/local/bin/
```

## Usage

```bash
# Log calories directly
cali 150

# Log water intake
cali log water 16

# Log protein
cali log protein 30

# Log carbs
cali log carbs 45

# Log fat
cali log fat 15

# View today's summary
cali summary

# View summary for a specific date
cali summary --date 2025-03-20

# View all historical data
cali history

# Reset today's data
cali reset

# Show help
cali
```

## Data Storage

All nutrition data is stored in `data/cali_data.json` within the project directory. Each day's nutrition information is stored as a separate entry, allowing you to track your nutrition over time.

## Requirements

- Rust 1.50 or higher
- Terminal with color support

## License

MIT
