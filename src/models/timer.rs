// Timer model and state machine implementation
use std::time::{Duration, Instant};

/// Timer state enumeration representing all possible timer states
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

/// Timer error types
#[derive(Debug, Clone, PartialEq)]
pub enum TimerError {
    InvalidDuration(String),
    InvalidState(String),
}

impl std::fmt::Display for TimerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimerError::InvalidDuration(msg) => write!(f, "Invalid duration: {}", msg),
            TimerError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
        }
    }
}

impl std::error::Error for TimerError {}

/// Timer control interface - core business logic
pub trait TimerControl {
    /// Start a new timer with the specified duration
    fn start(&mut self, duration: Duration) -> Result<(), TimerError>;
    
    /// Pause the currently running timer
    fn pause(&mut self) -> Result<(), TimerError>;
    
    /// Resume a paused timer
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

/// Main timer implementation
#[derive(Debug, Clone)]
pub struct Timer {
    pub state: TimerState,
    pub original_duration: Duration,
    pub completion_time: Option<Instant>,
}

impl Timer {
    /// Create a new timer in stopped state
    pub fn new() -> Self {
        Self {
            state: TimerState::Stopped,
            original_duration: Duration::from_secs(0),
            completion_time: None,
        }
    }
    
    /// Validate that a duration is within acceptable bounds
    fn validate_duration(duration: Duration) -> Result<(), TimerError> {
        if duration.is_zero() {
            return Err(TimerError::InvalidDuration(
                "Duration must be greater than zero".to_string()
            ));
        }
        
        // Maximum 24 hours
        if duration > Duration::from_secs(24 * 60 * 60) {
            return Err(TimerError::InvalidDuration(
                "Duration cannot exceed 24 hours".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Helper to calculate remaining time for a running timer
    fn calculate_remaining_time(started_at: Instant, original_duration: Duration) -> Duration {
        let elapsed = started_at.elapsed();
        if elapsed >= original_duration {
            Duration::ZERO
        } else {
            original_duration - elapsed
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl TimerControl for Timer {
    fn start(&mut self, duration: Duration) -> Result<(), TimerError> {
        // Validate duration
        Self::validate_duration(duration)?;
        
        // Check current state
        match self.state {
            TimerState::Stopped | TimerState::Finished => {
                self.state = TimerState::Running {
                    started_at: Instant::now(),
                    remaining_duration: duration,
                };
                self.original_duration = duration;
                self.completion_time = None;
                Ok(())
            }
            _ => Err(TimerError::InvalidState(
                "Cannot start timer: timer is already running or paused".to_string()
            ))
        }
    }
    
    fn pause(&mut self) -> Result<(), TimerError> {
        match &self.state {
            TimerState::Running { started_at, remaining_duration } => {
                let current_remaining = Self::calculate_remaining_time(*started_at, *remaining_duration);
                self.state = TimerState::Paused {
                    remaining_duration: current_remaining,
                };
                Ok(())
            }
            _ => Err(TimerError::InvalidState(
                "Cannot pause timer: timer is not running".to_string()
            ))
        }
    }
    
    fn resume(&mut self) -> Result<(), TimerError> {
        match &self.state {
            TimerState::Paused { remaining_duration } => {
                self.state = TimerState::Running {
                    started_at: Instant::now(),
                    remaining_duration: *remaining_duration,
                };
                Ok(())
            }
            _ => Err(TimerError::InvalidState(
                "Cannot resume timer: timer is not paused".to_string()
            ))
        }
    }
    
    fn reset(&mut self) {
        self.state = TimerState::Stopped;
        self.original_duration = Duration::from_secs(0);
        self.completion_time = None;
    }
    
    fn state(&self) -> &TimerState {
        &self.state
    }
    
    fn remaining_time(&self) -> Option<Duration> {
        match &self.state {
            TimerState::Running { started_at, remaining_duration } => {
                Some(Self::calculate_remaining_time(*started_at, *remaining_duration))
            }
            TimerState::Paused { remaining_duration } => Some(*remaining_duration),
            _ => None,
        }
    }
    
    fn is_finished(&self) -> bool {
        matches!(self.state, TimerState::Finished)
    }
    
    fn tick(&mut self) -> bool {
        match &self.state {
            TimerState::Running { started_at, remaining_duration } => {
                let current_remaining = Self::calculate_remaining_time(*started_at, *remaining_duration);
                
                if current_remaining.is_zero() {
                    // Timer has finished
                    self.state = TimerState::Finished;
                    self.completion_time = Some(Instant::now());
                    true // State changed
                } else {
                    false // No state change
                }
            }
            _ => false // No state change for non-running timers
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    
    #[test]
    fn test_new_timer_is_stopped() {
        let timer = Timer::new();
        assert!(matches!(timer.state, TimerState::Stopped));
        assert_eq!(timer.remaining_time(), None);
        assert!(!timer.is_finished());
    }
    
    #[test]
    fn test_start_valid_duration() {
        let mut timer = Timer::new();
        let duration = Duration::from_secs(60);
        
        let result = timer.start(duration);
        assert!(result.is_ok());
        assert!(matches!(timer.state, TimerState::Running { .. }));
        
        // Allow for small timing differences - remaining time should be close to original
        let remaining = timer.remaining_time().unwrap();
        assert!(remaining <= duration);
        assert!(remaining >= duration - Duration::from_millis(100)); // Allow 100ms tolerance
    }
    
    #[test]
    fn test_start_zero_duration_fails() {
        let mut timer = Timer::new();
        let result = timer.start(Duration::ZERO);
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TimerError::InvalidDuration(_)));
    }
    
    #[test]
    fn test_start_excessive_duration_fails() {
        let mut timer = Timer::new();
        let duration = Duration::from_secs(25 * 60 * 60); // 25 hours
        
        let result = timer.start(duration);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TimerError::InvalidDuration(_)));
    }
    
    #[test]
    fn test_pause_and_resume() {
        let mut timer = Timer::new();
        let duration = Duration::from_secs(60);
        
        timer.start(duration).unwrap();
        
        // Pause
        let pause_result = timer.pause();
        assert!(pause_result.is_ok());
        assert!(matches!(timer.state, TimerState::Paused { .. }));
        
        // Resume
        let resume_result = timer.resume();
        assert!(resume_result.is_ok());
        assert!(matches!(timer.state, TimerState::Running { .. }));
    }
    
    #[test]
    fn test_reset_from_any_state() {
        let mut timer = Timer::new();
        
        // Reset from stopped
        timer.reset();
        assert!(matches!(timer.state, TimerState::Stopped));
        
        // Reset from running
        timer.start(Duration::from_secs(60)).unwrap();
        timer.reset();
        assert!(matches!(timer.state, TimerState::Stopped));
        
        // Reset from paused
        timer.start(Duration::from_secs(60)).unwrap();
        timer.pause().unwrap();
        timer.reset();
        assert!(matches!(timer.state, TimerState::Stopped));
    }
    
    #[test]
    fn test_timer_completion() {
        let mut timer = Timer::new();
        
        // Start very short timer
        timer.start(Duration::from_millis(10)).unwrap();
        
        // Wait for completion
        thread::sleep(Duration::from_millis(20));
        
        // Tick should detect completion
        let state_changed = timer.tick();
        assert!(state_changed);
        assert!(timer.is_finished());
        assert!(matches!(timer.state, TimerState::Finished));
    }
    
    #[test]
    fn test_remaining_time_decreases() {
        let mut timer = Timer::new();
        let duration = Duration::from_secs(10);
        
        timer.start(duration).unwrap();
        let initial_remaining = timer.remaining_time().unwrap();
        
        thread::sleep(Duration::from_millis(100));
        let updated_remaining = timer.remaining_time().unwrap();
        
        assert!(updated_remaining < initial_remaining);
    }
}