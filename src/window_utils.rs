use bevy::{
	prelude::{
		Resource, Res, ResMut,
		Single, With,
		Window, WindowPosition,
		IVec2, UVec2,
		MessageReader,
	},
	window::{
		Monitor, PrimaryMonitor, WindowResized, WindowResolution
	},
};

// =============================================================================
// Window size and position
// =============================================================================

#[derive(Resource)]
pub struct VirtualResolution(pub UVec2);
impl Default for VirtualResolution {
	fn default() -> Self {
		Self(UVec2::new(1920, 1080))	// Should probably default to monitor size.
	}
}

#[derive(Resource)]
pub struct WindowScaling(pub bool, pub f32);
impl Default for WindowScaling {
	fn default() -> Self {
		Self(false, 1.0)
	}
}

#[derive(Resource)]
pub struct WindowAwaitsCentering(pub bool);
impl Default for WindowAwaitsCentering {
	fn default() -> Self {
		Self(false)
	}
}

pub fn init_window_resolution_scale_factor(
	mut window: Single<&mut Window>,
	virtual_resolution: Res<VirtualResolution>,
	mut window_awaits_centering: ResMut<WindowAwaitsCentering>,
	window_scaling: Res<WindowScaling>,
) {
	window.resolution =
		if window_scaling.0 {
			WindowResolution::from(virtual_resolution.0).with_scale_factor_override(window_scaling.1)
		} else {
			WindowResolution::from(virtual_resolution.0)
		};

	window_awaits_centering.0 = true;
}

pub fn on_window_resized(
	mut resize_reader: MessageReader<WindowResized>,
	mut window: Single<&mut Window>,
	monitor: Single<&Monitor, With<PrimaryMonitor>>,
	mut window_awaits_centering: ResMut<WindowAwaitsCentering>,
	window_scaling: Res<WindowScaling>,
) {
	if window_awaits_centering.0 {
		for e in resize_reader.read() {
			window_awaits_centering.0 = false;

			let monitor_width = monitor.physical_width as i32;
			let monitor_height = monitor.physical_height as i32;
			let monitor_offset = monitor.physical_position;

			let window_width = (e.width * window_scaling.1) as i32;
			let window_height = (e.height * window_scaling.1) as i32;

			let pos_x = monitor_offset.x + (monitor_width - window_width) / 2;
			let pos_y = monitor_offset.y + (monitor_height - window_height) / 2;
			
			window.position = WindowPosition::At(IVec2::new(pos_x, pos_y));
		}
	}
}