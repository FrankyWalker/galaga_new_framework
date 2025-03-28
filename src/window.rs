use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::event::{ElementState, WindowEvent, KeyEvent};
use winit::keyboard::{PhysicalKey, KeyCode};
use winit::application::ApplicationHandler;
use winit::window::{Window, WindowId, WindowAttributes};

#[cfg(target_os = "android")]
use winit::platform::android::{EventLoopBuilderExtAndroid, activity::AndroidApp};

#[cfg(target_arch="wasm32")]
use winit::platform::web::{WindowExtWebSys, EventLoopExtWebSys};

use std::sync::Arc;
use std::sync::Mutex;

use crate::WindowApp;

pub struct Winit<A: WindowApp> {
    width: u32,
    height: u32,
    scale_factor: f32,
    window: Option<Arc<Window>>,
    app: Option<A>,
    #[cfg(not(target_arch="wasm32"))]
    runtime: tokio::runtime::Runtime
}

impl<A: WindowApp + 'static> Winit<A> {
    pub fn new() -> Self {
        Winit{
            width: 0,
            height: 0,
            scale_factor: 1.0,
            window: None,
            app: None,
            #[cfg(not(target_arch="wasm32"))]
            runtime: tokio::runtime::Runtime::new().unwrap()
        }
    }

    #[cfg(target_os="android")]
    pub fn start(&mut self, log_level: log::LevelFilter, app: AndroidApp) {
        android_logger::init_once(
            android_logger::Config::default().with_max_level(log_level),
        );

        let event_loop = EventLoop::builder()
        .with_android_app(app)
        .build()
        .unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.run_app(self).unwrap();
    }

    #[cfg(target_arch="wasm32")]
    pub fn start(self, log_level: log::Level) {
        
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log_level).expect("Couldn't initialize logger");
   
        let event_loop = EventLoop::new().unwrap();

      //let window = event_loop.create_window(
      //    winit::window::Window::default_attributes()
      //).unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.spawn_app(self)
        //wasm_bindgen_futures::spawn_local(future)
    }

    #[cfg(not(target_arch="wasm32"))]
    #[cfg(not(target_os="android"))]
    pub fn start(&mut self, log_level: log::LevelFilter) {
        env_logger::builder().filter_level(log_level).init();

        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.run_app(self).unwrap();
    }

    fn window(&self) -> Arc<Window> {self.window.clone().unwrap()}
}

impl<A: WindowApp + 'static> ApplicationHandler for Winit<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(Arc::new(event_loop.create_window(Window::default_attributes()).unwrap()));

        let size = self.window().inner_size();
        self.width = size.width;
        self.height = size.height;
        self.scale_factor = self.window().scale_factor() as f32;

        #[cfg(not(target_arch="wasm32"))]
        {
            self.app = Some(self.runtime.block_on(A::new(Box::new(self.window()))));
        }

        #[cfg(target_arch="wasm32")]
        {
            let window = Box::new(self.window().clone());
            self.window().request_inner_size(winit::dpi::PhysicalSize::new(450, 400));
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(self.window().canvas()?);
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");

            let app = Arc::new(Mutex::new(None));
            let app2 = app.clone();

            wasm_bindgen_futures::spawn_local(async move {
                log::info!("BUILDING APP");
                *app2.lock().unwrap() = Some(A::new(window).await);
                log::info!("BUILT APP");
            });

            let mut app = Some(app);
            self.app = loop {
                match Arc::try_unwrap(app.take().unwrap()) {
                    Ok(app) => {break app.into_inner().unwrap();},
                    Err(arc) => app = Some(arc)
                }
            };

            //log::info!("{:?}", app.lock().unwrap());
            //log::info!("{:?}", app);
            //self.app = Arc::into_inner(app).unwrap().into_inner().unwrap();
            // println!("ABOUT TO CREATE WINDOW");

            // use winit::dpi::PhysicalSize;
            // let window = event_loop.create_window(
            //     WindowAttributes::new()
            // ).unwrap();
    
            // let _ = window.request_inner_size(PhysicalSize::new(450, 400));
    
            // use winit::platform::web::WindowExtWebSys;
            // web_sys::window()
            //     .and_then(|win| win.document())
            //     .and_then(|doc| {
            //         let dst = doc.get_element_by_id("wasm-example")?;
            //         let canvas = web_sys::Element::from(window.canvas()?);
            //         dst.append_child(&canvas).ok()?;
            //         Some(())
            //     })
            //     .expect("Couldn't append canvas to document body.");
        }

        self.window().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if id == self.window().id() {
            match event {
                WindowEvent::CloseRequested |
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    event_loop.exit();
                },
                WindowEvent::RedrawRequested => {

                
                    #[cfg(target_arch="wasm32")]
                    {
                        let app = Arc::new(Mutex::new(self.app.take().unwrap()));
                        let width = self.width;
                        let height = self.height;
                        let scale_factor = self.scale_factor;
                        wasm_bindgen_futures::spawn_local(async move {
                            app.lock().unwrap().prepare(
                                width, height, scale_factor
                            ).await;

                            app.lock().unwrap().render().await
                        });
                    }

                    #[cfg(not(target_arch="wasm32"))]
                    {
                        self.runtime.block_on(
                            self.app.as_mut().unwrap().prepare(
                                self.width, self.height, self.scale_factor
                            )
                        );
                        self.runtime.block_on(
                            self.app.as_mut().unwrap().render()
                        );
                    }

                    //self.window().request_redraw();
                },
                WindowEvent::Resized(size) => {
                    self.width = size.width;
                    self.height = size.height;
                    self.window().request_redraw();
                },
                WindowEvent::ScaleFactorChanged{scale_factor, ..} => {
                    self.scale_factor = scale_factor as f32;
                    self.window().request_redraw();
                }
                _ => (),
            }
        }
    }
}
