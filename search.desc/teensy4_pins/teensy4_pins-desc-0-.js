searchState.loadedDescShard("teensy4_pins", 0, "Hardware pins for the Teensy 4.0, 4.1 and MicroMod boards\nA configuration capable of compile-time, <code>const</code> …\nDrive strength\nThe hysteresis (HYS) bit controls whether a pin acts as a …\nUse the keeper, instead of a pull up or pull down resistor.\nOpen Drain Enable Field\nThe pull up, pull down, or keeper configuration.\n100KOhm pull <strong>down</strong>\n100KOhm pull <strong>up</strong>\n22KOhm pull <strong>up</strong>\n47KOhm pull <strong>up</strong>\n150 Ohm @ 3.3V, 260 Ohm@1.8V\nR0 / 2\nR0 / 3\nR0 / 4\nSlew Rate\nSets electrical characteristics of a pin in a given …\nCommon pinout\nApplies the configuration <code>config</code> for the supplied pad\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns <code>true</code> if this <code>Config</code> was created using <code>zero()</code>, …\nCreate a <code>Config</code> that will only modify the specified fields\nSet the drive strength\nSet the hysteresis bit\nSet the open drain value\nSet the flag that enables the keeper or pull-up / …\nSet the the pull-up / pull-down or keeper selection bit\nSet the pull up / pull down / keeper configuration.\nSet the pull-up / pull-down value\nSet the slew rate\nSet the pin speed\nTeensy 4.0 specific APIs\nTeensy 4.1 specific APIs\nTeensy MicroMod specific APIs\nCreate a <code>Config</code> that will zero any unspecified field\nPin 0 (common)\nPin 1 (common)\nPin 10 (common)\nPin 11 (common)\nPin 12 (common)\nPin 13 (common)\nPin 14 (common)\nPin 15 (common)\nPin 16 (common)\nPin 17 (common)\nPin 18 (common)\nPin 19 (common)\nPin 2 (common)\nPin 20 (common)\nPin 21 (common)\nPin 22 (common)\nPin 23 (common)\nPin 24 (common)\nPin 25 (common)\nPin 26 (common)\nPin 27 (common)\nPin 28 (common)\nPin 29 (common)\nPin 3 (common)\nPin 30 (common)\nPin 31 (common)\nPin 32 (common)\nPin 33 (common)\nPin 4 (common)\nPin 5 (common)\nPin 6 (common)\nPin 7 (common)\nPin 8 (common)\nPin 9 (common)\nType-erased Teensy 4.0 pins\nPin 34 (4.0)\nPin 35 (4.0)\nPin 36 (4.0)\nPin 37 (4.0)\nPin 38 (4.0)\nPin 39 (4.0)\nTeensy 4.0 pins\nErase the types of all pins\nReturns the argument unchanged.\nConstrain the processor pads to the Teensy 4.0 pins\nCalls <code>U::from(self)</code>.\nCreate an instance of <code>Pins</code> when you do not have a handle …\nPin 0\nPin 1\nPin 10\nPin 11\nPin 12\nPin 13\nPin 14\nPin 15\nPin 16\nPin 17\nPin 18\nPin 19\nPin 2\nPin 20\nPin 21\nPin 22\nPin 23\nPin 24\nPin 25\nPin 26\nPin 27\nPin 28\nPin 29\nPin 3\nPin 30\nPin 31\nPin 32\nPin 33\nPin 34\nPin 35\nPin 36\nPin 37\nPin 38\nPin 39\nPin 4\nPin 5\nPin 6\nPin 7\nPin 8\nPin 9\nType-erased Teensy 4.1 pins\nPin 34 (4.1)\nPin 35 (4.1)\nPin 36 (4.1)\nPin 37 (4.1)\nPin 38 (4.1)\nPin 39 (4.1)\nPin 40 (4.1)\nPin 41 (4.1)\nPin 42 (4.1)\nPin 43 (4.1)\nPin 44 (4.1)\nPin 45 (4.1)\nPin 46 (4.1)\nPin 47 (4.1)\nPin 48 (4.1)\nPin 49 (4.1)\nPin 50 (4.1)\nPin 51 (4.1)\nPin 52 (4.1)\nPin 53 (4.1)\nPin 54 (4.1)\nTeensy 4.1 pins\nErase the types of all pins\nReturns the argument unchanged.\nConstrain the processor pads to the Teensy 4.1 pins\nCalls <code>U::from(self)</code>.\nCreate an instance of <code>Pins</code> when you do not have a handle …\nPin 0\nPin 1\nPin 10\nPin 11\nPin 12\nPin 13\nPin 14\nPin 15\nPin 16\nPin 17\nPin 18\nPin 19\nPin 2\nPin 20\nPin 21\nPin 22\nPin 23\nPin 24\nPin 25\nPin 26\nPin 27\nPin 28\nPin 29\nPin 3\nPin 30\nPin 31\nPin 32\nPin 33\nPin 34\nPin 35\nPin 36\nPin 37\nPin 38\nPin 39\nPin 4\nPin 40\nPin 41\nPin 42\nPin 43\nPin 44\nPin 45\nPin 46\nPin 47\nPin 48\nPin 49\nPin 5\nPin 50\nPin 51\nPin 52\nPin 53\nPin 54\nPin 6\nPin 7\nPin 8\nPin 9\nType-erased Teensy MicroMod pins\nPin 34 (MicroMod)\nPin 35 (MicroMod)\nPin 36 (MicroMod)\nPin 37 (MicroMod)\nPin 38 (MicroMod)\nPin 39 (MicroMod)\nPin 40 (MicroMod)\nPin 41 (MicroMod)\nPin 42 (MicroMod)\nPin 43 (MicroMod)\nPin 44 (MicroMod)\nPin 45 (MicroMod)\nTeensy MicroMod pins\nErase the types of all pins\nReturns the argument unchanged.\nConstrain the processor pads to the Teensy MicroMod pins\nCalls <code>U::from(self)</code>.\nCreate an instance of <code>Pins</code> when you do not have a handle …\nPin 0\nPin 1\nPin 10\nPin 11\nPin 12\nPin 13\nPin 14\nPin 15\nPin 16\nPin 17\nPin 18\nPin 19\nPin 2\nPin 20\nPin 21\nPin 22\nPin 23\nPin 24\nPin 25\nPin 26\nPin 27\nPin 28\nPin 29\nPin 3\nPin 30\nPin 31\nPin 32\nPin 33\nPin 34\nPin 35\nPin 36\nPin 37\nPin 38\nPin 39\nPin 4\nPin 40\nPin 41\nPin 42\nPin 43\nPin 44\nPin 45\nPin 5\nPin 6\nPin 7\nPin 8\nPin 9")