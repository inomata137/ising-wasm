#[wasm_bindgen::prelude::wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub struct Model {
    size: usize,
    spins: Vec<bool>,
    beta_j: f64,
    rng: rand::rngs::ThreadRng,
    context: web_sys::CanvasRenderingContext2d,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl Model {
    pub fn new(size: usize, beta_j: f64, canvas_id: &str) -> Self {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let mut spins = vec![false; size * size];
        rng.fill(&mut spins[..]);

        let canvas_size = size as u32;
        let canvas = get_canvas(canvas_id);
        canvas.set_width(canvas_size);
        canvas.set_height(canvas_size);

        let context = get_context(canvas);
        Self { size, spins, beta_j, rng, context }
    }

    fn neightbor_positions(&self, pos: usize) -> [usize; 4] {
        let size = self.size;
        let x = pos % size;
        let y = pos / size;
        let left = if x == 0 { size - 1 } else { x - 1 };
        let right = if x == size - 1 { 0 } else { x + 1 };
        let up = if y == 0 { size - 1 } else { y - 1 };
        let down = if y == size - 1 { 0 } else { y + 1 };
        [left + y * size, right + y * size, x + up * size, x + down * size]
    }

    fn neightbor_spins(&self, pos: usize) -> [bool; 4] {
        self.neightbor_positions(pos).map(|p| self.spins[p])
    }

    pub fn update_metropolis(&mut self) {
        use rand::Rng;

        for pos in 0..(self.size * self.size) {
            let env_spin = self.neightbor_spins(pos)
                .iter()
                .map(|b| if *b { 1. } else { -1. })
                .sum::<f64>();
            let log_acc = -2.0 * self.beta_j * env_spin * if self.spins[pos] { 1. } else { -1. };
            if log_acc > 0. || self.rng.gen_range(0.0..1.0) < log_acc.exp() {
                self.spins[pos] = !self.spins[pos];
            }
        }
    }

    pub fn draw(&self) {
        let ctx = &self.context;
        let canvas_size = ctx.canvas().unwrap().height() as f64;
        ctx.clear_rect(0., 0., canvas_size, canvas_size);
        ctx.set_fill_style_str("white");
        ctx.fill_rect(0., 0., canvas_size, canvas_size);
        ctx.set_fill_style_str("black");
        let cell_size = canvas_size / self.size as f64;
        for (pos, &spin) in self.spins.iter().enumerate() {
            let x = (pos % self.size) as f64 * cell_size;
            let y = (pos / self.size) as f64 * cell_size;
            if !spin {
                ctx.fill_rect(x, y, cell_size, cell_size);
            }
        }
    }
}

fn get_canvas(id: &str) -> web_sys::HtmlCanvasElement {
    use wasm_bindgen::JsCast;

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();
    canvas
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .map_err(|_| ())
      .unwrap()
}

fn get_context(canvas: web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
    use wasm_bindgen::JsCast;

    canvas
        .get_context("2d")
        .expect("coudn't get context")
        .expect("no context")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}
