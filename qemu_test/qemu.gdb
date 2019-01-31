target extended-remote :3333

file target/thumbv7m-none-eabi/release/thread_msp
load

# print demangled symbols
set print asm-demangle on

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault

# *try* to stop at the user entry point (it might be gone due to inlining)
break main

# start the process but immediately halt the processor
stepi
