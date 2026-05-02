use macroquad::prelude::*;

pub struct ClickableText<'a> {
    pub label: &'a str,
    pub tx: f32,
    pub y: f32,
    pub size: f32,
    pub color: Color,
    pub hover_color: Color,
    pub font: &'a Font,
}

impl<'a> ClickableText<'a> {
    pub fn new(label: &'a str, cx: f32, y: f32, size: f32, color: Color, hover_color: Color, font: &'a Font) -> Self {
        let w = measure_text(label, Some(font), size as u16, 1.0).width;
        Self { label, tx: cx - w / 2.0, y, size, color, hover_color, font }
    }

    pub fn new_left(label: &'a str, x: f32, y: f32, size: f32, color: Color, hover_color: Color, font: &'a Font) -> Self {
        Self { label, tx: x, y, size, color, hover_color, font }
    }

    fn measure(&self) -> TextDimensions {
        measure_text(self.label, Some(self.font), self.size as u16, 1.0)
    }

    pub fn hovered(&self) -> bool {
        let dims = self.measure();
        let (mx, my) = mouse_position();
        mx >= self.tx - 6.0
            && mx <= self.tx + dims.width + 6.0
            && my >= self.y - dims.height - 2.0
            && my <= self.y + 4.0
    }

    pub fn clicked(&self) -> bool {
        self.hovered() && is_mouse_button_pressed(MouseButton::Left)
    }

    pub fn draw(&self) {
        let hov = self.hovered();
        let color = if hov { self.hover_color } else { self.color };
        draw_text_ex(self.label, self.tx, self.y, TextParams {
            font: Some(self.font),
            font_size: self.size as u16,
            font_scale: 1.0,
            color,
            ..Default::default()
        });
    }
}

pub fn apply_cursor(hovering: bool) {
    if hovering {
        miniquad::window::set_mouse_cursor(miniquad::CursorIcon::Pointer);
    } else {
        miniquad::window::set_mouse_cursor(miniquad::CursorIcon::Default);
    }
}
