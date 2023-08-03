mod operations;

fn main() {
	operations::set_slow_device_micro_watt(5000000).unwrap();
	operations::set_fast_device_micro_watt(7000000).unwrap();


	println!("{}", operations::get_slow_device_micro_watt().unwrap());
	println!("{}", operations::get_fast_device_micro_watt().unwrap());
}
