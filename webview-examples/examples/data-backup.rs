#![windows_subsystem = "windows"]

extern crate web_view;

use web_view::*;
use std::process::Command;

#[cfg(target_os = "windows")]
static SOURCE_DIR: &str = "%homepath%\\AppData\\LocalLow\\IronGate\\Valheim\\";
#[cfg(target_os = "windows")]
static DESTINATION_DIR: &str = "%homepath%\\Backup\\";
#[cfg(target_os = "linux")]
static SOURCE_DIR: &str = "$HOME/.config/unity3d/IronGate/Valheim/";
#[cfg(target_os = "linux")]
static DESTINATION_DIR: &str = "$HOME/Backup/";

static CHARA_DIR_NAME: &str = "characters";
static WORLD_DIR_NAME: &str = "worlds";

fn main() {
	make_dir();
	web_view::builder()
		.title("Data Backup")
		.content(Content::Html(&format!("{}{}{}", HTML_FIRST_HALF, HTML_JS_EXTERNAL, HTML_SECOND_HALF)))
		.size(800, 600)
		.resizable(true)
		.debug(true)
		.user_data("")
		.invoke_handler(|_webview, arg| {
			match arg {
				"save_all" => {
					save(CHARA_DIR_NAME);
					save(WORLD_DIR_NAME);
				},
				"load_all" => {
					load(CHARA_DIR_NAME);
					load(WORLD_DIR_NAME);
				},
				"save_characters" => {
					save(CHARA_DIR_NAME);
				},
				"load_characters" => {
					load(CHARA_DIR_NAME);
				},
				"save_worlds" => {
					save(WORLD_DIR_NAME);
				},
				"load_worlds" => {
					load(WORLD_DIR_NAME);
				},
				_ => (),
			}

			Ok(())
		})
		.run()
		.unwrap();
}

fn make_dir() {
	if cfg!(windows) {
		let _ = Command::new("cmd")
			.args(&["/C", &format!("mkdir {} > NUL 2>&1", DESTINATION_DIR)])
			.output()
			.expect("failed to execute process");
	} else {
		let _ = Command::new("sh")
			.arg("-c")
			.arg(&format!("mkdir -p {}", DESTINATION_DIR))
			.output()
			.expect("failed to execute process");
	}
}

fn save(dir_name: &str) {
	if cfg!(windows) {
		let _ = Command::new("cmd")
			.args(&["/C", &format!("xcopy {}{} {}{} /I /Y",
				SOURCE_DIR,
				dir_name,
				DESTINATION_DIR,
				dir_name)])
			.output()
			.expect("failed to execute process");
	} else {
		let _ = Command::new("sh")
			.arg("-c")
			.arg(&format!("cp -fR {}{} {}",
				SOURCE_DIR,
				dir_name,
				DESTINATION_DIR))
			.output()
			.expect("failed to execute process");
	}
}

fn load(dir_name: &str) {
	if cfg!(windows) {
		let _ = Command::new("cmd")
			.args(&["/C", &format!("xcopy {}{} {}{} /I /Y",
				DESTINATION_DIR,
				dir_name,
				SOURCE_DIR,
				dir_name)])
			.output()
			.expect("failed to execute process");
	} else {
		let _ = Command::new("sh")
			.arg("-c")
			.arg(&format!("cp -fR {}{} {}",
				DESTINATION_DIR,
				dir_name,
				SOURCE_DIR))
			.output()
			.expect("failed to execute process");
	}
}

#[cfg(target_os = "windows")]
const HTML_JS_EXTERNAL: &str = "external.invoke(arg);";
#[cfg(target_os = "linux")]
const HTML_JS_EXTERNAL: &str = "window.webkit.messageHandlers.external.postMessage(arg);";

const HTML_FIRST_HALF: &str = r#"
<!doctype html>
<html>
	<body>
		<div>
			<h2>All</h2>
			<button onclick="run('save_all')">save</button>
			<button onclick="run('load_all')">load</button>
		</div>
		<div>
			<p>&nbsp;</p>
		</div>
		<div>
			<h2>Characters</h2>
			<button onclick="run('save_characters')">save</button>
			<button onclick="run('load_characters')">load</button>
		</div>
		<div>
			<p>&nbsp;</p>
		</div>
		<div>
			<h2>Worlds</h2>
			<button onclick="run('save_worlds')">save</button>
			<button onclick="run('load_worlds')">load</button>
		</div>
		<div>
			<p>&nbsp;</p>
			<p>&nbsp;</p>
		</div>
		<div>
			<p>
				<h5 id="message"></h5>
			</p>
		</div>
		<script>
var run = function(arg) {
	document.getElementById('message').innerText = 'wait a moment.';
"#;
const HTML_SECOND_HALF: &str = r#"
	document.getElementById('message').innerText = arg.replace('_', ' ') + ' complete.';
};
		</script>
	</body>
</html>
"#;
