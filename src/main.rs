// GhostTimer - Windows Desktop Timer Widget
// Main application entry point

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use ghost_timer::{AppState, VERSION};
use std::time::Duration;

fn main() -> Result<(), eframe::Error> {
    println!("GhostTimer v{} starting...", VERSION);
    
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(250.0, 120.0)),
        min_window_size: Some(egui::vec2(200.0, 100.0)),
        transparent: true,
        decorated: false,
        always_on_top: true,
        resizable: false,
        ..Default::default()
    };
    
    eframe::run_native(
        "GhostTimer",
        options,
        Box::new(|_cc| Box::new(TimerApp::new())),
    )
}

struct TimerApp {
    app_state: AppState,
    timer_minutes: String,
    timer_seconds: String,
    last_tick: std::time::Instant,
}

impl TimerApp {
    fn new() -> Self {
        Self {
            app_state: AppState::new(),
            timer_minutes: "5".to_string(),
            timer_seconds: "0".to_string(),
            last_tick: std::time::Instant::now(),
        }
    }
    
    fn format_time(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
    
    fn parse_timer_input(&self) -> Option<Duration> {
        let minutes: u64 = self.timer_minutes.parse().ok()?;
        let seconds: u64 = self.timer_seconds.parse().ok()?;
        
        if minutes > 59 || seconds > 59 {
            return None;
        }
        
        Some(Duration::from_secs(minutes * 60 + seconds))
    }
}

impl eframe::App for TimerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update timer state
        if self.last_tick.elapsed() >= Duration::from_millis(100) {
            if self.app_state.tick_timer() {
                // Timer state changed, request repaint
                ctx.request_repaint();
            }
            self.last_tick = std::time::Instant::now();
        }
        
        // Get transparency for UI rendering
        let transparency = self.app_state.window_transparency();
        
        // Main UI
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgba_unmultiplied(40, 40, 40, (255.0 * transparency) as u8))
                    .rounding(8.0)
                    .inner_margin(12.0)
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    // Timer display
                    let time_text = match self.app_state.timer_state() {
                        ghost_timer::models::timer::TimerState::Stopped => {
                            "00:00".to_string()
                        }
                        ghost_timer::models::timer::TimerState::Finished => {
                            "DONE!".to_string()
                        }
                        _ => {
                            if let Some(remaining) = self.app_state.remaining_time() {
                                Self::format_time(remaining)
                            } else {
                                "00:00".to_string()
                            }
                        }
                    };
                    
                    ui.add_space(5.0);
                    
                    // Large timer display
                    ui.label(egui::RichText::new(time_text)
                        .size(32.0)
                        .color(egui::Color32::WHITE)
                        .family(egui::FontFamily::Monospace)
                    );
                    
                    ui.add_space(8.0);
                    
                    // Show controls when hovered or when stopped
                    let is_stopped = matches!(self.app_state.timer_state(), ghost_timer::models::timer::TimerState::Stopped);
                    let is_finished = self.app_state.is_timer_finished();
                    
                    if ui.rect_contains_pointer(ui.available_rect_before_wrap()) {
                        self.app_state.handle_mouse_enter();
                    } else {
                        self.app_state.handle_mouse_leave();
                    }
                    
                    if self.app_state.are_controls_visible() || is_stopped || is_finished {
                        // Timer input (only when stopped)
                        if is_stopped {
                            ui.horizontal(|ui| {
                                ui.add(egui::TextEdit::singleline(&mut self.timer_minutes)
                                    .desired_width(30.0)
                                    .hint_text("5")
                                );
                                ui.label(":");
                                ui.add(egui::TextEdit::singleline(&mut self.timer_seconds)
                                    .desired_width(30.0)
                                    .hint_text("00")
                                );
                                ui.label("(mm:ss)");
                            });
                            
                            ui.add_space(5.0);
                        }
                        
                        // Control buttons
                        ui.horizontal(|ui| {
                            match self.app_state.timer_state() {
                                ghost_timer::models::timer::TimerState::Stopped => {
                                    if ui.button("▶ Start").clicked() {
                                        if let Some(duration) = self.parse_timer_input() {
                                            let _ = self.app_state.start_timer(duration);
                                        }
                                    }
                                }
                                ghost_timer::models::timer::TimerState::Running { .. } => {
                                    if ui.button("⏸ Pause").clicked() {
                                        let _ = self.app_state.pause_timer();
                                    }
                                    if ui.button("⏹ Stop").clicked() {
                                        self.app_state.reset_timer();
                                    }
                                }
                                ghost_timer::models::timer::TimerState::Paused { .. } => {
                                    if ui.button("▶ Resume").clicked() {
                                        let _ = self.app_state.resume_timer();
                                    }
                                    if ui.button("⏹ Stop").clicked() {
                                        self.app_state.reset_timer();
                                    }
                                }
                                ghost_timer::models::timer::TimerState::Finished => {
                                    if ui.button("✓ Done").clicked() {
                                        self.app_state.reset_timer();
                                    }
                                }
                            }
                        });
                    }
                });
            });
        
        // Handle dragging
        if ctx.input(|i| i.pointer.any_pressed()) {
            let pointer_pos = ctx.input(|i| i.pointer.interact_pos().unwrap_or_default());
            self.app_state.handle_drag_start(pointer_pos.x as i32, pointer_pos.y as i32);
        }
        
        if ctx.input(|i| i.pointer.any_released()) {
            self.app_state.handle_drag_end();
        }
        
        // Request repaint for smooth timer updates
        if !matches!(self.app_state.timer_state(), ghost_timer::models::timer::TimerState::Stopped) {
            ctx.request_repaint_after(Duration::from_millis(100));
        }
        
        // Show notification when timer finishes
        if self.app_state.was_notification_triggered() {
            // Flash the title or show visual indication
            ctx.request_repaint();
        }
    }
}
