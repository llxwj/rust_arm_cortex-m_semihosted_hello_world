/*
Copyright (c) 2015 Michael V. Franklin
     
This file is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This file is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this file.  If not, see <http://www.gnu.org/licenses/>.
*/

MEMORY
{
  CCRAM (rxw) : ORIGIN = 0x10000000, LENGTH =   64k
  SRAM  (rxw) : ORIGIN = 0x20000000, LENGTH =  192k
  FLASH (rx)  : ORIGIN = 0x08000000, LENGTH = 2048k
}

/* Falling stack starts at the end of the CCM */
_stackStart = ORIGIN(CCRAM) + LENGTH(CCRAM);

SECTIONS
{
    /* Not ready to implement exceptions yet, so discard this to prevent 
    linker errors */
    /DISCARD/ :
    {
        *(.ARM.extab*)
        *(.ARM.exidx*)
    }
    
    /* Appears to be just meta-data, so discard it */
    /DISCARD/ :
    {
        *(.ARM.attributes*)
        *(.comment)
    }    

    /* Code */
    .text :
    {
        LONG(_stackStart);                 /* Initial stack pointer */
        KEEP(*(.isr_vector))               /* ISR vector entry point to main */
        *(.text)                           /* Our executable code */
        *(.text*) 
    } > FLASH
    
    .rodata : ALIGN(4)
    {
        *(.rodata)
        *(.rodata*)
    } > FLASH
    
    __text_end__ = .;
    
    .data : AT(__text_end__) ALIGN(4)
    {
         __data_start__ = .;
        *(.data)
        *(.data.*)
        __data_end__ = .;           
    } > SRAM
    
    .bss : ALIGN(4)
    {
         __bss_start__ = .;
        *(.bss)
        *(.bss.*)
        __bss_end__ = .;           
    } > SRAM
}