#!/usr/bin/env python3

"""
Per default the gpio pins of the pi are set to FLOATING (input mode). This means the
outputs on the step pins are not well defined and the stepper motors may
start to spin randomly which could destroy the belts. We can avoid this by setting the
pins to OUTPUT mode on startup with this script.
"""

import RPi.GPIO as GPIO

GPIO.setmode(GPIO.BCM)

gpio_numbers = [27, 17, 6, 13, 26]

for gpio_number in gpio_numbers:
    GPIO.setup(gpio_number, GPIO.OUT)

for gpio_number in gpio_numbers:
    print("GPIO no " + str(gpio_number) + ": " + str(GPIO.input(gpio_number)))
