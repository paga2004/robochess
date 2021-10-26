use std::thread;
use std::time::Duration;

use rust_gpiozero::{DigitalInputDevice, Servo};

use crate::stepper::StepperMotor;

// some constants that can be easily tweaked
const SLOW_DELAY: f32 = 0.008;
const FAST_DELAY: f32 = 0.004;
const MIN_X: i32 = 0;
const MAX_X: i32 = 2400;
const MIN_Y: i32 = 0;
const MAX_Y: i32 = 2200;

fn saftey_delay() {
    thread::sleep(Duration::new(0, 500_000_000));
}

/// Struct to control the two motors and the servo on a high level. The `x` and `y` values are
/// always initialized correctly because this struct can only be created with `HBot::new`, which
/// performs the init sequence.
pub struct HBot {
    m1: StepperMotor,
    m2: StepperMotor,
    b1: DigitalInputDevice,
    b2: DigitalInputDevice,
    s: Servo,
    x: i32,
    y: i32,
}

impl HBot {
    /// Creates a new HBot and performs the init sequence.
    pub fn new(
        m1_step_pin: u8,
        m1_dir_pin: u8,
        m2_step_pin: u8,
        m2_dir_pin: u8,
        b1_pin: u8,
        b2_pin: u8,
        servo_pin: u8,
    ) -> Self {
        let m1 = StepperMotor::new(m1_step_pin, m1_dir_pin);
        let m2 = StepperMotor::new(m2_step_pin, m2_dir_pin);
        let b1 = DigitalInputDevice::new(b1_pin);
        let b2 = DigitalInputDevice::new(b2_pin);
        let s = Servo::new(servo_pin);

        let mut res = Self {
            m1,
            m2,
            b1,
            b2,
            s,
            x: 0,
            y: 0,
        };
        res.init_sequence();
        res
    }

    fn init_sequence(&mut self) {
        self.s.max();
        self.wait();
        println!("touch bottom");
        if self.b1.is_active() {
            self.m1.turn_steps(SLOW_DELAY, -100);
            self.m2.turn_steps(SLOW_DELAY, 100);
            self.m1.wait();
            self.m2.wait();
            saftey_delay();
        }
        self.m1.turn(true, SLOW_DELAY);
        self.m2.turn(false, SLOW_DELAY);
        self.b1.wait_for_active(None);
        self.m1.stop();
        self.m2.stop();
        self.m1.turn_steps(SLOW_DELAY, -40);
        self.m2.turn_steps(SLOW_DELAY, 40);
        self.m1.wait();
        self.m2.wait();
        self.m1.stop();
        self.m2.stop();

        saftey_delay();

        println!("touch right");
        if self.b2.is_active() {
            self.m1.turn_steps(SLOW_DELAY, -100);
            self.m2.turn_steps(SLOW_DELAY, -100);
            self.m1.wait();
            self.m2.wait();
            saftey_delay();
        }
        self.m1.turn(true, SLOW_DELAY);
        self.m2.turn(true, SLOW_DELAY);
        self.b2.wait_for_active(None);
        self.m1.stop();
        self.m2.stop();
        self.m1.turn_steps(SLOW_DELAY, -40);
        self.m2.turn_steps(SLOW_DELAY, -40);
        self.m1.wait();
        self.m2.wait();
        self.m1.stop();
        self.m2.stop();

        saftey_delay();

        println!("init sequence completed");
    }

    pub fn move_to_xy(&mut self, x: i32, y: i32, delay: f32) {
        assert!(MIN_X <= x && x <= MAX_X);
        assert!(MIN_Y <= y && y <= MAX_Y);

        let dx = x - self.x;
        let dy = y - self.y;

        let steps1 = -dx - dy;
        let steps2 = -dx + dy;

        let duration = steps1.abs().max(steps2.abs()) as f32 * delay;
        self.m1.turn_steps(duration / steps1.abs() as f32, steps1);
        self.m2.turn_steps(duration / steps2.abs() as f32, steps2);
        self.m1.wait();
        self.m2.wait();
        self.x = x;
        self.y = y;
    }

    pub fn move_to_xy_slow(&mut self, x: i32, y: i32) {
        self.move_to_xy(x, y, SLOW_DELAY);
    }

    pub fn move_to_xy_fast(&mut self, x: i32, y: i32) {
        self.move_to_xy(x, y, FAST_DELAY);
    }

    pub fn up(&mut self) {
        self.s.min();
        self.wait();
    }

    pub fn down(&mut self) {
        self.s.max();
        self.wait();
    }

    pub fn wait(&self) {
        saftey_delay();
    }
}
