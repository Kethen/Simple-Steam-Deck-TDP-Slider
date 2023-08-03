// see https://lists.freedesktop.org/archives/amd-gfx/2021-February/059075.html
enum DeviceType{
	Slow,
	Fast,
}

fn probe_hwmon_path () -> Result<String, String>{
	let path_obj = std::path::Path::new("/sys/devices/pci0000:00/0000:00:08.1/0000:04:00.0/hwmon");
	if !path_obj.is_dir(){
		return Err(format!("{} is not a directory", path_obj.display()));
	}

	match path_obj.read_dir(){
		Ok(read_dir) => {
			for entry in read_dir{
				match entry{
					Ok(dir_entry) => {
						return Ok(format!("{}", dir_entry.path().display()));
					},
					Err(e) => {
						return Err(format!("Failed enumrating content of {}, {}", path_obj.display(), e));
					}
				}
			}
			return Err(format!("{} is empty", path_obj.display()));
		},
		Err(e) => {
			return Err(format!("Failed enumerating content of {}, {}", path_obj.display(), e));
		}
	}
}

fn probe_device(base_name: &str) -> Result<String, String>{
	let hwmon_path = match probe_hwmon_path(){
		Ok(p) => p,
		Err(e) => {
			return Err(e);
		}
	};

	let path = format!("{}/{}", hwmon_path, base_name);
	let path_obj = std::path::Path::new(&path);
	if path_obj.exists(){
		return Ok(format!("{}", path));
	}else{
		return Err(format!("{} does not exist", path));
	}
}

fn probe_slow_device() -> Result<String, String>{
	return probe_device("power1_cap");
}

fn probe_fast_device() -> Result<String, String>{
	return probe_device("power2_cap");
}

fn get_device_micro_watt(device_type: DeviceType) -> Result<u32, String>{
	let device_path = match device_type{
		DeviceType::Slow => {
			probe_slow_device()
		},
		DeviceType::Fast => {
			probe_fast_device()
		}
	};

	let device_path = match device_path{
		Ok(p) => p,
		Err(e) => {
			return Err(e);
		}
	};

	match std::fs::read(&device_path){
		Ok(s) => {
			let read_string = match std::string::String::from_utf8(s){
				Ok(rs) => rs,
				Err(e) => {
					return Err(format!("Failed reading string from {}, {}", device_path, e));
				}
			};
			match read_string.trim().parse::<u32>() {
				Ok(val) => {
					return Ok(val);
				},
				Err(e) => {
					return Err(format!("Failed parsing {} as u32, {}", read_string, e));
				}
			}
		},
		Err(e) => {
			return Err(format!("Failed reading {}, {}", device_path, e));
		}
	}
}

fn set_device_micro_watt(device_type: DeviceType, power_micro_watt: u32) -> Result<(), String>{
	let device_path = match device_type{
		DeviceType::Slow => {
			probe_slow_device()
		},
		DeviceType::Fast => {
			probe_fast_device()
		}
	};

	let device_path = match device_path{
		Ok(p) => p,
		Err(e) => {
			return Err(e);
		}
	};

	match std::fs::write(&device_path, format!("{}", power_micro_watt)){
		Ok(_) => {},
		Err(e) => {
			return Err(format!("Failed writing to {}, {}", device_path, e));
		}
	}

	return Ok(());
}

pub fn get_slow_device_micro_watt() -> Result<u32, String> {
	return get_device_micro_watt(DeviceType::Slow);
}

pub fn get_fast_device_micro_watt() -> Result<u32, String> {
	return get_device_micro_watt(DeviceType::Fast);
}

pub fn set_slow_device_micro_watt(power_micro_watt: u32) -> Result<(), String>{
	return set_device_micro_watt(DeviceType::Slow, power_micro_watt);
}

pub fn set_fast_device_micro_watt(power_micro_watt: u32) -> Result<(), String>{
	return set_device_micro_watt(DeviceType::Fast, power_micro_watt);
}


pub struct BacklightDevice{
	pub path:String,
	pub max_brightness:u32
}

pub fn probe_backlight_device() -> Result<BacklightDevice, String>{
	let re = regex::Regex::new("^/sys/class/backlight/amdgpu_bl[0-9]+$").unwrap();
	let path_obj = std::path::Path::new("/sys/class/backlight");

	match path_obj.read_dir(){
		Ok(read_dir) => {
			for entry in read_dir{
				match entry{
					Ok(dir_entry) => {
						let path = format!("{}", dir_entry.path().display());
						if re.is_match(&path){
							let brightness_path = format!("{}/brightness", path);
							if !std::path::Path::new(&brightness_path).exists(){
								return Err(format!("{} has no brightness node", path))
							}
							let max_brightness_path = format!("{}/max_brightness", path);
							let max_brightness_bytes = match std::fs::read(&max_brightness_path){
								Ok(b) => b,
								Err(e) => {
									return Err(format!("Failed reading {}, {}", max_brightness_path, e));
								}
							};
							let max_brightness_string = match std::string::String::from_utf8(max_brightness_bytes){
								Ok(s) => s,
								Err(e) => {
									return Err(format!("Failed parsing {} as string, {}", max_brightness_path, e));
								}
							};
							let max_brightness = match max_brightness_string.trim().parse::<u32>(){
								Ok(m) => m,
								Err(e) => {
									return Err(format!("Failed parsing {} as u32, {}", max_brightness_path, e));
								}
							};
							return Ok(
								BacklightDevice{
									path:brightness_path,
									max_brightness:max_brightness,
								}
							);
						}
					},
					Err(e) => {
						return Err(format!("Failed enumerating content of {}, {}", path_obj.display(), e));
					}
				}
			}
			return Err(format!("amdgpu backlight not found"))
		},
		Err(e) => {
			return Err(format!("Failed enumerating content of {}, {}", path_obj.display(), e));
		}
	}
}

pub fn get_brightness(device: &BacklightDevice) -> Result<u32, String>{
	let brightness_bytes = match std::fs::read(&device.path){
		Ok(b) => b,
		Err(e) => {
			return Err(format!("Failed reading {}, {}", device.path, e));
		}
	};
	let brightness_string = match std::string::String::from_utf8(brightness_bytes){
		Ok(s) => s,
		Err(e) => {
			return Err(format!("Failed parsing {} as string, {}", device.path, e));
		}
	};
	let brightness = match brightness_string.trim().parse::<u32>(){
		Ok(m) => m,
		Err(e) => {
			return Err(format!("Failed parsing {} as u32, {}", device.path, e));
		}
	};
	return Ok(brightness);
}

pub fn set_brightness(device: &BacklightDevice, brightness:u32) -> Result<(), String>{
	if brightness > device.max_brightness{
		return Err(format!("Desired brightness {} is bigger than max brightness {}", brightness, device.max_brightness));
	}
	match std::fs::write(&device.path, format!("{}", brightness)){
		Ok(_) => {},
		Err(e) => {
			return Err(format!("Failed writing to {}, {}", device.path, e));
		}
	}
	return Ok(());
}
