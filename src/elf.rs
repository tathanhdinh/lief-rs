use libc;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SECTION_TYPES {
    SHT_NULL               = 0,
    SHT_PROGBITS           = 1,
    SHT_SYMTAB             = 2,
    SHT_STRTAB             = 3,
    SHT_RELA               = 4,
    SHT_HASH               = 5,
    SHT_DYNAMIC            = 6,
    SHT_NOTE               = 7,
    SHT_NOBITS             = 8,
    SHT_REL                = 9,
    SHT_SHLIB              = 10,
    SHT_DYNSYM             = 11,
    SHT_INIT_ARRAY         = 14,
    SHT_FINI_ARRAY         = 15,
    SHT_PREINIT_ARRAY      = 16,
    SHT_GROUP              = 17,
    SHT_SYMTAB_SHNDX       = 18,
    SHT_LOOS               = 1610612736,
    SHT_GNU_ATTRIBUTES     = 1879048181,
    SHT_GNU_HASH           = 1879048182,
    SHT_GNU_verdef         = 1879048189,
    SHT_GNU_verneed        = 1879048190,
    SHT_GNU_versym         = 1879048191,
    SHT_LOPROC             = 1879048192,
    SHT_ARM_EXIDX          = 1879048193,
    SHT_ARM_PREEMPTMAP     = 1879048194,
    SHT_ARM_ATTRIBUTES     = 1879048195,
    SHT_ARM_DEBUGOVERLAY   = 1879048196,
    SHT_ARM_OVERLAYSECTION = 1879048197,
    SHT_MIPS_REGINFO       = 1879048198,
    SHT_MIPS_OPTIONS       = 1879048205,
    SHT_MIPS_ABIFLAGS      = 1879048234,
    SHT_HIPROC             = 2147483647,
    SHT_LOUSER             = 2147483648,
    SHT_HIUSER             = 4294967295,
}

pub const SECTION_TYPES_SHT_HIOS: SECTION_TYPES = SECTION_TYPES::SHT_GNU_versym;
pub const SECTION_TYPES_SHT_HEX_ORDERED: SECTION_TYPES = SECTION_TYPES::SHT_LOPROC;
pub const SECTION_TYPES_SHT_X86_64_UNWIND: SECTION_TYPES = SECTION_TYPES::SHT_ARM_EXIDX;
