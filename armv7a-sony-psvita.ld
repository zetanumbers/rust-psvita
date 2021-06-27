ENTRY(_start)
SECTIONS
{
  /* SCE ELF format requires text to begin at 0x81000000 */
  .text 0x81000000 : {
    *(.text .text.*)
  }

  .ARM.exidx ALIGN(0x10000) : {
    *(.ARM.exidx)
  }
  /* Keep these sections around, even though they may appear unused to the linker */
  .eh_frame_hdr : { KEEP(*(.eh_frame_hdr)) *(.eh_frame_entry .eh_frame_entry.*) }

  /* Add symbols for LLVM's libunwind */
  __eh_frame_hdr_start = SIZEOF(.eh_frame_hdr) > 0 ? ADDR(.eh_frame_hdr) : 0;
  __eh_frame_hdr_end = SIZEOF(.eh_frame_hdr) > 0 ? . : 0;
  .eh_frame :
  {
    __eh_frame_start = .;
    KEEP(*(.eh_frame))
    *(.eh_frame.*)
    __eh_frame_end = .;
  }

}