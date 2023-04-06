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
async fn download_potatosmp() -> String {
	let has_curseforge = user_has_curseforge();

	if has_curseforge {
		let home_dir = home_dir().expect("Failed to retrieve home directory.").as_path().display().to_string();
		let potato_smp_folder = home_dir.to_owned() + "/curseforge/minecraft/instances/PotatoSMP";
		let documents_folder = home_dir.to_owned() + "/documents/TEMPPOTATOSMP";
		let has_potatosmp_installed = Path::new(&potato_smp_folder).is_dir();

		let zip_target = "https://codeload.github.com/Minecraft-SMPs/PotatoSMP/legacy.zip/main";
		let response = match reqwest::get(zip_target).await {
			Ok(response) => response,
			Err(_error) => {
				return ("Request failed, do you have an internet connection?").into()
			}
		};

		let body = response.bytes().await.expect("body invalid");
		let content = Cursor::new(body);

		if has_potatosmp_installed {
			match fs::remove_dir_all(potato_smp_folder.to_owned() + "/mods") {
				Ok(response) => response,
				Err(_error) => {
					println!("No mods folder found.")
				}
			}
			zip_extract::extract(content, &PathBuf::from(&documents_folder), true).expect("Unexpected error");

			fs::rename(documents_folder.to_owned() + "/mods", potato_smp_folder.to_owned() + "/mods").expect("Unexpected error");
			match fs::remove_dir_all(&documents_folder) {
				Ok(response) => response,
				Err(_error) => {
					println!("Failed to remove folder.")
				}
			}
		} else {
			zip_extract::extract(content, &PathBuf::from(&potato_smp_folder), true).expect("Unexpected error");
		}

		"Successfully installed Potato SMP modpack!".into()
	} else {
		"You're missing CurseForge! Please download Curseforge before continuing.".into()
	}
}

fn main() {
	tauri::Builder::default()
	  .invoke_handler(tauri::generate_handler![download_potatosmp])
	  .run(tauri::generate_context!())
	  .expect("failed to run app");
}