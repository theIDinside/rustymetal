.section ".text.boot"

.global _boot_cores

_boot_cores:
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, 2f

1:  wfe
    b       1b
2: // cpu id == 0

    // set stack before our code
    ldr     x1, =_boot_cores
    mov     sp, x1

    // jump to our Rust code
    b      reset
    // failsafe, halt this core
    b       1b