Redshift Controller
======================================

[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.0-4baaaa.svg)](code-of-conduct.md)

### Table of Contents
1. [Purpose](https://github.com/jeremymreed/redshiftctl#purpose)
2. [Motivation](https://github.com/jeremymreed/redshiftctl#motivation)
3. [Install](https://github.com/jeremymreed/redshiftctl#usage)
4. [Issues](https://github.com/jeremymreed/redshiftctl#issues)
5. [License](https://github.com/jeremymreed/redshiftctl#license)


# Purpose:
This is a small Rust program to control redshift automatically.  It sets color temperature based on the time of the day.  Currently this program is hardcoded to swich to daytime color temperature at 6:00, and nighttime color temperature at 18:00.

This program is meant to be run by a systemd service.  Look at redshift-scripts.

# Motivation:
I want to automatically set the color temperature via redshift.  I'd like a different color temperature for daytime and nighttime.

# Install:
You need to have [Rust](https://www.rust-lang.org/) installed.  You will need cargo to build this software.

Clone this repository.

Modify files as needed.  Take a look at main.rs.  To change the times for morning/night, update lines 16 and 17.  These lines create the DateTime variables that the logic uses to determine if is day or night.

To build:
```
cargo build --release
```

To run locally:
```
cargo run
```

To install (this builds the software in release mode):
```
./install.sh
```

# Issues:

[Issues on github](https://github.com/jeremymreed/redshiftctl/issues)

# License:
This program is licensed under the MIT License.
