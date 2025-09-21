// Timer Core Interface Contract
// This file defines the interface that the timer logic module must implement

use std::time::{Duration, Instant};

/// Timer control interface - core business logic
pub trait TimerControl {
    /// Start a new timer with the specified duration
    /// 
    /// # Errors
    /// - `InvalidDuration` if duration is 0 or > 24 hours
    /// - `InvalidState` if timer is already running
    fn start(&mut self, duration: Duration) -> Result<(), TimerError>;
    
    /// Pause the currently running timer
    /// 
    /// # Errors
    /// - `InvalidState` if timer is not running
    fn pause(&mut self) -> Result<(), TimerError>;
    
    /// Resume a paused timer
    /// 
    /// # Errors
    /// - `InvalidState` if timer is not paused
    fn resume(&mut self) -> Result<(), TimerError>;
    
    /// Reset timer to stopped state
    fn reset(&mut self);
    
    /// Get current timer state
    fn state(&self) -> &TimerState;
    
    /// Get remaining time (if running or paused)
    fn remaining_time(&self) -> Option<Duration>;
    
    /// Check if timer has finished (countdown reached zero)
    fn is_finished(&self) -> bool;
    
    /// Update timer state based on elapsed time
    /// Returns true if state changed (requires UI update)
    fn tick(&mut self) -> bool;
}

/// Timer notification interface
pub trait TimerNotifications {
    /// Called when timer finishes countdown
    fn on_timer_finished(&self);
    
    /// Called when timer state changes
    fn on_state_changed(&self, old_state: &TimerState, new_state: &TimerState);
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimerState {
    Stopped,
    Running { 
        started_at: Instant,
        remaining_duration: Duration,
    },
    Paused {
        remaining_duration: Duration,
    },
    Finished,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimerError {
    InvalidDuration(String),
    InvalidState(String),
}

#[cfg(test)]
mod timer_contract_tests {
    use super::*;
    
    // Contract test: Starting a timer with valid duration should succeed
    #[test]
    fn start_timer_with_valid_duration() {
        // This test will fail until implementation is created
        panic!("Timer implementation not yet created");
    }
    
    // Contract test: Starting timer twice should fail
    #[test]
    fn start_timer_twice_should_fail() {
        panic!("Timer implementation not yet created");
    }
    
    // Contract test: Pausing stopped timer should fail
    #[test]
    fn pause_stopped_timer_should_fail() {
        panic!("Timer implementation not yet created");
    }
    
    // Contract test: Timer should finish when countdown reaches zero
    #[test]
    fn timer_should_finish_when_countdown_complete() {
        panic!("Timer implementation not yet created");
    }
}