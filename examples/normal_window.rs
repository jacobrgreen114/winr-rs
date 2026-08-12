// Normal Window

use winr::application::*;
use winr::window::*;
use winr::*;

struct MyWindow;

impl WindowCallbacks for MyWindow {
    type Error = WingmanError;
}

#[derive(Default)]
struct MyApp {
    window_main: Option<WindowPtr<MyWindow>>,
}

impl ApplicationCallbacks for MyApp {
    type Exit = ();
    type Error = WingmanError;

    fn on_startup(
        this: &mut ApplicationRunning<Self>,
        event: &EventStartup,
    ) -> Result<(), Self::Error> {
        let mut window = window()
            .with_callbacks(MyWindow)
            .with_title("wingman [normal_window]")
            .with_style(Windowed::default().with_extent(Extent::new(800, 600)))
            .build(this)?;
        window.show()?;

        this.window_main = Some(window);

        Ok(())
    }

    fn on_events_handled(
        this: &mut ApplicationRunning<Self>,
        event: &EventHandled,
    ) -> Result<(), Self::Error> {
        if let Some(window) = &this.window_main
            && !window.is_closed()
        {
            this.wait_for_events()
        } else {
            this.shutdown()
        }
    }
}

fn main() -> Result<(), WingmanError> {
    application()
        .with_callbacks(MyApp::default())
        .build()?
        .run()
}
