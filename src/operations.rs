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
