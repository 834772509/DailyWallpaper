// 不显示窗口
#![windows_subsystem = "windows"]

use std::{env, fs};
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::thread::{sleep, Thread};
use std::time::Duration;
use anyhow::Result;
use serde_json::{Value};
use winapi::shared::minwindef::TRUE;
use winapi::um::winnt::PVOID;

fn main() {
    let wallpaper_path = PathBuf::from(r"C:\Windows\Web\Wallpaper\DailyWallpaper");
    let wallpaper_file = wallpaper_path.join("wallpaper.jpg");
    if !wallpaper_path.exists() && fs::create_dir_all(&wallpaper_path).is_err() {
        println!("创建壁纸目录失败");
        return;
    }

    println!("正在获取壁纸...");
    let mut img_url = String::new();
    while img_url.is_empty() {
        if let Ok(url) = get_bing_picture_url() {
            img_url = url;
        }
        // 每隔三分钟请求一次
        std::thread::sleep(Duration::from_millis(60 * 3))
    }

    println!("正在保存壁纸...");
    if save_wallpaper(&img_url, &wallpaper_file).is_err() {
        println!("保存壁纸失败");
        return;
    }

    println!("正在设置壁纸...");
    if !set_wallpaper(&wallpaper_file) {
        println!("设置壁纸失败");
        return;
    }
    println!("壁纸设置完成");

    if !is_self_start() && !add_self_Start() {
        println!("增加开启自启动失败");
        return;
    }
}

/// 增加开机自启动
fn add_self_Start() -> bool {
    let path = Path::new(r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\StartUp").join(env::current_exe().unwrap().file_name().unwrap());
    std::fs::copy(env::current_exe().unwrap(), path).unwrap_or(0) != 0
}

/// 是否开机自启动
fn is_self_start() -> bool {
    let path = Path::new(r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\StartUp").join(env::current_exe().unwrap().file_name().unwrap());
    path.exists()
}

/// 获取必应壁纸链接
fn get_bing_picture_url() -> Result<String> {
    let url = "https://cn.bing.com/HPImageArchive.aspx?format=js&idx=-1&n=1";
    let body = reqwest::blocking::get(url)?.text()?;
    let api_info: Value = serde_json::from_str(&body)?;
    let img_url = format!("https://cn.bing.com{}", api_info["images"][0]["url"].to_string().trim_matches('"'));
    Ok(img_url)
}

/// 保存壁纸
fn save_wallpaper(url: &str, file_path: &Path) -> Result<()> {
    let file = reqwest::blocking::get(url)?.bytes()?;
    fs::write(file_path, file)?;
    Ok(())
}

/// 设置壁纸
fn set_wallpaper(file_path: &Path) -> bool {
    let file_path: Vec<u16> = std::ffi::OsStr::new(file_path).encode_wide().chain(once(0)).collect();
    let result = unsafe { winapi::um::winuser::SystemParametersInfoW(20, 0, file_path.as_ptr() as PVOID, 0) };
    return result == TRUE;
}
