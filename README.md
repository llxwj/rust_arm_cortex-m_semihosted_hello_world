# ARM Cortex-M Semihosted "Hello World" in Rust
This is a simple experiment to generate a minimal semihosted "Hello World" program for the ARM Cortex-M platform (specifically the [STM32F429I-Disco](http://www.st.com/web/catalog/tools/FM116/SC959/SS1532/PF259090) board) using the Rust programming language.  It is my first Rust program.

All that is required to generate a working executable is the Rust compiler and the GNU linker.  It does not require the Rust runtime, the Rust core library (libcore), the Rust standard library, or the C standard library.  There are also no assembly files needed start execution.  All code can be found in a single Rust source file, `source/hello_world.rs`.  It must be linked with the `link/link.ld` linker script to make an executable binary that can run on a STM32F429I-DISCO board.

Dependencies
------------
This project needs a GNU ARM Cortex-M cross-compiling toolchain for the linker, GDB, and other binary utilities.  The one I used can be donwloaded from [GNU Tools for ARM Embedded Processors](https://launchpad.net/gcc-arm-embedded).

Deploying the binary to the board requires [OpenOCD](http://openocd.sourceforge.net/) with the [ST-Link driver](https://github.com/texane/stlink).  I'm using Arch Linux so I was able to obtain the OpenOCD package from the Arch Linux's official repository and the ST-Link driver was available in the Arch Linux User Repository (AUR).

Build
-----
On a Linux host, simply run the `build.sh` shell script.

Execution
---------
With the STM32F429I-DISCO board connected to the host PC via USB, run `openocd.sh` in one terminal session and `gdb.sh` in another terminal session.  In the GDB terminal session, type the following to deploy the binary to the board and begin execution:
```
monitor reset halt
load
monitor reset init
continue
```
You should then see "Hello World!" printed repeatedly in the OpenOCD terminal session.
