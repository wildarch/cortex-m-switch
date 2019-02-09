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

# svc_{0..255}.s
for i in {0..255}
do
    func_name="cortex_m_switch_svc_$i"
    out_file="svc_$i.s"

    rm -f $out_file "svc_$i.a"

    echo ".global $func_name" >> $out_file
    echo ".type $func_name,%function" >> $out_file
    echo ".thumb_func" >> $out_file
    echo "$func_name:" >> $out_file
    echo "    svc #$i" >> $out_file
    echo "    mov pc, lr" >> $out_file
    echo "" >> $out_file

    arm-none-eabi-as -march=armv7-m $out_file -o "svc_$i.o"
    arm-none-eabi-ar crs "svc_$i.a" "svc_$i.o"
    rm $out_file "svc_$i.o"
done

# svc.rs
SRCPATH="$SCRIPTPATH/../src"
SVCFILE="$SRCPATH/svc.rs"

rm -f $SVCFILE

# svc! macro rule
echo "#[macro_export]" >> $SVCFILE
echo "macro_rules! svc {" >> $SVCFILE
for i in {0..255} 
do
    echo "    ($i \$(, \$arg_name:ident: \$arg_val:expr)*) => {" >> $SVCFILE
    echo "        {" >> $SVCFILE
    echo "            #[link(name = \"svc_$i\")]" >> $SVCFILE
    echo "            extern \"C\" {" >> $SVCFILE
    echo "                #[allow(dead_code)]" >> $SVCFILE
    echo "                pub fn cortex_m_switch_svc_$i(\$(\$arg_name: u32)*);" >> $SVCFILE
    echo "            }" >> $SVCFILE
    echo "            cortex_m_switch_svc_$i(\$(\$arg_val)*)" >> $SVCFILE
    echo "        }" >> $SVCFILE
    echo "    };" >> $SVCFILE

done
echo "}" >> $SVCFILE

