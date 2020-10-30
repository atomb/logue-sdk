/*
    BSD 3-Clause License

    Copyright (c) 2018, KORG INC.
    All rights reserved.

    Redistribution and use in source and binary forms, with or without
    modification, are permitted provided that the following conditions are met:

    * Redistributions of source code must retain the above copyright notice, this
      list of conditions and the following disclaimer.

    * Redistributions in binary form must reproduce the above copyright notice,
      this list of conditions and the following disclaimer in the documentation
      and/or other materials provided with the distribution.

    * Neither the name of the copyright holder nor the names of its
      contributors may be used to endorse or promote products derived from
      this software without specific prior written permission.

    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
    AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
    IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
    DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
    FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
    DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
    SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
    CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
    OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
    OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//*/

/*
 *  File: userosc.ld
 *
 *  Linker Script for user oscillators
 */

/* Entry Point */
ENTRY(_hook_init)

/* Specify the memory areas */
MEMORY
{
  SRAM   (rx) : org = 0x20000000, len = 32K
}

/* ----------------------------------------------------------------------------- */
/* Define output sections */

SECTIONS
{

  .hooks : ALIGN(16) SUBALIGN(16)
  {
    . = ALIGN(4);
    _hooks_start = .;
    KEEP(*(.hooks))
    . = ALIGN(4);
    _hooks_end = .;
  } > SRAM

  /* Constructors */
  .init_array : ALIGN(4) SUBALIGN(4)
  {
    . = ALIGN(4);
    PROVIDE(__init_array_start = .);
    KEEP(*(SORT(.init_array.*)))
    KEEP(*(.init_array*))
    . = ALIGN(4);
    PROVIDE(__init_array_end = .);
  } > SRAM

  /* Common Code */
  .text : ALIGN(4) SUBALIGN(4)
  {
    . = ALIGN(4);
    _text_start = .;
    *(.text)
    *(.text.*)
    *(.glue_7)         /* glue arm to thumb code */
    *(.glue_7t)        /* glue thumb to arm code */
    *(.gcc*)
    . = ALIGN(4);
    _text_end = .;
  } > SRAM

  /* Constants and strings */
  .rodata : ALIGN(4) SUBALIGN(4)
  {
    . = ALIGN(4);
    _rodata_start = .;
    *(.rodata)
    *(.rodata.*)
    . = ALIGN(4);
    _rodata_end = .;
  } > SRAM

  /* Read-write data */
  .data ALIGN(8) : ALIGN(8) SUBALIGN(8)
  {
    . = ALIGN(8);
    _data_start = .;
    *(.data)
    *(.data.*)
    . = ALIGN(8);
    _data_end = .;
  } > SRAM

  /* Uninitialized variables */
  .bss (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    _bss_start = .;
    *(.bss)
    *(.bss.*)
    *(COMMON)
    . = ALIGN(4);
    _bss_end = .;
  } > SRAM

  /* Exception sections */
  .ARM.extab : ALIGN(4) SUBALIGN(4)
  {
    . = ALIGN(4);
    __extab_start = .;
    *(.ARM.extab* .gnu.linkonce.armextab.*)
    . = ALIGN(4);
    __extab_end = .;
  } > SRAM

  .ARM.exidx : ALIGN(4) SUBALIGN(4)
  { /* Note: Aligning when there's no content for this section throws a warning. Looks like a linker bug. */
    /* . = ALIGN(4); */
    __exidx_start = .;
    *(.ARM.exidx* .gnu.linkonce.armexidx.*)
    /* . = ALIGN(4); */
    __exidx_end = .;
  } > SRAM

  .eh_frame_hdr : ALIGN(4) SUBALIGN(4)
  {
    . = ALIGN(4);
    _eh_frame_hdr_start = .;
    *(.eh_frame_hdr)
    . = ALIGN(4);
    _eh_frame_hdr_end = .;
  } > SRAM

  .eh_frame : ALIGN(4) SUBALIGN(4) ONLY_IF_RO
  {
    . = ALIGN(4);
    _eh_frame_start = .;
    *(.eh_frame)
    . = ALIGN(4);
    _eh_frame_end = .;
  } > SRAM

  /*
  /DISCARD/
  {
    libc.a   ( * )
    libm.a   ( * )
    libgcc.a ( * )
  }
  //*/

  /* .ARM.attributes 0 : { *(.ARM.attributes) } //*/
}

k_osc_api_version = 0x0800f000;
k_osc_api_platform = 0x0800f004;
midi_to_hz_lut_f = 0x0800f100;
sqrtm2log_lut_f = 0x0800f360;
tanpi_lut_f = 0x0800f764;
log_lut_f = 0x0800fb68;
bitres_lut_f = 0x0800ff6c;
wt_par_lut_f = 0x08010170;
wt_par_notes = 0x08010f8c;
wt_sqr_lut_f = 0x08010f94;
wt_sqr_notes = 0x08011db0;
wt_saw_lut_f = 0x08011db8;
wt_saw_notes = 0x08012bd4;
wt_sine_lut_f = 0x08012bdc;
schetzen_lut_f = 0x08012de0;
cubicsat_lut_f = 0x08012fe4;
wavesA = 0x080131e8;
wavesB = 0x0801546c;
wavesC = 0x080174ec;
wavesD = 0x0801915c;
wavesE = 0x0801abc4;
wavesF = 0x0801ca3c;
_osc_mcu_hash = 0x0801eabd;
_osc_bl_saw_idx = 0x0801eac9;
_osc_bl_sqr_idx = 0x0801ebb1;
_osc_bl_par_idx = 0x0801ec99;
_osc_rand = 0x0801ed81;
_osc_white = 0x0801edb9;
