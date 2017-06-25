MEMORY
{
	FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 512K
  RAM (rwx) : ORIGIN = 0x100000C8, LENGTH = (32K - 0xC8 - 32)  /* topmost 32 bytes used by IAP functions */

  USB_RAM(rwx) : ORIGIN = 0x2007C000, LENGTH = 16K
  ETH_RAM(rwx) : ORIGIN = 0x20080000, LENGTH = 16K
}


_stack_size = 4K;
_stack_start = ORIGIN(RAM) + _stack_size;

_heap_start = _stack_start;
_heap_end = ORIGIN(RAM) + LENGTH(RAM);

