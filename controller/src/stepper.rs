use rust_gpiozero::DigitalOutputDevice;

const INF: i32 = 1_000_000_000;

/// Low level control for a stepper motor. This struct has no context and does not know how many
/// steps the motor has already turned.
pub struct StepperMotor {
    step: DigitalOutputDevice,
    dir: DigitalOutputDevice,
}

impl StepperMotor {
    pub fn new(step_pin: u8, dir_pin: u8) -> Self {
        Self {
            step: DigitalOutputDevice::new(step_pin),
            dir: DigitalOutputDevice::new(dir_pin),
        }
    }

    /// Stop the motor from turning.
    pub fn stop(&mut self) {
        self.step.off();
    }

    /// Turn the motor and wait a given number of seconds between each step. The actual direction also depends on the wiring.
    pub fn turn(&mut self, dir: bool, delay: f32) {
        // The library does never reset the blink count and there is no way to reset it manually. I
        // think this is a bug. So if `DigitalOutputDevice::set_blink_count` has been called before
        // it might stop blinking eventually. The work around is to just set the blink count to a
        // high number.
        if dir {
            self.turn_steps(delay, INF);
        } else {
            self.turn_steps(delay, -INF);
        }
    }

    /// Turn the motor for a given number of steps and wait a given number of seconds between each step. The actual direction also depends on the wiring.
    ///
    /// # Note
    ///
    /// This function returns immediatley. Use [`StepperMotor::wait`] to actually wait until the
    /// motor has turned the given amount of steps.
    pub fn turn_steps(&mut self, delay: f32, steps: i32) {
        if steps < 0 {
            self.step.set_blink_count(-steps);
            self.dir.off();
        } else {
            self.step.set_blink_count(steps);
            self.dir.on();
        }
        self.step.blink(delay / 2.0, delay / 2.0);
    }

    /// Block until the given amount of steps have passed.
    pub fn wait(&mut self) {
        self.step.wait();
    }
}
