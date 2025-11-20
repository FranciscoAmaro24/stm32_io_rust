# Stm32_io_rust 
Small boilerplate for an stm32h7xx nucleo for io between UART and RTT channels (rx/tx and rtt). 

This code should run on any stm32h7xx nucleo. I did not test for other boards.

## Requirments 
To run this file you need to download probe run and embed cargo crates

## How to run
Set the target for your own development board in the Embed.toml file if you want to run with ```embed run``` or/and on the config file for ```cargo run```

## Test using minicom
I would also recommend to install minicom on your kernel so input from the UART terminal can be tested

## Windows
If you're running windows I would reccomend to run WSL and to install ```usbipd``` so you can give access to your usb ports via the wsl

### Debug
Using `Minicom` with a split terminal is a nice way to debug it.

On the kernel run ```minicom -D /dev/ttyACM0 -b 115200``` or replace ttyACM0 by the tyy port that the usb is connected

## Usefull Links
[Usbipd](https://learn.microsoft.com/en-us/windows/wsl/connect-usb)

[Minicom](https://learn.microsoft.com/en-us/windows/wsl/connect-usb](https://www.cyberciti.biz/tips/connect-soekris-single-board-computer-using-minicom.html)https://www.cyberciti.biz/tips/connect-soekris-single-board-computer-using-minicom.html)

[Wsl](https://learn.microsoft.com/en-us/windows/wsl/install)

The [memory.x](https://github.com/astraw/nucleo-h743zi) file was taken from another boilerplate repository for the nucleo-h743zi

Dont forget to add the target


