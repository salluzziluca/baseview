use std::num::NonZeroU32;
use std::time::Duration;

use rtrb::{Consumer, RingBuffer};

#[cfg(target_os = "macos")]
use baseview::{copy_to_clipboard, MouseEvent};
use baseview::{
    Event, EventStatus, PhySize, Window, WindowEvent, WindowHandler, WindowScalePolicy,
};

#[derive(Debug, Clone)]
enum Message {
    Hello,
}

struct OpenWindowExample {
    rx: Consumer<Message>,
    current_size: PhySize,
    frame_count: u32,
}

impl WindowHandler for OpenWindowExample {
    fn on_frame(&mut self, window: &mut Window) {
        // Create softbuffer objects locally for each frame - this avoids lifetime issues
        let ctx = softbuffer::Context::new(&*window).expect("Failed to create softbuffer context");
        let mut surface = softbuffer::Surface::new(&ctx, &*window).expect("Failed to create softbuffer surface");
        
        if let (Some(width), Some(height)) = 
            (NonZeroU32::new(self.current_size.width), NonZeroU32::new(self.current_size.height))
        {
            surface.resize(width, height).expect("Failed to resize surface");
            
            let mut buf = surface.buffer_mut().expect("Failed to get buffer");
            
            // Use the correct softbuffer format: 0RGB (highest 8 bits must be 0)
            // Format: 00000000RRRRRRRRGGGGGGGGBBBBBBBB
            let colors = [
                0x00FF0000, // Red
                0x0000FF00, // Green  
                0x000000FF, // Blue
                0x00FFFF00, // Yellow
                0x00FF00FF, // Magenta
                0x0000FFFF, // Cyan
                0x00FFFFFF, // White
                0x00808080, // Gray
            ];
            
            let color = colors[(self.frame_count / 60) as usize % colors.len()];
            
            // Debug: Print current color and buffer info
            println!("Frame {}: Using color 0x{:08X}, buffer size: {}", 
                     self.frame_count, color, buf.len());
            
            // Fill the entire buffer with a solid color - no pattern, no transparency
            buf.fill(color);
            
            // Double-check: manually set every pixel to make sure
            for pixel in buf.iter_mut() {
                *pixel = color;
            }
            
            buf.present().expect("Failed to present buffer");
            self.frame_count += 1;
        }

        while let Ok(message) = self.rx.pop() {
            println!("Message: {:?}", message);
        }
    }

    fn on_event(&mut self, _window: &mut Window, event: Event) -> EventStatus {
        match &event {
            #[cfg(target_os = "macos")]
            Event::Mouse(MouseEvent::ButtonPressed { .. }) => copy_to_clipboard("This is a test!"),
            Event::Window(WindowEvent::Resized(info)) => {
                println!("Resized: {:?}", info);
                let new_size = info.physical_size();
                self.current_size = new_size;
            }
            _ => {}
        }

        log_event(&event);

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

    let (mut tx, rx) = RingBuffer::new(128);

    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(5));

        if tx.push(Message::Hello).is_err() {
            println!("Failed sending message");
        }
    });

    Window::open_blocking(window_open_options, |_window| {
        OpenWindowExample {
            rx,
            current_size: PhySize::new(512, 512),
            frame_count: 0,
        }
    });
}

fn log_event(event: &Event) {
    match event {
        Event::Mouse(e) => println!("Mouse event: {:?}", e),
        Event::Keyboard(e) => println!("Keyboard event: {:?}", e),
        Event::Window(e) => println!("Window event: {:?}", e),
    }
}
