#!/bin/bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

rm -f *.a

arm-none-eabi-as -march=armv7-m SysTick_handler.s -o SysTick_handler.o
arm-none-eabi-ar crs SysTick_handler.a SysTick_handler.o
rm SysTick_handler.o

arm-none-eabi-as -march=armv7-m PendSV_handler.s -o PendSV_handler.o
arm-none-eabi-ar crs PendSV_handler.a PendSV_handler.o
rm PendSV_handler.o

arm-none-eabi-as -march=armv7-m SVCall_handler.s -o SVCall_handler.o
arm-none-eabi-ar crs SVCall_handler.a SVCall_handler.o
rm SVCall_handler.o
