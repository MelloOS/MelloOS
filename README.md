# MelloOS

MelloOS is a modern operating system built from scratch in Rust, focusing on safety, performance, and extensibility. The project demonstrates advanced OS concepts including memory management, preemptive multitasking, and hardware interrupt handling.

## 🌟 Features

### Core System
- ✨ **Bare-metal kernel** written in Rust (`no_std`)
- 🚀 **UEFI boot** via Limine bootloader (v8.x)
- 🖥️ **Framebuffer driver** with 8x8 bitmap font rendering
- 📝 **Serial port** debugging output (COM1)
- 🔧 **Automated build system** with Makefile

### Memory Management System
- 🧠 **Physical Memory Manager (PMM)**
  - Bitmap-based frame allocator (4KB frames)
  - Automatic memory zeroing for security
  - Contiguous allocation support for DMA
  - Memory statistics tracking
  
- 📄 **Virtual Memory (Paging)**
  - 4-level page tables (PML4 → PDPT → PD → PT)
  - Per-section permissions (RX, R, RW+NX)
  - Guard pages for overflow protection
  - TLB invalidation support
  - HHDM (Higher Half Direct Mapping)
  
- 💾 **Kernel Heap Allocator**
  - Buddy System algorithm (64B to 1MB blocks)
  - Thread-safe with Mutex protection
  - `kmalloc()` and `kfree()` API
  - Automatic block splitting and merging
  - 16MB kernel heap

### Task Scheduler
- ⚡ **Preemptive Multitasking**
  - Round-Robin scheduling algorithm
  - Fair time-sharing (10ms time slices at 100 Hz)
  - O(1) task selection with circular queue
  - Maximum 64 concurrent tasks
  
- 🔄 **Context Switching**
  - Assembly-optimized (< 1 microsecond)
  - Full CPU context save/restore
  - Per-task 8KB stacks
  - System V ABI compliant
  
- ⏱️ **Timer Interrupt System**
  - PIT (Programmable Interval Timer) at 100 Hz
  - PIC (Programmable Interrupt Controller) remapping
  - IDT (Interrupt Descriptor Table) configuration
  - Automatic EOI handling

### Security Features
- 🔒 **NX Bit Support** - Non-executable pages prevent code execution in data regions
- 🛡️ **Write Protection** - Kernel respects page-level write permissions
- 🧹 **Memory Zeroing** - All allocated memory is zeroed before use
- 🚧 **Guard Pages** - Unmapped pages catch stack/heap overflow
- 🔐 **Stack Isolation** - Each task has its own isolated stack

## 📋 Prerequisites

Before you begin, you'll need to install the following dependencies:

### Required Tools

1. **Rust Toolchain** (latest stable)
   - Rust compiler and Cargo package manager
   - Target: `x86_64-unknown-none` for bare-metal development

2. **QEMU** (version 5.0+)
   - System emulator for x86_64 architecture
   - Used for testing and development

3. **xorriso**
   - ISO 9660 filesystem creation tool
   - Required for building bootable ISO images

4. **OVMF** (Optional but recommended)
   - UEFI firmware for QEMU
   - Enables UEFI boot testing

5. **Git**
   - Version control and for downloading Limine bootloader

## 🔧 Installation

### macOS

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Add bare-metal target
rustup target add x86_64-unknown-none

# Install development tools via Homebrew
brew install qemu xorriso git

# Install UEFI firmware (optional but recommended)
brew install --cask edk2-ovmf
```

### Linux (Ubuntu/Debian)

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Add bare-metal target
rustup target add x86_64-unknown-none

# Install development tools
sudo apt update
sudo apt install -y qemu-system-x86 xorriso ovmf git make
```

### Linux (Arch)

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Add bare-metal target
rustup target add x86_64-unknown-none

# Install development tools
sudo pacman -S qemu-full xorriso edk2-ovmf git make
```

### Verify Installation

```bash
# Check Rust installation
rustc --version
cargo --version

# Check QEMU installation
qemu-system-x86_64 --version

# Check xorriso installation
xorriso --version

# Verify bare-metal target
rustup target list | grep x86_64-unknown-none
```

## 🚀 Building and Running

### Quick Start

```bash
# Clone the repository
git clone <repository-url>
cd mellos

# Build and run in one command
make run
```

### Build Commands

#### 1. Build Kernel Binary

Compile the kernel to an ELF executable:

```bash
make build
```

**What it does:**
- Runs `cargo build --release` with the `x86_64-unknown-none` target
- Produces kernel binary at `kernel/target/x86_64-unknown-none/release/mellos-kernel`
- Applies linker script (`kernel/linker.ld`) for proper memory layout

#### 2. Create Bootable ISO

Build a bootable ISO image:

```bash
make iso
```

**What it does:**
- Builds the kernel (if not already built)
- Downloads Limine bootloader v8.x (if not present)
- Creates ISO directory structure in `iso_root/`
- Copies kernel binary and bootloader files
- Generates `mellos.iso` using xorriso
- Installs Limine bootloader to the ISO

#### 3. Run in QEMU

Launch the kernel in QEMU emulator:

```bash
make run
```

**What it does:**
- Creates ISO image (if not present)
- Starts QEMU with appropriate settings:
  - Machine type: Q35 (modern chipset)
  - Memory: 2GB RAM
  - Boot device: CD-ROM (ISO)
  - Serial output: stdio (for debugging)
  - UEFI firmware: OVMF (if available)

**To exit QEMU:**
- Press `Ctrl+C` in the terminal, or
- Close the QEMU window

#### 4. Clean Build Artifacts

Remove all generated files:

```bash
make clean
```

**What it removes:**
- Cargo build artifacts (`kernel/target/`)
- ISO image (`mellos.iso`)
- ISO root directory (`iso_root/`)
- Downloaded Limine bootloader (`limine/`)

### Boot Sequence

When you run `make run`, the following happens:

```
1. QEMU starts with UEFI firmware
   ↓
2. Limine bootloader loads from ISO
   ↓
3. Limine reads boot/limine.conf configuration
   ↓
4. Limine loads kernel.elf into memory
   ↓
5. Limine provides system information:
   - Memory map
   - Framebuffer details
   - HHDM offset
   - Kernel addresses
   ↓
6. Kernel _start() function begins execution
   ↓
7. Serial port initialization (COM1)
   ↓
8. Framebuffer initialization
   ↓
9. Memory Management initialization:
   - HHDM setup
   - CPU protection (NX bit, write protection)
   - Physical Memory Manager (PMM)
   - Paging system with kernel section mapping
   - Kernel heap allocator (16MB)
   - Memory management tests
   ↓
10. Task Scheduler initialization:
    - Idle task creation (ID 0)
    - IDT setup for interrupts
    - PIC remapping (IRQ 0-15 → vectors 32-47)
    - PIT configuration (100 Hz timer)
    - Demo task spawning (Task A, Task B)
    ↓
11. Interrupts enabled (sti instruction)
    ↓
12. Welcome message displayed: "Hello from MelloOS ✨"
    ↓
13. Multitasking begins:
    - Timer fires every 10ms
    - Tasks switch in Round-Robin order
    - Serial output shows context switches
    - Demo tasks print alternating output
```

### Expected Output

**On Screen (Framebuffer):**
```
Hello from MelloOS ✨
```

**On Serial Console:**
```
[KERNEL] MelloOS starting...
[KERNEL] Getting framebuffer response...
[KERNEL] Creating framebuffer instance...
[KERNEL] Clearing screen...
[KERNEL] Initializing memory management...
[MM] Initializing memory management...
[MM] Total memory: 2048 MB
[MM] Free memory: 2032 MB
[MM] ✓ PMM tests passed
[MM] ✓ Paging tests passed
[MM] ✓ Allocator tests passed
[KERNEL] Writing message to screen...
[KERNEL] Initializing scheduler...
[SCHED] INFO: Initializing scheduler...
[SCHED] INFO: Created idle task (id 0)
[SCHED] INFO: Scheduler initialized!
[KERNEL] Spawning demonstration tasks...
[SCHED] INFO: Spawned task 1: Task A
[SCHED] INFO: Spawned task 2: Task B
[KERNEL] Initializing timer interrupt...
[TIMER] Initializing timer interrupt system...
[TIMER] Setting up IDT...
[TIMER] IDT loaded successfully
[TIMER] Remapping PIC...
[TIMER] PIC remapped: Master=32-39, Slave=40-47
[TIMER] Configuring PIT for 100 Hz...
[TIMER] PIT configured with divisor 11931 (100 Hz)
[TIMER] Timer initialized at 100 Hz
[KERNEL] Enabling interrupts...
[KERNEL] Scheduler initialization complete!
[KERNEL] Boot complete! Entering idle loop...
[SCHED] First switch → Task 1 (Task A)
A
[SCHED] Switch #1 → Task 2 (Task B)
B
[SCHED] Switch #2 → Task 1 (Task A)
A
[SCHED] Switch #3 → Task 2 (Task B)
B
...
```

## 🏗️ Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     MelloOS Kernel                          │
│                                                             │
│  ┌───────────────┐  ┌──────────────┐  ┌─────────────────┐ │
│  │  Framebuffer  │  │    Serial    │  │   Panic Handler │ │
│  │    Driver     │  │     Port     │  │                 │ │
│  └───────────────┘  └──────────────┘  └─────────────────┘ │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │           Task Scheduler (sched/)                    │  │
│  │  - Round-Robin algorithm                             │  │
│  │  - Context switching (< 1μs)                         │  │
│  │  - Timer interrupts (100 Hz)                         │  │
│  │  - Task Control Blocks                               │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │        Memory Management (mm/)                       │  │
│  │  ┌────────────┐ ┌──────────┐ ┌──────────────────┐   │  │
│  │  │    PMM     │ │  Paging  │ │  Heap Allocator  │   │  │
│  │  │  (Bitmap)  │ │ (4-level)│ │ (Buddy System)   │   │  │
│  │  └────────────┘ └──────────┘ └──────────────────┘   │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                  Hardware Abstraction                       │
│  - x86_64 CPU (registers, instructions)                    │
│  - PIT (Programmable Interval Timer)                       │
│  - PIC (Programmable Interrupt Controller)                 │
│  - Serial Port (COM1)                                      │
│  - Framebuffer (UEFI GOP)                                  │
└─────────────────────────────────────────────────────────────┘
```

### Memory Management Architecture

#### 1. Physical Memory Manager (PMM)

**Location:** `kernel/src/mm/pmm.rs`

**Features:**
- Bitmap-based frame allocator (1 bit per 4KB frame)
- O(n) allocation with last_alloc optimization → O(1) average case
- Automatic memory zeroing for security
- Contiguous frame allocation for DMA devices
- Memory statistics (total/free memory in MB)

**API:**
```rust
pub fn alloc_frame() -> Option<PhysAddr>;
pub fn free_frame(phys_addr: PhysAddr);
pub fn alloc_contiguous(count: usize, align: usize) -> Option<PhysAddr>;
```

#### 2. Virtual Memory (Paging)

**Location:** `kernel/src/mm/paging.rs`

**Features:**
- 4-level page tables: PML4 → PDPT → PD → PT
- Per-section memory permissions:
  - `.text`: Read + Execute (RX)
  - `.rodata`: Read only (R)
  - `.data/.bss`: Read + Write + No Execute (RW+NX)
- Guard pages for overflow detection
- TLB invalidation with `invlpg` instruction
- Virtual address translation

**Page Table Flags:**
```rust
PRESENT     = 1 << 0   // Page is in memory
WRITABLE    = 1 << 1   // Page is writable
USER        = 1 << 2   // Accessible from user mode
NO_EXECUTE  = 1 << 63  // Page is not executable (NXE bit)
GLOBAL      = 1 << 8   // Not flushed from TLB
```

**API:**
```rust
pub fn map_page(virt: VirtAddr, phys: PhysAddr, flags: u64) -> Result<()>;
pub fn unmap_page(virt: VirtAddr) -> Result<()>;
pub fn translate(virt: VirtAddr) -> Option<PhysAddr>;
```

#### 3. Kernel Heap Allocator

**Location:** `kernel/src/mm/allocator.rs`

**Features:**
- Buddy System algorithm for efficient allocation
- Block sizes: 64B, 128B, 256B, ..., 1MB (15 orders)
- O(log n) allocation and deallocation
- Automatic block splitting and coalescing
- Thread-safe with `spin::Mutex`
- 16MB kernel heap at `0xFFFF_A000_0000_0000`

**API:**
```rust
pub fn kmalloc(size: usize) -> *mut u8;
pub fn kfree(ptr: *mut u8, size: usize);
pub fn allocated_bytes() -> usize;
```

### Task Scheduler Architecture

#### 1. Scheduler Core

**Location:** `kernel/src/sched/mod.rs`

**Data Structures:**
```rust
// Circular queue for O(1) operations
struct TaskQueue {
    tasks: [TaskId; MAX_TASKS],  // Ring buffer
    head: usize,
    tail: usize,
    count: usize,
}

// Scheduler state (single mutex for atomicity)
struct SchedState {
    runqueue: TaskQueue,         // Ready tasks
    current: Option<TaskId>,     // Currently running task
    next_tid: usize,             // Next task ID to assign
}

// Task table (heap-allocated tasks)
static TASK_TABLE: Mutex<[TaskPtr; MAX_TASKS]>;
```

**Round-Robin Algorithm:**
```
1. Timer interrupt fires (every 10ms)
2. Current task moved to back of runqueue
3. Next task popped from front of runqueue
4. Task states updated (Running → Ready, Ready → Running)
5. Context switch performed
6. Next task resumes execution
```

**API:**
```rust
pub fn init_scheduler();
pub fn spawn_task(name: &'static str, entry: fn() -> !) -> Result<TaskId>;
pub fn tick();  // Called by timer interrupt
```

#### 2. Context Switching

**Location:** `kernel/src/sched/context.rs`

**CPU Context Structure:**
```rust
#[repr(C)]
pub struct CpuContext {
    r15: u64,  // Callee-saved registers
    r14: u64,  // (System V ABI)
    r13: u64,
    r12: u64,
    rbp: u64,
    rbx: u64,
    rsp: u64,  // Stack pointer
}
```

**Context Switch Flow:**
```asm
context_switch:
    ; Save current task's registers
    push rbx, rbp, r12, r13, r14, r15
    mov [rdi + 48], rsp          ; Save RSP to current.rsp
    
    ; Load next task's registers
    mov rsp, [rsi + 48]          ; Load RSP from next.rsp
    pop r15, r14, r13, r12, rbp, rbx
    
    ; Return to next task
    ret                          ; Jump to return address on stack
```

**Performance:**
- Context switch time: < 1 microsecond
- Register save/restore: ~50 CPU cycles
- Total overhead: ~1% at 100 Hz

#### 3. Task Management

**Location:** `kernel/src/sched/task.rs`

**Task Control Block:**
```rust
pub struct Task {
    id: TaskId,              // Unique identifier
    name: &'static str,      // Human-readable name
    stack: *mut u8,          // Stack base address
    stack_size: usize,       // 8KB per task
    state: TaskState,        // Ready, Running, or Sleeping
    context: CpuContext,     // Saved CPU state
}
```

**Task States:**
```
     spawn()
        ↓
    ┌─────────┐
    │  Ready  │←─────┐
    └────┬────┘      │
         │           │
         │ schedule()│ preempt
         ▼           │
    ┌─────────┐     │
    │ Running │─────┘
    └─────────┘
```

**Stack Layout (8KB per task):**
```
High Address
┌─────────────────┐
│  entry_point    │ ← Pushed by Task::new
├─────────────────┤
│ entry_trampoline│ ← Return address
├─────────────────┤
│  R15 - RBX      │ ← Initial register values (zeros)
├─────────────────┤ ← Initial RSP
│                 │
│   Stack Space   │ 8KB (grows downward)
│                 │
└─────────────────┘
Low Address
```

#### 4. Timer Interrupt System

**Location:** `kernel/src/sched/timer.rs`

**Components:**

1. **PIT (Programmable Interval Timer)**
   - Base frequency: 1,193,182 Hz
   - Configured for 100 Hz (10ms intervals)
   - Mode 3: Square wave generator

2. **PIC (Programmable Interrupt Controller)**
   - Master PIC: IRQ 0-7 → Vectors 32-39
   - Slave PIC: IRQ 8-15 → Vectors 40-47
   - Timer (IRQ0) → Vector 32

3. **IDT (Interrupt Descriptor Table)**
   - 256 entries (0-255)
   - Vectors 0-31: CPU exceptions
   - Vector 32: Timer interrupt handler
   - Vectors 33-255: Available for future use

**Interrupt Flow:**
```
1. PIT fires interrupt (IRQ0)
   ↓
2. CPU automatically:
   - Disables interrupts (IF=0)
   - Saves SS, RSP, RFLAGS, CS, RIP to stack
   - Jumps to IDT entry 32
   ↓
3. Timer handler:
   - Sends EOI to PIC
   - Calls scheduler tick()
   ↓
4. Scheduler:
   - Selects next task (Round-Robin)
   - Performs context switch
   ↓
5. Next task resumes
   - Eventually executes iretq
   - Restores RIP, CS, RFLAGS, RSP, SS
   - Re-enables interrupts (IF=1)
```

### Security Architecture

#### Memory Protection

1. **NX Bit (No Execute)**
   - Enabled via EFER MSR (bit 11)
   - Data pages marked with NO_EXECUTE flag
   - Prevents code execution in data regions
   - Mitigates buffer overflow attacks

2. **Write Protection**
   - Enabled via CR0 register (bit 16)
   - Kernel respects page-level write permissions
   - Read-only pages cannot be written
   - Protects code and constant data

3. **Memory Zeroing**
   - All allocated frames zeroed before use
   - Prevents information leakage
   - Ensures clean state for new allocations

4. **Guard Pages**
   - Unmapped pages around stack and heap
   - Trigger page faults on overflow/underflow
   - Early detection of memory corruption

5. **Stack Isolation**
   - Each task has separate 8KB stack
   - Stacks allocated from kernel heap
   - No shared stack space between tasks
   - Prevents cross-task stack corruption

## 📁 Project Structure

```
mellos/
├── .cargo/
│   └── config.toml                  # Cargo build configuration
│
├── .github/
│   ├── workflows/
│   │   ├── build-and-release.yml    # Automated releases on tags
│   │   └── test-develop.yml         # CI/CD for develop branch
│   └── BRANCH_PROTECTION.md         # Branch protection setup guide
│
├── .kiro/
│   └── specs/                       # Design specifications
│       ├── memory-management/       # Memory management spec
│       │   ├── requirements.md      # MM requirements (EARS format)
│       │   ├── design.md            # MM architecture design
│       │   └── tasks.md             # MM implementation tasks
│       └── task-scheduler/          # Task scheduler spec
│           ├── requirements.md      # Scheduler requirements
│           ├── design.md            # Scheduler architecture
│           └── tasks.md             # Scheduler implementation tasks
│
├── kernel/                          # Kernel source code
│   ├── Cargo.toml                   # Dependencies: limine, spin, x86_64
│   ├── linker.ld                    # Linker script (memory layout)
│   ├── rust-toolchain.toml          # Rust toolchain specification
│   └── src/
│       ├── main.rs                  # Kernel entry point (_start)
│       ├── framebuffer.rs           # Framebuffer driver + 8x8 font
│       ├── serial.rs                # Serial port driver (COM1)
│       ├── panic.rs                 # Panic handler
│       │
│       ├── mm/                      # Memory Management subsystem
│       │   ├── mod.rs               # MM coordinator, HHDM, init
│       │   ├── pmm.rs               # Physical Memory Manager
│       │   ├── paging.rs            # Virtual memory (4-level paging)
│       │   ├── allocator.rs         # Kernel heap (Buddy System)
│       │   └── log.rs               # MM logging utilities
│       │
│       └── sched/                   # Task Scheduler subsystem
│           ├── mod.rs               # Scheduler core, Round-Robin
│           ├── task.rs              # Task Control Block (TCB)
│           ├── context.rs           # Context switching (assembly)
│           └── timer.rs             # Timer interrupts (PIT, PIC, IDT)
│
├── boot/
│   ├── limine.cfg                   # Limine bootloader config
│   └── limine.conf                  # Alternative config name
│
├── docs/                            # Documentation
│   ├── memory-management-logging.md # MM logging guide
│   └── task-scheduler.md            # Scheduler documentation
│
├── tools/                           # Development tools
│   ├── qemu.sh                      # QEMU launch script
│   ├── test_boot.sh                 # Boot testing script
│   └── verify_build.sh              # Build verification script
│
├── iso_root/                        # ISO build directory (generated)
│   ├── boot/
│   │   ├── kernel.elf               # Kernel binary
│   │   └── limine/                  # Bootloader files
│   └── EFI/BOOT/                    # UEFI boot files
│
├── limine/                          # Limine bootloader (downloaded)
│
├── Makefile                         # Build automation
├── README.md                        # This file
├── CHANGELOG.md                     # Version history
└── mellos.iso                       # Bootable ISO (generated)
```

### Key Directories

- **`kernel/src/`** - All kernel source code
  - **`mm/`** - Memory management (PMM, paging, heap allocator)
  - **`sched/`** - Task scheduler (Round-Robin, context switching, timer)
  
- **`.kiro/specs/`** - Design specifications and implementation plans
  - Requirements in EARS format
  - Architecture diagrams and design decisions
  - Task tracking with completion status

- **`docs/`** - User-facing documentation
  - API usage guides
  - Architecture explanations
  - Troubleshooting tips

- **`tools/`** - Development and testing scripts
  - QEMU automation
  - Build verification
  - Boot testing

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

## 🧪 Testing

### Automated Testing

#### Build Verification

Run automated build verification to ensure everything compiles correctly:

```bash
./tools/verify_build.sh
```

**What it checks:**
- ✅ Kernel binary exists and is valid ELF
- ✅ Required strings present in kernel
- ✅ ISO image created successfully
- ✅ Kernel present in ISO
- ✅ QEMU is available
- ✅ Limine bootloader files present
- ✅ Configuration files valid

**Expected output:**
```
✓ Kernel binary exists
✓ Kernel is valid ELF file
✓ ISO image exists
✓ Kernel found in ISO
✓ QEMU is available
✓ Limine files present
✓ All checks passed!
```

#### CI/CD Testing

GitHub Actions automatically runs tests on every push to `develop` branch:

```yaml
# .github/workflows/test-develop.yml
- Build kernel
- Create ISO
- Run build verification
- Test ISO bootability in QEMU
```

View test results at: `https://github.com/<your-repo>/actions`

### Manual Testing

#### Visual Testing

Test the kernel in QEMU with graphical output:

```bash
make run
```

**Expected behavior:**

1. **QEMU Window Opens**
   - Black screen initially
   - Limine bootloader menu (3 second timeout)

2. **Kernel Boots**
   - Screen clears to black
   - Serial output shows initialization messages

3. **Memory Management Initializes**
   - Serial: `[MM] Initializing memory management...`
   - Serial: `[MM] Total memory: 2048 MB`
   - Serial: `[MM] ✓ PMM tests passed`
   - Serial: `[MM] ✓ Paging tests passed`
   - Serial: `[MM] ✓ Allocator tests passed`

4. **Scheduler Initializes**
   - Serial: `[SCHED] INFO: Initializing scheduler...`
   - Serial: `[SCHED] INFO: Spawned task 1: Task A`
   - Serial: `[SCHED] INFO: Spawned task 2: Task B`
   - Serial: `[TIMER] Timer initialized at 100 Hz`

5. **Welcome Message Displays**
   - Screen: **"Hello from MelloOS ✨"** (white text, top-left)

6. **Multitasking Begins**
   - Serial: `[SCHED] First switch → Task 1 (Task A)`
   - Serial: `A` (from Task A)
   - Serial: `[SCHED] Switch #1 → Task 2 (Task B)`
   - Serial: `B` (from Task B)
   - Pattern repeats: A, B, A, B, ...

**To exit:** Press `Ctrl+C` or close QEMU window

#### Memory Management Tests

The kernel runs comprehensive memory tests automatically during boot:

**PMM (Physical Memory Manager) Tests:**
```
Test 1: Frame allocation returns valid address
Test 2: Multiple allocations return different frames
Test 3: Free and reallocation reuses frame
```

**Paging Tests:**
```
Test 1: Map page and translate address
Test 2: Unmap page and verify unmapped
```

**Allocator Tests:**
```
Test 1: kmalloc(1024) returns non-null pointer
Test 2: Memory write and read works
Test 3: kfree() completes without error
Test 4: Multiple allocations (10x 64 bytes)
Test 5: Multiple frees
```

**All tests must pass** for the kernel to continue booting.

#### Scheduler Tests

The kernel demonstrates multitasking with demo tasks:

**Task A:**
- Prints "A" to serial console
- Busy-waits for ~10ms
- Repeats forever

**Task B:**
- Prints "B" to serial console
- Busy-waits for ~10ms
- Repeats forever

**Expected output pattern:**
```
A
[SCHED] Switch #1 → Task 2 (Task B)
B
[SCHED] Switch #2 → Task 1 (Task A)
A
[SCHED] Switch #3 → Task 2 (Task B)
B
...
```

**Verification:**
- ✅ Tasks alternate (A, B, A, B pattern)
- ✅ Context switches logged every 10ms
- ✅ No crashes or hangs
- ✅ System remains stable for 100+ switches

### Performance Testing

#### Context Switch Performance

Measure context switch time:

```rust
// In scheduler code
let start = read_tsc();  // Read timestamp counter
context_switch(&mut old_ctx, &new_ctx);
let end = read_tsc();
let cycles = end - start;
```

**Expected:** < 150 CPU cycles (< 1 microsecond @ 3 GHz)

#### Scheduler Overhead

At 100 Hz (10ms time slices):
- 100 context switches per second
- ~0.05 μs per switch
- Total overhead: ~0.001% CPU time

**Measurement:**
```bash
# Run for 10 seconds and count switches
make run
# Wait 10 seconds
# Check serial output for switch count
# Expected: ~1000 switches in 10 seconds
```

#### Memory Allocation Performance

Test allocation speed:

```rust
let start = read_tsc();
for _ in 0..1000 {
    let ptr = kmalloc(64);
    kfree(ptr, 64);
}
let end = read_tsc();
let avg_cycles = (end - start) / 2000;  // 1000 alloc + 1000 free
```

**Expected:** < 500 cycles per allocation

### Stress Testing

#### Memory Stress Test

Allocate and free memory repeatedly:

```rust
fn memory_stress_test() -> ! {
    loop {
        // Allocate 100 blocks
        let mut ptrs = [core::ptr::null_mut(); 100];
        for i in 0..100 {
            ptrs[i] = kmalloc(1024);
        }
        
        // Free all blocks
        for i in 0..100 {
            kfree(ptrs[i], 1024);
        }
    }
}
```

**Expected:** No memory leaks, stable operation

#### Scheduler Stress Test

Spawn many tasks:

```rust
// Spawn 50 tasks
for i in 0..50 {
    spawn_task(&format!("task_{}", i), task_fn)
        .expect("Failed to spawn task");
}
```

**Expected:** All tasks run fairly, no starvation

#### Long-Running Test

Run the kernel for extended periods:

```bash
# Run for 1 hour
timeout 3600 make run
```

**Expected:**
- No crashes
- No memory leaks
- Consistent performance
- No task starvation

### Debugging Tests

#### Enable Verbose Logging

Modify logging to see more details:

```rust
// In mod.rs, always log context switches
sched_log!("Switch #{} → Task {} ({})", count, new_task.id, new_task.name);

// In pmm.rs, log all allocations
mm_log!("Allocated frame at 0x{:x}", frame_addr);
```

#### Test with Different Configurations

**Different timer frequencies:**
```rust
init_timer(10);    // 10 Hz - 100ms time slices
init_timer(100);   // 100 Hz - 10ms time slices (default)
init_timer(1000);  // 1000 Hz - 1ms time slices
```

**Different memory sizes:**
```bash
# 512MB RAM
qemu-system-x86_64 -m 512M -cdrom mellos.iso ...

# 4GB RAM
qemu-system-x86_64 -m 4G -cdrom mellos.iso ...
```

**Different CPU counts (for future SMP):**
```bash
qemu-system-x86_64 -smp 2 -cdrom mellos.iso ...
```

### Test Results

All tests should pass with the following results:

| Test Category | Status | Notes |
|--------------|--------|-------|
| Build Verification | ✅ PASS | All checks pass |
| Memory Management | ✅ PASS | All tests pass |
| Task Scheduler | ✅ PASS | Tasks alternate correctly |
| Context Switch | ✅ PASS | < 1 μs per switch |
| Long-Running | ✅ PASS | Stable for 1+ hours |
| Memory Stress | ✅ PASS | No leaks detected |
| Scheduler Stress | ✅ PASS | Fair scheduling maintained |

### Reporting Issues

If you encounter test failures:

1. **Capture serial output:**
   ```bash
   make run 2>&1 | tee test-output.log
   ```

2. **Check QEMU monitor:**
   ```bash
   qemu-system-x86_64 -monitor stdio -cdrom mellos.iso
   # In monitor: info registers, info mem
   ```

3. **Run build verification:**
   ```bash
   ./tools/verify_build.sh
   ```

4. **Create an issue with:**
   - Test output log
   - QEMU version
   - Host OS and version
   - Steps to reproduce
   - Expected vs actual behavior

## ✅ Current Capabilities

### What MelloOS Can Do

**Boot and Initialization**
- ✅ UEFI boot via Limine bootloader v8.x
- ✅ Framebuffer graphics initialization (any resolution)
- ✅ Serial port debugging (COM1 at 38400 baud)
- ✅ System information from bootloader (memory map, HHDM, kernel addresses)

**Memory Management**
- ✅ Physical memory tracking and allocation (4KB frames)
- ✅ Virtual memory with 4-level page tables
- ✅ Dynamic memory allocation (64B to 1MB blocks)
- ✅ Memory protection (NX bit, write protection)
- ✅ Guard pages for overflow detection
- ✅ Automatic memory zeroing for security
- ✅ Comprehensive memory management tests

**Task Scheduler**
- ✅ Preemptive multitasking (up to 64 tasks)
- ✅ Round-Robin scheduling algorithm
- ✅ Timer-based context switching (100 Hz, 10ms time slices)
- ✅ Per-task 8KB stacks
- ✅ Full CPU context save/restore
- ✅ Task spawning and management
- ✅ Interrupt handling (IDT, PIC, PIT)
- ✅ Context switch performance < 1 microsecond

**Display and Output**
- ✅ Pixel-level framebuffer access
- ✅ 8x8 bitmap font rendering (ASCII + emoji)
- ✅ String and character drawing
- ✅ Screen clearing with any color
- ✅ Serial console output for debugging

**Development and Testing**
- ✅ Automated build system (Makefile)
- ✅ QEMU testing environment
- ✅ CI/CD with GitHub Actions
- ✅ Build verification scripts
- ✅ Automated memory management tests
- ✅ End-to-end scheduler integration tests

## ⚠️ Current Limitations

### What MelloOS Cannot Do (Yet)

**Hardware Support**
- ❌ Keyboard input (no keyboard driver)
- ❌ Disk I/O (no storage drivers)
- ❌ Network stack (no network drivers)
- ❌ USB support
- ❌ Sound output

**System Features**
- ❌ User space (all code runs in kernel mode)
- ❌ System calls interface
- ❌ Process management (fork, exec)
- ❌ File system (no VFS or filesystem drivers)
- ❌ Multi-core support (SMP)
- ❌ Power management (ACPI)

**Scheduler Features**
- ❌ Priority-based scheduling
- ❌ Sleep/wake mechanisms
- ❌ Wait queues
- ❌ CPU affinity
- ❌ Real-time scheduling

**Memory Features**
- ❌ Demand paging (page faults)
- ❌ Swap space
- ❌ Memory-mapped files
- ❌ Shared memory between tasks
- ❌ Copy-on-write

**Security**
- ❌ User/kernel mode separation
- ❌ Privilege levels (ring 0/3)
- ❌ Address space isolation
- ❌ Sandboxing

### Why These Limitations Exist

MelloOS is currently in **Phase 3** of development, focusing on core kernel functionality:
- **Phase 1**: Boot and basic display ✅
- **Phase 2**: Memory management ✅
- **Phase 3**: Task scheduler ✅
- **Phase 4**: Advanced scheduling (planned)
- **Phase 5**: SMP support (planned)
- **Phase 6**: User space and system calls (planned)

The current implementation provides a solid foundation for future features. Each limitation represents a planned enhancement in the roadmap.

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

## 💻 Development Guide

### API Usage

#### Memory Management APIs

**Allocating Memory:**
```rust
use crate::mm::allocator::{kmalloc, kfree};

// Allocate 1KB of memory
let ptr = kmalloc(1024);

if !ptr.is_null() {
    // Memory is automatically zeroed
    unsafe {
        // Use the memory
        *ptr = 0x42;
        *(ptr.offset(1)) = 0x43;
    }
    
    // Free when done (must pass same size)
    kfree(ptr, 1024);
} else {
    // Out of memory - handle error
    serial_println!("Failed to allocate memory");
}
```

**Important Notes:**
- ✅ Always check if `kmalloc()` returns null
- ✅ Always call `kfree()` with the same size used in `kmalloc()`
- ✅ Memory is automatically zeroed for security
- ✅ All allocations are thread-safe (Mutex protected)
- ❌ Don't use after free
- ❌ Don't double free

**Physical Memory:**
```rust
use crate::mm::pmm::PhysicalMemoryManager;

// Allocate a 4KB physical frame
let frame = pmm.alloc_frame();

if let Some(phys_addr) = frame {
    // Use the frame
    serial_println!("Allocated frame at 0x{:x}", phys_addr);
    
    // Free when done
    pmm.free_frame(phys_addr);
}
```

**Virtual Memory:**
```rust
use crate::mm::paging::{PageMapper, PageTableFlags};

let mut mapper = PageMapper::new();

// Map a virtual page to a physical frame
let virt_addr = 0xFFFF_B000_0000_0000;
let phys_addr = pmm.alloc_frame().unwrap();

mapper.map_page(
    virt_addr,
    phys_addr,
    PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
    &mut pmm,
).expect("Failed to map page");

// Translate virtual to physical
if let Some(phys) = mapper.translate(virt_addr) {
    serial_println!("Virtual 0x{:x} → Physical 0x{:x}", virt_addr, phys);
}

// Unmap when done
mapper.unmap_page(virt_addr).expect("Failed to unmap");
```

#### Task Scheduler APIs

**Spawning Tasks:**
```rust
use crate::sched::spawn_task;

// Define a task function (must never return)
fn my_task() -> ! {
    loop {
        serial_println!("Task is running!");
        
        // Do some work
        for _ in 0..1_000_000 {
            unsafe { core::arch::asm!("nop"); }
        }
    }
}

// Spawn the task
match spawn_task("my_task", my_task) {
    Ok(task_id) => {
        serial_println!("Spawned task with ID: {}", task_id);
    }
    Err(e) => {
        serial_println!("Failed to spawn task: {:?}", e);
    }
}
```

**Task Requirements:**
- ✅ Must have signature `fn() -> !` (never returns)
- ✅ Must contain an infinite loop
- ✅ Can use up to 8KB of stack
- ✅ Can call `kmalloc`/`kfree` for dynamic memory
- ❌ Don't return from the function
- ❌ Don't use more than 8KB stack (no deep recursion)

**Initializing the Scheduler:**
```rust
use crate::sched::{init_scheduler, spawn_task};
use crate::sched::timer::init_timer;

// 1. Initialize scheduler (creates idle task)
init_scheduler();

// 2. Spawn your tasks
spawn_task("task_a", task_a).expect("Failed to spawn task_a");
spawn_task("task_b", task_b).expect("Failed to spawn task_b");

// 3. Initialize timer at 100 Hz
unsafe {
    init_timer(100);
}

// 4. Enable interrupts
unsafe {
    core::arch::asm!("sti");
}

// 5. Idle loop (scheduler will preempt this)
loop {
    unsafe { core::arch::asm!("hlt"); }
}
```

#### Logging APIs

**Memory Management Logging:**
```rust
use crate::{mm_log, mm_info, mm_error, mm_test_ok};

mm_log!("Initializing subsystem...");
mm_info!("Total memory: {} MB", total_mb);
mm_error!("Out of memory");
mm_test_ok!("Test passed");

// Format addresses in hexadecimal
let addr = 0x1000;
mm_log!("Allocated frame at 0x{:x}", addr);

// Format sizes with appropriate units
use crate::mm::log::format_size;
let (value, unit) = format_size(16 * 1024 * 1024);
mm_log!("Heap size: {} {}", value, unit);  // "Heap size: 16 MB"
```

**Scheduler Logging:**
```rust
use crate::{sched_log, sched_info, sched_warn, sched_error};

sched_log!("Context switch to task {}", task_id);
sched_info!("Spawned task: {}", name);
sched_warn!("Runqueue is empty");
sched_error!("Failed to allocate stack");
```

**Serial Output:**
```rust
use crate::{serial_print, serial_println};

serial_print!("Hello, ");
serial_println!("world!");
serial_println!("Value: {}", 42);
```

### Adding New Features

#### 1. Create a New Module

```rust
// kernel/src/mymodule.rs
pub fn my_function() {
    serial_println!("Hello from my module!");
}
```

```rust
// kernel/src/main.rs
mod mymodule;

fn _start() -> ! {
    // ...
    mymodule::my_function();
    // ...
}
```

#### 2. Add a New Task

```rust
fn my_new_task() -> ! {
    loop {
        // Your task logic here
        serial_println!("My task is running");
        
        // Yield CPU time
        for _ in 0..1_000_000 {
            unsafe { core::arch::asm!("nop"); }
        }
    }
}

// In main.rs
spawn_task("my_new_task", my_new_task).expect("Failed to spawn task");
```

#### 3. Add a New Interrupt Handler

```rust
// In timer.rs or new interrupt module
extern "C" fn my_interrupt_handler() {
    // Handle the interrupt
    serial_println!("Interrupt received!");
    
    // Send EOI if needed
    unsafe {
        send_eoi();
    }
}

// Register in IDT
unsafe {
    IDT.entries[33].set_handler(
        my_interrupt_handler as usize,
        code_selector
    );
}
```

### Debugging Tips

#### Serial Console Debugging

```rust
// Add debug output anywhere in the kernel
serial_println!("[DEBUG] Variable value: {}", value);
serial_println!("[DEBUG] Address: 0x{:x}", addr);
serial_println!("[DEBUG] Entering function: {}", function_name);
```

#### QEMU Monitor

Start QEMU with monitor access:
```bash
qemu-system-x86_64 -monitor stdio -cdrom mellos.iso ...
```

Useful monitor commands:
```
info registers    # Show CPU registers
info mem          # Show memory mappings
info tlb          # Show TLB entries
info pic          # Show PIC state
info irq          # Show interrupt statistics
x /10x 0x1000     # Examine memory at address
```

#### Memory Debugging

```rust
// Check memory statistics
let total_mb = pmm.total_memory_mb();
let free_mb = pmm.free_memory_mb();
serial_println!("Memory: {} MB total, {} MB free", total_mb, free_mb);

// Check heap usage
let allocated = allocator::allocated_bytes();
serial_println!("Heap allocated: {} bytes", allocated);

// Validate pointers
if ptr.is_null() {
    serial_println!("ERROR: Null pointer!");
}

// Check alignment
if addr % 4096 != 0 {
    serial_println!("ERROR: Address not page-aligned!");
}
```

#### Scheduler Debugging

```rust
// Check task count
let sched = SCHED.lock();
serial_println!("Runqueue length: {}", sched.runqueue.len());

// Check current task
if let Some(id) = sched.current {
    serial_println!("Current task: {}", id);
}

// Check context switch count
let switches = SWITCH_COUNT.load(Ordering::Relaxed);
serial_println!("Total context switches: {}", switches);
```

#### Build Verification

```bash
# Run automated build verification
./tools/verify_build.sh

# Check kernel binary
file kernel/target/x86_64-unknown-none/release/mellos-kernel

# Check ISO structure
xorriso -indev mellos.iso -find

# Disassemble kernel
objdump -d kernel/target/x86_64-unknown-none/release/mellos-kernel | less
```

### Common Issues and Solutions

#### Issue: Kernel Hangs After `sti`

**Cause:** IDT not properly initialized or timer not configured

**Solution:**
```rust
// Ensure proper initialization order
init_idt();           // First
remap_pic();          // Second
init_pit_timer(100);  // Third
core::arch::asm!("sti");  // Finally
```

#### Issue: Triple Fault / Reboot Loop

**Cause:** Stack overflow or invalid memory access

**Solution:**
```rust
// Add stack validation
if task.context.rsp == 0 {
    panic!("Task has null RSP!");
}

// Check stack bounds
let stack_bottom = task.stack as u64;
let stack_top = stack_bottom + 8192;
if task.context.rsp < stack_bottom || task.context.rsp >= stack_top {
    panic!("RSP outside stack bounds!");
}
```

#### Issue: Out of Memory

**Cause:** Too many allocations or memory leak

**Solution:**
```rust
// Check available memory
let free_mb = pmm.free_memory_mb();
if free_mb < 10 {
    serial_println!("WARNING: Low memory! {} MB free", free_mb);
}

// Always free allocated memory
let ptr = kmalloc(1024);
// ... use ptr ...
kfree(ptr, 1024);  // Don't forget!
```

#### Issue: Tasks Not Switching

**Cause:** Timer not firing or runqueue empty

**Solution:**
```rust
// Check timer ticks
let ticks = get_tick_count();
serial_println!("Timer ticks: {}", ticks);

// Check runqueue
let sched = SCHED.lock();
if sched.runqueue.is_empty() {
    serial_println!("WARNING: Runqueue is empty!");
}
```

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

## 🗺️ Roadmap

### Phase 1: Boot and Display ✅ COMPLETED

- [x] UEFI boot with Limine bootloader
- [x] Framebuffer driver with pixel access
- [x] 8x8 bitmap font rendering
- [x] Basic text output on screen
- [x] Serial port debugging output
- [x] Panic handler

**Status:** Completed in initial release

### Phase 2: Memory Management ✅ COMPLETED

- [x] Physical Memory Manager (PMM)
  - [x] Bitmap-based frame allocator
  - [x] Memory statistics tracking
  - [x] Contiguous allocation for DMA
- [x] Virtual Memory (Paging)
  - [x] 4-level page tables
  - [x] Per-section permissions
  - [x] Guard pages
  - [x] TLB invalidation
- [x] Kernel Heap Allocator
  - [x] Buddy System algorithm
  - [x] kmalloc/kfree API
  - [x] Thread-safe allocation
- [x] Security Features
  - [x] NX bit support
  - [x] Write protection
  - [x] Memory zeroing
- [x] Comprehensive testing
- [x] Documentation

**Status:** Completed with full test coverage

### Phase 3: Task Scheduler ✅ COMPLETED

- [x] Task Management
  - [x] Task Control Blocks (TCB)
  - [x] Task states (Ready, Running, Sleeping)
  - [x] Per-task 8KB stacks
- [x] Scheduler Core
  - [x] Round-Robin algorithm
  - [x] Circular runqueue (O(1) operations)
  - [x] Task spawning API
- [x] Context Switching
  - [x] Assembly-optimized switching
  - [x] Full register save/restore
  - [x] Entry trampoline for new tasks
- [x] Timer Interrupt System
  - [x] PIT configuration (100 Hz)
  - [x] PIC remapping
  - [x] IDT setup
  - [x] Interrupt handler
- [x] Integration and Testing
  - [x] End-to-end integration tests
  - [x] Performance benchmarks
  - [x] Documentation

**Status:** Completed with < 1μs context switch time

### Phase 4: Advanced Scheduling 🚧 IN PROGRESS

- [ ] Priority-Based Scheduling
  - [ ] Task priorities (high, normal, low)
  - [ ] Priority queues
  - [ ] Priority inheritance
- [ ] Sleep/Wake Mechanisms
  - [ ] `sleep_until(time)` API
  - [ ] `wake_task(id)` API
  - [ ] Wait queues
- [ ] Scheduler Improvements
  - [ ] CPU affinity hints
  - [ ] Load balancing preparation
  - [ ] Scheduler statistics

**Target:** Q2 2025

### Phase 5: SMP Support 📋 PLANNED

- [ ] Multi-Core Detection
  - [ ] ACPI parsing
  - [ ] CPU enumeration
  - [ ] APIC initialization
- [ ] Per-CPU Data Structures
  - [ ] Per-CPU runqueues
  - [ ] Per-CPU idle tasks
  - [ ] CPU-local storage
- [ ] Synchronization
  - [ ] Spinlocks with backoff
  - [ ] Read-write locks
  - [ ] Atomic operations
- [ ] Load Balancing
  - [ ] Work stealing
  - [ ] CPU migration
  - [ ] Affinity enforcement

**Target:** Q3 2025

### Phase 6: User Space 📋 PLANNED

- [ ] Privilege Separation
  - [ ] Ring 0/3 separation
  - [ ] User mode tasks
  - [ ] Kernel mode tasks
- [ ] System Calls
  - [ ] System call interface
  - [ ] Parameter validation
  - [ ] Error handling
- [ ] Process Management
  - [ ] Process creation (fork)
  - [ ] Program execution (exec)
  - [ ] Process termination
  - [ ] Process table
- [ ] Address Space Isolation
  - [ ] Per-process page tables
  - [ ] Address space switching
  - [ ] Copy-on-write

**Target:** Q4 2025

### Phase 7: Device Drivers 📋 PLANNED

- [ ] Driver Framework
  - [ ] Device abstraction
  - [ ] Driver registration
  - [ ] Interrupt routing
- [ ] Input Devices
  - [ ] PS/2 keyboard driver
  - [ ] PS/2 mouse driver
  - [ ] USB HID support
- [ ] Storage Devices
  - [ ] ATA/SATA driver
  - [ ] NVMe driver
  - [ ] Partition detection
- [ ] Network Devices
  - [ ] E1000 driver
  - [ ] Virtio-net driver
  - [ ] Network stack basics

**Target:** 2026

### Phase 8: File System 📋 PLANNED

- [ ] Virtual File System (VFS)
  - [ ] VFS abstraction layer
  - [ ] File operations
  - [ ] Directory operations
- [ ] File System Implementations
  - [ ] FAT32 (read/write)
  - [ ] ext2 (read-only)
  - [ ] Custom FS (future)
- [ ] File System Features
  - [ ] Mounting/unmounting
  - [ ] Path resolution
  - [ ] File caching

**Target:** 2026

### Future Enhancements 💡

- [ ] Advanced Memory Features
  - [ ] Demand paging
  - [ ] Swap space
  - [ ] Memory-mapped files
  - [ ] Huge pages (2MB/1GB)
- [ ] IPC (Inter-Process Communication)
  - [ ] Message passing
  - [ ] Shared memory
  - [ ] Pipes
  - [ ] Signals
- [ ] Network Stack
  - [ ] TCP/IP implementation
  - [ ] Socket API
  - [ ] Network protocols
- [ ] Graphics
  - [ ] GUI framework
  - [ ] Window manager
  - [ ] Graphics acceleration
- [ ] Security
  - [ ] Sandboxing
  - [ ] Capabilities
  - [ ] Secure boot
  - [ ] Encryption

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

## 🙏 Acknowledgments

### Projects and Communities

- **[Limine Bootloader](https://github.com/limine-bootloader/limine)** - Modern, feature-rich UEFI bootloader
- **[Rust Embedded Community](https://github.com/rust-embedded)** - Tools and guidance for embedded Rust
- **[OSDev Wiki](https://wiki.osdev.org/)** - Comprehensive OS development resources
- **[Phil Opp's Blog](https://os.phil-opp.com/)** - "Writing an OS in Rust" tutorial series

### Technical References

- **Intel 64 and IA-32 Architectures Software Developer's Manual** - CPU architecture reference
- **AMD64 Architecture Programmer's Manual** - x86_64 architecture details
- **System V AMD64 ABI** - Calling convention and binary interface
- **xv6 (MIT)** - Educational Unix-like OS for learning
- **Linux Kernel** - Production OS implementation reference

### Tools and Libraries

- **Rust Programming Language** - Safe systems programming
- **QEMU** - Fast and flexible emulator for testing
- **Cargo** - Rust package manager and build system
- **GitHub Actions** - CI/CD automation

### Special Thanks

- The Rust language team for creating a safe systems programming language
- The OSDev community for answering countless questions
- All contributors to open-source OS development resources

## 📄 License

This project is open source and available for educational purposes.

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Open issues for bugs or feature requests
- Submit pull requests with improvements
- Improve documentation
- Share your experience using MelloOS

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`make run` and verify output)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Run Clippy for lints (`cargo clippy`)
- Add comments for complex logic
- Update documentation for API changes
- Include tests for new features

## 📞 Contact

For questions, suggestions, or discussions about MelloOS:
- Open an issue on GitHub
- Check the documentation in `docs/`
- Review the specifications in `.kiro/specs/`

---

**MelloOS** - A modern operating system built from scratch in Rust 🦀✨
