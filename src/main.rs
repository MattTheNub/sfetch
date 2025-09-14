use {config::*, std::env::consts::ARCH, std::env::var, std::fs::*, std::process::Command};
mod config;

fn pkg_count(pkgs: &mut usize, commands: &[&str], args: &[&str]) {
	for cmd in commands {
		let list = Command::new(cmd).args(args).output();
		let list = list.map(|output| String::from_utf8(output.stdout).unwrap());
		*pkgs += list.unwrap_or_default().split('\n').count() - 1;
	}
}

fn main() {
	let hn = read_to_string("/proc/sys/kernel/hostname").unwrap();
	print!("\x1b[1;36m{}\x1b[0m@\x1b[1;36m{}", var("USER").unwrap(), hn);

	let mut os_release = read_to_string("/etc/os-release").unwrap();
	os_release.retain(|_| !OS.is_empty());

	if let Some(line) = os_release.lines().find(|line| line.starts_with("NAME=")) {
		println!("{}{} {}", OS, &line[5..line.len()].replace('"', ""), ARCH);
	}

	if !UPTIME.is_empty() {
		let uptime = read_to_string("/proc/uptime").unwrap();
		let uptime = uptime.split('.').next().unwrap().parse::<u64>().unwrap() / 60;
		println!("{}{}h {}m", UPTIME, uptime / 60, uptime % 60);
	}

	if !KERNEL.is_empty() {
		let kernel_file = read_to_string("/proc/version").unwrap();
		let kernel = kernel_file.split(' ').nth(2).unwrap();
		println!("{}{}", KERNEL, kernel);
	}

	for (val, prefix) in [(var("SHELL"), SHELL), (var("DESKTOP_SESSION"), DE)] {
		if let (Ok(val), false) = (val, prefix.is_empty()) {
			println!("{}{}", prefix, val.split('/').next_back().unwrap());
		}
	}

	if !PACKAGES.is_empty() {
		let mut pkgs = 0;
		pkg_count(&mut pkgs, &["xbps-query"], &["-l"]);
		pkg_count(&mut pkgs, &["flatpak", "kiss"], &["list"]);
		pkg_count(&mut pkgs, &["apt"], &["list", "--installed"]);
		pkg_count(&mut pkgs, &["pacman"], &["-Qq", "--color", "never"]);
		pkg_count(&mut pkgs, &["rpm"], &["-qa"]);
		pkg_count(&mut pkgs, &["apk"], &["info"]);
		pkg_count(&mut pkgs, &["opkg"], &["list-installed"]);
		pkg_count(&mut pkgs, &["lvu"], &["installed"]);
		println!("{}{}", PACKAGES, pkgs);
	}

	let cpu = read_to_string("/proc/cpuinfo").unwrap_or_default();
	let cpu = cpu.lines().find(|l| l.starts_with("model name"));
	if let (Some(cpu), false) = (cpu, CPU.is_empty()) {
		println!("{}{}", CPU, cpu.split(':').nth(1).unwrap().trim());
	}

	if !MEM.is_empty() {
		let mem_file = read_to_string("/proc/meminfo").unwrap();

		let total = mem_file.lines().find(|l| l.starts_with("MemTotal:"));
		let avail = mem_file.lines().find(|l| l.starts_with("MemAvailable:"));

		let total = total.unwrap().split(':').nth(1).unwrap().rsplit(' ').nth(1);
		let avail = avail.unwrap().split(':').nth(1).unwrap().rsplit(' ').nth(1);

		let total_num = total.unwrap().parse::<u64>().unwrap() / 1024;
		let avail_num = total_num - avail.unwrap().parse::<u64>().unwrap() / 1024;

		println!("{}{} / {} MiB", MEM, avail_num, total_num);
	}

	let mut base = 30u8;
	for enabled in &[SHOW_REGULAR_COLORS, SHOW_INTENSE_COLORS] {
		if *enabled {
			(base..=(base + 7)).for_each(|color| print!("\x1b[0;{}m{}", color, COLOR_STRING));
			println!();
		}
		base += 60;
	}

	println!("\x1b[38;2;128;128;128msfetch   v1.4.0");
}
