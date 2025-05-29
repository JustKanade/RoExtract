#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod log;
mod gui;
mod logic;
mod updater;
mod config;
mod locale;

use std::path::PathBuf;
use std::sync::Arc;

use clap::{Parser, ValueEnum};

// CLI definitions
#[derive(ValueEnum, Clone, Debug)]
enum Category {
    Music,
    Sounds,
    Images,
    Ktx,
    Rbxm,
}

// Implement `Display` for `Category`
impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// List assets
    #[arg(short, long)]
    list: bool,

    /// Set mode, using this is generally recommended, if this is not provided, the program will run the same function across each mode
    #[arg(short, long, value_name = "CATEGORY")]
    mode: Option<Category>,

    /// Extract asset, extract directory if no asset provided
    #[arg(short, long)]
    extract: Option<String>,

    /// Extract all assets to directory
    #[arg(long)]
    extract_all: bool,

    /// Add a file extension automatically
    #[arg(long)]
    extension: bool,

    /// Define a destination path
    #[arg(short, long)]
    dest: Option<PathBuf>,

    /// Swap two assets
    #[arg(short, long)]
    swap: Option<String>,

    /// Return the cache directory
    #[arg(short, long)]
    cache_dir: bool,

    /// Connect to the internet to check for updates
    #[arg(long)]
    check_for_updates: bool,

    /// Connect to the internet to download new update binary
    #[arg(long)]
    download_new_update: bool,
}

// ======================= Chinese Font Support Functions =======================

/// Set up Chinese font support - simplified version
pub fn setup_chinese_fonts(ctx: &egui::Context) {
    if let Some(font_path) = find_system_chinese_font() {
        if let Ok(font_bytes) = std::fs::read(&font_path) {
            configure_chinese_font(ctx, font_bytes);
            return;
        }
    }
    
    // If no font is found, use fallback configuration
    configure_fallback_font(ctx);
}

/// Find Chinese fonts in the system
fn find_system_chinese_font() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        // Common Chinese font paths on Windows system
        let possible_fonts = vec![
            "C:/Windows/Fonts/msyh.ttc",      // Microsoft YaHei
            "C:/Windows/Fonts/simsun.ttc",    // SimSun
            "C:/Windows/Fonts/simhei.ttf",    // SimHei
            "C:/Windows/Fonts/STXIHEI.TTF",   // STXihei
            "C:/Windows/Fonts/SIMYOU.TTF",    // YouYuan
        ];
        
        for font_path in possible_fonts {
            if std::path::Path::new(font_path).exists() {
                return Some(font_path.to_string());
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        
        // Use fc-list command to find Chinese fonts
        if let Ok(output) = Command::new("fc-list")
            .arg(":lang=zh")
            .output() 
        {
            if let Ok(stdout) = std::str::from_utf8(&output.stdout) {
                for line in stdout.lines() {
                    if let Some(font_path) = line.split(':').next() {
                        let path = font_path.trim();
                        if path.ends_with(".ttf") || path.ends_with(".otf") || path.ends_with(".ttc") {
                            if std::path::Path::new(path).exists() {
                                return Some(path.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        // Fallback paths
        let possible_fonts = vec![
            "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
            "/usr/share/fonts/truetype/arphic/uming.ttc",
            "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
            "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
        ];
        
        for font_path in possible_fonts {
            if std::path::Path::new(font_path).exists() {
                return Some(font_path.to_string());
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        let possible_fonts = vec![
            "/System/Library/Fonts/PingFang.ttc",           // PingFang
            "/System/Library/Fonts/Hiragino Sans GB.ttc",   // Hiragino Sans GB Simplified Chinese
            "/Library/Fonts/Songti.ttc",                    // Songti
            "/System/Library/Fonts/STHeiti Light.ttc",      // STHeiti
        ];
        
        for font_path in possible_fonts {
            if std::path::Path::new(font_path).exists() {
                return Some(font_path.to_string());
            }
        }
    }
    
    None
}

/// Configure Chinese font
fn configure_chinese_font(ctx: &egui::Context, font_bytes: Vec<u8>) {
    let mut fonts = egui::FontDefinitions::default();
    
    // Add Chinese font data
    fonts.font_data.insert(
        "chinese_font".to_owned(),
        Arc::new(egui::FontData::from_owned(font_bytes)),
    );
    
    // Set Chinese font as preferred font for Proportional font family
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "chinese_font".to_owned());
    
    // Add Chinese font to Monospace font family as fallback
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("chinese_font".to_owned());
    
    // Apply font configuration
    ctx.set_fonts(fonts);
}

/// Configure fallback font (when system font is not found)
fn configure_fallback_font(ctx: &egui::Context) {
    // If there are embedded font files, they can be used here
    // Example: fonts.font_data.insert("fallback_chinese".to_owned(), egui::FontData::from_static(include_bytes!("../fonts/chinese_font.ttf")));
    
    // Currently only using system default fonts, but adjusting font configuration for better unicode character display
    let fonts = egui::FontDefinitions::default();
    
    // Ensure default fonts support more unicode characters
    // This may not perfectly display Chinese, but at least won't show boxes
    ctx.set_fonts(fonts);
    
    println!("Warning: No suitable Chinese font found, Chinese display may be incomplete");
}

// ======================= Core Functionality Functions =======================

fn get_tab(category: Category) -> String {
    category.to_string().to_lowercase().replace("ktx","ktx-files").replace("rbxm","rbxm-files")
}

fn list(tab: String) {
    let cache_directory = logic::get_mode_cache_directory(&tab);
    logic::refresh(cache_directory, tab, true, true); // cli_list_mode is set to true, this will print assets to console
}

fn extract(tab: String, asset: Option<String>, destination: Option<PathBuf>, add_extension: bool) {
    let cache_directory = logic::get_mode_cache_directory(&tab);
    if let Some(asset) = asset {
        let dest = destination.unwrap_or_else(|| asset.clone().into());
        logic::extract_file(cache_directory.join(asset), &tab, dest, add_extension);
    } else {
        if let Some(dest) = destination {
            logic::refresh(cache_directory.clone(), tab.clone(), true, true);
            logic::extract_dir(cache_directory, dest, tab, true, false);
        } else {
            eprintln!("Please provide either a destination path or an asset to extract! --help for more details.")
        }
    }
}

fn main() {
    let args = Cli::parse();

    if args.list {
        if let Some(category) = args.mode {
            list(get_tab(category));
        } else {
            // Not enough arguments - go through all categories
            for category in logic::get_categories() {
                list(category);
            }
        }
    } else if args.extract.is_some() || args.extract_all {
        if let Some(category) = args.mode {
            extract(get_tab(category), args.extract, args.dest, args.extension);
        } else {
            // Not enough arguments - go through all categories
            if let Some(destination) = args.dest {
                logic::extract_all(destination, true, false);
            } else {
                eprintln!("--dest is required to extract all assets. --help for more details")
            }
        }
    } else if let Some(asset) = args.swap {
        if let Some(dest) = args.dest {
            let dir = logic::get_mode_cache_directory(&get_tab(args.mode.unwrap_or(Category::Images)));
            logic::swap_assets(dir, &asset, &dest.to_string_lossy().to_string());
        } else {
            eprintln!("--dest is required for swapping assets, --help for more details")
        }
    } else if args.cache_dir {
        println!("{}", logic::get_cache_directory().display());
    } else if args.check_for_updates {
        updater::check_for_updates(false, false);
    } else if args.download_new_update {
        updater::check_for_updates(false, true);
    } else {
        // If nothing passed, run GUI
        gui::run_gui();
    }
    
    // The program is now closing
    config::save_config_file();
    
    if !updater::run_install_script(false) {
        // Only run if the install script hasn't ran
        logic::clean_up(); // Remove the temporary directory if one has been created
    }
}
