use iced::widget::{column, row, /*button,*/ text, vertical_space};
use iced::{Alignment/*, Length*/};
use iced_audio::{HSlider};

use iced::Application;

mod operations;

struct TdpSlider{
	slow_tdp_micro_watt: u32,
	fast_tdp_micro_watt: u32,

	tdp_range: iced_audio::IntRange,
	slider_tick_marks:iced_audio::tick_marks::Group,
	slider_text_marks:iced_audio::text_marks::Group,

	backlight_brightness_range: iced_audio::IntRange,
	backlight_brightness:u32,
	backlight_device: operations::BacklightDevice,
	backlight_brightness_tick_marks:iced_audio::tick_marks::Group,
	backlight_brightness_text_marks:iced_audio::text_marks::Group,
}

#[derive(Debug, Clone)]
enum Message{
	Event(iced::Event),
	ChangeFastTdpWatt(iced_audio::Normal),
	ChangeSlowTdpWatt(iced_audio::Normal),
	ChangeBrightness(iced_audio::Normal),
	Close,
}

impl iced::Application for TdpSlider{
	type Message = Message;
	type Executor = iced::executor::Default;
	type Flags = ();
	type Theme = iced::Theme;
	
	fn new(_flags:()) -> (Self, iced::Command<Message>){
		let slow_tdp_micro_watt = operations::get_slow_device_micro_watt().unwrap();
		let fast_tdp_micro_watt = operations::get_fast_device_micro_watt().unwrap();
		let tdp_range = iced_audio::IntRange::new(3, 15);
		let slider_tick_marks = iced_audio::tick_marks::Group::evenly_spaced(15 - 3 + 1, iced_audio::native::tick_marks::Tier::Three);
		let slider_text_marks = iced_audio::text_marks::Group::min_max_and_center("3", "15", "9");
		let backlight_device = operations::probe_backlight_device().unwrap();
		let backlight_brightness = operations::get_brightness(&backlight_device).unwrap();
		let backlight_brightness_range = iced_audio::IntRange::new(0, i32::try_from(backlight_device.max_brightness).unwrap());
		let backlight_brightness_tick_marks = iced_audio::tick_marks::Group::min_max(iced_audio::native::tick_marks::Tier::Three);
		let backlight_brightness_text_marks = iced_audio::text_marks::Group::min_max("0", &format!("{}", backlight_device.max_brightness));
		return (
			TdpSlider{
				slow_tdp_micro_watt:slow_tdp_micro_watt,
				fast_tdp_micro_watt:fast_tdp_micro_watt,
				tdp_range:tdp_range,
				slider_tick_marks:slider_tick_marks,
				slider_text_marks:slider_text_marks,
				backlight_device:backlight_device,
				backlight_brightness:backlight_brightness,
				backlight_brightness_range:backlight_brightness_range,
				backlight_brightness_tick_marks:backlight_brightness_tick_marks,
				backlight_brightness_text_marks:backlight_brightness_text_marks,
			},
			iced::Command::none()
		)
	}

	fn title(&self) -> String{
		return format!("Simple Steam Deck TDP Slider");
	}

	fn update(&mut self, message: Message) -> iced::Command<Message>{
		match message{
			Message::Event(e) => {
				match e{
					iced::Event::Window(e) => {
						match e{
							iced::window::Event::CloseRequested => {
								return self.update(Message::Close);
							},
							iced::window::Event::Focused => {
								self.slow_tdp_micro_watt = operations::get_slow_device_micro_watt().unwrap();
								self.fast_tdp_micro_watt = operations::get_fast_device_micro_watt().unwrap();
								self.backlight_brightness = operations::get_brightness(&self.backlight_device).unwrap();
							},
							_ => {},
						}
					},
					iced::Event::Mouse(e) => {
						match e{
							iced::mouse::Event::ButtonPressed(e) => {
								match e{
									iced::mouse::Button::Left => {
										return iced::window::drag();
									},
									_ => {}
								}
							},
							_ => {},
						}
					}
					_ => {}
				}
			},
			Message::ChangeSlowTdpWatt(tdp_watt) => {
				self.slow_tdp_micro_watt = u32::try_from(self.tdp_range.unmap_to_value(tdp_watt)).unwrap() * 1000000;
				operations::set_slow_device_micro_watt(self.slow_tdp_micro_watt).unwrap();
			},
			Message::ChangeFastTdpWatt(tdp_watt) => {
				self.fast_tdp_micro_watt = u32::try_from(self.tdp_range.unmap_to_value(tdp_watt)).unwrap() * 1000000;
				operations::set_fast_device_micro_watt(self.fast_tdp_micro_watt).unwrap();
			},
			Message::ChangeBrightness(brightness) => {
				self.backlight_brightness = u32::try_from(self.backlight_brightness_range.unmap_to_value(brightness)).unwrap();
				operations::set_brightness(&self.backlight_device, self.backlight_brightness).unwrap();
			}
			Message::Close => {
				std::process::exit(0);
			}
		}
		return iced::Command::none();
	}

	fn subscription(&self) -> iced::Subscription<Message>{
		return iced::subscription::events().map(Message::Event);
	}

	fn view(&self) -> iced::Element<Message>{
		let slow_slider_param = self.tdp_range.normal_param(i32::try_from(self.slow_tdp_micro_watt / 1000000).unwrap(), 15);
		let fast_slider_param = self.tdp_range.normal_param(i32::try_from(self.fast_tdp_micro_watt / 1000000).unwrap(), 15);
		let backlight_brightness_slider_param = self.backlight_brightness_range.normal_param(i32::try_from(self.backlight_brightness).unwrap(), 0);
		column![
			/* does not work with wgpu
			column![row![
				button("X").on_press(Message::Close)
			]].align_items(Alignment::End)
			.width(Length::Fill),
			*/
			row![
				column![
					text(format!("Long term TDP: {}W", self.slow_tdp_micro_watt / 1000000)),
					vertical_space(8),
					HSlider::new(slow_slider_param, Message::ChangeSlowTdpWatt).tick_marks(&self.slider_tick_marks).text_marks(&self.slider_text_marks)
				]
			],
			vertical_space(16),
			row![
				column![
					text(format!("Short term TDP: {}W", self.fast_tdp_micro_watt / 1000000)),
					vertical_space(8),
					HSlider::new(fast_slider_param, Message::ChangeFastTdpWatt).tick_marks(&self.slider_tick_marks).text_marks(&self.slider_text_marks)
				]
			],
			vertical_space(16),
			row![
				column![
					text(format!("Backlight brightness: {}", self.backlight_brightness)),
					vertical_space(8),
					HSlider::new(backlight_brightness_slider_param, Message::ChangeBrightness).tick_marks(&self.backlight_brightness_tick_marks).text_marks(&self.backlight_brightness_text_marks)
				]
			]
		].align_items(Alignment::Start)
		.padding(10)
		.into()
	}

	fn theme(&self) -> iced::Theme{
		return iced::theme::Theme::Dark;
	}
}

fn main() {
	std::env::remove_var("WAYLAND_DISPLAY");

	let mut settings = iced::Settings::default();
	settings.window.resizable = false;
	settings.window.size = (400, 200);
	/* does not work with wgpu
	settings.window.decorations = false;
	settings.window.transparent = true;
	*/
	settings.exit_on_close_request = false;
	TdpSlider::run(settings).unwrap();
}
