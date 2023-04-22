// TODO: REWORK THIS FILE

// TODO: name this example "performance test"?

use std::collections::VecDeque;
use std::rc::Rc;

use error_iter::ErrorIter;
use log::{debug, error, warn, Log};
use pixels::{wgpu, Pixels, SurfaceTexture};
use wasm_bindgen::JsCast;
use winit::event::VirtualKeyCode;
use winit::platform::web::WindowBuilderExtWebSys;
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use beet_core::BeetCore;

// TODO: rename this example package
fn main() {
    // TODO: change this
    BeetCore::hello();

    run_wrapper::<TmpGame>();
}

// TODO: ???
struct TmpGame {
    blue_up: bool,
    blue: u8,
    iters: u32,
    red_px: usize,
}

// TODO: ???
impl GameApp for TmpGame {
    fn init() -> Self {
        Self {
            blue_up: true,
            blue: 0x88,
            iters: 10,
            red_px: 0,
        }
    }

    fn options() -> GameOptions {
        GameOptions {
            window_title: "Beet Core: \"Minimal\" example".to_string(),
            // TODO: use glam for vectors instead of separate w and h
            canvas_width: 64,
            canvas_height: 32,
            canvas_scale: 8,
            // TODO: better example name
            html_canvas_selector: "#tmp-canvas".to_string(),
        }
    }

    fn update(&mut self) {
        if self.blue == u8::MAX {
            self.blue_up = false;
        } else if self.blue == u8::MIN {
            self.blue_up = true;
        }
        self.blue = (self.blue as i32 + if self.blue_up { 1 } else { -1 }) as u8;

        self.iters += 10;
        // TODO: make it configurable with keyboard buttons
        if self.iters > 100 {
            self.iters = 10;
        }
        self.red_px += 1;
        if self.red_px >= (Self::options().canvas_width * Self::options().canvas_height) as usize {
            self.red_px = 0;
        }
    }

    fn draw(&mut self, draw_api: &mut DrawApi) {
        // web_sys::console::log_1(&format!("it {}", self.iters).into());
        // for _ in 0..self.iters {
        draw_api.fill([0x00, 0xff - self.blue, self.blue, 0xff]);
        // }
        draw_api.set_px(self.red_px, [0xff, 0x00, 0x00, 0xff]);
    }
}

// TODO: adapt this
struct GameOptions {
    window_title: String,
    canvas_width: u32,
    canvas_height: u32,
    canvas_scale: u32,
    html_canvas_selector: String,
}

// TODO: move to lib, rework
trait GameApp {
    fn init() -> Self;
    fn options() -> GameOptions;
    fn update(&mut self);
    fn draw(&mut self, draw_api: &mut DrawApi);
}

struct DrawApi {
    deque: VecDeque<DrawCmd>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum DrawCmd {
    Fill([u8; 4]),
    SetPx(usize, [u8; 4]),
}

impl DrawApi {
    fn fill(&mut self, color: [u8; 4]) {
        self.deque.push_back(DrawCmd::Fill(color));
    }

    fn set_px(&mut self, target: usize, color: [u8; 4]) {
        self.deque.push_back(DrawCmd::SetPx(target, color));
    }

    fn execute(&mut self, cmd: DrawCmd, screen: &mut [u8]) {
        match cmd {
            DrawCmd::Fill(color) => {
                for pix in screen.chunks_exact_mut(4) {
                    pix.copy_from_slice(&color);
                }
            },
            DrawCmd::SetPx(target, color) => {
                screen[target * 4] = color[0];
                screen[target * 4 + 1] = color[1];
                screen[target * 4 + 2] = color[2];
                screen[target * 4 + 3] = color[3];
            },
        }
    }
}

impl DrawApi {
    fn new() -> Self {
        Self {
            deque: VecDeque::new(),
        }
    }
}

// TODO: simplify this
fn run_wrapper<A: GameApp + 'static>() {
    console_error_panic_hook::set_once();

    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Warn).expect("should initialize logger");

    wasm_bindgen_futures::spawn_local(run::<A>());
}

fn get_html_window_size() -> LogicalSize<f64> {
    let html_window = web_sys::window().unwrap();
    LogicalSize::new(
        html_window.inner_width().unwrap().as_f64().unwrap(),
        html_window.inner_height().unwrap().as_f64().unwrap(),
    )
}

async fn run<A: GameApp + 'static>() {
    let mut game_app = A::init();
    let options = A::options();

    let event_loop = EventLoop::new();

    let winit_window = {
        let canvas_candidates = web_sys::window()
            .and_then(|html_window| html_window.document())
            .and_then(|html_document| {
                html_document
                    .query_selector_all(&options.html_canvas_selector)
                    .ok()
            })
            .expect("should be able to query HTML with use of selector `{}`");
        let canvas = canvas_candidates.get(0).unwrap_or_else(|| {
            panic!(
                "should find a <canvas> HTML element that matches selector `{}`",
                options.html_canvas_selector
            )
        });
        let canvas = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("should cast from Node to HtmlCanvasElement");

        WindowBuilder::new()
            .with_title(options.window_title)
            .with_inner_size(LogicalSize::new(
                (options.canvas_width * options.canvas_scale) as f64,
                (options.canvas_height * options.canvas_scale) as f64,
            ))
            .with_min_inner_size(LogicalSize::new(
                options.canvas_width as f64,
                options.canvas_height as f64,
            ))
            .with_canvas(Some(canvas))
            // Argument for a `false` below: otherwise Cmd+Opt+I doesn't open dev tools when focused on the <canvas>
            .with_prevent_default(false)
            .build(&event_loop)
            .expect("should build a winit window")
    };
    let winit_window = Rc::new(winit_window);

    let winit_window_clone_1 = Rc::clone(&winit_window);
    winit_window_clone_1.set_inner_size(get_html_window_size());

    // Copy-pasted from https://github.com/parasyte/pixels/blob/main/examples/minimal-web/src/main.rs#L90-L97
    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
        winit_window_clone_1.set_inner_size(get_html_window_size())
    }) as Box<dyn FnMut(_)>);
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();

    let winit_window_clone_2 = Rc::clone(&winit_window);
    let mut pixels = {
        let window_size = winit_window_clone_2.inner_size();
        let surface_texture = SurfaceTexture::new(
            window_size.width,
            window_size.height,
            winit_window_clone_2.as_ref(),
        );
        Pixels::new_async(options.canvas_width, options.canvas_height, surface_texture)
            .await
            .expect("should create `pixels` instance")
    };
    // TODO: set from the outside
    pixels.clear_color(wgpu::Color {
        r: 1.0,
        g: 0.5,
        b: 0.2,
        a: 1.0,
    });

    let mut draw_api = DrawApi::new();

    let mut debug_pause = false;

    let mut input = WinitInputHelper::new();

    const EXPECTED_DELTA: f64 = 1000.0 / 60.0;
    let performance = web_sys::window()
        .and_then(|html_window| html_window.performance())
        .expect("should be able to access `window.performance`");

    // process 1st frame w/o wait
    let mut accumulated_delta = EXPECTED_DELTA;

    let mut prev_now = performance.now();

    let mut resume_for_1_frame = false;

    event_loop.run(move |event, _, control_flow| {
        // Poll is recommended for games. See: https://docs.rs/winit/0.28.3/winit/#event-handling
        control_flow.set_poll();

        // TODO: rewrite as `match` maybe?

        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            debug!("Event::RedrawRequested");
            let now = performance.now();
            accumulated_delta += now - prev_now;
            prev_now = now;

            while accumulated_delta > EXPECTED_DELTA {
                // web_sys::console::log_1(&format!("> ad {}", accumulated_delta).into());
                if accumulated_delta > 2.0 * EXPECTED_DELTA {
                    error!(
                        "LONG UPDATE: {} (expected: {})",
                        accumulated_delta, EXPECTED_DELTA
                    );
                }

                // TODO: configure key from the outside
                if !debug_pause || resume_for_1_frame {
                    game_app.update();
                }
                resume_for_1_frame = false;

                if accumulated_delta < 2.0 * EXPECTED_DELTA {
                    game_app.draw(&mut draw_api);

                    while let Some(cmd) = draw_api.deque.pop_front() {
                        draw_api.execute(cmd, pixels.frame_mut());
                        // TODO: interleave drawing with updates to avoid delayed update after
                        //       each long draw. For example count in the main update loop how many
                        //       updates we need to perform, then perform them here. It might be
                        //       important for avoiding collision logic issues etc.
                    }

                    if let Err(err) = pixels.render() {
                        error!("failed to render pixels: {}", err.to_string());
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                } else {
                    warn!("skipped draw");
                }

                accumulated_delta -= EXPECTED_DELTA;
            }
        }

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
        if input.update(&event) {
            // TODO: make it configurable from the outside
            // This allows to stop the game in a browser just by pressing Esc
            #[cfg(debug_assertions)]
            if input.key_pressed(VirtualKeyCode::Escape) {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // TODO: make it configurable from the outside
            #[cfg(debug_assertions)]
            if input.key_pressed(VirtualKeyCode::Period) {
                debug_pause = !debug_pause;
            }
            #[cfg(debug_assertions)]
            if input.key_pressed_os(VirtualKeyCode::Comma) {
                resume_for_1_frame = true;
                debug_pause = true;
            }

            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    error!("failed to resize the surface: {}", err.to_string());
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            winit_window.request_redraw();
        }
    });
}
