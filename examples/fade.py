#!/usr/bin/env python3

from struct import unpack
from subprocess import PIPE, Popen
from threading import Thread
from time import sleep

# map linux keycode to key name see https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h
keymap = {
  1: 'ESC',           2: '1',             3: '2',             4: '3',
  5: '4',             6: '5',             7: '6',             8: '7',
  9: '8',             10: '9',            11: '0',            12: 'MINUS',
  13: 'EQUAL',        14: 'BACKSPACE',    15: 'TAB',          16: 'Q',
  17: 'W',            18: 'E',            19: 'R',            20: 'T',
  21: 'Y',            22: 'U',            23: 'I',            24: 'O',
  25: 'P',            26: 'LEFTBRACE',    27: 'RIGHTBRACE',   28: 'ENTER',
  29: 'LEFTCTRL',     30: 'A',            31: 'S',            32: 'D',
  33: 'F',            34: 'G',            35: 'H',            36: 'J',
  37: 'K',            38: 'L',            39: 'SEMICOLON',    40: 'APOSTROPHE',
  41: 'GRAVE',        42: 'LEFTSHIFT',    43: 'BACKSLASH',    44: 'Z',
  45: 'X',            46: 'C',            47: 'V',            48: 'B',
  49: 'N',            50: 'M',            51: 'COMMA',        52: 'DOT',
  53: 'SLASH',        54: 'RIGHTSHIFT',   55: 'KPASTERISK',   56: 'LEFTALT',
  57: 'SPACE',        58: 'CAPSLOCK',     59: 'F1',           60: 'F2',
  61: 'F3',           62: 'F4',           63: 'F5',           64: 'F6',
  65: 'F7',           66: 'F8',           67: 'F9',           68: 'F10',
  69: 'NUMLOCK',      70: 'SCROLLLOCK',   71: 'KP7',          72: 'KP8',
  73: 'KP9',          74: 'KPMINUS',      75: 'KP4',          76: 'KP5',
  77: 'KP6',          78: 'KPPLUS',       79: 'KP1',          80: 'KP2',
  81: 'KP3',          82: 'KP0',          83: 'KPDOT',        87: 'F11',
  88: 'F12',          89: 'RO',           96: 'KPENTER',      97: 'RIGHTCTRL',
  98: 'KPSLASH',      99: 'SYSRQ',        100: 'RIGHTALT',    102: 'HOME',
  103: 'UP',          104: 'PAGEUP',      105: 'LEFT',        106: 'RIGHT',
  107: 'END',         108: 'DOWN',        109: 'PAGEDOWN',    110: 'INSERT',
  111: 'DELETE',      119: 'PAUSE',       125: 'LEFTMETA',    126: 'RIGHTMETA',
  127: 'COMPOSE',     135: 'PASTE',       158: 'BACK',
}

# Dictionary of key color values
keys = {}

# Start a thread to read keyboard events
def read_kbd():
  # Open the keyboard in read-binary mode
  kbd = open('/dev/input/by-id/usb-ROCCAT_ROCCAT_Vulcan_AIMO-event-kbd', 'rb')
  while True:
    # Wait for keyboard event
    data = kbd.read(24)

    # Unpack event data ignoring timestamp
    (_, _, _, _, type, code, pressed) = unpack('4IHHI', data)

    # Skip non key-press events
    if type != 1 or pressed != 1: continue

    # Set key value
    keys[keymap[code]] = 255

t = Thread(target=read_kbd)
t.start()

# Open progam in text mode and line buffered to set keyboard RGB
rgb_set = Popen(
  'roccat-vulcan-set',
  bufsize=1,
  stdin=PIPE,
  universal_newlines=True
)

# Main loop, reduce key color values every 50ms
while True:
  sleep(0.05)

  # Nothing to do
  if len(keys) == 0: continue

  # Reduce key color values
  for key, value in keys.copy().items():
    keys[key] = max(value - 6, 0)

    # Send key value to program
    rgb_set.stdin.write(f' {key} {keys[key]} 0 0')
    if keys[key] == 0: del keys[key]

  # Newline to send values to keyboard
  rgb_set.stdin.write('\n')
