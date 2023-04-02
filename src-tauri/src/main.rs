#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::io::Cursor;
use std::path::{PathBuf, Path};
use std::fs;
use tauri::api::path::home_dir;

// NOTE: THIS PROJECT IS CURRENTLY VERY MESSY & ONLY MEANT FOR ONE PURPOSE
// Currently, the bare minimum is that it "works."

fn user_has_curseforge() -> bool {
	let home_dir = home_dir().expect("Failed to retrieve home directory.").as_path().display().to_string();
	let target = home_dir.to_owned() + "/curseforge";
	let has_curseforge = Path::new(&target).is_dir();

	return has_curseforge;
}

#[tauri::command]
fn check_potatosmp_updates() {
	let has_curseforge = user_has_curseforge();
	let home_dir = home_dir().expect("Failed to retrieve home directory.").as_path().display().to_string();
	let target = home_dir.to_owned() + "/curseforge/minecraft/instances";
	let has_potatosmp_installed = Path::new(&target).is_dir();

	if has_potatosmp_installed {

	}
}

#[tauri::command]
async fn download_potatosmp() -> String {
	let has_curseforge = user_has_curseforge();

	if has_curseforge {
		let home_dir = home_dir().expect("Failed to retrieve home directory.").as_path().display().to_string();
		let target = home_dir.to_owned() + "/curseforge/minecraft/instances/PotatoSMP";
		let has_potatosmp_installed = Path::new(&target).is_dir();

		let zip_target = "https://codeload.github.com/Parritz/PotatoSMP/legacy.zip/main";
		let response_result = reqwest::get(zip_target).await;
		let response = match response_result {
			Ok(response) => response,
			Err(_error) => {
				return ("Request failed, do you have an internet connection?").into()
			}
		};

		let body = response.bytes().await.expect("body invalid");
		let content = Cursor::new(body);

		if has_potatosmp_installed {
			fs::remove_dir_all(&target).expect("How did this manage to fail???");
		}

		zip_extract::extract(content, &PathBuf::from(&target), true).expect("Error");
		"Successfully installed Potato SMP modpack!".into()
	} else {
		"You're missing CurseForge! Please download Curseforge before continuing.".into()
	}
}

// Also in main.rs
fn main() {
	tauri::Builder::default()
	  // This is where you pass in your commands
	  .invoke_handler(tauri::generate_handler![download_potatosmp])
	  .run(tauri::generate_context!())
	  .expect("failed to run app");
}