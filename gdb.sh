
arm-none-eabi-gdb hello_world -ex "target remote localhost:3333" -ex "monitor arm semihosting enable"
