mod config;

fn pkg_count(pkgs: &mut usize, commands: &[&str], args: &[&str]) {
	for cmd in commands {
		let list = match std::process::Command::new(cmd).args(args).output() {
			Ok(output) => String::from_utf8(output.stdout).unwrap(),
			_ => "".to_string(),
		};
		*pkgs += list.split('\n').count() - 1;
	}
}

fn main() {
	let hostname = std::fs::read_to_string("/proc/sys/kernel/hostname").unwrap();
	print!("\x1b[1;36m{}", std::env::var("USER").unwrap());
	print!("\x1b[0m@\x1b[1;36m{}", hostname);

	let os_release = std::fs::read_to_string("/etc/os-release").unwrap();

	for line in os_release.lines() {
		if line.starts_with("NAME=") && !config::OS.is_empty() {
			print!("{}{}", config::OS, &line[5..line.len()].replace('"', ""));
			println!(" {}", std::env::consts::ARCH);
		}
	}

	if !config::UPTIME.is_empty() {
		let uptime = std::fs::read_to_string("/proc/uptime").unwrap();
		let uptime: u64 = uptime.split('.').collect::<Vec<&str>>()[0].parse().unwrap();
		print!("{}", config::UPTIME);
		println!("{}h {}m", uptime / 60 / 60, uptime / 60 % 60);
	}

	if !config::KERNEL.is_empty() {
		let kernel_file = std::fs::read_to_string("/proc/version").unwrap();
		let kernel = kernel_file.split(' ').collect::<Vec<&str>>()[2];
		println!("{}{}", config::KERNEL, kernel);
	}

	let shell_name = std::env::var("SHELL");
	if let (Ok(shell_name), false) = (shell_name, config::SHELL.is_empty()) {
		let shell_vec = shell_name.split('/').collect::<Vec<&str>>();
		println!("{}{}", config::SHELL, shell_vec.last().unwrap());
	}
	let desktop = std::env::var("DESKTOP_SESSION");
	if let (Ok(desktop), false) = (desktop, config::DE.is_empty()) {
		let desktop_vec = desktop.split('/').collect::<Vec<&str>>();
		println!("{}{}", config::DE, desktop_vec.last().unwrap());
	}

	if !config::PACKAGES.is_empty() {
		let mut pkgs = 0;
		pkg_count(&mut pkgs, &["xbps-query"], &["-l"]);
		pkg_count(&mut pkgs, &["flatpak", "apt", "kiss"], &["list"]);
		pkg_count(&mut pkgs, &["pacman"], &["-Qq", "--color", "never"]);
		pkg_count(&mut pkgs, &["rpm"], &["-qa"]);
		pkg_count(&mut pkgs, &["apk"], &["info"]);
		pkg_count(&mut pkgs, &["opkg"], &["list-installed"]);
		pkg_count(&mut pkgs, &["lvu"], &["installed"]);
		println!("{}{}", config::PACKAGES, pkgs);
	}

	let mut base = 30u8;
	for enabled in &[config::SHOW_REGULAR_COLORS, config::SHOW_INTENSE_COLORS] {
		if *enabled {
			for color in base..=(base + 7) {
				print!("\x1b[0;{}m{}", color, config::COLOR_STRING);
			}
			println!()
		}
		base += 60;
	}

	println!("\x1b[38;2;128;128;128msfetch   v1.2.2");
}
