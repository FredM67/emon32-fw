/* Memory layout for SAMD21J17A */
/* Based on the original linker script */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* SAMD21J17A has 128KB Flash, 16KB RAM */
  FLASH : ORIGIN = 0x00000000 + 8K, LENGTH = 128K - 8K  /* Reserve 8K for bootloader */
  RAM : ORIGIN = 0x20000000, LENGTH = 16K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
/* _stext = ORIGIN(FLASH) + 0x400; */

/* Example of putting non-initialized variables into custom RAM locations. */
/* This assumes you have defined a region RAM2 above, and in the Rust
   sources added the attribute `#[link_section = ".ram2bss"]` to the data
   you want to place there. */
/* Note that the section will not be zero-initialized by the runtime! */
/* SECTIONS {
     .ram2bss (NOLOAD) : ALIGN(4) {
       . = ALIGN(4);
       __sram2bss = .;
       *(.ram2bss);
       . = ALIGN(4);
       __eram2bss = .;
     } > RAM2
   } INSERT AFTER .bss;
*/