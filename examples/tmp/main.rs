// TODO: REWORK THIS FILE

// TODO: name this example "performance test"?

use std::collections::VecDeque;
use std::rc::Rc;

use log::{debug, error, warn};
use pixels::{Pixels, SurfaceTexture};
use wasm_bindgen::JsCast;
use winit::dpi::LogicalPosition;
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
        if self.iters > 1_000 {
            self.iters = 10;
        }
        self.red_px += 1;
        if self.red_px >= (Self::options().canvas_width * Self::options().canvas_height) as usize {
            self.red_px = 0;
        }
    }

    fn draw(&mut self, draw_api: &mut DrawApi) {
        web_sys::console::log_1(&format!("it {}", self.iters).into());
        for _ in 0..self.iters {
            draw_api.fill([0x00, 0xff - self.blue, self.blue, 0xff]);
        }
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
    pixels: Pixels,
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

    fn execute(&mut self, cmd: DrawCmd) {
        match cmd {
            DrawCmd::Fill(color) => {
                let screen = self.pixels.frame_mut();
                for pix in screen.chunks_exact_mut(4) {
                    pix.copy_from_slice(&color);
                }
            },
            DrawCmd::SetPx(target, color) => {
                let screen = self.pixels.frame_mut();
                screen[target * 4] = color[0];
                screen[target * 4 + 1] = color[1];
                screen[target * 4 + 2] = color[2];
                screen[target * 4 + 3] = color[3];
            },
        }
    }

    fn render(&self) -> Result<(), pixels::Error> {
        self.pixels.render()
    }
}

impl DrawApi {
    fn new(pixels: Pixels) -> Self {
        Self {
            pixels,
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

async fn run<A: GameApp + 'static>() {
    let mut game_app = A::init();
    let options = A::options();

    let event_loop = EventLoop::new();

    let winit_window = {
        let canvas_candidates = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| {
                document
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

    // Retrieve current width and height dimensions of browser client window
    // let get_window_size = || {
    //     let client_window = web_sys::window().unwrap();
    //     LogicalSize::new(
    //         client_window.inner_width().unwrap().as_f64().unwrap(),
    //         client_window.inner_height().unwrap().as_f64().unwrap(),
    //     )
    // };

    // let winit_window = Rc::clone(&window);

    // Initialize winit window with current dimensions of browser client
    // window.set_inner_size(get_window_size());

    // let client_window = web_sys::window().unwrap();

    // Listen for resize event on browser client. Adjust winit window dimensions
    // on event trigger
    // let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
    //     let size = get_window_size();
    //     window.set_inner_size(size)
    // }) as Box<dyn FnMut(_)>);
    // client_window
    //     .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
    //     .unwrap();
    // closure.forget();

    let pixels = {
        let window_size = winit_window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, winit_window.as_ref());
        Pixels::new_async(options.canvas_width, options.canvas_height, surface_texture)
            .await
            .expect("should create `pixels` instance")
    };
    let mut draw_api = DrawApi::new(pixels);

    let mut paused = false;

    // TODO: use `if window_id == window.id()` match branch condition

    let mut input = WinitInputHelper::new();

    const EXPECTED_DELTA: f64 = 1000.0 / 60.0;
    let performance = web_sys::window()
        .and_then(|window| window.performance())
        .expect("should be able to access `window.performance`");

    // process 1st frame w/o wait
    let mut accumulated_delta = EXPECTED_DELTA;

    let mut prev_now = performance.now();

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
                web_sys::console::log_1(&format!("> aUd {}", accumulated_delta).into());
                if accumulated_delta > 2.0 * EXPECTED_DELTA {
                    error!("LONG UPDATE: {} >> {}", accumulated_delta, EXPECTED_DELTA);
                }

                if !paused {
                    game_app.update();
                }

                if accumulated_delta < 2.0 * EXPECTED_DELTA {
                    game_app.draw(&mut draw_api);

                    while let Some(cmd) = draw_api.deque.pop_front() {
                        draw_api.execute(cmd);
                        // TODO: interleave drawing with updates to avoid delayed update after
                        //       each long draw. For example count in the main update loop how many
                        //       updates we need to perform, then perform them here. It might be
                        //       important for avoiding collision logic issues etc.
                    }

                    if let Err(err) = draw_api.render() {
                        log_error("pixels.render", err);
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
            // TODO: ??? does it make sense at all for WASM ???
            // if input.key_pressed(VirtualKeyCode::Escape)
            //     || input.close_requested()
            //     || input.destroyed()
            // {
            //     *control_flow = ControlFlow::Exit;
            //     return;
            // }

            if input.key_pressed(VirtualKeyCode::P) {
                paused = !paused;
            }
            // if input.key_pressed_os(VirtualKeyCode::Space) {
            // Space is frame-step, so ensure we're paused
            // paused = true;
            // }
            // if input.key_pressed(VirtualKeyCode::R) {
            //     life.randomize();
            // }
            // Handle mouse. This is a bit involved since support some simple
            // line drawing (mostly because it makes nice looking patterns).
            // let (mouse_cell, mouse_prev_cell) = input
            //     .mouse()
            //     .map(|(mx, my)| {
            //         let (dx, dy) = input.mouse_diff();
            //         let prev_x = mx - dx;
            //         let prev_y = my - dy;
            //
            //         let (mx_i, my_i) = pixels
            //             .window_pos_to_pixel((mx, my))
            //             .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));
            //
            //         let (px_i, py_i) = pixels
            //             .window_pos_to_pixel((prev_x, prev_y))
            //             .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));
            //
            //         (
            //             (mx_i as isize, my_i as isize),
            //             (px_i as isize, py_i as isize),
            //         )
            //     })
            //     .unwrap_or_default();

            // Resize the window
            // if let Some(size) = input.window_resized() {
            //     if let Err(err) = pixels.resize_surface(size.width, size.height) {
            //         log_error("pixels.resize_surface", err);
            //         *control_flow = ControlFlow::Exit;
            //         return;
            //     }
            // }
            // if !paused || input.key_pressed_os(VirtualKeyCode::Space) {
            //     life.update();
            // }

            winit_window.request_redraw();
        }
    });
}

// TODO: ???
fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    //     for source in err.sources().skip(1) {
    //         error!("  Caused by: {source}");
    //     }
}

// fn generate_seed() -> (u64, u64) {
//     use byteorder::{ByteOrder, NativeEndian};
//     use getrandom::getrandom;

// let mut seed = [0_u8; 16];

// getrandom(&mut seed).expect("failed to getrandom");
//
// (
//     NativeEndian::read_u64(&seed[0..8]),
//     NativeEndian::read_u64(&seed[8..16]),
// )
// }

// const BIRTH_RULE: [bool; 9] = [false, false, false, true, false, false, false, false, false];
// const SURVIVE_RULE: [bool; 9] = [false, false, true, true, false, false, false, false, false];
// const INITIAL_FILL: f32 = 0.3;

// #[derive(Clone, Copy, Debug, Default)]
// struct Cell {
//     alive: bool,
// Used for the trail effect. Always 255 if `self.alive` is true (We could
// use an enum for Cell, but it makes several functions slightly more
// complex, and doesn't actually make anything any simpler here, or save any
// memory, so we don't)
// heat: u8,
// }

// impl Cell {
//     fn new(alive: bool) -> Self {
//         Self { alive, heat: 0 }
//     }
//
//     #[must_use]
//     fn update_neibs(self, n: usize) -> Self {
//         let next_alive = if self.alive {
//             SURVIVE_RULE[n]
//         } else {
//             BIRTH_RULE[n]
//         };
//         self.next_state(next_alive)
//     }
//
//     #[must_use]
//     fn next_state(mut self, alive: bool) -> Self {
//         self.alive = alive;
//         if self.alive {
//             self.heat = 255;
//         } else {
//             self.heat = self.heat.saturating_sub(1);
//         }
//         self
//     }
//
//     fn set_alive(&mut self, alive: bool) {
//         *self = self.next_state(alive);
//     }
//
//     fn cool_off(&mut self, decay: f32) {
//         if !self.alive {
//             let heat = (self.heat as f32 * decay).clamp(0.0, 255.0);
//             assert!(heat.is_finite());
//             self.heat = heat as u8;
//         }
//     }
// }

// #[derive(Clone, Debug)]
// struct ConwayGrid {
//     cells: Vec<Cell>,
//     width: usize,
//     height: usize,
// Should always be the same size as `cells`. When updating, we read from
// `cells` and write to `scratch_cells`, then swap. Otherwise it's not in
// use, and `cells` should be updated directly.
// scratch_cells: Vec<Cell>,
// }

// impl ConwayGrid {
//     fn new_empty(width: usize, height: usize) -> Self {
//         assert!(width != 0 && height != 0);
//         let size = width.checked_mul(height).expect("too big");
//         Self {
//             cells: vec![Cell::default(); size],
//             scratch_cells: vec![Cell::default(); size],
//             width,
//             height,
//         }
//     }
//
//     fn new_random(width: usize, height: usize) -> Self {
//         let mut result = Self::new_empty(width, height);
//         result.randomize();
//         result
//     }
//
//     fn randomize(&mut self) {
//         let mut rng: randomize::PCG32 = generate_seed().into();
//         for c in self.cells.iter_mut() {
//             let alive = randomize::f32_half_open_right(rng.next_u32()) > INITIAL_FILL;
//             *c = Cell::new(alive);
//         }
// run a few simulation iterations for aesthetics (If we don't, the
// noise is ugly)
// for _ in 0..3 {
//     self.update();
// }
// Smooth out noise in the heatmap that would remain for a while
// for c in self.cells.iter_mut() {
//     c.cool_off(0.4);
// }
// }

// fn count_neibs(&self, x: usize, y: usize) -> usize {
//     let (xm1, xp1) = if x == 0 {
//         (self.width - 1, x + 1)
//     } else if x == self.width - 1 {
//         (x - 1, 0)
//     } else {
//         (x - 1, x + 1)
//     };
//     let (ym1, yp1) = if y == 0 {
//         (self.height - 1, y + 1)
//     } else if y == self.height - 1 {
//         (y - 1, 0)
//     } else {
//         (y - 1, y + 1)
//     };
//     self.cells[xm1 + ym1 * self.width].alive as usize
//         + self.cells[x + ym1 * self.width].alive as usize
//         + self.cells[xp1 + ym1 * self.width].alive as usize
//         + self.cells[xm1 + y * self.width].alive as usize
// /            + self.cells[xp1 + y * self.width].alive as usize
//             + self.cells[xm1 + yp1 * self.width].alive as usize
//             + self.cells[x + yp1 * self.width].alive as usize
//             + self.cells[xp1 + yp1 * self.width].alive as usize
//     }

// fn update(&mut self) {
//     for y in 0..self.height {
//         for x in 0..self.width {
//             let neibs = self.count_neibs(x, y);
//             let idx = x + y * self.width;
//             let next = self.cells[idx].update_neibs(neibs);
// Write into scratch_cells, since we're still reading from `self.cells`
// self.scratch_cells[idx] = next;
// }
// }
// std::mem::swap(&mut self.scratch_cells, &mut self.cells);
// }

// fn toggle(&mut self, x: isize, y: isize) -> bool {
//     if let Some(i) = self.grid_idx(x, y) {
//         let was_alive = self.cells[i].alive;
//         self.cells[i].set_alive(!was_alive);
//         !was_alive
//     } else {
//         false
//     }
// }

// fn draw(&self, screen: &mut [u8]) {
//     debug_assert_eq!(screen.len(), 4 * self.cells.len());
//     for (c, pix) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {
//         let color = if c.alive {
//             [0, 0xff, 0xff, 0xff]
//         } else {
//             [0, 0, c.heat, 0xff]
//         };
//         pix.copy_from_slice(&color);
//     }
// }

// fn set_line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, alive: bool) {
// probably should do sutherland-hodgeman if this were more serious.
// instead just clamp the start pos, and draw until moving towards the
// end pos takes us out of bounds.
// let x0 = x0.clamp(0, self.width as isize);
// let y0 = y0.clamp(0, self.height as isize);
// for (x, y) in line_drawing::Bresenham::new((x0, y0), (x1, y1)) {
//     if let Some(i) = self.grid_idx(x, y) {
//         self.cells[i].set_alive(alive);
//     } else {
//         break;
//     }
// }
// }

// fn grid_idx<I: std::convert::TryInto<usize>>(&self, x: I, y: I) -> Option<usize> {
//     match (x.try_into(), y.try_into()) {
//         (Ok(x), Ok(y)) if x < self.width && y < self.height => Some(x + y * self.width),
//         _ => None,
//     }
// }
// }
