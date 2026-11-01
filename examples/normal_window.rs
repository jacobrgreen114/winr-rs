#[cfg(windows)]
fn main() -> winr::WinrResult<()> {
    use winr::Extent;
    use winr::application::{Event, EventLoop};
    use winr::window::{WindowBuilder, WindowEvent};

    let event_loop = EventLoop::new()?;
    // Application state is ordinary Rust state captured by a generic handler.
    let mut main_window = None;
    event_loop.run(move |event_loop, event| {
        match event {
            Event::Started => {
                let window = WindowBuilder::new()
                    .with_title("winr [normal_window]")
                    .with_extent(Extent::new(800, 600))
                    .build(event_loop)?;
                window.show()?;
                main_window = Some(window.id());
            }
            Event::Window {
                id,
                event: WindowEvent::CloseRequested,
            } if Some(id) == main_window => {
                if let Some(window) = event_loop.window(id) {
                    window.close()?;
                }
            }
            Event::Window {
                event: WindowEvent::Closed,
                id,
            } if Some(id) == main_window => event_loop.exit(),
            _ => {}
        }
        Ok(())
    })
}
#[cfg(not(windows))]
fn main() {
    eprintln!("The window example requires Windows.");
}
