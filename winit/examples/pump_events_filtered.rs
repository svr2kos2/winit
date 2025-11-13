#![allow(clippy::single_match)]

// Limit this example to only Windows platform where filtering is supported.
#[cfg(windows_platform)]
fn main() -> std::process::ExitCode {
    use std::process::ExitCode;
    use std::thread::sleep;
    use std::time::Duration;

    use winit::application::ApplicationHandler;
    use winit::event::WindowEvent;
    use winit::event_loop::pump_events::{EventLoopExtPumpEvents, PumpStatus};
    use winit::event_loop::EventLoop;
    use winit::platform::windows::EventLoopExtWindows;
    use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use winit::window::{Window, WindowAttributes, WindowId};

    #[path = "util/fill.rs"]
    mod fill;

    #[derive(Default, Debug)]
    struct PumpDemo {
        window: Option<Box<dyn Window>>,
    }

    impl ApplicationHandler for PumpDemo {
        fn can_create_surfaces(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop) {
            let window_attributes = WindowAttributes::default()
                .with_title("Filtered pump_events example (Windows only)");
            self.window = Some(event_loop.create_window(window_attributes).unwrap());
        }

        fn window_event(
            &mut self,
            event_loop: &dyn winit::event_loop::ActiveEventLoop,
            _window_id: WindowId,
            event: WindowEvent,
        ) {
            println!("{event:?}");

            let window = match self.window.as_ref() {
                Some(window) => window,
                None => return,
            };

            match event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::RedrawRequested => {
                    fill::fill_window(window.as_ref());
                    window.request_redraw();
                },
                _ => (),
            }
        }
    }

    let mut event_loop = EventLoop::new().unwrap();

    tracing_subscriber::fmt::init();

    let mut app = PumpDemo::default();

    // First pump to create the window
    let _ = event_loop.pump_app_events(Some(Duration::ZERO), &mut app);

    // Now get the window handle and set up filtering
    if let Some(window) = &app.window {
        if let Ok(handle) = window.window_handle() {
            if let RawWindowHandle::Win32(win32_handle) = handle.as_raw() {
                let hwnd = win32_handle.hwnd.get() as isize;
                println!("Setting message filtering for HWND: 0x{:X}", hwnd);
                
                // Set filtering - now only this window's messages will be processed
                event_loop.set_filtering_window(Some(hwnd));
                
                println!("Message filtering enabled!");
                println!("This event loop will now only process messages for the winit window.");
                println!("Other windows on this thread (e.g., from Flutter) won't be affected.");
            }
        }
    }

    loop {
        let timeout = Some(Duration::ZERO);
        let status = event_loop.pump_app_events(timeout, &mut app);

        if let PumpStatus::Exit(exit_code) = status {
            break ExitCode::from(exit_code as u8);
        }

        // Sleep for 1/60 second to simulate application work
        //
        // Since `pump_events` doesn't block it will be important to
        // throttle the loop in the app somehow.
        println!("Update()");
        sleep(Duration::from_millis(16));
    }
}

#[cfg(not(windows_platform))]
fn main() {
    println!("This example is only supported on Windows.");
    println!("On other platforms, use pump_events.rs example instead.");
}
