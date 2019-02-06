.global cortex_m_switch_svc
.type cortex_m_switch_svc,%function
.thumb
.thumb_func
.syntax unified
cortex_m_switch_svc:
    /* We use the EABI convention.
     *
     * Alternatively, we could opt to put the system call number in the instruction itself,
     * but this makes the number relatively difficult to retrieve:
     * One would need to first find the old PC on the stack, check if the system call was made 
     * in arm or thumb mode, then retrieve the number from the instruction. Reading it from a
     * register on the stack is significantly easier.
     */
    svc #0
    mov pc, lr
