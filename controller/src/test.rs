mod stepper;
use std::thread;
use std::time::Duration;
use stepper::StepperMotor;

use rust_gpiozero::{DigitalInputDevice, DigitalOutputDevice, Servo};

fn main() {
    let mut m1 = DigitalOutputDevice::new(27);
    let mut m2 = DigitalOutputDevice::new(6);
    let mut d1 = DigitalOutputDevice::new(17);
    let mut d2 = DigitalOutputDevice::new(26);
    let mut s = Servo::new(13);
    let mut b1 = DigitalInputDevice::new(16);
    let mut b2 = DigitalInputDevice::new(5);

    println!("testing stepper 1");
    m1.set_blink_count(200);
    d1.off();
    m1.blink(0.01, 0.01);
    m1.wait();
    d1.on();
    m1.blink(0.01, 0.01);
    m1.wait();

    println!("testing stepper 2");
    m2.set_blink_count(200);
    d2.off();
    m2.blink(0.01, 0.01);
    m2.wait();
    d2.on();
    m2.blink(0.01, 0.01);
    m2.wait();

    println!("testing servo");
    println!("min");
    s.min();
    thread::sleep(Duration::new(3, 0));
    println!("mid");
    s.mid();
    thread::sleep(Duration::new(3, 0));
    println!("max");
    s.max();

    println!("testing button 1");
    b1.wait_for_active(None);
    println!("testing button 2");
    b2.wait_for_active(None);
    println!("OK");
}
