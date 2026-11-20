# This outlines the planned structure for this project
Releases on each milestone until first stable-release

## Architecture
We will use BusyBox as an end target with an init process as /init
The boot sequence looks like:
1. Firmware/Bootloader
  - Load the kernel
2. The Kernel
  - CPU, Memory, Interrupts, Scheduling
3. Kernel subsystems
  - VFS, syscalls, drivers, process manager
4. Initial ram filesystem
  - Has things like /init and BusyBox and libraries etc etc
5. PID 1: init
  - The first thing in user space that starts services and environments
6. BusyBox / Shell
  - BusyBox is the current goal but there are a lot of frameworks

## Milestone 0: Can print to VGA
We hit this already

## Milestone 1: Finish the kernel foundation
Handle:
- GDT/IDT
- Exceptions
- Interrupts
- Memory
- Dynamic kernel allocations
- Serial output (VGA is cool but serial is better)
Comprehensively hit this too!

## Milestone 2: Run the first user-mode program
Need a way to enter ring three (like iretq)
GDT entries for user-mode code and data
TSS has to be configured
Some sort of user-mode stack
Seperate address spaces for process
A way to return from userspace to kernelspace

## Milestone 3: Build system call interface
Stuff like Read/Write, Exit, getpid so on and so forth
The end goal is to have a Linux compatible syscall ABI. Initially it will be different.
POSIX compatibility 

## Milestone 4: Processes and Scheduling
A functional PCB (Process Control Block)
Scheduler that cares about some process state
Scheduler that can context switch, preempt, block, wake and then handle termination and cleanup.
Keep track of process relationships

Basically run 2 User space programs and swap between them.

## Milestone 5: Build the VFS
Virtual filesystem is a must.
End goal for VFS is to have directories, files, file descriptors, mounting, pipes, standard streams and permissions.
In-memory or ramfs + initramfs or some such 
A persistent filesystem (something simple)

## Milestone 6: ELF loading and initramfs
The ELF loader should be able to:
  - Parse ELF headers
  - Validate executable architecture.
  - Read program headers
  - Map PT_LOAD segments 
  - Apply correct page permissions. 
  - Create userspace stack 
  - Set up the initial process args and env 
  - Jump to executable entry point
The initramfs should function as a way to give the kernel an initial filesystem:
  - Goal to use newc CPIO as its straight forward

## Milestone 7: Get BusyBox or some other thing running
Use a statically linked BusyBox binary for x86_64 Linux using musl
Static linking temporarily avoids the need for a dynamic linker and shared-library loading immediately.

## Milestone 8: Polish towards usable operating system
At this point it is a progressive implementation to get towards Linux level functionality.
Milestone 8 will be only be completed once but will functionally run forever or until a new greater milestone comes to the edge of this programs development.
