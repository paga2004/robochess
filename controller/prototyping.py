#!/usr/bin/env python3

"""
This script can be run with `python3 -i` to prototype, which is way faster than
recompiling rust code on the raspberry pi zero.
"""

from gpiozero import *

m1 = DigitalOutputDevice(27)
m2 = DigitalOutputDevice(6)
d1 = DigitalOutputDevice(17)
d2 = DigitalOutputDevice(26)
s = Servo(13)
b1 = Button(16, pull_up=False)
b2 = Button(5, pull_up=False)
