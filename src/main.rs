use std::io;
use pdf_canvas::graphicsstate::Color;
use pdf_canvas::{BuiltinFont, Canvas, Pdf};
use rand::prelude::ThreadRng;
use rand::Rng;

const PAGE_WIDTH: f32 = 595.0;
const PAGE_HEIGHT: f32 = 842.0;
const PAGE_MARGIN: f32 = 10.0;
const INTERNAL_WIDTH: f32 = PAGE_WIDTH - 2.0 * PAGE_MARGIN;
// const INTERNAL_HEIGHT: f32 = PAGE_HEIGHT - 2.0 * PAGE_MARGIN;
const INTERNAL_LEFT: f32 = PAGE_MARGIN;
// const INTERNAL_BOTTOM: f32 = PAGE_MARGIN;
const INTERNAL_RIGHT: f32 = PAGE_WIDTH - PAGE_MARGIN;
const INTERNAL_TOP: f32 = PAGE_HEIGHT - PAGE_MARGIN;
const INTERNAL_CENTER_X: f32 = INTERNAL_LEFT + INTERNAL_WIDTH / 2.0;
// const INTERNAL_CENTER_Y: f32 = INTERNAL_TOP - INTERNAL_HEIGHT / 2.0;

struct Formula {
    left: i32,
    right: i32,
}
fn main() {
    let mut formulas: Vec<Formula> = Vec::new();
    let mut rng = rand::thread_rng();
    for _i in 0..60 {
        formulas.push(Formula { left: 10 + random(&mut rng, 90), right: 2 + random(&mut rng, 8)});
    }
    print(formulas);
}

fn random(rng: &mut ThreadRng, number: i32) -> i32 {
    (rng.gen::<f32>() * number as f32).floor() as i32
}


fn print(formulas: Vec<Formula>) {
    let mut document = Pdf::create("text.pdf").unwrap();
    document.set_title("Two-digit and one-digit multiplication (5 minutes) ");
    document
        .render_page(PAGE_WIDTH, PAGE_HEIGHT, |c| {
            let mut cursor_y = INTERNAL_TOP - 28.0;
            print_one_test(&formulas[0..30], c, &mut cursor_y)?;
            cursor_y -= 128.0;
            print_one_test(&formulas[30..60], c, &mut cursor_y)?;
            Ok(())
        })
        .unwrap();
    document.finish().unwrap();
}

fn print_one_test(formulas: &[Formula], c: &mut Canvas, cursor_y: &mut f32) -> io::Result<()> {
    c.center_text(
        INTERNAL_CENTER_X,
        *cursor_y,
        BuiltinFont::Helvetica,
        18.0,
        "Two-digit and one-digit multiplication (5 minutes)",
    )?;
    *cursor_y -= 9.0;
    c.set_stroke_color(Color::rgb(0, 0, 0))?;
    c.line(INTERNAL_LEFT, *cursor_y, INTERNAL_RIGHT, *cursor_y)?;
    c.stroke()?;

    for (index, formula) in formulas.iter().enumerate() {
        let x = [INTERNAL_LEFT + 20.0, INTERNAL_LEFT + 220.0, INTERNAL_LEFT + 420.0][index % 3];
        // let y = cursor_y - 28.0 * (index / 3 + 1) as f32;
        if index % 3 == 0 {
            *cursor_y -= 28.0;
        }
        c.left_text(x, *cursor_y, BuiltinFont::Helvetica, 14.0, format!("{} x {} = ", formula.left, formula.right).as_str())?;
    }
    Ok(())
}
