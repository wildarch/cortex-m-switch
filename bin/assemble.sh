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

arm-none-eabi-as -march=armv7-m svc.s -o svc.o
arm-none-eabi-ar crs svc.a svc.o
rm svc.o

# svc_extra.s
rm -f svc_extra.s svc_extra.a

for i in {0..255}
do
    func_name="cortex_m_switch_svc_$i"

    echo ".global $func_name" >> svc_extra.s
    echo ".type $func_name,%function" >> svc_extra.s
    echo ".thumb_func" >> svc_extra.s
    echo "$func_name:" >> svc_extra.s
    echo "    svc #$i" >> svc_extra.s
    echo "    mov pc, lr" >> svc_extra.s
    echo "" >> svc_extra.s
done

# svc.rs
SRCPATH="$SCRIPTPATH/../src"
SVCFILE="$SRCPATH/svc.rs"

rm -f $SVCFILE

# extern definitions
echo "#[link(name = \"svc_extra\")]" >> $SVCFILE
echo "extern \"C\" {" >> $SVCFILE
for i in {0..255}
do
    func_name="cortex_m_switch_svc_$i"

    echo "    #[allow(dead_code)]" >> $SVCFILE
    echo "    pub fn $func_name();" >> $SVCFILE

done
echo "}" >> $SVCFILE

echo "" >> $SVCFILE

# svc! macro rule
echo "#[macro_export]" >> $SVCFILE
echo "macro_rules! svc {" >> $SVCFILE
for i in {0..255} 
do
    echo "    ($i) => { unsafe { \$crate::svc::cortex_m_switch_svc_$i() } };" >> $SVCFILE
done
echo "}" >> $SVCFILE

arm-none-eabi-as -march=armv7-m svc_extra.s -o svc_extra.o
arm-none-eabi-ar crs svc_extra.a svc_extra.o
rm svc_extra.o
