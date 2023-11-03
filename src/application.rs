pub struct Application {}

impl Application {
    pub fn run(controller: impl ApplicationController) {
        
    }
}

pub trait ApplicationController {
    fn on_init(&self, app: &Application) {}
}
