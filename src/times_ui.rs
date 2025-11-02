use eframe::egui;

use lox_time::time_scales::DynTimeScale;

pub(crate) fn time_scale_combo(ui: &mut egui::Ui, label: &str, scale: &mut DynTimeScale) {
    egui::ComboBox::from_label(label)
        .selected_text(format!("{:?}", scale))
        .show_ui(ui, |ui| {
            // DynTimeScale
            ui.selectable_value(scale, DynTimeScale::Tai, "TAI");
            ui.selectable_value(scale, DynTimeScale::Tcb, "TCB");
            ui.selectable_value(scale, DynTimeScale::Tcg, "TCG");
            ui.selectable_value(scale, DynTimeScale::Tdb, "TBD");
            ui.selectable_value(scale, DynTimeScale::Tt, "TT");
            ui.selectable_value(scale, DynTimeScale::Ut1, "UT1");
        });
}
