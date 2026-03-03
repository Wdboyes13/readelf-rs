# Status on progress

## 32-bit support
So far there is no 32-bit support yet.

## 64-bit support
So far 64-bit support is the main goal.  
Completed parts are
- ELF Header
- Section headers  
In progress:
- Program headers

## System V ABI Conformance Notes
We aim for conformance to `System V Application Binary Interface - DRAFT - 24 April 2001`.  
This specification is technically not ratified yet, though most modern System V implementations use it.  
We do not support any processor or OS specific extensions currently. The only exception to this is the `EM_AARCH64` machine specificed by `ELF for the Arm® 64-bit Architecture (AArch64)` (07 April 2025).
