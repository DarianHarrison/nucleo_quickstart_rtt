# nucleo_quickstart_rtt
 
## Debug

0. make sure to follow setup prerequisites: [https://github.com/DarianHarrison/nucleo_quickstart/docs/prerequisites.md](https://github.com/DarianHarrison/nucleo_quickstart/blob/main/docs/prerequisites.md)

1. Note: in this configuration you **don't need** to have openodc running on one terminal

2. sumply, do the follwoing on a single terminal
```bash
cargo run --release --example blinky
cargo run --release --example rtt_hello_world
```

## References

crates:
 https://crates.io/crates/stm32l4xx-hal

datasheet:
 https://www.st.com/resource/en/datasheet/stm32l432kb.pdf

rtt:
 https://docs.rs/rtt-target/latest/rtt_target/index.html

basic gdb functions:
 https://docs.rust-embedded.org/discovery/f3discovery/appendix/2-how-to-use-gdb/index.html

gdb:
 https://www.sourceware.org/gdb/documentation/

board:
![Alt Text](./docs/nucleo_l432kc.png)

The DSP capabilities of ARM Cortex-M4 and Cortex-M7 Processors:
 https://www.st.com/en/microcontrollers-microprocessors/stm32g473pc.html

 https://community.arm.com/cfs-file/__key/communityserver-blogs-components-weblogfiles/00-00-00-21-42/7563.ARM-white-paper-_2D00_-DSP-capabilities-of-Cortex_2D00_M4-and-Cortex_2D00_M7.pdf


configuration and examples in this repo are based on: 
 https://github.com/andy31415/rust_stm32f411_demos
 https://github.com/probe-rs/rtt-target
