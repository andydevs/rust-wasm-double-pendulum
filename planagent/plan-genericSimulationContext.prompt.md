## Plan: Generic Simulation Context

TL;DR — Introduce a small simulation API so users implement a `Simulation` (trait) that receives a `FrameContext` each frame.
Add a `SimulationRunner` to own the animation loop (computing `dt`, frame number, canvas size) and call `update`/`render`.
Minimal changes: extend `src/anim.rs` to forward the requestAnimationFrame timestamp, add types & runner in `src/sim.rs`, and
wire the runner from `src/lib.rs`. Keep Rust owning the DOM (current behavior). This keeps the design simple now and leaves
hooks for pause/resume, renderer abstraction, and TypeScript bindings later.

## Design

**Sim Context Data Structures**

```rust
struct FrameCtx {
    pub frame: usize,
    pub timestamp: f64,
    pub dt: f64
}

struct WindowCtx {
    pub width: f32,
    pub height: f32
}

struct RenderCtx {
    pub window: WindowCtx,
    pub frame: FrameCtx,
    pub ctx2d: CanvasRenderingCtx2d
}

struct UpdateCtx {
    pub frame: FrameCtx
}
```

**Simulation Trait**

```rust
/// Trait implementers are responsible for holding state!
trait Simulation {
    fn render(&self, render: &mut RenderCtx);

    fn update(&mut self, update: &mut UpdateCtx);
}
```

**Simulation Runner**

```rust
struct SimulationRunner<S: Simulation> {
    /* Global state variables */
    simulation: S // Dynamic dispatch if need be
}

impl SimulationRunner {
    fn initialize(state: S) -> Self {
        // Call Simulation initialize
    }

    /// Consume self
    fn run(self) -> Result<(), JsValue> {
        // Initialize variables

        anim_frame_loop(move |time| {
            // Build render ctx
            self.simulation.render(/* render ctx */);

            // Build update ctx
            self.simulation.update(/* update ctx */);
        })
    }

}
```

**Steps**

1. Define the frame context and trait
    - Add `FrameContext` and `Simulation` trait to `src/sim.rs`.
    - Suggested symbols:
        - `pub struct FrameContext { pub dt: f64, pub timestamp: f64, pub frame: u64, pub width: u32, pub height: u32 }`
        - `pub trait Simulation { fn update(&mut self, ctx: &mut FrameContext); fn render(&self, ctx: &FrameContext, ctx2d: &web_sys::CanvasRenderingContext2d); }`
    - Rationale: trait + composition fits the preferred style and is easy to implement for existing sims.

2. Make requestAnimationFrame provide timestamps
    - Update `src/anim.rs` to expose a timestamp-aware helper:
        - Change or add `pub fn animation_frame_loop<F: FnMut(f64) + 'static>(callback: F) -> Result<(), JsValue>`
        - Internally pass the `timestamp: f64` argument from the `requestAnimationFrame` callback into the `F`.
    - Rationale: DOM rAF supplies high-res timestamps; runner computes `dt` from these.

3. Implement SimulationRunner orchestrator
    - Add `SimulationRunner` in `src/sim.rs` (near trait definitions).
    - Responsibilities:
        - Acquire `HtmlCanvasElement` and `CanvasRenderingContext2d` (keep DOM ownership in Rust).
        - Hold `Box<dyn Simulation>` (or generic param if desired later).
        - Maintain `last_timestamp: Option<f64>` and `frame: u64`.
        - On each rAF tick, compute `dt = (timestamp - last_timestamp)/1000.0` (seconds), update `FrameContext`, call `sim.update(&mut ctx)` then `sim.render(&ctx, &ctx2d)`.
        - Use `anim::animation_frame_loop` to schedule frames and manage `Closure` lifetimes as before.
    - Expose an ergonomic constructor and `run()` method: `pub fn run(self) -> Result<(), JsValue>`.

4. Adapt the current simulation implementation as an example
    - Convert the current code in `src/sim.rs` (the `run` function’s closure-based state) into an example implementation type:
        - e.g., `pub struct ExampleSim { /* state */ } impl Simulation for ExampleSim { ... }`
    - Wire `SimulationRunner::new(Box::new(ExampleSim::default()), canvas, ctx2d).run()` from `src/lib.rs` in the `#[wasm_bindgen(start)]` initializer.
    - This is non-breaking: keep `console_error_panic_hook` and DOM lookups in `main()` as they exist.

5. Documentation & small API hygiene
    - Update README.md and comments in `src/sim.rs` explaining how to implement `Simulation`.
    - Add examples in `src/` (or a `examples/` folder) showing a simple `Simulation` implementation.
    - Add a short unit/integration test harness if feasible (note: wasm/display tests may be manual).

**Verification**

- Build and run locally:
    - `wasm-pack build` or the existing `npm`/`webpack` build pipeline used by this repo (same steps you use now).
    - Load `public/index.html` (or run the dev server) and confirm the canvas animates as before.
- Manual checks:
    - Console shows no panics; animation continues.
    - Log `FrameContext` values on first frames to verify `dt`, `timestamp`, `frame`, and canvas `width/height`.
- Optional: add a temporary `console_log` in `SimulationRunner` to confirm dt calculations.

**Decisions**

- API style: Trait-based `Simulation` with composition (matches preference).
- Frame context: include `dt`, `timestamp`, `frame_number`, and canvas `width/height`.
- Lifecycle: keep a simple fire-and-forget `run()` runner (no pause/resume for now).
- DOM ownership: Rust will continue to query and own canvas/context (as requested).

**Future improvements (non-blocking)**

- Expose a cancel handle (pause/resume/stop) from `SimulationRunner`.
- Make `SimulationRunner` generic over `S: Simulation` to avoid `Box<dyn Simulation>` if zero-cost abstractions are important.
- Add a `Renderer` trait to separate drawing backend (2D canvas vs WebGL).
- Export TypeScript types for `FrameContext` in `pkg` bindings.
- Add an API for injecting input/events (resize, mouse) into `FrameContext` or a separate event queue.
