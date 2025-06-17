use eframe::egui;
use eframe::egui::{FontDefinitions, Vec2};

pub fn get_font() -> FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "space_mono".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/space_mono.ttf")).into(),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "space_mono".to_owned());
    fonts
}

pub fn set_styles(cc : &eframe::CreationContext<'_> ) {
    cc.egui_ctx.style_mut(|style| {
        style.spacing.button_padding = Vec2::new(10.0, 6.0);
        style.spacing.item_spacing = Vec2::new(6.0, 4.0);
        style.spacing.indent = 20.0;
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(16.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(15.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(20.0, egui::FontFamily::Proportional),
        );
    });
    cc.egui_ctx.set_pixels_per_point(1.1);
}