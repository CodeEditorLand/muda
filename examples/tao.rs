// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![allow(unused)]
use muda::{
	accelerator::{Accelerator, Code, Modifiers},
	dpi::{PhysicalPosition, Position},
	AboutMetadata,
	CheckMenuItem,
	ContextMenu,
	IconMenuItem,
	Menu,
	MenuEvent,
	MenuItem,
	PredefinedMenuItem,
	Submenu,
};
#[cfg(target_os = "macos")]
use tao::platform::macos::WindowExtMacOS;
#[cfg(target_os = "linux")]
use tao::platform::unix::WindowExtUnix;
#[cfg(target_os = "windows")]
use tao::platform::windows::{EventLoopBuilderExtWindows, WindowExtWindows};
use tao::{
	event::{ElementState, Event, MouseButton, WindowEvent},
	event_loop::{ControlFlow, EventLoopBuilder},
	window::{Window, WindowBuilder},
};

enum UserEvent {
    MenuEvent(muda::MenuEvent),
}

fn main() {
    let mut event_loop_builder = EventLoopBuilder::<UserEvent>::with_user_event();

	let menu_bar = Menu::new();

    // setup accelerator handler on Windows
    #[cfg(target_os = "windows")]
    {
        let menu_bar = menu_bar.clone();
        event_loop_builder.with_msg_hook(move |msg| {
            use windows_sys::Win32::UI::WindowsAndMessaging::{TranslateAcceleratorW, MSG};
            unsafe {
                let msg = msg as *const MSG;
                let translated = TranslateAcceleratorW((*msg).hwnd, menu_bar.haccel() as _, msg);
                translated == 1
            }
        });
    }

		event_loop_builder.with_msg_hook(move |msg| {
			use windows_sys::Win32::UI::WindowsAndMessaging::{TranslateAcceleratorW, MSG};

    // set a menu event handler that wakes up the event loop
    let proxy = event_loop.create_proxy();
    muda::MenuEvent::set_event_handler(Some(move |event| {
        proxy.send_event(UserEvent::MenuEvent(event));
    }));

    let window = WindowBuilder::new()
        .with_title("Window 1")
        .build(&event_loop)
        .unwrap();
    let window2 = WindowBuilder::new()
        .with_title("Window 2")
        .build(&event_loop)
        .unwrap();

				let translated = TranslateAcceleratorW((*msg).hwnd, menu_bar.haccel() as _, msg);

				translated == 1
			}
		});
	}

	let event_loop = event_loop_builder.build();

	let window = WindowBuilder::new().with_title("Window 1").build(&event_loop).unwrap();

	let window2 = WindowBuilder::new().with_title("Window 2").build(&event_loop).unwrap();

	#[cfg(target_os = "macos")]
	{
		let app_m = Submenu::new("App", true);

		menu_bar.append(&app_m);

		app_m.append_items(&[
			&PredefinedMenuItem::about(None, None),
			&PredefinedMenuItem::separator(),
			&PredefinedMenuItem::services(None),
			&PredefinedMenuItem::separator(),
			&PredefinedMenuItem::hide(None),
			&PredefinedMenuItem::hide_others(None),
			&PredefinedMenuItem::show_all(None),
			&PredefinedMenuItem::separator(),
			&PredefinedMenuItem::quit(None),
		]);
	}

	let file_m = Submenu::new("&File", true);

	let edit_m = Submenu::new("&Edit", true);

	let window_m = Submenu::new("&Window", true);

	menu_bar.append_items(&[&file_m, &edit_m, &window_m]);

	let custom_i_1 = MenuItem::with_id(
		"custom-i-1",
		"C&ustom 1",
		true,
		Some(Accelerator::new(Some(Modifiers::ALT), Code::KeyC)),
	);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                window_id,
                ..
            } => {
                window_cursor_position.x = position.x;
                window_cursor_position.y = position.y;
            }
            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        state: ElementState::Released,
                        button: MouseButton::Right,
                        ..
                    },
                window_id,
                ..
            } => {
                show_context_menu(
                    if window_id == window.id() {
                        &window
                    } else {
                        &window2
                    },
                    &file_m,
                    if use_window_pos {
                        Some(window_cursor_position.into())
                    } else {
                        None
                    },
                );
                use_window_pos = !use_window_pos;
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                if event.id == custom_i_1.id() {
                    file_m.insert(&MenuItem::new("New Menu Item", true, None), 2);
                }
                println!("{event:?}");
            }
            _ => (),
        }
    })
}

fn show_context_menu(window:&Window, menu:&dyn ContextMenu, position:Option<Position>) {
	println!("Show context menu at position {position:?}");
	#[cfg(target_os = "windows")]
	unsafe {
		menu.show_context_menu_for_hwnd(window.hwnd() as _, position);
	}
	#[cfg(target_os = "linux")]
	menu.show_context_menu_for_gtk_window(window.gtk_window().as_ref(), position);
	#[cfg(target_os = "macos")]
	unsafe {
		menu.show_context_menu_for_nsview(window.ns_view() as _, position);
	}
}

fn load_icon(path:&std::path::Path) -> muda::Icon {
	let (icon_rgba, icon_width, icon_height) = {
		let image = image::open(path).expect("Failed to open icon path").into_rgba8();

		let (width, height) = image.dimensions();

		let rgba = image.into_raw();
		(rgba, width, height)
	};

	muda::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
