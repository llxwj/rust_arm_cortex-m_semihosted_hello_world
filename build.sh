
# Stop execution when an error occurs
set -e

# Compile to .rlib library
rustc --target=thumbv7em-none-eabi -O -Z no-landing-pads source/hello_world.rs --emit obj -o hello_world.o

# Link .o file using the provided linker script to generate an executable
# binary that can be deployed to the board's flash memory.  Remove the
# .o file aftwards as it is no longer needed
arm-none-eabi-ld hello_world.o --gc-sections -Tlink/link.ld -o hello_world
rm hello_world.o

# Display how much flash memory will be consumed by the executable
arm-none-eabi-size hello_world