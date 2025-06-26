use eframe::egui::{self, Color32, FontDefinitions, Stroke, Vec2, style::Visuals};

pub fn get_font() -> FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "instrument_serif".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/space_mono.ttf")).into(),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "instrument_serif".to_owned());
    fonts
}

pub fn set_styles(cc : &eframe::CreationContext<'_> ) {
    let mut visuals = Visuals::dark();
    visuals.override_text_color = Some(Color32::from_rgb(230, 230, 230));
    visuals.hyperlink_color = Color32::from_rgb(255, 107, 107);
    visuals.faint_bg_color = Color32::from_rgb(31, 31, 31);
    visuals.extreme_bg_color = Color32::from_rgb(45, 45, 45);
    visuals.code_bg_color = Color32::from_rgb(45, 45, 45);
    visuals.warn_fg_color = Color32::YELLOW;
    visuals.error_fg_color = Color32::RED;
    visuals.window_fill = Color32::from_rgb(31, 31, 31);
    visuals.window_stroke = Stroke::new(1.0, Color32::from_rgb(45, 45, 45));

    cc.egui_ctx.set_visuals(visuals);
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
