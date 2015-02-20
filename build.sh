
set -e

rustc --target=thumbv7em-none-eabi -O source/hello_world.rs
ar x libhello_world.rlib hello_world.o
rm libhello_world.rlib
arm-none-eabi-ld hello_world.o --gc-sections -Tlink/link.ld -o hello_world
rm hello_world.o
arm-none-eabi-size hello_world