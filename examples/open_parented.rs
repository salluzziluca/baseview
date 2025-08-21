use baseview::{
    Event, EventStatus, PhySize, Window, WindowEvent, WindowHandle, WindowHandler,
    WindowScalePolicy,
};
use std::num::NonZeroU32;

struct ParentWindowHandler {
    current_size: PhySize,
    damaged: bool,
    _child_window: Option<WindowHandle>,
}

impl ParentWindowHandler {
    pub fn new(window: &mut Window) -> Self {
        let window_open_options = baseview::WindowOpenOptions {
            title: "baseview child".into(),
            size: baseview::Size::new(256.0, 256.0),
            scale: WindowScalePolicy::SystemScaleFactor,

            // TODO: Add an example that uses the OpenGL context
            #[cfg(feature = "opengl")]
            gl_config: None,
        };
        let child_window =
            Window::open_parented(window, window_open_options, ChildWindowHandler::new);

        // TODO: no way to query physical size initially?
        Self {
            current_size: PhySize::new(512, 512),
            damaged: true,
            _child_window: Some(child_window),
        }
    }
}

impl WindowHandler for ParentWindowHandler {
    fn on_frame(&mut self, window: &mut Window) {
        // Create softbuffer objects locally for each frame
        let ctx = softbuffer::Context::new(&*window).expect("Failed to create softbuffer context");
        let mut surface = softbuffer::Surface::new(&ctx, &*window).expect("Failed to create softbuffer surface");
        
        if let (Some(width), Some(height)) = 
            (NonZeroU32::new(self.current_size.width), NonZeroU32::new(self.current_size.height))
        {
            surface.resize(width, height).expect("Failed to resize surface");
            
            let mut buf = surface.buffer_mut().expect("Failed to get buffer");
            if self.damaged {
                buf.fill(0xFFAAAAAA);
                self.damaged = false;
            }
            buf.present().expect("Failed to present buffer");
        }
    }

    fn on_event(&mut self, _window: &mut Window, event: Event) -> EventStatus {
        match event {
            Event::Window(WindowEvent::Resized(info)) => {
                println!("Parent Resized: {:?}", info);
                let new_size = info.physical_size();
                self.current_size = new_size;
                self.damaged = true;
            }
            Event::Mouse(e) => println!("Parent Mouse event: {:?}", e),
            Event::Keyboard(e) => println!("Parent Keyboard event: {:?}", e),
            Event::Window(e) => println!("Parent Window event: {:?}", e),
        }

        EventStatus::Captured
    }
}

struct ChildWindowHandler {
    current_size: PhySize,
    damaged: bool,
}

impl ChildWindowHandler {
    pub fn new(_window: &mut Window) -> Self {
        // TODO: no way to query physical size initially?
        Self { current_size: PhySize::new(256, 256), damaged: true }
    }
}

impl WindowHandler for ChildWindowHandler {
    fn on_frame(&mut self, window: &mut Window) {
        // Create softbuffer objects locally for each frame
        let ctx = softbuffer::Context::new(&*window).expect("Failed to create softbuffer context");
        let mut surface = softbuffer::Surface::new(&ctx, &*window).expect("Failed to create softbuffer surface");
        
        if let (Some(width), Some(height)) = 
            (NonZeroU32::new(self.current_size.width), NonZeroU32::new(self.current_size.height))
        {
            surface.resize(width, height).expect("Failed to resize surface");
            
            let mut buf = surface.buffer_mut().expect("Failed to get buffer");
            if self.damaged {
                buf.fill(0xFFAA0000);
                self.damaged = false;
            }
            buf.present().expect("Failed to present buffer");
        }
    }

    fn on_event(&mut self, _window: &mut Window, event: Event) -> EventStatus {
        match event {
            Event::Window(WindowEvent::Resized(info)) => {
                println!("Child Resized: {:?}", info);
                let new_size = info.physical_size();
                self.current_size = new_size;
                self.damaged = true;
            }
            Event::Mouse(e) => println!("Child Mouse event: {:?}", e),
            Event::Keyboard(e) => println!("Child Keyboard event: {:?}", e),
            Event::Window(e) => println!("Child Window event: {:?}", e),
        }

        EventStatus::Captured
    }
}

fn main() {
    let window_open_options = baseview::WindowOpenOptions {
        title: "baseview".into(),
        size: baseview::Size::new(512.0, 512.0),
        scale: WindowScalePolicy::SystemScaleFactor,

        // TODO: Add an example that uses the OpenGL context
        #[cfg(feature = "opengl")]
        gl_config: None,
    };

    Window::open_blocking(window_open_options, ParentWindowHandler::new);
}
