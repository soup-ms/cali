use chrono::Local;
use clap::{CommandFactory, Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Optional calories to log (shorthand for 'log calories VALUE')
    calories: Option<f32>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Log nutrition data
    Log {
        /// Type of nutrition to log
        #[command(subcommand)]
        nutrition_type: NutritionType,
    },

    /// Show nutrition summary
    Summary {
        /// Date to show summary for (format: YYYY-MM-DD), defaults to today
        #[arg(short, long)]
        date: Option<String>,
    },

    /// Show all recorded nutrition data
    History,

    /// Reset today's nutrition data
    Reset,
}

#[derive(Subcommand, Debug, Clone)]
enum NutritionType {
    /// Log calories
    Calories {
        /// Amount of calories
        amount: f32,
    },
    /// Log water intake in fluid ounces (fl oz)
    Water {
        /// Amount of water in fluid ounces (fl oz)
        fl_oz: f32,
    },
    /// Log protein intake in grams
    Protein {
        /// Amount of protein in grams
        grams: f32,
    },
    /// Log carbohydrates intake in grams
    Carbs {
        /// Amount of carbohydrates in grams
        grams: f32,
    },
    /// Log fat intake in grams
    Fat {
        /// Amount of fat in grams
        grams: f32,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DailyLog {
    date: String,
    calories: f32,
    water: f32,
    protein: f32,
    carbs: f32,
    fat: f32,
}

impl DailyLog {
    fn new(date: String) -> Self {
        DailyLog {
            date,
            calories: 0.0,
            water: 0.0,
            protein: 0.0,
            carbs: 0.0,
            fat: 0.0,
        }
    }
}

fn get_data_file_path() -> PathBuf {
    // Store the data file in the project directory
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("data");
    std::fs::create_dir_all(&path).expect("Failed to create data directory");
    path.push("cali_data.json");
    path
}

fn load_logs() -> io::Result<Vec<DailyLog>> {
    let path = get_data_file_path();

    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let logs: Vec<DailyLog> = serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
    Ok(logs)
}

fn save_logs(logs: &[DailyLog]) -> io::Result<()> {
    let path = get_data_file_path();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    let json = serde_json::to_string_pretty(logs)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn get_or_create_today_log(logs: &mut Vec<DailyLog>) -> &mut DailyLog {
    let today = Local::now().date_naive().format("%Y-%m-%d").to_string();

    if let Some(index) = logs.iter().position(|log| log.date == today) {
        &mut logs[index]
    } else {
        logs.push(DailyLog::new(today));
        logs.last_mut().unwrap()
    }
}

fn log_nutrition(logs: &mut Vec<DailyLog>, nutrition_type: NutritionType) {
    let today_log = get_or_create_today_log(logs);

    match nutrition_type {
        NutritionType::Calories { amount } => {
            today_log.calories += amount;
            println!(
                "{} {} {}. {} {}",
                "Logged".green(),
                amount.to_string().green().bold(),
                "calories".green(),
                "Total today:".green(),
                today_log.calories.to_string().green().bold(),
            );
        }
        NutritionType::Water { fl_oz } => {
            today_log.water += fl_oz;
            println!(
                "{} {} {}. {} {}",
                "Logged".blue(),
                fl_oz.to_string().blue().bold(),
                "fl oz of water".blue(),
                "Total today:".blue(),
                today_log.water.to_string().blue().bold()
            );
        }
        NutritionType::Protein { grams } => {
            today_log.protein += grams;
            println!(
                "{} {} {}. {} {}",
                "Logged".yellow(),
                grams.to_string().yellow().bold(),
                "grams of protein".yellow(),
                "Total today:".yellow(),
                today_log.protein.to_string().yellow().bold()
            );
        }
        NutritionType::Carbs { grams } => {
            today_log.carbs += grams;
            println!(
                "{} {} {}. {} {}",
                "Logged".purple(),
                grams.to_string().purple().bold(),
                "grams of carbs".purple(),
                "Total today:".purple(),
                today_log.carbs.to_string().purple().bold()
            );
        }
        NutritionType::Fat { grams } => {
            today_log.fat += grams;
            println!(
                "{} {} {}. {} {}",
                "Logged".red(),
                grams.to_string().red().bold(),
                "grams of fat".red(),
                "Total today:".red(),
                today_log.fat.to_string().red().bold()
            );
        }
    }
}

fn reset_today_log(logs: &mut Vec<DailyLog>) -> io::Result<()> {
    let today = Local::now().date_naive().format("%Y-%m-%d").to_string();

    if let Some(index) = logs.iter().position(|log| log.date == today) {
        logs[index] = DailyLog::new(today);
        println!("{}", "Today's nutrition data has been reset.".bold());
    } else {
        println!("{}", "No data for today to reset.".bold());
    }

    save_logs(logs)
}

fn show_summary(logs: &[DailyLog], date_str: Option<String>) {
    let date = match date_str {
        Some(d) => d,
        None => Local::now().date_naive().format("%Y-%m-%d").to_string(),
    };

    if let Some(log) = logs.iter().find(|l| l.date == date) {
        println!("{} {}", "Nutrition Summary for".bold(), log.date.bold());
        println!("{}", "-------------------------".bold());
        println!(
            "{}: {}",
            "Calories".green(),
            log.calories.to_string().green().bold()
        );
        println!(
            "{}: {}",
            "Water".blue(),
            format!("{:.1} fl oz", log.water).blue().bold()
        );
        println!(
            "{}: {}",
            "Protein".yellow(),
            format!("{:.1}g", log.protein).yellow().bold()
        );
        println!(
            "{}: {}",
            "Carbs".purple(),
            format!("{:.1}g", log.carbs).purple().bold()
        );
        println!(
            "{}: {}",
            "Fat".red(),
            format!("{:.1}g", log.fat).red().bold()
        );
    } else {
        println!("No data found for {}", date);
    }
}

fn show_all_logs(logs: &[DailyLog]) {
    if logs.is_empty() {
        println!("{}", "No nutrition data found.".bold());
        return;
    }

    println!("{}", "All Nutrition Records".bold());
    println!("{}", "===================".bold());

    // Sort logs by date (newest first)
    let mut sorted_logs = logs.to_vec();
    sorted_logs.sort_by(|a, b| b.date.cmp(&a.date));

    for log in sorted_logs {
        println!("\n{} {}", "Date:".bold(), log.date.bold());
        println!("{}", "-------------------------".bold());
        println!(
            "{}: {}",
            "Calories".green(),
            log.calories.to_string().green().bold()
        );
        println!(
            "{}: {}",
            "Water".blue(),
            format!("{:.1} fl oz", log.water).blue().bold()
        );
        println!(
            "{}: {}",
            "Protein".yellow(),
            format!("{:.1}g", log.protein).yellow().bold()
        );
        println!(
            "{}: {}",
            "Carbs".purple(),
            format!("{:.1}g", log.carbs).purple().bold()
        );
        println!(
            "{}: {}",
            "Fat".red(),
            format!("{:.1}g", log.fat).red().bold()
        );
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut logs = load_logs()?;

    match &cli.command {
        Some(Commands::Log { nutrition_type }) => {
            log_nutrition(&mut logs, nutrition_type.clone());
        }
        Some(Commands::Summary { date }) => {
            show_summary(&logs, date.clone());
        }
        Some(Commands::History) => {
            show_all_logs(&logs);
        }
        Some(Commands::Reset) => {
            reset_today_log(&mut logs)?;
        }
        None => {
            // If calories are provided directly, log them
            if let Some(calories) = cli.calories {
                log_nutrition(&mut logs, NutritionType::Calories { amount: calories });
            } else {
                // No command or calories provided, print help
                Cli::command().print_help().unwrap();
                return Ok(());
            }
        }
    }

    save_logs(&logs)?;
    Ok(())
}
