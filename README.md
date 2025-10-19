# MelloOS

MelloOS เป็นระบบปฏิบัติการที่สร้างขึ้นตั้งแต่ศูนย์ด้วยภาษา Rust โดยมุ่งเน้นความปลอดภัย ความทันสมัย และความสามารถในการขยายต่อได้ในอนาคต โปรเจกต์นี้เริ่มต้นจากการพัฒนา Kernel ที่สามารถบูตผ่าน UEFI bootloader และแสดงข้อความบนหน้าจอได้

## Features

- ✨ Bare-metal kernel เขียนด้วย Rust (`no_std`)
- 🚀 บูตผ่าน UEFI firmware ด้วย Limine bootloader
- 🖥️ Framebuffer driver สำหรับการแสดงผลบนหน้าจอ
- 🧠 **Memory Management System**
  - Physical Memory Manager (PMM) with bitmap allocator
  - 4-level paging system with NX bit support
  - Kernel heap allocator using Buddy System algorithm
  - Memory protection with guard pages
  - HHDM (Higher Half Direct Mapping) support
- ⚡ **Task Scheduler**
  - Preemptive multitasking with Round-Robin scheduling
  - Hardware timer interrupts (PIT at 100 Hz)
  - Context switching with full register save/restore
  - Task Control Blocks (TCB) with per-task stacks
  - Interrupt Descriptor Table (IDT) configuration
- 🔧 Build system อัตโนมัติด้วย Makefile
- 🧪 ทดสอบได้ง่ายด้วย QEMU emulator
- 🔒 Security features: NX bit, write protection, memory zeroing

## Prerequisites

ก่อนเริ่มต้น คุณต้องติดตั้ง dependencies ต่อไปนี้:

### Required Dependencies

1. **Rust Toolchain**
   - Rust compiler และ Cargo package manager
   - Target สำหรับ bare-metal x86_64

2. **QEMU**
   - QEMU system emulator สำหรับ x86_64

3. **xorriso**
   - ISO image creation tool

4. **OVMF**
   - UEFI firmware สำหรับ QEMU

5. **Git**
   - สำหรับดาวน์โหลด Limine bootloader

## Installation

### macOS

```bash
# ติดตั้ง Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# เพิ่ม Rust target สำหรับ bare-metal
rustup target add x86_64-unknown-none

# ติดตั้ง QEMU, xorriso ด้วย Homebrew
brew install qemu xorriso

# ติดตั้ง OVMF (UEFI firmware)
brew install --cask edk2-ovmf
```

### Linux (Ubuntu/Debian)

```bash
# ติดตั้ง Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# เพิ่ม Rust target สำหรับ bare-metal
rustup target add x86_64-unknown-none

# ติดตั้ง QEMU, xorriso, OVMF
sudo apt update
sudo apt install qemu-system-x86 xorriso ovmf git
```

### Linux (Arch)

```bash
# ติดตั้ง Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# เพิ่ม Rust target สำหรับ bare-metal
rustup target add x86_64-unknown-none

# ติดตั้ง QEMU, xorriso, OVMF
sudo pacman -S qemu-full xorriso edk2-ovmf git
```

## Building

### Build Kernel

คอมไพล์ kernel binary:

```bash
make build
```

คำสั่งนี้จะ:
- รัน `cargo build --release` เพื่อคอมไพล์ kernel
- สร้าง ELF binary ที่ `kernel/target/x86_64-unknown-none/release/kernel`

### Create Bootable ISO

สร้าง ISO image ที่สามารถบูตได้:

```bash
make iso
```

คำสั่งนี้จะ:
- คอมไพล์ kernel (ถ้ายังไม่ได้ build)
- ดาวน์โหลด Limine bootloader (ถ้ายังไม่มี)
- สร้างโครงสร้างไดเรกทอรีสำหรับ ISO
- คัดลอก kernel binary และ bootloader files
- สร้าง `mellos.iso` ด้วย xorriso
- ติดตั้ง Limine bootloader ลงใน ISO

### Run in QEMU

รัน kernel ใน QEMU emulator:

```bash
make run
```

คำสั่งนี้จะ:
- สร้าง ISO image (ถ้ายังไม่มี)
- เริ่มต้น QEMU ด้วย UEFI firmware
- บูต MelloOS จาก ISO
- Initialize memory management system
- Run memory management tests
- แสดงหน้าต่าง QEMU พร้อมข้อความ "Hello from MelloOS ✨"

หากต้องการปิด QEMU ให้กด `Ctrl+C` ใน terminal หรือปิดหน้าต่าง QEMU

### What Happens During Boot

1. **Limine Bootloader** loads the kernel and provides system information
2. **Framebuffer Initialization** sets up graphics output
3. **Memory Management Initialization**:
   - HHDM offset configuration
   - CPU protection features (NX bit, write protection)
   - Physical Memory Manager initialization
   - Page table setup with kernel section mapping
   - Heap region mapping (16MB at 0xFFFF_A000_0000_0000)
   - Guard page installation
   - Kernel heap allocator initialization
4. **Memory Tests** verify all MM components work correctly
5. **Task Scheduler Initialization**:
   - Idle task creation (task ID 0)
   - Interrupt Descriptor Table (IDT) setup
   - PIC (Programmable Interrupt Controller) remapping
   - PIT (Programmable Interval Timer) configuration at 100 Hz
   - Demo tasks spawned (Task A and Task B)
6. **Interrupts Enabled** - timer begins triggering context switches
7. **Welcome Message** displays on screen
8. **Multitasking** - tasks switch every 10ms, displaying alternating output

### Clean Build Artifacts

ลบไฟล์ที่สร้างขึ้นจากการ build:

```bash
make clean
```

คำสั่งนี้จะลบ:
- Cargo build artifacts
- ISO image
- Temporary directories

## Architecture

### Memory Management

MelloOS implements a comprehensive memory management system with three main components:

1. **Physical Memory Manager (PMM)**
   - Bitmap-based frame allocator (4KB frames)
   - Tracks free and used physical memory
   - Supports contiguous allocation for DMA
   - Automatic memory zeroing for security

2. **Paging System**
   - 4-level page tables (PML4 → PDPT → PD → PT)
   - Per-section permissions (RX for .text, R for .rodata, RW+NX for .data)
   - Guard pages for stack/heap overflow protection
   - TLB invalidation support

3. **Kernel Heap Allocator**
   - Buddy System algorithm (64B to 1MB blocks)
   - Thread-safe with Mutex protection
   - `kmalloc()` and `kfree()` API
   - Automatic block splitting and merging

### Task Scheduler

MelloOS implements a preemptive multitasking scheduler with the following components:

1. **Round-Robin Scheduling**
   - Fair time-sharing between all tasks
   - Each task gets equal CPU time (10ms time slices at 100 Hz)
   - Simple FIFO runqueue for task management
   - Automatic task rotation on timer interrupts

2. **Context Switching**
   - Full CPU context save/restore (callee-saved registers)
   - Per-task 8KB stacks allocated from kernel heap
   - Assembly-optimized context switch routine
   - Follows x86_64 System V ABI calling convention

3. **Timer Interrupt System**
   - PIT (Programmable Interval Timer) configured at 100 Hz
   - PIC (Programmable Interrupt Controller) remapped to avoid conflicts
   - IDT (Interrupt Descriptor Table) with timer handler at vector 32
   - Automatic EOI (End of Interrupt) handling

4. **Task Management**
   - Task Control Blocks (TCB) with unique IDs
   - Task states: Ready, Running, Sleeping
   - Idle task (ID 0) runs when no other tasks available
   - Thread-safe task table with mutex protection

### Security Features

- **NX Bit Support**: Non-executable pages prevent code execution in data regions
- **Write Protection**: Kernel respects page-level write permissions
- **Memory Zeroing**: All allocated memory is zeroed before use
- **Guard Pages**: Unmapped pages around critical regions catch overflow/underflow
- **Stack Isolation**: Each task has its own isolated stack

## Project Structure

```
mellos/
├── .cargo/
│   └── config.toml          # Cargo build configuration
├── .github/
│   ├── workflows/
│   │   ├── build-and-release.yml    # Release automation
│   │   └── test-develop.yml         # CI/CD testing
│   └── BRANCH_PROTECTION.md         # Branch protection guide
├── .kiro/
│   └── specs/
│       └── memory-management/       # Memory management spec
├── kernel/
│   ├── Cargo.toml           # Kernel dependencies (limine, spin, x86_64)
│   ├── linker.ld            # Linker script
│   └── src/
│       ├── main.rs          # Kernel entry point
│       ├── framebuffer.rs   # Framebuffer driver with 8x8 font
│       ├── panic.rs         # Panic handler
│       ├── mm/              # Memory management subsystem
│       │   ├── mod.rs       # MM coordinator and HHDM
│       │   ├── pmm.rs       # Physical Memory Manager
│       │   ├── paging.rs    # Virtual memory and page tables
│       │   ├── allocator.rs # Kernel heap allocator
│       │   └── log.rs       # MM logging utilities
│       └── sched/           # Task scheduler subsystem
│           ├── mod.rs       # Scheduler core and runqueue
│           ├── task.rs      # Task Control Block (TCB)
│           ├── context.rs   # Context switching (assembly)
│           └── timer.rs     # Timer interrupt handling
├── boot/
│   └── limine.cfg           # Bootloader configuration
├── docs/
│   └── memory-management-logging.md # MM logging documentation
├── tools/
│   ├── qemu.sh              # QEMU launch script
│   ├── test_boot.sh         # Boot testing script
│   └── verify_build.sh      # Build verification script
├── Makefile                 # Build automation
└── README.md                # This file
```

## Troubleshooting

### Build Errors

**Problem:** `error: target 'x86_64-unknown-none' not found`

**Solution:** ติดตั้ง Rust target:
```bash
rustup target add x86_64-unknown-none
```

---

**Problem:** `cargo: command not found`

**Solution:** ติดตั้ง Rust toolchain และเพิ่ม Cargo ใน PATH:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

**Problem:** Linker errors เกี่ยวกับ `_start` symbol

**Solution:** ตรวจสอบว่า `linker.ld` ถูกกำหนดใน `.cargo/config.toml` และมี `#[no_mangle]` บน `_start` function

### ISO Creation Errors

**Problem:** `xorriso: command not found`

**Solution:** ติดตั้ง xorriso:
- macOS: `brew install xorriso`
- Ubuntu/Debian: `sudo apt install xorriso`
- Arch: `sudo pacman -S xorriso`

---

**Problem:** `limine: command not found` หรือ Limine files ไม่พบ

**Solution:** Makefile จะดาวน์โหลด Limine อัตโนมัติ แต่ถ้ามีปัญหา ให้ลอง clone manually:
```bash
git clone https://github.com/limine-bootloader/limine.git --branch=v8.x-binary --depth=1
cd limine
make
```

### QEMU Errors

**Problem:** `qemu-system-x86_64: command not found`

**Solution:** ติดตั้ง QEMU:
- macOS: `brew install qemu`
- Ubuntu/Debian: `sudo apt install qemu-system-x86`
- Arch: `sudo pacman -S qemu-full`

---

**Problem:** `Could not open '/usr/share/ovmf/OVMF.fd'`

**Solution:** OVMF firmware path อาจแตกต่างกันในแต่ละระบบ แก้ไข `tools/qemu.sh`:

- macOS (Homebrew): `/opt/homebrew/share/edk2-ovmf/x64/OVMF.fd`
- Ubuntu/Debian: `/usr/share/OVMF/OVMF_CODE.fd`
- Arch: `/usr/share/edk2-ovmf/x64/OVMF.fd`

หรือรัน QEMU โดยไม่ใช้ UEFI (legacy BIOS mode):
```bash
qemu-system-x86_64 -M q35 -m 2G -cdrom mellos.iso -boot d
```

---

**Problem:** QEMU เปิดแต่หน้าจอดำ

**Solution:** 
1. ตรวจสอบว่า ISO ถูกสร้างสำเร็จ: `ls -lh mellos.iso`
2. ตรวจสอบ serial output ใน terminal
3. ลอง rebuild: `make clean && make run`

---

**Problem:** ข้อความไม่แสดงบนหน้าจอ QEMU

**Solution:**
1. ตรวจสอบว่า framebuffer request ใน `main.rs` ถูกต้อง
2. ตรวจสอบว่า Limine configuration มี `PROTOCOL=limine`
3. ลอง rebuild kernel: `make clean && make build && make iso && make run`

### Runtime Errors

**Problem:** Kernel panic ทันทีหลังบูต

**Solution:**
1. ตรวจสอบ panic message ใน serial output
2. ตรวจสอบว่า framebuffer response จาก Limine ไม่เป็น null
3. เพิ่ม debug output ใน panic handler

---

**Problem:** Kernel หยุดทำงานโดยไม่แสดง error

**Solution:**
1. เพิ่ม serial port debugging
2. ใช้ QEMU monitor: กด `Ctrl+Alt+2` เพื่อเข้า monitor mode
3. ตรวจสอบ CPU state ด้วย `info registers` ใน QEMU monitor

## Testing

### Automated Build Verification

รันการทดสอบอัตโนมัติเพื่อตรวจสอบว่า build process ทำงานถูกต้อง:

```bash
./tools/verify_build.sh
```

สคริปต์นี้จะตรวจสอบ:
- ✓ Kernel binary ถูกสร้างสำเร็จ
- ✓ ข้อความที่ต้องการอยู่ใน kernel
- ✓ ISO image ถูกสร้างและมี kernel
- ✓ QEMU พร้อมใช้งาน
- ✓ Limine bootloader files ครบถ้วน
- ✓ Configuration files ถูกต้อง

### Manual Visual Testing

เนื่องจาก kernel แสดงผลผ่าน framebuffer (graphical display) คุณต้องทดสอบด้วยตาเองว่าข้อความแสดงถูกต้อง:

```bash
make run
```

**คาดหวังผลลัพธ์:**
1. หน้าต่าง QEMU เปิดขึ้น
2. Limine bootloader menu ปรากฏ (รอ 3 วินาที)
3. Kernel บูตอัตโนมัติ
4. Memory management system initializes (internal tests run)
5. Task scheduler initializes (idle task, timer, demo tasks)
6. ข้อความ **"Hello from MelloOS ✨"** แสดงบนหน้าจอ
7. Tasks begin switching - you'll see alternating "A" and "B" output
8. Serial console shows `[SCHED]` messages with context switch information

### Memory Management Tests

The kernel automatically runs comprehensive memory management tests during initialization:

- **PMM Tests**: Frame allocation, multiple allocations, free/reallocation
- **Paging Tests**: Page mapping, translation, unmapping
- **Allocator Tests**: kmalloc/kfree, memory read/write, multiple allocations

All tests must pass for the kernel to display the welcome message.

### Task Scheduler Tests

The kernel demonstrates multitasking with two demo tasks:

- **Task A**: Prints "A" repeatedly with busy-wait delays
- **Task B**: Prints "B" repeatedly with busy-wait delays
- **Context Switches**: Logged to serial console with `[SCHED]` prefix
- **Round-Robin**: Tasks alternate every 10ms (100 Hz timer)

You can observe the alternating output on the screen and detailed scheduler logs in the serial console.

## Current Capabilities

MelloOS currently provides:

✅ **Boot and Initialization**
- UEFI boot via Limine bootloader
- Framebuffer graphics initialization
- System information from bootloader (memory map, kernel addresses, HHDM offset)

✅ **Memory Management**
- Physical memory tracking and allocation (4KB frames)
- Virtual memory with 4-level page tables
- Dynamic memory allocation (64B to 1MB blocks)
- Memory protection and security features
- Automatic testing of all MM components

✅ **Task Scheduler**
- Preemptive multitasking with Round-Robin algorithm
- Timer-based context switching (100 Hz)
- Per-task stacks and CPU contexts
- Task spawning and management
- Interrupt handling (IDT, PIC, PIT)

✅ **Display**
- Pixel-level framebuffer access
- 8x8 bitmap font rendering
- String and character drawing
- Screen clearing and color support

✅ **Development Tools**
- Automated build system
- QEMU testing environment
- CI/CD with GitHub Actions
- Build verification scripts

## Limitations

Current limitations to be aware of:

⚠️ **Basic Interrupts Only**: Only timer interrupts implemented (no keyboard, disk, etc.)
⚠️ **No Serial Driver**: Serial output works but no proper driver infrastructure
⚠️ **Single Core**: Multi-core support not implemented
⚠️ **No User Space**: Only kernel tasks run, no user processes
⚠️ **No File System**: No storage or file system support
⚠️ **Simple Scheduling**: Round-Robin only, no priorities or advanced algorithms

## CI/CD

โปรเจกต์นี้ใช้ GitHub Actions สำหรับการทดสอบและ release อัตโนมัติ:

### Automated Testing (Develop Branch)

เมื่อมีการ push หรือสร้าง Pull Request ไปยัง `develop` branch:
- ✅ Build kernel อัตโนมัติ
- ✅ รัน build verification tests
- ✅ สร้าง ISO image
- ✅ ทดสอบการ boot ใน QEMU

ดูรายละเอียดได้ที่: `.github/workflows/test-develop.yml`

### Branch Protection

เพื่อความปลอดภัยของโค้ด แนะนำให้ตั้งค่า Branch Protection สำหรับ `develop` และ `main` branches:
- ✅ ต้องผ่าน Pull Request เท่านั้น
- ✅ ต้องผ่าน automated tests ก่อน merge
- ✅ ต้องได้รับ code review approval

ดูคู่มือการตั้งค่าได้ที่: `.github/BRANCH_PROTECTION.md`

### Automated Releases

เมื่อสร้าง version tag (เช่น `v1.0.0`):
- ✅ Build kernel และสร้าง ISO
- ✅ สร้าง GitHub Release อัตโนมัติ
- ✅ แนบ `mellos.iso` ไฟล์สำหรับดาวน์โหลด

ดูรายละเอียดได้ที่: `.github/workflows/build-and-release.yml`

## Development

### Adding New Features

1. แก้ไขโค้ดใน `kernel/src/`
2. Build และทดสอบ: `make run`
3. ตรวจสอบผลลัพธ์ใน QEMU

### Using Memory Management APIs

The kernel provides memory management APIs for dynamic allocation:

```rust
use crate::mm::allocator::{kmalloc, kfree};

// Allocate memory
let ptr = kmalloc(1024);  // Allocate 1KB
if !ptr.is_null() {
    // Use memory
    unsafe {
        *ptr = 0x42;
    }
    
    // Free memory when done
    kfree(ptr, 1024);
}
```

**Important Notes:**
- Always check if `kmalloc()` returns null (out of memory)
- Always call `kfree()` with the same size used in `kmalloc()`
- Memory is automatically zeroed for security
- All allocations are thread-safe (protected by Mutex)

### Memory Management Logging

The MM subsystem provides logging macros with `[MM]` prefix:

```rust
use crate::{mm_log, mm_info, mm_error, mm_test_ok};

mm_log!("Initializing subsystem...");
mm_info!("Total memory: {} MB", total_mb);
mm_error!("Out of memory");
mm_test_ok!("Test passed");
```

See `docs/memory-management-logging.md` for complete documentation.

### Debugging Tips

- ใช้ `serial stdio` ใน QEMU เพื่อดู debug output
- Memory management operations are logged with `[MM]` prefix
- ใช้ QEMU monitor สำหรับ low-level debugging
- ตรวจสอบ memory layout ด้วย `objdump -h kernel/target/x86_64-unknown-none/release/mellos-kernel`
- ตรวจสอบ page tables ด้วย QEMU monitor: `info mem`, `info tlb`
- รัน automated tests ด้วย `./tools/verify_build.sh` ก่อนทดสอบใน QEMU
- ดู memory statistics: allocated frames, free memory, heap usage

## Technical Details

### Memory Layout

```
Virtual Address Space:
0x0000_0000_0000_0000 - 0x0000_7FFF_FFFF_FFFF : User space (not used yet)
0xFFFF_8000_0000_0000 - 0xFFFF_9FFF_FFFF_FFFF : HHDM (direct physical mapping)
0xFFFF_A000_0000_0000 - 0xFFFF_A000_00FF_FFFF : Kernel heap (16MB)
0xFFFF_FFFF_8000_0000 - 0xFFFF_FFFF_FFFF_FFFF : Kernel code/data

Task Stacks:
- Each task has an 8KB stack allocated from kernel heap
- Stacks grow downward from high addresses
- Stack pointer (RSP) saved in Task Control Block during context switch
```

### Page Table Flags

- **.text section**: `PRESENT | GLOBAL` (Read + Execute)
- **.rodata section**: `PRESENT | NO_EXECUTE | GLOBAL` (Read only)
- **.data/.bss section**: `PRESENT | WRITABLE | NO_EXECUTE | GLOBAL` (Read + Write)
- **Heap pages**: `PRESENT | WRITABLE | NO_EXECUTE` (Read + Write)

### Buddy Allocator Orders

```
Order 0:  64 bytes   (2^6)
Order 1:  128 bytes  (2^7)
Order 2:  256 bytes  (2^8)
...
Order 14: 1 MB       (2^20)
```

### Interrupt Vector Mapping

```
CPU Exceptions:      0-31   (Reserved by CPU)
Timer (IRQ0):        32     (0x20) - PIT interrupt
Keyboard (IRQ1):     33     (0x21) - Not yet implemented
Other IRQs:          34-47  (0x22-0x2F) - Available for future use
```

### Context Switch Mechanism

1. **Timer Interrupt Fires** (every 10ms at 100 Hz)
2. **CPU Saves State** (RIP, CS, RFLAGS, RSP, SS automatically)
3. **Handler Sends EOI** to PIC (allows next interrupt)
4. **Scheduler Selects Next Task** (Round-Robin from runqueue)
5. **Context Switch**:
   - Save current task's callee-saved registers (RBX, RBP, R12-R15)
   - Save current RSP to current task's context
   - Load next task's RSP from next task's context
   - Restore next task's callee-saved registers
   - Return to next task (ret instruction)
6. **Next Task Resumes** from where it was interrupted

### Dependencies

The kernel uses the following Rust crates:

- **limine** (0.5): Bootloader protocol implementation
- **spin** (0.9): Spinlock for thread-safe synchronization
- **x86_64** (0.15): x86_64 architecture support

## Resources

### Documentation

- [Memory Management Spec](.kiro/specs/memory-management/) - Complete specification
- [MM Logging Guide](docs/memory-management-logging.md) - Logging utilities documentation
- [Task Scheduler Spec](.kiro/specs/task-scheduler/) - Scheduler design and implementation

### External Resources

- [Rust Embedded Book](https://rust-embedded.github.io/book/)
- [OSDev Wiki](https://wiki.osdev.org/)
- [Limine Bootloader](https://github.com/limine-bootloader/limine)
- [Writing an OS in Rust](https://os.phil-opp.com/)
- [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)

## License

This project is open source and available for educational purposes.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## Roadmap

### Completed ✅

- [x] Phase 1: Basic kernel boot with Limine
- [x] Phase 1: Framebuffer driver with 8x8 bitmap font
- [x] Phase 2: Physical Memory Manager (bitmap allocator)
- [x] Phase 2: 4-level paging system
- [x] Phase 2: Kernel heap allocator (Buddy System)
- [x] Phase 2: Memory protection (NX bit, write protection)
- [x] Phase 2: Guard pages for overflow protection
- [x] Phase 3: Task scheduler with Round-Robin algorithm
- [x] Phase 3: Timer interrupt handling (PIT, PIC, IDT)
- [x] Phase 3: Context switching mechanism
- [x] Phase 3: Preemptive multitasking
- [x] Automated testing and CI/CD

### In Progress 🚧

- [ ] Serial port driver infrastructure
- [ ] Keyboard driver and input handling
- [ ] Advanced interrupt handling (more ISRs)

### Planned 📋

- [ ] Priority-based scheduling
- [ ] Sleep/wake mechanisms
- [ ] System calls interface
- [ ] User space support
- [ ] Process management (fork, exec)
- [ ] Virtual File System (VFS)
- [ ] Device driver framework
- [ ] Multi-core support (SMP)
- [ ] Network stack

## Performance

Current performance characteristics:

**Memory Management:**
- **Frame Allocation**: O(n) worst case, O(1) average with last_alloc optimization
- **Heap Allocation**: O(log n) for buddy system operations
- **Page Mapping**: O(1) with existing page tables, O(4) when creating new tables
- **TLB Invalidation**: Single page invalidation with `invlpg`

**Task Scheduler:**
- **Context Switch**: < 1 microsecond (assembly-optimized)
- **Task Selection**: O(1) with circular queue
- **Timer Frequency**: 100 Hz (10ms time slices)
- **Scheduling Overhead**: ~1% CPU time at 100 Hz

## Acknowledgments

- Limine bootloader team for excellent UEFI bootloader
- Rust embedded community for tools and guidance
- OSDev community for comprehensive OS development resources
- Phil Opp for "Writing an OS in Rust" blog series
