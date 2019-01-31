.global PendSV
.type PendSV,%function
.thumb
.thumb_func
.syntax unified
    PendSV:
            mov r0, lr

            /* Don't push to psp stack if we weren't using it. */
            and r1, r0, #4
            cbz r1, skip_stack_push

            /* Save psp stack. */
            mrs r12, psp
            stmdb r12!, {r4, r5, r6, r7, r8, r9, r10, r11}
            msr psp, r12
        skip_stack_push:
            bl cortex_m_switch_PendSV

            /* Don't pop psp stack if we're not switching to it. */
            and r1, r0, #4
            cbz r1, skip_stack_pop

            /* Restore psp stack. */
            mrs r12, psp
            ldmfd r12!, {r4, r5, r6, r7, r8, r9, r10, r11}
            msr psp, r12
        skip_stack_pop:
            bx r0
