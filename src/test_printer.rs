use crate::{Formula, Test};
use std::io::Result;
use pdf_canvas::graphicsstate::Color;
use pdf_canvas::{BuiltinFont, Canvas, Pdf};
use crate::formula::Operator;

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

pub struct TestPrinter {
    tests: Vec<Test>,
}

impl TestPrinter {
    pub fn new(tests: Vec<Test>) -> TestPrinter {
        TestPrinter { tests }
    }
    pub(crate) fn print(&self) {
        let mut document = Pdf::create("test.pdf").unwrap();
        document.set_title("Two-digit and one-digit multiplication (3 minutes) ");
        for test in &self.tests {
            document
                .render_page(PAGE_WIDTH, PAGE_HEIGHT, |c| {
                    let mut cursor_y = INTERNAL_TOP - 28.0;
                    self.print_one_test(&test.formulas, c, &mut cursor_y)?;
                    Ok(())
                })
                .unwrap();
        }
        document.finish().unwrap();
    }

    fn print_one_test(&self, formulas: &[Formula], c: &mut Canvas, cursor_y: &mut f32) -> Result<()> {
        Self::print_test_header(c, cursor_y)?;
        Self::print_formulas(formulas, c, cursor_y)?;
        Ok(())
    }

    fn print_formulas(formulas: &[Formula], c: &mut Canvas, cursor_y: &mut f32) -> Result<()> {
        for (index, formula) in formulas.iter().enumerate() {
            let x = [INTERNAL_LEFT + 10.0, INTERNAL_LEFT + 210.0, INTERNAL_LEFT + 410.0][index % 3];
            if index % 3 == 0 {
                *cursor_y -= 28.0;
            }
            c.left_text(x, *cursor_y, BuiltinFont::Helvetica, 14.0,
                        format!("{} {} {} = {}", Self::blank_or_value(formula.left), Self::operator(formula.operator), Self::blank_or_value(formula.right), Self::blank_or_value(formula.result)).as_str())?;
        }
        Ok(())
    }

    fn operator(operator: Operator) -> String {
        match operator {
            Operator::Add => {"+".to_string()}
            Operator::Multiple => {"×".to_string()}
        }
    }

    fn blank_or_value(value: Option<f32>) -> String {
        match value {
            None => { "_____".to_string() }
            Some(v) => {
                if v < 10.0 {
                    format!("  {:.2}", v).trim_end_matches(".00").to_string()
                } else {
                    format!("{:.2}", v).trim_end_matches(".00").to_string()
                }
            }
        }
    }

    fn print_test_header(c: &mut Canvas, cursor_y: &mut f32) -> Result<()> {
        c.center_text(
            INTERNAL_CENTER_X,
            *cursor_y,
            BuiltinFont::Helvetica,
            18.0,
            "Two-digit and one-digit multiplication (3 minutes)",
        )?;
        *cursor_y -= 9.0;
        c.set_stroke_color(Color::rgb(0, 0, 0))?;
        c.line(INTERNAL_LEFT, *cursor_y, INTERNAL_RIGHT, *cursor_y)?;
        c.stroke()
    }
}
