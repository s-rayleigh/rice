Rice
====
Rice is a tool to configure SteelSeries Rival 700 mouse under the Linux operating system.

Usage
-----
For now, it's only possible to set logo and wheel color.

### Commands
To change the light color of the mouse wheel enter:
```
# rice wheel-color <color>
```

To change the light color of the mouse logo enter:
```
# rice logo-color <color>
```

Color should be `red` or `green` or `blue` or the `HEX RGB`.  
Example:
```	
# rice logo-color "#FF8B00"
```

Build
-----
1. Make sure you have installed `rust` compiler and `cargo`.
2. Run 
```
$ git clone https://github.com/s-rayleigh/rice.git`
$ cd rice
$ cargo build --release
```
3. Rice should be in `target/release` directory.


Copyright and license
---------------------
Copyright 2016-2018 Vladislav Pashaiev.

Code of the Rice project distributed under the terms of GPL-3.0 license. See [COPYING](COPYING) for details.