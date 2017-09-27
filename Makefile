KNAME  := rustboot
BITS   := 64
ARCH   := x86_64
LD     := $(ARCH)-elf-ld
QEMU   := qemu-system-$(ARCH)
TRIPLE := $(ARCH)-$(KNAME)
TARGET := target/debug/lib$(KNAME).a

RUSTC  := rustc
NASM   := nasm
CARGO  := xargo

all: floppy.img

.SUFFIXES: .o .rs .asm

.PHONY: clean run

$(TARGET):
	$(CARGO) build 	

.asm.o:
	$(NASM) -f elf$(BITS) -o $@ $<

floppy.img: loader.bin $(TARGET)
	dd if=/dev/zero of=$@ bs=512 count=2 &>/dev/null
	cat $^ | dd if=/dev/stdin of=$@ conv=notrunc &>/dev/null

loader.bin: loader.asm
	$(NASM) -o $@ -f bin $<

main.bin: linker.ld main.o *.rs
	$(LD) -m elf_$(ARCH) -o $@ -T linker.ld 

run: floppy.img
	$(QEMU) -drive format=raw,if=floppy,file=$<

clean:
	$(CARGO) clean
	rm -f $(TARGET) *.bin *.o *.img
