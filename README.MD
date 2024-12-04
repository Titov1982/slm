## slm - System Load Manager

A very simple console application for monitoring system load.
Written in RUST using the RATATUI framework and a small sysinfo library for obtaining system information.

-----------------------------

Operating systems supported:

  - Linux
  - MacOS

-----------------------------
### Version: 0.1.0
Dependencies:

 - sysinfo="0.32.0"
 - chrono = "0.4.38"
 - crossterm = "0.28.1"
 - ratatui = "0.29.0"

![slm image](/assets/slm_0.1.0.png)

### Version: 0.2.0
Dependencies:

- sysinfo="0.32.0"
- chrono = "0.4.38"
- crossterm = "0.28.1"
- ratatui = "0.29.0"
- csv = "1.3.1"
- clap= { version = "4.5.22", features = ["derive"] }

Description:

1. Added the ability to read recorded information about the system operation (processor load and memory usage). Information is logged using the "slmd" daemon.
2. Added the ability to set utility operation parameters in the command line.