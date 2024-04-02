rm -R target
cargo rustc -r -- -C link-arg=--script=./linker.ld
arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/release/lenox ./LenoxOS/kernel7.img