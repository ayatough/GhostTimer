// GhostTimer - Windows Desktop Timer Widget
// Main application entry point

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use ghost_timer::{AppState, VERSION};
use std::time::Duration;

fn main() -> Result<(), eframe::Error> {
    println!("GhostTimer v{} starting...", VERSION);
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([250.0, 120.0])
            .with_min_inner_size([200.0, 100.0])
            .with_transparent(true)
            .with_decorations(false)
            .with_always_on_top()
            .with_resizable(false),
        ..Default::default()
    };
    
    eframe::run_native(
        "GhostTimer",
        options,
        Box::new(|_cc| Ok(Box::new(TimerApp::new()))),
    )
}

struct TimerApp {
    app_state: AppState,
    timer_minutes: String,
    timer_seconds: String,
    last_tick: std::time::Instant,
    is_dragging: bool,
    drag_start_pos: Option<egui::Pos2>,
    last_timer_state: ghost_timer::models::timer::TimerState,
    is_editing_timer: bool,
    edit_text: String,
    cursor_pos: usize,  // Position in edit_text
    edit_field: EditField, // Whether editing minutes or seconds
}

#[derive(Clone, Copy, PartialEq)]
enum EditField {
    Minutes,
    Seconds,
}

impl TimerApp {
    fn new() -> Self {
        Self {
            app_state: AppState::new(),
            timer_minutes: "5".to_string(),
            timer_seconds: "0".to_string(),
            last_tick: std::time::Instant::now(),
            is_dragging: false,
            drag_start_pos: None,
            last_timer_state: ghost_timer::models::timer::TimerState::Stopped,
            is_editing_timer: false,
            edit_text: "05:00".to_string(),
            cursor_pos: 0,
            edit_field: EditField::Minutes,
        }
    }
    
    fn play_notification_sound(&self) {
        #[cfg(windows)]
        {
            use std::thread;
            thread::spawn(|| {
                unsafe {
                    winapi::um::winuser::MessageBeep(0xFFFFFFFF); // Default system sound
                }
            });
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
    
    fn parse_timer_edit_text(&self) -> Option<Duration> {
        // Parse "mm:ss" format
        let parts: Vec<&str> = self.edit_text.split(':').collect();
        if parts.len() != 2 {
            return None;
        }
        
        let minutes: u64 = parts[0].parse().ok()?;
        let seconds: u64 = parts[1].parse().ok()?;
        
        if minutes > 999 || seconds > 59 {
            return None;
        }
        
        Some(Duration::from_secs(minutes * 60 + seconds))
    }
    
    fn format_edit_time(&self, duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
    
    
    fn increment_time(&mut self) {
        if let Some(duration) = self.parse_timer_edit_text() {
            let total_seconds = duration.as_secs();
            let minutes = total_seconds / 60;
            let seconds = total_seconds % 60;
            
            let (new_minutes, new_seconds) = match self.edit_field {
                EditField::Minutes => {
                    let new_minutes = if minutes >= 999 { 0 } else { minutes + 1 };
                    (new_minutes, seconds)
                }
                EditField::Seconds => {
                    if seconds >= 59 {
                        let new_minutes = if minutes >= 999 { 0 } else { minutes + 1 };
                        (new_minutes, 0)
                    } else {
                        (minutes, seconds + 1)
                    }
                }
            };
            
            self.edit_text = format!("{:02}:{:02}", new_minutes, new_seconds);
        }
    }
    
    fn decrement_time(&mut self) {
        if let Some(duration) = self.parse_timer_edit_text() {
            let total_seconds = duration.as_secs();
            let minutes = total_seconds / 60;
            let seconds = total_seconds % 60;
            
            let (new_minutes, new_seconds) = match self.edit_field {
                EditField::Minutes => {
                    let new_minutes = if minutes == 0 { 999 } else { minutes - 1 };
                    (new_minutes, seconds)
                }
                EditField::Seconds => {
                    if seconds == 0 {
                        let new_minutes = if minutes == 0 { 999 } else { minutes - 1 };
                        (new_minutes, 59)
                    } else {
                        (minutes, seconds - 1)
                    }
                }
            };
            
            self.edit_text = format!("{:02}:{:02}", new_minutes, new_seconds);
        }
    }
    
    fn handle_char_input(&mut self, c: char) {
        if c.is_ascii_digit() {
            let mut chars: Vec<char> = self.edit_text.chars().collect();
            if self.cursor_pos < chars.len() && chars[self.cursor_pos] != ':' {
                chars[self.cursor_pos] = c;
                self.edit_text = chars.iter().collect();
                
                // Move cursor to next position, skipping colon
                self.cursor_pos += 1;
                let chars_len = self.edit_text.len();
                if self.cursor_pos < chars_len {
                    let current_char = self.edit_text.chars().nth(self.cursor_pos);
                    if current_char == Some(':') {
                        self.cursor_pos += 1;
                        self.edit_field = EditField::Seconds;
                    }
                }
                
                // Validate and fix the input
                if let Some(duration) = self.parse_timer_edit_text() {
                    self.edit_text = self.format_edit_time(duration);
                }
            }
        }
    }
}

impl eframe::App for TimerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update timer state
        if self.last_tick.elapsed() >= Duration::from_millis(100) {
            if self.app_state.tick_timer() {
                // Timer state changed, request repaint
                ctx.request_repaint();
                
                // Check if timer just finished and play notification sound
                let current_state = self.app_state.timer_state().clone();
                if matches!(current_state, ghost_timer::models::timer::TimerState::Finished) &&
                   !matches!(self.last_timer_state, ghost_timer::models::timer::TimerState::Finished) {
                    self.play_notification_sound();
                }
                self.last_timer_state = current_state;
            }
            self.last_tick = std::time::Instant::now();
        }
        
        // Get transparency for UI rendering
        let transparency = self.app_state.window_transparency();
        
        // Main UI
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgba_unmultiplied(40, 40, 40, (255.0 * transparency) as u8))
                    .corner_radius(8.0)
                    .inner_margin(12.0)
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    // Editable timer display
                    let is_stopped = matches!(self.app_state.timer_state(), ghost_timer::models::timer::TimerState::Stopped);
                    let is_finished = self.app_state.is_timer_finished();
                    
                    ui.add_space(5.0);
                    
                    // Timer display - editable when stopped, showing time when running
                    if is_stopped && self.is_editing_timer {
                        // Custom edit mode - looks exactly like display but handles input
                        let text_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * transparency) as u8);
                        
                        // Handle keyboard input
                        ctx.input(|i| {
                            // Handle character input (numbers only)
                            for event in &i.events {
                                if let egui::Event::Text(text) = event {
                                    for c in text.chars() {
                                        self.handle_char_input(c);
                                    }
                                }
                            }
                            
                            // Handle arrow keys
                            if i.key_pressed(egui::Key::ArrowUp) {
                                self.increment_time();
                            }
                            if i.key_pressed(egui::Key::ArrowDown) {
                                self.decrement_time();
                            }
                            
                            // Handle left/right arrows to move between minutes and seconds
                            if i.key_pressed(egui::Key::ArrowLeft) {
                                self.edit_field = EditField::Minutes;
                                self.cursor_pos = 1; // End of minutes
                            }
                            if i.key_pressed(egui::Key::ArrowRight) {
                                self.edit_field = EditField::Seconds;
                                self.cursor_pos = 4; // End of seconds
                            }
                        });
                        
                        // Create edit display matching the exact structure of display mode
                        let edit_response = ui.label(
                            egui::RichText::new(&self.edit_text)
                                .size(32.0)
                                .color(text_color)
                                .family(egui::FontFamily::Monospace)
                        );
                        
                        // Draw background highlight behind the active field
                        let rect = edit_response.rect;
                        let char_width = rect.width() / 5.0; // Approximate width per character in "mm:ss"
                        
                        if self.edit_field == EditField::Minutes {
                            // Highlight minutes (first 2 characters)
                            let highlight_rect = egui::Rect::from_min_size(
                                rect.min,
                                egui::Vec2::new(char_width * 2.0, rect.height())
                            );
                            ui.painter().rect_filled(
                                highlight_rect,
                                2.0,
                                egui::Color32::from_rgba_unmultiplied(255, 255, 0, (255.0 * transparency * 0.3) as u8)
                            );
                        } else {
                            // Highlight seconds (last 2 characters)
                            let highlight_rect = egui::Rect::from_min_size(
                                egui::Pos2::new(rect.min.x + char_width * 3.0, rect.min.y),
                                egui::Vec2::new(char_width * 2.0, rect.height())
                            );
                            ui.painter().rect_filled(
                                highlight_rect,
                                2.0,
                                egui::Color32::from_rgba_unmultiplied(255, 255, 0, (255.0 * transparency * 0.3) as u8)
                            );
                        }
                        
                        // Show help text
                        ui.add_space(2.0);
                        ui.label(egui::RichText::new("Type numbers, ↑↓ to change, ←→ to switch field")
                            .size(10.0)
                            .color(egui::Color32::from_rgba_unmultiplied(200, 200, 200, (255.0 * transparency * 0.8) as u8))
                        );
                        
                        // Exit edit mode on Enter or Escape
                        if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            self.is_editing_timer = false;
                            // Update timer values from edit text
                            if let Some(duration) = self.parse_timer_edit_text() {
                                let total_seconds = duration.as_secs();
                                self.timer_minutes = (total_seconds / 60).to_string();
                                self.timer_seconds = (total_seconds % 60).to_string();
                            }
                        }
                        
                        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                            self.is_editing_timer = false;
                            // Reset edit text to current timer values
                            if let Some(duration) = self.parse_timer_input() {
                                self.edit_text = self.format_edit_time(duration);
                            }
                        }
                    } else {
                        // Display mode - show timer
                        let time_text = match self.app_state.timer_state() {
                            ghost_timer::models::timer::TimerState::Stopped => {
                                if let Some(duration) = self.parse_timer_input() {
                                    self.format_edit_time(duration)
                                } else {
                                    "05:00".to_string()
                                }
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
                        
                        let timer_response = ui.add(
                            egui::Label::new(
                                egui::RichText::new(&time_text)
                                    .size(32.0)
                                    .color(egui::Color32::WHITE)
                                    .family(egui::FontFamily::Monospace)
                            ).sense(egui::Sense::click())
                        );
                        
                        // Enter edit mode on click (only when stopped)
                        if timer_response.clicked() && is_stopped {
                            self.is_editing_timer = true;
                            self.edit_text = time_text;
                            self.cursor_pos = 0;
                            self.edit_field = EditField::Minutes;
                        }
                        
                        // Show click hint when hovered and stopped
                        if timer_response.hovered() && is_stopped {
                            ui.painter().rect_filled(
                                timer_response.rect,
                                4.0,
                                egui::Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * transparency * 0.1) as u8)
                            );
                        }
                    }
                    
                    ui.add_space(8.0);
                    
                    // Handle mouse hover for controls visibility
                    if ui.rect_contains_pointer(ui.available_rect_before_wrap()) {
                        self.app_state.handle_mouse_enter();
                    } else {
                        self.app_state.handle_mouse_leave();
                    }
                    
                    if self.app_state.are_controls_visible() || is_stopped || is_finished {
                        
                        // Control buttons as transparent clickable areas
                        ui.horizontal(|ui| {
                            let button_text_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * transparency) as u8);
                            
                            match self.app_state.timer_state() {
                                ghost_timer::models::timer::TimerState::Stopped => {
                                    let start_response = ui.add(
                                        egui::Label::new(
                                            egui::RichText::new("▶ Start")
                                                .color(button_text_color)
                                                .size(14.0)
                                        ).sense(egui::Sense::click())
                                    );
                                    
                                    // Add hover effect
                                    if start_response.hovered() {
                                        ui.painter().rect_filled(
                                            start_response.rect,
                                            2.0,
                                            egui::Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * transparency * 0.1) as u8)
                                        );
                                    }
                                    
                                    if start_response.clicked() {
                                        if let Some(duration) = self.parse_timer_input() {
                                            let _ = self.app_state.start_timer(duration);
                                        }
                                    }
                                }
                                ghost_timer::models::timer::TimerState::Running { .. } => {
                                    let pause_response = ui.add(
                                        egui::Label::new(
                                            egui::RichText::new("⏸ Pause")
                                                .color(button_text_color)
                                                .size(14.0)
                                        ).sense(egui::Sense::click())
                                    );
                                    
                                    if pause_response.hovered() {
                                        ui.painter().rect_filled(
                                            pause_response.rect,
                                            2.0,
                                            egui::Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * transparency * 0.1) as u8)
                                        );
                                    }
                                    
                                    if pause_response.clicked() {
                                        let _ = self.app_state.pause_timer();
                                    }
                                    
                                    ui.add_space(10.0);
                                    
                                    let stop_response = ui.add(
                                        egui::Label::new(
                                            egui::RichText::new("⏹ Stop")
                                                .color(button_text_color)
                                                .size(14.0)
                                        ).sense(egui::Sense::click())
                                    );
                                    
                                    if stop_response.hovered() {
                                        ui.painter().rect_filled(
                                            stop_response.rect,
                                            2.0,
                                            egui::Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * transparency * 0.1) as u8)
                                        );
                                    }
                                    
                                    if stop_response.clicked() {
                                        self.app_state.reset_timer();
                                    }
                                }
                                ghost_timer::models::timer::TimerState::Paused { .. } => {
                                    let resume_response = ui.add(
                                        egui::Label::new(
                                            egui::RichText::new("▶ Resume")
                                                .color(button_text_color)
                                                .size(14.0)
                                        ).sense(egui::Sense::click())
                                    );
                                    
                                    if resume_response.hovered() {
                                        ui.painter().rect_filled(
                                            resume_response.rect,
                                            2.0,
                                            egui::Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * transparency * 0.1) as u8)
                                        );
                                    }
                                    
                                    if resume_response.clicked() {
                                        let _ = self.app_state.resume_timer();
                                    }
                                    
                                    ui.add_space(10.0);
                                    
                                    let stop_response = ui.add(
                                        egui::Label::new(
                                            egui::RichText::new("⏹ Stop")
                                                .color(button_text_color)
                                                .size(14.0)
                                        ).sense(egui::Sense::click())
                                    );
                                    
                                    if stop_response.hovered() {
                                        ui.painter().rect_filled(
                                            stop_response.rect,
                                            2.0,
                                            egui::Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * transparency * 0.1) as u8)
                                        );
                                    }
                                    
                                    if stop_response.clicked() {
                                        self.app_state.reset_timer();
                                    }
                                }
                                ghost_timer::models::timer::TimerState::Finished => {
                                    let done_response = ui.add(
                                        egui::Label::new(
                                            egui::RichText::new("✓ Done")
                                                .color(button_text_color)
                                                .size(14.0)
                                        ).sense(egui::Sense::click())
                                    );
                                    
                                    if done_response.hovered() {
                                        ui.painter().rect_filled(
                                            done_response.rect,
                                            2.0,
                                            egui::Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * transparency * 0.1) as u8)
                                        );
                                    }
                                    
                                    if done_response.clicked() {
                                        self.app_state.reset_timer();
                                    }
                                }
                            }
                        });
                    }
                });
            });
        
        // Handle dragging to move window
        let pointer_pos = ctx.input(|i| i.pointer.interact_pos());
        
        if ctx.input(|i| i.pointer.primary_pressed()) && pointer_pos.is_some() {
            self.is_dragging = true;
            self.drag_start_pos = pointer_pos;
        }
        
        if self.is_dragging && ctx.input(|i| i.pointer.primary_down()) {
            if let (Some(current_pos), Some(_start_pos)) = (pointer_pos, self.drag_start_pos) {
                self.drag_start_pos = Some(current_pos); // Update for continuous dragging
                // Use context to move window
                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }
        }
        
        if ctx.input(|i| i.pointer.primary_released()) {
            self.is_dragging = false;
            self.drag_start_pos = None;
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
