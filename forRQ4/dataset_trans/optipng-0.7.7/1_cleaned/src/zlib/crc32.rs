use core::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_types::*;
pub type ptrdiff_t = isize;

pub type z_crc_t = c_uint;

static mut crc_table: [[z_crc_t; 256]; 8] = [
    [
        0 as c_ulong as z_crc_t,
        0x77073096 as c_ulong as z_crc_t,
        0xee0e612c as c_ulong as z_crc_t,
        0x990951ba as c_ulong as z_crc_t,
        0x76dc419 as c_ulong as z_crc_t,
        0x706af48f as c_ulong as z_crc_t,
        0xe963a535 as c_ulong as z_crc_t,
        0x9e6495a3 as c_ulong as z_crc_t,
        0xedb8832 as c_ulong as z_crc_t,
        0x79dcb8a4 as c_ulong as z_crc_t,
        0xe0d5e91e as c_ulong as z_crc_t,
        0x97d2d988 as c_ulong as z_crc_t,
        0x9b64c2b as c_ulong as z_crc_t,
        0x7eb17cbd as c_ulong as z_crc_t,
        0xe7b82d07 as c_ulong as z_crc_t,
        0x90bf1d91 as c_ulong as z_crc_t,
        0x1db71064 as c_ulong as z_crc_t,
        0x6ab020f2 as c_ulong as z_crc_t,
        0xf3b97148 as c_ulong as z_crc_t,
        0x84be41de as c_ulong as z_crc_t,
        0x1adad47d as c_ulong as z_crc_t,
        0x6ddde4eb as c_ulong as z_crc_t,
        0xf4d4b551 as c_ulong as z_crc_t,
        0x83d385c7 as c_ulong as z_crc_t,
        0x136c9856 as c_ulong as z_crc_t,
        0x646ba8c0 as c_ulong as z_crc_t,
        0xfd62f97a as c_ulong as z_crc_t,
        0x8a65c9ec as c_ulong as z_crc_t,
        0x14015c4f as c_ulong as z_crc_t,
        0x63066cd9 as c_ulong as z_crc_t,
        0xfa0f3d63 as c_ulong as z_crc_t,
        0x8d080df5 as c_ulong as z_crc_t,
        0x3b6e20c8 as c_ulong as z_crc_t,
        0x4c69105e as c_ulong as z_crc_t,
        0xd56041e4 as c_ulong as z_crc_t,
        0xa2677172 as c_ulong as z_crc_t,
        0x3c03e4d1 as c_ulong as z_crc_t,
        0x4b04d447 as c_ulong as z_crc_t,
        0xd20d85fd as c_ulong as z_crc_t,
        0xa50ab56b as c_ulong as z_crc_t,
        0x35b5a8fa as c_ulong as z_crc_t,
        0x42b2986c as c_ulong as z_crc_t,
        0xdbbbc9d6 as c_ulong as z_crc_t,
        0xacbcf940 as c_ulong as z_crc_t,
        0x32d86ce3 as c_ulong as z_crc_t,
        0x45df5c75 as c_ulong as z_crc_t,
        0xdcd60dcf as c_ulong as z_crc_t,
        0xabd13d59 as c_ulong as z_crc_t,
        0x26d930ac as c_ulong as z_crc_t,
        0x51de003a as c_ulong as z_crc_t,
        0xc8d75180 as c_ulong as z_crc_t,
        0xbfd06116 as c_ulong as z_crc_t,
        0x21b4f4b5 as c_ulong as z_crc_t,
        0x56b3c423 as c_ulong as z_crc_t,
        0xcfba9599 as c_ulong as z_crc_t,
        0xb8bda50f as c_ulong as z_crc_t,
        0x2802b89e as c_ulong as z_crc_t,
        0x5f058808 as c_ulong as z_crc_t,
        0xc60cd9b2 as c_ulong as z_crc_t,
        0xb10be924 as c_ulong as z_crc_t,
        0x2f6f7c87 as c_ulong as z_crc_t,
        0x58684c11 as c_ulong as z_crc_t,
        0xc1611dab as c_ulong as z_crc_t,
        0xb6662d3d as c_ulong as z_crc_t,
        0x76dc4190 as c_ulong as z_crc_t,
        0x1db7106 as c_ulong as z_crc_t,
        0x98d220bc as c_ulong as z_crc_t,
        0xefd5102a as c_ulong as z_crc_t,
        0x71b18589 as c_ulong as z_crc_t,
        0x6b6b51f as c_ulong as z_crc_t,
        0x9fbfe4a5 as c_ulong as z_crc_t,
        0xe8b8d433 as c_ulong as z_crc_t,
        0x7807c9a2 as c_ulong as z_crc_t,
        0xf00f934 as c_ulong as z_crc_t,
        0x9609a88e as c_ulong as z_crc_t,
        0xe10e9818 as c_ulong as z_crc_t,
        0x7f6a0dbb as c_ulong as z_crc_t,
        0x86d3d2d as c_ulong as z_crc_t,
        0x91646c97 as c_ulong as z_crc_t,
        0xe6635c01 as c_ulong as z_crc_t,
        0x6b6b51f4 as c_ulong as z_crc_t,
        0x1c6c6162 as c_ulong as z_crc_t,
        0x856530d8 as c_ulong as z_crc_t,
        0xf262004e as c_ulong as z_crc_t,
        0x6c0695ed as c_ulong as z_crc_t,
        0x1b01a57b as c_ulong as z_crc_t,
        0x8208f4c1 as c_ulong as z_crc_t,
        0xf50fc457 as c_ulong as z_crc_t,
        0x65b0d9c6 as c_ulong as z_crc_t,
        0x12b7e950 as c_ulong as z_crc_t,
        0x8bbeb8ea as c_ulong as z_crc_t,
        0xfcb9887c as c_ulong as z_crc_t,
        0x62dd1ddf as c_ulong as z_crc_t,
        0x15da2d49 as c_ulong as z_crc_t,
        0x8cd37cf3 as c_ulong as z_crc_t,
        0xfbd44c65 as c_ulong as z_crc_t,
        0x4db26158 as c_ulong as z_crc_t,
        0x3ab551ce as c_ulong as z_crc_t,
        0xa3bc0074 as c_ulong as z_crc_t,
        0xd4bb30e2 as c_ulong as z_crc_t,
        0x4adfa541 as c_ulong as z_crc_t,
        0x3dd895d7 as c_ulong as z_crc_t,
        0xa4d1c46d as c_ulong as z_crc_t,
        0xd3d6f4fb as c_ulong as z_crc_t,
        0x4369e96a as c_ulong as z_crc_t,
        0x346ed9fc as c_ulong as z_crc_t,
        0xad678846 as c_ulong as z_crc_t,
        0xda60b8d0 as c_ulong as z_crc_t,
        0x44042d73 as c_ulong as z_crc_t,
        0x33031de5 as c_ulong as z_crc_t,
        0xaa0a4c5f as c_ulong as z_crc_t,
        0xdd0d7cc9 as c_ulong as z_crc_t,
        0x5005713c as c_ulong as z_crc_t,
        0x270241aa as c_ulong as z_crc_t,
        0xbe0b1010 as c_ulong as z_crc_t,
        0xc90c2086 as c_ulong as z_crc_t,
        0x5768b525 as c_ulong as z_crc_t,
        0x206f85b3 as c_ulong as z_crc_t,
        0xb966d409 as c_ulong as z_crc_t,
        0xce61e49f as c_ulong as z_crc_t,
        0x5edef90e as c_ulong as z_crc_t,
        0x29d9c998 as c_ulong as z_crc_t,
        0xb0d09822 as c_ulong as z_crc_t,
        0xc7d7a8b4 as c_ulong as z_crc_t,
        0x59b33d17 as c_ulong as z_crc_t,
        0x2eb40d81 as c_ulong as z_crc_t,
        0xb7bd5c3b as c_ulong as z_crc_t,
        0xc0ba6cad as c_ulong as z_crc_t,
        0xedb88320 as c_ulong as z_crc_t,
        0x9abfb3b6 as c_ulong as z_crc_t,
        0x3b6e20c as c_ulong as z_crc_t,
        0x74b1d29a as c_ulong as z_crc_t,
        0xead54739 as c_ulong as z_crc_t,
        0x9dd277af as c_ulong as z_crc_t,
        0x4db2615 as c_ulong as z_crc_t,
        0x73dc1683 as c_ulong as z_crc_t,
        0xe3630b12 as c_ulong as z_crc_t,
        0x94643b84 as c_ulong as z_crc_t,
        0xd6d6a3e as c_ulong as z_crc_t,
        0x7a6a5aa8 as c_ulong as z_crc_t,
        0xe40ecf0b as c_ulong as z_crc_t,
        0x9309ff9d as c_ulong as z_crc_t,
        0xa00ae27 as c_ulong as z_crc_t,
        0x7d079eb1 as c_ulong as z_crc_t,
        0xf00f9344 as c_ulong as z_crc_t,
        0x8708a3d2 as c_ulong as z_crc_t,
        0x1e01f268 as c_ulong as z_crc_t,
        0x6906c2fe as c_ulong as z_crc_t,
        0xf762575d as c_ulong as z_crc_t,
        0x806567cb as c_ulong as z_crc_t,
        0x196c3671 as c_ulong as z_crc_t,
        0x6e6b06e7 as c_ulong as z_crc_t,
        0xfed41b76 as c_ulong as z_crc_t,
        0x89d32be0 as c_ulong as z_crc_t,
        0x10da7a5a as c_ulong as z_crc_t,
        0x67dd4acc as c_ulong as z_crc_t,
        0xf9b9df6f as c_ulong as z_crc_t,
        0x8ebeeff9 as c_ulong as z_crc_t,
        0x17b7be43 as c_ulong as z_crc_t,
        0x60b08ed5 as c_ulong as z_crc_t,
        0xd6d6a3e8 as c_ulong as z_crc_t,
        0xa1d1937e as c_ulong as z_crc_t,
        0x38d8c2c4 as c_ulong as z_crc_t,
        0x4fdff252 as c_ulong as z_crc_t,
        0xd1bb67f1 as c_ulong as z_crc_t,
        0xa6bc5767 as c_ulong as z_crc_t,
        0x3fb506dd as c_ulong as z_crc_t,
        0x48b2364b as c_ulong as z_crc_t,
        0xd80d2bda as c_ulong as z_crc_t,
        0xaf0a1b4c as c_ulong as z_crc_t,
        0x36034af6 as c_ulong as z_crc_t,
        0x41047a60 as c_ulong as z_crc_t,
        0xdf60efc3 as c_ulong as z_crc_t,
        0xa867df55 as c_ulong as z_crc_t,
        0x316e8eef as c_ulong as z_crc_t,
        0x4669be79 as c_ulong as z_crc_t,
        0xcb61b38c as c_ulong as z_crc_t,
        0xbc66831a as c_ulong as z_crc_t,
        0x256fd2a0 as c_ulong as z_crc_t,
        0x5268e236 as c_ulong as z_crc_t,
        0xcc0c7795 as c_ulong as z_crc_t,
        0xbb0b4703 as c_ulong as z_crc_t,
        0x220216b9 as c_ulong as z_crc_t,
        0x5505262f as c_ulong as z_crc_t,
        0xc5ba3bbe as c_ulong as z_crc_t,
        0xb2bd0b28 as c_ulong as z_crc_t,
        0x2bb45a92 as c_ulong as z_crc_t,
        0x5cb36a04 as c_ulong as z_crc_t,
        0xc2d7ffa7 as c_ulong as z_crc_t,
        0xb5d0cf31 as c_ulong as z_crc_t,
        0x2cd99e8b as c_ulong as z_crc_t,
        0x5bdeae1d as c_ulong as z_crc_t,
        0x9b64c2b0 as c_ulong as z_crc_t,
        0xec63f226 as c_ulong as z_crc_t,
        0x756aa39c as c_ulong as z_crc_t,
        0x26d930a as c_ulong as z_crc_t,
        0x9c0906a9 as c_ulong as z_crc_t,
        0xeb0e363f as c_ulong as z_crc_t,
        0x72076785 as c_ulong as z_crc_t,
        0x5005713 as c_ulong as z_crc_t,
        0x95bf4a82 as c_ulong as z_crc_t,
        0xe2b87a14 as c_ulong as z_crc_t,
        0x7bb12bae as c_ulong as z_crc_t,
        0xcb61b38 as c_ulong as z_crc_t,
        0x92d28e9b as c_ulong as z_crc_t,
        0xe5d5be0d as c_ulong as z_crc_t,
        0x7cdcefb7 as c_ulong as z_crc_t,
        0xbdbdf21 as c_ulong as z_crc_t,
        0x86d3d2d4 as c_ulong as z_crc_t,
        0xf1d4e242 as c_ulong as z_crc_t,
        0x68ddb3f8 as c_ulong as z_crc_t,
        0x1fda836e as c_ulong as z_crc_t,
        0x81be16cd as c_ulong as z_crc_t,
        0xf6b9265b as c_ulong as z_crc_t,
        0x6fb077e1 as c_ulong as z_crc_t,
        0x18b74777 as c_ulong as z_crc_t,
        0x88085ae6 as c_ulong as z_crc_t,
        0xff0f6a70 as c_ulong as z_crc_t,
        0x66063bca as c_ulong as z_crc_t,
        0x11010b5c as c_ulong as z_crc_t,
        0x8f659eff as c_ulong as z_crc_t,
        0xf862ae69 as c_ulong as z_crc_t,
        0x616bffd3 as c_ulong as z_crc_t,
        0x166ccf45 as c_ulong as z_crc_t,
        0xa00ae278 as c_ulong as z_crc_t,
        0xd70dd2ee as c_ulong as z_crc_t,
        0x4e048354 as c_ulong as z_crc_t,
        0x3903b3c2 as c_ulong as z_crc_t,
        0xa7672661 as c_ulong as z_crc_t,
        0xd06016f7 as c_ulong as z_crc_t,
        0x4969474d as c_ulong as z_crc_t,
        0x3e6e77db as c_ulong as z_crc_t,
        0xaed16a4a as c_ulong as z_crc_t,
        0xd9d65adc as c_ulong as z_crc_t,
        0x40df0b66 as c_ulong as z_crc_t,
        0x37d83bf0 as c_ulong as z_crc_t,
        0xa9bcae53 as c_ulong as z_crc_t,
        0xdebb9ec5 as c_ulong as z_crc_t,
        0x47b2cf7f as c_ulong as z_crc_t,
        0x30b5ffe9 as c_ulong as z_crc_t,
        0xbdbdf21c as c_ulong as z_crc_t,
        0xcabac28a as c_ulong as z_crc_t,
        0x53b39330 as c_ulong as z_crc_t,
        0x24b4a3a6 as c_ulong as z_crc_t,
        0xbad03605 as c_ulong as z_crc_t,
        0xcdd70693 as c_ulong as z_crc_t,
        0x54de5729 as c_ulong as z_crc_t,
        0x23d967bf as c_ulong as z_crc_t,
        0xb3667a2e as c_ulong as z_crc_t,
        0xc4614ab8 as c_ulong as z_crc_t,
        0x5d681b02 as c_ulong as z_crc_t,
        0x2a6f2b94 as c_ulong as z_crc_t,
        0xb40bbe37 as c_ulong as z_crc_t,
        0xc30c8ea1 as c_ulong as z_crc_t,
        0x5a05df1b as c_ulong as z_crc_t,
        0x2d02ef8d as c_ulong as z_crc_t,
    ],
    [
        0 as c_ulong as z_crc_t,
        0x191b3141 as c_ulong as z_crc_t,
        0x32366282 as c_ulong as z_crc_t,
        0x2b2d53c3 as c_ulong as z_crc_t,
        0x646cc504 as c_ulong as z_crc_t,
        0x7d77f445 as c_ulong as z_crc_t,
        0x565aa786 as c_ulong as z_crc_t,
        0x4f4196c7 as c_ulong as z_crc_t,
        0xc8d98a08 as c_ulong as z_crc_t,
        0xd1c2bb49 as c_ulong as z_crc_t,
        0xfaefe88a as c_ulong as z_crc_t,
        0xe3f4d9cb as c_ulong as z_crc_t,
        0xacb54f0c as c_ulong as z_crc_t,
        0xb5ae7e4d as c_ulong as z_crc_t,
        0x9e832d8e as c_ulong as z_crc_t,
        0x87981ccf as c_ulong as z_crc_t,
        0x4ac21251 as c_ulong as z_crc_t,
        0x53d92310 as c_ulong as z_crc_t,
        0x78f470d3 as c_ulong as z_crc_t,
        0x61ef4192 as c_ulong as z_crc_t,
        0x2eaed755 as c_ulong as z_crc_t,
        0x37b5e614 as c_ulong as z_crc_t,
        0x1c98b5d7 as c_ulong as z_crc_t,
        0x5838496 as c_ulong as z_crc_t,
        0x821b9859 as c_ulong as z_crc_t,
        0x9b00a918 as c_ulong as z_crc_t,
        0xb02dfadb as c_ulong as z_crc_t,
        0xa936cb9a as c_ulong as z_crc_t,
        0xe6775d5d as c_ulong as z_crc_t,
        0xff6c6c1c as c_ulong as z_crc_t,
        0xd4413fdf as c_ulong as z_crc_t,
        0xcd5a0e9e as c_ulong as z_crc_t,
        0x958424a2 as c_ulong as z_crc_t,
        0x8c9f15e3 as c_ulong as z_crc_t,
        0xa7b24620 as c_ulong as z_crc_t,
        0xbea97761 as c_ulong as z_crc_t,
        0xf1e8e1a6 as c_ulong as z_crc_t,
        0xe8f3d0e7 as c_ulong as z_crc_t,
        0xc3de8324 as c_ulong as z_crc_t,
        0xdac5b265 as c_ulong as z_crc_t,
        0x5d5daeaa as c_ulong as z_crc_t,
        0x44469feb as c_ulong as z_crc_t,
        0x6f6bcc28 as c_ulong as z_crc_t,
        0x7670fd69 as c_ulong as z_crc_t,
        0x39316bae as c_ulong as z_crc_t,
        0x202a5aef as c_ulong as z_crc_t,
        0xb07092c as c_ulong as z_crc_t,
        0x121c386d as c_ulong as z_crc_t,
        0xdf4636f3 as c_ulong as z_crc_t,
        0xc65d07b2 as c_ulong as z_crc_t,
        0xed705471 as c_ulong as z_crc_t,
        0xf46b6530 as c_ulong as z_crc_t,
        0xbb2af3f7 as c_ulong as z_crc_t,
        0xa231c2b6 as c_ulong as z_crc_t,
        0x891c9175 as c_ulong as z_crc_t,
        0x9007a034 as c_ulong as z_crc_t,
        0x179fbcfb as c_ulong as z_crc_t,
        0xe848dba as c_ulong as z_crc_t,
        0x25a9de79 as c_ulong as z_crc_t,
        0x3cb2ef38 as c_ulong as z_crc_t,
        0x73f379ff as c_ulong as z_crc_t,
        0x6ae848be as c_ulong as z_crc_t,
        0x41c51b7d as c_ulong as z_crc_t,
        0x58de2a3c as c_ulong as z_crc_t,
        0xf0794f05 as c_ulong as z_crc_t,
        0xe9627e44 as c_ulong as z_crc_t,
        0xc24f2d87 as c_ulong as z_crc_t,
        0xdb541cc6 as c_ulong as z_crc_t,
        0x94158a01 as c_ulong as z_crc_t,
        0x8d0ebb40 as c_ulong as z_crc_t,
        0xa623e883 as c_ulong as z_crc_t,
        0xbf38d9c2 as c_ulong as z_crc_t,
        0x38a0c50d as c_ulong as z_crc_t,
        0x21bbf44c as c_ulong as z_crc_t,
        0xa96a78f as c_ulong as z_crc_t,
        0x138d96ce as c_ulong as z_crc_t,
        0x5ccc0009 as c_ulong as z_crc_t,
        0x45d73148 as c_ulong as z_crc_t,
        0x6efa628b as c_ulong as z_crc_t,
        0x77e153ca as c_ulong as z_crc_t,
        0xbabb5d54 as c_ulong as z_crc_t,
        0xa3a06c15 as c_ulong as z_crc_t,
        0x888d3fd6 as c_ulong as z_crc_t,
        0x91960e97 as c_ulong as z_crc_t,
        0xded79850 as c_ulong as z_crc_t,
        0xc7cca911 as c_ulong as z_crc_t,
        0xece1fad2 as c_ulong as z_crc_t,
        0xf5facb93 as c_ulong as z_crc_t,
        0x7262d75c as c_ulong as z_crc_t,
        0x6b79e61d as c_ulong as z_crc_t,
        0x4054b5de as c_ulong as z_crc_t,
        0x594f849f as c_ulong as z_crc_t,
        0x160e1258 as c_ulong as z_crc_t,
        0xf152319 as c_ulong as z_crc_t,
        0x243870da as c_ulong as z_crc_t,
        0x3d23419b as c_ulong as z_crc_t,
        0x65fd6ba7 as c_ulong as z_crc_t,
        0x7ce65ae6 as c_ulong as z_crc_t,
        0x57cb0925 as c_ulong as z_crc_t,
        0x4ed03864 as c_ulong as z_crc_t,
        0x191aea3 as c_ulong as z_crc_t,
        0x188a9fe2 as c_ulong as z_crc_t,
        0x33a7cc21 as c_ulong as z_crc_t,
        0x2abcfd60 as c_ulong as z_crc_t,
        0xad24e1af as c_ulong as z_crc_t,
        0xb43fd0ee as c_ulong as z_crc_t,
        0x9f12832d as c_ulong as z_crc_t,
        0x8609b26c as c_ulong as z_crc_t,
        0xc94824ab as c_ulong as z_crc_t,
        0xd05315ea as c_ulong as z_crc_t,
        0xfb7e4629 as c_ulong as z_crc_t,
        0xe2657768 as c_ulong as z_crc_t,
        0x2f3f79f6 as c_ulong as z_crc_t,
        0x362448b7 as c_ulong as z_crc_t,
        0x1d091b74 as c_ulong as z_crc_t,
        0x4122a35 as c_ulong as z_crc_t,
        0x4b53bcf2 as c_ulong as z_crc_t,
        0x52488db3 as c_ulong as z_crc_t,
        0x7965de70 as c_ulong as z_crc_t,
        0x607eef31 as c_ulong as z_crc_t,
        0xe7e6f3fe as c_ulong as z_crc_t,
        0xfefdc2bf as c_ulong as z_crc_t,
        0xd5d0917c as c_ulong as z_crc_t,
        0xcccba03d as c_ulong as z_crc_t,
        0x838a36fa as c_ulong as z_crc_t,
        0x9a9107bb as c_ulong as z_crc_t,
        0xb1bc5478 as c_ulong as z_crc_t,
        0xa8a76539 as c_ulong as z_crc_t,
        0x3b83984b as c_ulong as z_crc_t,
        0x2298a90a as c_ulong as z_crc_t,
        0x9b5fac9 as c_ulong as z_crc_t,
        0x10aecb88 as c_ulong as z_crc_t,
        0x5fef5d4f as c_ulong as z_crc_t,
        0x46f46c0e as c_ulong as z_crc_t,
        0x6dd93fcd as c_ulong as z_crc_t,
        0x74c20e8c as c_ulong as z_crc_t,
        0xf35a1243 as c_ulong as z_crc_t,
        0xea412302 as c_ulong as z_crc_t,
        0xc16c70c1 as c_ulong as z_crc_t,
        0xd8774180 as c_ulong as z_crc_t,
        0x9736d747 as c_ulong as z_crc_t,
        0x8e2de606 as c_ulong as z_crc_t,
        0xa500b5c5 as c_ulong as z_crc_t,
        0xbc1b8484 as c_ulong as z_crc_t,
        0x71418a1a as c_ulong as z_crc_t,
        0x685abb5b as c_ulong as z_crc_t,
        0x4377e898 as c_ulong as z_crc_t,
        0x5a6cd9d9 as c_ulong as z_crc_t,
        0x152d4f1e as c_ulong as z_crc_t,
        0xc367e5f as c_ulong as z_crc_t,
        0x271b2d9c as c_ulong as z_crc_t,
        0x3e001cdd as c_ulong as z_crc_t,
        0xb9980012 as c_ulong as z_crc_t,
        0xa0833153 as c_ulong as z_crc_t,
        0x8bae6290 as c_ulong as z_crc_t,
        0x92b553d1 as c_ulong as z_crc_t,
        0xddf4c516 as c_ulong as z_crc_t,
        0xc4eff457 as c_ulong as z_crc_t,
        0xefc2a794 as c_ulong as z_crc_t,
        0xf6d996d5 as c_ulong as z_crc_t,
        0xae07bce9 as c_ulong as z_crc_t,
        0xb71c8da8 as c_ulong as z_crc_t,
        0x9c31de6b as c_ulong as z_crc_t,
        0x852aef2a as c_ulong as z_crc_t,
        0xca6b79ed as c_ulong as z_crc_t,
        0xd37048ac as c_ulong as z_crc_t,
        0xf85d1b6f as c_ulong as z_crc_t,
        0xe1462a2e as c_ulong as z_crc_t,
        0x66de36e1 as c_ulong as z_crc_t,
        0x7fc507a0 as c_ulong as z_crc_t,
        0x54e85463 as c_ulong as z_crc_t,
        0x4df36522 as c_ulong as z_crc_t,
        0x2b2f3e5 as c_ulong as z_crc_t,
        0x1ba9c2a4 as c_ulong as z_crc_t,
        0x30849167 as c_ulong as z_crc_t,
        0x299fa026 as c_ulong as z_crc_t,
        0xe4c5aeb8 as c_ulong as z_crc_t,
        0xfdde9ff9 as c_ulong as z_crc_t,
        0xd6f3cc3a as c_ulong as z_crc_t,
        0xcfe8fd7b as c_ulong as z_crc_t,
        0x80a96bbc as c_ulong as z_crc_t,
        0x99b25afd as c_ulong as z_crc_t,
        0xb29f093e as c_ulong as z_crc_t,
        0xab84387f as c_ulong as z_crc_t,
        0x2c1c24b0 as c_ulong as z_crc_t,
        0x350715f1 as c_ulong as z_crc_t,
        0x1e2a4632 as c_ulong as z_crc_t,
        0x7317773 as c_ulong as z_crc_t,
        0x4870e1b4 as c_ulong as z_crc_t,
        0x516bd0f5 as c_ulong as z_crc_t,
        0x7a468336 as c_ulong as z_crc_t,
        0x635db277 as c_ulong as z_crc_t,
        0xcbfad74e as c_ulong as z_crc_t,
        0xd2e1e60f as c_ulong as z_crc_t,
        0xf9ccb5cc as c_ulong as z_crc_t,
        0xe0d7848d as c_ulong as z_crc_t,
        0xaf96124a as c_ulong as z_crc_t,
        0xb68d230b as c_ulong as z_crc_t,
        0x9da070c8 as c_ulong as z_crc_t,
        0x84bb4189 as c_ulong as z_crc_t,
        0x3235d46 as c_ulong as z_crc_t,
        0x1a386c07 as c_ulong as z_crc_t,
        0x31153fc4 as c_ulong as z_crc_t,
        0x280e0e85 as c_ulong as z_crc_t,
        0x674f9842 as c_ulong as z_crc_t,
        0x7e54a903 as c_ulong as z_crc_t,
        0x5579fac0 as c_ulong as z_crc_t,
        0x4c62cb81 as c_ulong as z_crc_t,
        0x8138c51f as c_ulong as z_crc_t,
        0x9823f45e as c_ulong as z_crc_t,
        0xb30ea79d as c_ulong as z_crc_t,
        0xaa1596dc as c_ulong as z_crc_t,
        0xe554001b as c_ulong as z_crc_t,
        0xfc4f315a as c_ulong as z_crc_t,
        0xd7626299 as c_ulong as z_crc_t,
        0xce7953d8 as c_ulong as z_crc_t,
        0x49e14f17 as c_ulong as z_crc_t,
        0x50fa7e56 as c_ulong as z_crc_t,
        0x7bd72d95 as c_ulong as z_crc_t,
        0x62cc1cd4 as c_ulong as z_crc_t,
        0x2d8d8a13 as c_ulong as z_crc_t,
        0x3496bb52 as c_ulong as z_crc_t,
        0x1fbbe891 as c_ulong as z_crc_t,
        0x6a0d9d0 as c_ulong as z_crc_t,
        0x5e7ef3ec as c_ulong as z_crc_t,
        0x4765c2ad as c_ulong as z_crc_t,
        0x6c48916e as c_ulong as z_crc_t,
        0x7553a02f as c_ulong as z_crc_t,
        0x3a1236e8 as c_ulong as z_crc_t,
        0x230907a9 as c_ulong as z_crc_t,
        0x824546a as c_ulong as z_crc_t,
        0x113f652b as c_ulong as z_crc_t,
        0x96a779e4 as c_ulong as z_crc_t,
        0x8fbc48a5 as c_ulong as z_crc_t,
        0xa4911b66 as c_ulong as z_crc_t,
        0xbd8a2a27 as c_ulong as z_crc_t,
        0xf2cbbce0 as c_ulong as z_crc_t,
        0xebd08da1 as c_ulong as z_crc_t,
        0xc0fdde62 as c_ulong as z_crc_t,
        0xd9e6ef23 as c_ulong as z_crc_t,
        0x14bce1bd as c_ulong as z_crc_t,
        0xda7d0fc as c_ulong as z_crc_t,
        0x268a833f as c_ulong as z_crc_t,
        0x3f91b27e as c_ulong as z_crc_t,
        0x70d024b9 as c_ulong as z_crc_t,
        0x69cb15f8 as c_ulong as z_crc_t,
        0x42e6463b as c_ulong as z_crc_t,
        0x5bfd777a as c_ulong as z_crc_t,
        0xdc656bb5 as c_ulong as z_crc_t,
        0xc57e5af4 as c_ulong as z_crc_t,
        0xee530937 as c_ulong as z_crc_t,
        0xf7483876 as c_ulong as z_crc_t,
        0xb809aeb1 as c_ulong as z_crc_t,
        0xa1129ff0 as c_ulong as z_crc_t,
        0x8a3fcc33 as c_ulong as z_crc_t,
        0x9324fd72 as c_ulong as z_crc_t,
    ],
    [
        0 as c_ulong as z_crc_t,
        0x1c26a37 as c_ulong as z_crc_t,
        0x384d46e as c_ulong as z_crc_t,
        0x246be59 as c_ulong as z_crc_t,
        0x709a8dc as c_ulong as z_crc_t,
        0x6cbc2eb as c_ulong as z_crc_t,
        0x48d7cb2 as c_ulong as z_crc_t,
        0x54f1685 as c_ulong as z_crc_t,
        0xe1351b8 as c_ulong as z_crc_t,
        0xfd13b8f as c_ulong as z_crc_t,
        0xd9785d6 as c_ulong as z_crc_t,
        0xc55efe1 as c_ulong as z_crc_t,
        0x91af964 as c_ulong as z_crc_t,
        0x8d89353 as c_ulong as z_crc_t,
        0xa9e2d0a as c_ulong as z_crc_t,
        0xb5c473d as c_ulong as z_crc_t,
        0x1c26a370 as c_ulong as z_crc_t,
        0x1de4c947 as c_ulong as z_crc_t,
        0x1fa2771e as c_ulong as z_crc_t,
        0x1e601d29 as c_ulong as z_crc_t,
        0x1b2f0bac as c_ulong as z_crc_t,
        0x1aed619b as c_ulong as z_crc_t,
        0x18abdfc2 as c_ulong as z_crc_t,
        0x1969b5f5 as c_ulong as z_crc_t,
        0x1235f2c8 as c_ulong as z_crc_t,
        0x13f798ff as c_ulong as z_crc_t,
        0x11b126a6 as c_ulong as z_crc_t,
        0x10734c91 as c_ulong as z_crc_t,
        0x153c5a14 as c_ulong as z_crc_t,
        0x14fe3023 as c_ulong as z_crc_t,
        0x16b88e7a as c_ulong as z_crc_t,
        0x177ae44d as c_ulong as z_crc_t,
        0x384d46e0 as c_ulong as z_crc_t,
        0x398f2cd7 as c_ulong as z_crc_t,
        0x3bc9928e as c_ulong as z_crc_t,
        0x3a0bf8b9 as c_ulong as z_crc_t,
        0x3f44ee3c as c_ulong as z_crc_t,
        0x3e86840b as c_ulong as z_crc_t,
        0x3cc03a52 as c_ulong as z_crc_t,
        0x3d025065 as c_ulong as z_crc_t,
        0x365e1758 as c_ulong as z_crc_t,
        0x379c7d6f as c_ulong as z_crc_t,
        0x35dac336 as c_ulong as z_crc_t,
        0x3418a901 as c_ulong as z_crc_t,
        0x3157bf84 as c_ulong as z_crc_t,
        0x3095d5b3 as c_ulong as z_crc_t,
        0x32d36bea as c_ulong as z_crc_t,
        0x331101dd as c_ulong as z_crc_t,
        0x246be590 as c_ulong as z_crc_t,
        0x25a98fa7 as c_ulong as z_crc_t,
        0x27ef31fe as c_ulong as z_crc_t,
        0x262d5bc9 as c_ulong as z_crc_t,
        0x23624d4c as c_ulong as z_crc_t,
        0x22a0277b as c_ulong as z_crc_t,
        0x20e69922 as c_ulong as z_crc_t,
        0x2124f315 as c_ulong as z_crc_t,
        0x2a78b428 as c_ulong as z_crc_t,
        0x2bbade1f as c_ulong as z_crc_t,
        0x29fc6046 as c_ulong as z_crc_t,
        0x283e0a71 as c_ulong as z_crc_t,
        0x2d711cf4 as c_ulong as z_crc_t,
        0x2cb376c3 as c_ulong as z_crc_t,
        0x2ef5c89a as c_ulong as z_crc_t,
        0x2f37a2ad as c_ulong as z_crc_t,
        0x709a8dc0 as c_ulong as z_crc_t,
        0x7158e7f7 as c_ulong as z_crc_t,
        0x731e59ae as c_ulong as z_crc_t,
        0x72dc3399 as c_ulong as z_crc_t,
        0x7793251c as c_ulong as z_crc_t,
        0x76514f2b as c_ulong as z_crc_t,
        0x7417f172 as c_ulong as z_crc_t,
        0x75d59b45 as c_ulong as z_crc_t,
        0x7e89dc78 as c_ulong as z_crc_t,
        0x7f4bb64f as c_ulong as z_crc_t,
        0x7d0d0816 as c_ulong as z_crc_t,
        0x7ccf6221 as c_ulong as z_crc_t,
        0x798074a4 as c_ulong as z_crc_t,
        0x78421e93 as c_ulong as z_crc_t,
        0x7a04a0ca as c_ulong as z_crc_t,
        0x7bc6cafd as c_ulong as z_crc_t,
        0x6cbc2eb0 as c_ulong as z_crc_t,
        0x6d7e4487 as c_ulong as z_crc_t,
        0x6f38fade as c_ulong as z_crc_t,
        0x6efa90e9 as c_ulong as z_crc_t,
        0x6bb5866c as c_ulong as z_crc_t,
        0x6a77ec5b as c_ulong as z_crc_t,
        0x68315202 as c_ulong as z_crc_t,
        0x69f33835 as c_ulong as z_crc_t,
        0x62af7f08 as c_ulong as z_crc_t,
        0x636d153f as c_ulong as z_crc_t,
        0x612bab66 as c_ulong as z_crc_t,
        0x60e9c151 as c_ulong as z_crc_t,
        0x65a6d7d4 as c_ulong as z_crc_t,
        0x6464bde3 as c_ulong as z_crc_t,
        0x662203ba as c_ulong as z_crc_t,
        0x67e0698d as c_ulong as z_crc_t,
        0x48d7cb20 as c_ulong as z_crc_t,
        0x4915a117 as c_ulong as z_crc_t,
        0x4b531f4e as c_ulong as z_crc_t,
        0x4a917579 as c_ulong as z_crc_t,
        0x4fde63fc as c_ulong as z_crc_t,
        0x4e1c09cb as c_ulong as z_crc_t,
        0x4c5ab792 as c_ulong as z_crc_t,
        0x4d98dda5 as c_ulong as z_crc_t,
        0x46c49a98 as c_ulong as z_crc_t,
        0x4706f0af as c_ulong as z_crc_t,
        0x45404ef6 as c_ulong as z_crc_t,
        0x448224c1 as c_ulong as z_crc_t,
        0x41cd3244 as c_ulong as z_crc_t,
        0x400f5873 as c_ulong as z_crc_t,
        0x4249e62a as c_ulong as z_crc_t,
        0x438b8c1d as c_ulong as z_crc_t,
        0x54f16850 as c_ulong as z_crc_t,
        0x55330267 as c_ulong as z_crc_t,
        0x5775bc3e as c_ulong as z_crc_t,
        0x56b7d609 as c_ulong as z_crc_t,
        0x53f8c08c as c_ulong as z_crc_t,
        0x523aaabb as c_ulong as z_crc_t,
        0x507c14e2 as c_ulong as z_crc_t,
        0x51be7ed5 as c_ulong as z_crc_t,
        0x5ae239e8 as c_ulong as z_crc_t,
        0x5b2053df as c_ulong as z_crc_t,
        0x5966ed86 as c_ulong as z_crc_t,
        0x58a487b1 as c_ulong as z_crc_t,
        0x5deb9134 as c_ulong as z_crc_t,
        0x5c29fb03 as c_ulong as z_crc_t,
        0x5e6f455a as c_ulong as z_crc_t,
        0x5fad2f6d as c_ulong as z_crc_t,
        0xe1351b80 as c_ulong as z_crc_t,
        0xe0f771b7 as c_ulong as z_crc_t,
        0xe2b1cfee as c_ulong as z_crc_t,
        0xe373a5d9 as c_ulong as z_crc_t,
        0xe63cb35c as c_ulong as z_crc_t,
        0xe7fed96b as c_ulong as z_crc_t,
        0xe5b86732 as c_ulong as z_crc_t,
        0xe47a0d05 as c_ulong as z_crc_t,
        0xef264a38 as c_ulong as z_crc_t,
        0xeee4200f as c_ulong as z_crc_t,
        0xeca29e56 as c_ulong as z_crc_t,
        0xed60f461 as c_ulong as z_crc_t,
        0xe82fe2e4 as c_ulong as z_crc_t,
        0xe9ed88d3 as c_ulong as z_crc_t,
        0xebab368a as c_ulong as z_crc_t,
        0xea695cbd as c_ulong as z_crc_t,
        0xfd13b8f0 as c_ulong as z_crc_t,
        0xfcd1d2c7 as c_ulong as z_crc_t,
        0xfe976c9e as c_ulong as z_crc_t,
        0xff5506a9 as c_ulong as z_crc_t,
        0xfa1a102c as c_ulong as z_crc_t,
        0xfbd87a1b as c_ulong as z_crc_t,
        0xf99ec442 as c_ulong as z_crc_t,
        0xf85cae75 as c_ulong as z_crc_t,
        0xf300e948 as c_ulong as z_crc_t,
        0xf2c2837f as c_ulong as z_crc_t,
        0xf0843d26 as c_ulong as z_crc_t,
        0xf1465711 as c_ulong as z_crc_t,
        0xf4094194 as c_ulong as z_crc_t,
        0xf5cb2ba3 as c_ulong as z_crc_t,
        0xf78d95fa as c_ulong as z_crc_t,
        0xf64fffcd as c_ulong as z_crc_t,
        0xd9785d60 as c_ulong as z_crc_t,
        0xd8ba3757 as c_ulong as z_crc_t,
        0xdafc890e as c_ulong as z_crc_t,
        0xdb3ee339 as c_ulong as z_crc_t,
        0xde71f5bc as c_ulong as z_crc_t,
        0xdfb39f8b as c_ulong as z_crc_t,
        0xddf521d2 as c_ulong as z_crc_t,
        0xdc374be5 as c_ulong as z_crc_t,
        0xd76b0cd8 as c_ulong as z_crc_t,
        0xd6a966ef as c_ulong as z_crc_t,
        0xd4efd8b6 as c_ulong as z_crc_t,
        0xd52db281 as c_ulong as z_crc_t,
        0xd062a404 as c_ulong as z_crc_t,
        0xd1a0ce33 as c_ulong as z_crc_t,
        0xd3e6706a as c_ulong as z_crc_t,
        0xd2241a5d as c_ulong as z_crc_t,
        0xc55efe10 as c_ulong as z_crc_t,
        0xc49c9427 as c_ulong as z_crc_t,
        0xc6da2a7e as c_ulong as z_crc_t,
        0xc7184049 as c_ulong as z_crc_t,
        0xc25756cc as c_ulong as z_crc_t,
        0xc3953cfb as c_ulong as z_crc_t,
        0xc1d382a2 as c_ulong as z_crc_t,
        0xc011e895 as c_ulong as z_crc_t,
        0xcb4dafa8 as c_ulong as z_crc_t,
        0xca8fc59f as c_ulong as z_crc_t,
        0xc8c97bc6 as c_ulong as z_crc_t,
        0xc90b11f1 as c_ulong as z_crc_t,
        0xcc440774 as c_ulong as z_crc_t,
        0xcd866d43 as c_ulong as z_crc_t,
        0xcfc0d31a as c_ulong as z_crc_t,
        0xce02b92d as c_ulong as z_crc_t,
        0x91af9640 as c_ulong as z_crc_t,
        0x906dfc77 as c_ulong as z_crc_t,
        0x922b422e as c_ulong as z_crc_t,
        0x93e92819 as c_ulong as z_crc_t,
        0x96a63e9c as c_ulong as z_crc_t,
        0x976454ab as c_ulong as z_crc_t,
        0x9522eaf2 as c_ulong as z_crc_t,
        0x94e080c5 as c_ulong as z_crc_t,
        0x9fbcc7f8 as c_ulong as z_crc_t,
        0x9e7eadcf as c_ulong as z_crc_t,
        0x9c381396 as c_ulong as z_crc_t,
        0x9dfa79a1 as c_ulong as z_crc_t,
        0x98b56f24 as c_ulong as z_crc_t,
        0x99770513 as c_ulong as z_crc_t,
        0x9b31bb4a as c_ulong as z_crc_t,
        0x9af3d17d as c_ulong as z_crc_t,
        0x8d893530 as c_ulong as z_crc_t,
        0x8c4b5f07 as c_ulong as z_crc_t,
        0x8e0de15e as c_ulong as z_crc_t,
        0x8fcf8b69 as c_ulong as z_crc_t,
        0x8a809dec as c_ulong as z_crc_t,
        0x8b42f7db as c_ulong as z_crc_t,
        0x89044982 as c_ulong as z_crc_t,
        0x88c623b5 as c_ulong as z_crc_t,
        0x839a6488 as c_ulong as z_crc_t,
        0x82580ebf as c_ulong as z_crc_t,
        0x801eb0e6 as c_ulong as z_crc_t,
        0x81dcdad1 as c_ulong as z_crc_t,
        0x8493cc54 as c_ulong as z_crc_t,
        0x8551a663 as c_ulong as z_crc_t,
        0x8717183a as c_ulong as z_crc_t,
        0x86d5720d as c_ulong as z_crc_t,
        0xa9e2d0a0 as c_ulong as z_crc_t,
        0xa820ba97 as c_ulong as z_crc_t,
        0xaa6604ce as c_ulong as z_crc_t,
        0xaba46ef9 as c_ulong as z_crc_t,
        0xaeeb787c as c_ulong as z_crc_t,
        0xaf29124b as c_ulong as z_crc_t,
        0xad6fac12 as c_ulong as z_crc_t,
        0xacadc625 as c_ulong as z_crc_t,
        0xa7f18118 as c_ulong as z_crc_t,
        0xa633eb2f as c_ulong as z_crc_t,
        0xa4755576 as c_ulong as z_crc_t,
        0xa5b73f41 as c_ulong as z_crc_t,
        0xa0f829c4 as c_ulong as z_crc_t,
        0xa13a43f3 as c_ulong as z_crc_t,
        0xa37cfdaa as c_ulong as z_crc_t,
        0xa2be979d as c_ulong as z_crc_t,
        0xb5c473d0 as c_ulong as z_crc_t,
        0xb40619e7 as c_ulong as z_crc_t,
        0xb640a7be as c_ulong as z_crc_t,
        0xb782cd89 as c_ulong as z_crc_t,
        0xb2cddb0c as c_ulong as z_crc_t,
        0xb30fb13b as c_ulong as z_crc_t,
        0xb1490f62 as c_ulong as z_crc_t,
        0xb08b6555 as c_ulong as z_crc_t,
        0xbbd72268 as c_ulong as z_crc_t,
        0xba15485f as c_ulong as z_crc_t,
        0xb853f606 as c_ulong as z_crc_t,
        0xb9919c31 as c_ulong as z_crc_t,
        0xbcde8ab4 as c_ulong as z_crc_t,
        0xbd1ce083 as c_ulong as z_crc_t,
        0xbf5a5eda as c_ulong as z_crc_t,
        0xbe9834ed as c_ulong as z_crc_t,
    ],
    [
        0 as c_ulong as z_crc_t,
        0xb8bc6765 as c_ulong as z_crc_t,
        0xaa09c88b as c_ulong as z_crc_t,
        0x12b5afee as c_ulong as z_crc_t,
        0x8f629757 as c_ulong as z_crc_t,
        0x37def032 as c_ulong as z_crc_t,
        0x256b5fdc as c_ulong as z_crc_t,
        0x9dd738b9 as c_ulong as z_crc_t,
        0xc5b428ef as c_ulong as z_crc_t,
        0x7d084f8a as c_ulong as z_crc_t,
        0x6fbde064 as c_ulong as z_crc_t,
        0xd7018701 as c_ulong as z_crc_t,
        0x4ad6bfb8 as c_ulong as z_crc_t,
        0xf26ad8dd as c_ulong as z_crc_t,
        0xe0df7733 as c_ulong as z_crc_t,
        0x58631056 as c_ulong as z_crc_t,
        0x5019579f as c_ulong as z_crc_t,
        0xe8a530fa as c_ulong as z_crc_t,
        0xfa109f14 as c_ulong as z_crc_t,
        0x42acf871 as c_ulong as z_crc_t,
        0xdf7bc0c8 as c_ulong as z_crc_t,
        0x67c7a7ad as c_ulong as z_crc_t,
        0x75720843 as c_ulong as z_crc_t,
        0xcdce6f26 as c_ulong as z_crc_t,
        0x95ad7f70 as c_ulong as z_crc_t,
        0x2d111815 as c_ulong as z_crc_t,
        0x3fa4b7fb as c_ulong as z_crc_t,
        0x8718d09e as c_ulong as z_crc_t,
        0x1acfe827 as c_ulong as z_crc_t,
        0xa2738f42 as c_ulong as z_crc_t,
        0xb0c620ac as c_ulong as z_crc_t,
        0x87a47c9 as c_ulong as z_crc_t,
        0xa032af3e as c_ulong as z_crc_t,
        0x188ec85b as c_ulong as z_crc_t,
        0xa3b67b5 as c_ulong as z_crc_t,
        0xb28700d0 as c_ulong as z_crc_t,
        0x2f503869 as c_ulong as z_crc_t,
        0x97ec5f0c as c_ulong as z_crc_t,
        0x8559f0e2 as c_ulong as z_crc_t,
        0x3de59787 as c_ulong as z_crc_t,
        0x658687d1 as c_ulong as z_crc_t,
        0xdd3ae0b4 as c_ulong as z_crc_t,
        0xcf8f4f5a as c_ulong as z_crc_t,
        0x7733283f as c_ulong as z_crc_t,
        0xeae41086 as c_ulong as z_crc_t,
        0x525877e3 as c_ulong as z_crc_t,
        0x40edd80d as c_ulong as z_crc_t,
        0xf851bf68 as c_ulong as z_crc_t,
        0xf02bf8a1 as c_ulong as z_crc_t,
        0x48979fc4 as c_ulong as z_crc_t,
        0x5a22302a as c_ulong as z_crc_t,
        0xe29e574f as c_ulong as z_crc_t,
        0x7f496ff6 as c_ulong as z_crc_t,
        0xc7f50893 as c_ulong as z_crc_t,
        0xd540a77d as c_ulong as z_crc_t,
        0x6dfcc018 as c_ulong as z_crc_t,
        0x359fd04e as c_ulong as z_crc_t,
        0x8d23b72b as c_ulong as z_crc_t,
        0x9f9618c5 as c_ulong as z_crc_t,
        0x272a7fa0 as c_ulong as z_crc_t,
        0xbafd4719 as c_ulong as z_crc_t,
        0x241207c as c_ulong as z_crc_t,
        0x10f48f92 as c_ulong as z_crc_t,
        0xa848e8f7 as c_ulong as z_crc_t,
        0x9b14583d as c_ulong as z_crc_t,
        0x23a83f58 as c_ulong as z_crc_t,
        0x311d90b6 as c_ulong as z_crc_t,
        0x89a1f7d3 as c_ulong as z_crc_t,
        0x1476cf6a as c_ulong as z_crc_t,
        0xaccaa80f as c_ulong as z_crc_t,
        0xbe7f07e1 as c_ulong as z_crc_t,
        0x6c36084 as c_ulong as z_crc_t,
        0x5ea070d2 as c_ulong as z_crc_t,
        0xe61c17b7 as c_ulong as z_crc_t,
        0xf4a9b859 as c_ulong as z_crc_t,
        0x4c15df3c as c_ulong as z_crc_t,
        0xd1c2e785 as c_ulong as z_crc_t,
        0x697e80e0 as c_ulong as z_crc_t,
        0x7bcb2f0e as c_ulong as z_crc_t,
        0xc377486b as c_ulong as z_crc_t,
        0xcb0d0fa2 as c_ulong as z_crc_t,
        0x73b168c7 as c_ulong as z_crc_t,
        0x6104c729 as c_ulong as z_crc_t,
        0xd9b8a04c as c_ulong as z_crc_t,
        0x446f98f5 as c_ulong as z_crc_t,
        0xfcd3ff90 as c_ulong as z_crc_t,
        0xee66507e as c_ulong as z_crc_t,
        0x56da371b as c_ulong as z_crc_t,
        0xeb9274d as c_ulong as z_crc_t,
        0xb6054028 as c_ulong as z_crc_t,
        0xa4b0efc6 as c_ulong as z_crc_t,
        0x1c0c88a3 as c_ulong as z_crc_t,
        0x81dbb01a as c_ulong as z_crc_t,
        0x3967d77f as c_ulong as z_crc_t,
        0x2bd27891 as c_ulong as z_crc_t,
        0x936e1ff4 as c_ulong as z_crc_t,
        0x3b26f703 as c_ulong as z_crc_t,
        0x839a9066 as c_ulong as z_crc_t,
        0x912f3f88 as c_ulong as z_crc_t,
        0x299358ed as c_ulong as z_crc_t,
        0xb4446054 as c_ulong as z_crc_t,
        0xcf80731 as c_ulong as z_crc_t,
        0x1e4da8df as c_ulong as z_crc_t,
        0xa6f1cfba as c_ulong as z_crc_t,
        0xfe92dfec as c_ulong as z_crc_t,
        0x462eb889 as c_ulong as z_crc_t,
        0x549b1767 as c_ulong as z_crc_t,
        0xec277002 as c_ulong as z_crc_t,
        0x71f048bb as c_ulong as z_crc_t,
        0xc94c2fde as c_ulong as z_crc_t,
        0xdbf98030 as c_ulong as z_crc_t,
        0x6345e755 as c_ulong as z_crc_t,
        0x6b3fa09c as c_ulong as z_crc_t,
        0xd383c7f9 as c_ulong as z_crc_t,
        0xc1366817 as c_ulong as z_crc_t,
        0x798a0f72 as c_ulong as z_crc_t,
        0xe45d37cb as c_ulong as z_crc_t,
        0x5ce150ae as c_ulong as z_crc_t,
        0x4e54ff40 as c_ulong as z_crc_t,
        0xf6e89825 as c_ulong as z_crc_t,
        0xae8b8873 as c_ulong as z_crc_t,
        0x1637ef16 as c_ulong as z_crc_t,
        0x48240f8 as c_ulong as z_crc_t,
        0xbc3e279d as c_ulong as z_crc_t,
        0x21e91f24 as c_ulong as z_crc_t,
        0x99557841 as c_ulong as z_crc_t,
        0x8be0d7af as c_ulong as z_crc_t,
        0x335cb0ca as c_ulong as z_crc_t,
        0xed59b63b as c_ulong as z_crc_t,
        0x55e5d15e as c_ulong as z_crc_t,
        0x47507eb0 as c_ulong as z_crc_t,
        0xffec19d5 as c_ulong as z_crc_t,
        0x623b216c as c_ulong as z_crc_t,
        0xda874609 as c_ulong as z_crc_t,
        0xc832e9e7 as c_ulong as z_crc_t,
        0x708e8e82 as c_ulong as z_crc_t,
        0x28ed9ed4 as c_ulong as z_crc_t,
        0x9051f9b1 as c_ulong as z_crc_t,
        0x82e4565f as c_ulong as z_crc_t,
        0x3a58313a as c_ulong as z_crc_t,
        0xa78f0983 as c_ulong as z_crc_t,
        0x1f336ee6 as c_ulong as z_crc_t,
        0xd86c108 as c_ulong as z_crc_t,
        0xb53aa66d as c_ulong as z_crc_t,
        0xbd40e1a4 as c_ulong as z_crc_t,
        0x5fc86c1 as c_ulong as z_crc_t,
        0x1749292f as c_ulong as z_crc_t,
        0xaff54e4a as c_ulong as z_crc_t,
        0x322276f3 as c_ulong as z_crc_t,
        0x8a9e1196 as c_ulong as z_crc_t,
        0x982bbe78 as c_ulong as z_crc_t,
        0x2097d91d as c_ulong as z_crc_t,
        0x78f4c94b as c_ulong as z_crc_t,
        0xc048ae2e as c_ulong as z_crc_t,
        0xd2fd01c0 as c_ulong as z_crc_t,
        0x6a4166a5 as c_ulong as z_crc_t,
        0xf7965e1c as c_ulong as z_crc_t,
        0x4f2a3979 as c_ulong as z_crc_t,
        0x5d9f9697 as c_ulong as z_crc_t,
        0xe523f1f2 as c_ulong as z_crc_t,
        0x4d6b1905 as c_ulong as z_crc_t,
        0xf5d77e60 as c_ulong as z_crc_t,
        0xe762d18e as c_ulong as z_crc_t,
        0x5fdeb6eb as c_ulong as z_crc_t,
        0xc2098e52 as c_ulong as z_crc_t,
        0x7ab5e937 as c_ulong as z_crc_t,
        0x680046d9 as c_ulong as z_crc_t,
        0xd0bc21bc as c_ulong as z_crc_t,
        0x88df31ea as c_ulong as z_crc_t,
        0x3063568f as c_ulong as z_crc_t,
        0x22d6f961 as c_ulong as z_crc_t,
        0x9a6a9e04 as c_ulong as z_crc_t,
        0x7bda6bd as c_ulong as z_crc_t,
        0xbf01c1d8 as c_ulong as z_crc_t,
        0xadb46e36 as c_ulong as z_crc_t,
        0x15080953 as c_ulong as z_crc_t,
        0x1d724e9a as c_ulong as z_crc_t,
        0xa5ce29ff as c_ulong as z_crc_t,
        0xb77b8611 as c_ulong as z_crc_t,
        0xfc7e174 as c_ulong as z_crc_t,
        0x9210d9cd as c_ulong as z_crc_t,
        0x2aacbea8 as c_ulong as z_crc_t,
        0x38191146 as c_ulong as z_crc_t,
        0x80a57623 as c_ulong as z_crc_t,
        0xd8c66675 as c_ulong as z_crc_t,
        0x607a0110 as c_ulong as z_crc_t,
        0x72cfaefe as c_ulong as z_crc_t,
        0xca73c99b as c_ulong as z_crc_t,
        0x57a4f122 as c_ulong as z_crc_t,
        0xef189647 as c_ulong as z_crc_t,
        0xfdad39a9 as c_ulong as z_crc_t,
        0x45115ecc as c_ulong as z_crc_t,
        0x764dee06 as c_ulong as z_crc_t,
        0xcef18963 as c_ulong as z_crc_t,
        0xdc44268d as c_ulong as z_crc_t,
        0x64f841e8 as c_ulong as z_crc_t,
        0xf92f7951 as c_ulong as z_crc_t,
        0x41931e34 as c_ulong as z_crc_t,
        0x5326b1da as c_ulong as z_crc_t,
        0xeb9ad6bf as c_ulong as z_crc_t,
        0xb3f9c6e9 as c_ulong as z_crc_t,
        0xb45a18c as c_ulong as z_crc_t,
        0x19f00e62 as c_ulong as z_crc_t,
        0xa14c6907 as c_ulong as z_crc_t,
        0x3c9b51be as c_ulong as z_crc_t,
        0x842736db as c_ulong as z_crc_t,
        0x96929935 as c_ulong as z_crc_t,
        0x2e2efe50 as c_ulong as z_crc_t,
        0x2654b999 as c_ulong as z_crc_t,
        0x9ee8defc as c_ulong as z_crc_t,
        0x8c5d7112 as c_ulong as z_crc_t,
        0x34e11677 as c_ulong as z_crc_t,
        0xa9362ece as c_ulong as z_crc_t,
        0x118a49ab as c_ulong as z_crc_t,
        0x33fe645 as c_ulong as z_crc_t,
        0xbb838120 as c_ulong as z_crc_t,
        0xe3e09176 as c_ulong as z_crc_t,
        0x5b5cf613 as c_ulong as z_crc_t,
        0x49e959fd as c_ulong as z_crc_t,
        0xf1553e98 as c_ulong as z_crc_t,
        0x6c820621 as c_ulong as z_crc_t,
        0xd43e6144 as c_ulong as z_crc_t,
        0xc68bceaa as c_ulong as z_crc_t,
        0x7e37a9cf as c_ulong as z_crc_t,
        0xd67f4138 as c_ulong as z_crc_t,
        0x6ec3265d as c_ulong as z_crc_t,
        0x7c7689b3 as c_ulong as z_crc_t,
        0xc4caeed6 as c_ulong as z_crc_t,
        0x591dd66f as c_ulong as z_crc_t,
        0xe1a1b10a as c_ulong as z_crc_t,
        0xf3141ee4 as c_ulong as z_crc_t,
        0x4ba87981 as c_ulong as z_crc_t,
        0x13cb69d7 as c_ulong as z_crc_t,
        0xab770eb2 as c_ulong as z_crc_t,
        0xb9c2a15c as c_ulong as z_crc_t,
        0x17ec639 as c_ulong as z_crc_t,
        0x9ca9fe80 as c_ulong as z_crc_t,
        0x241599e5 as c_ulong as z_crc_t,
        0x36a0360b as c_ulong as z_crc_t,
        0x8e1c516e as c_ulong as z_crc_t,
        0x866616a7 as c_ulong as z_crc_t,
        0x3eda71c2 as c_ulong as z_crc_t,
        0x2c6fde2c as c_ulong as z_crc_t,
        0x94d3b949 as c_ulong as z_crc_t,
        0x90481f0 as c_ulong as z_crc_t,
        0xb1b8e695 as c_ulong as z_crc_t,
        0xa30d497b as c_ulong as z_crc_t,
        0x1bb12e1e as c_ulong as z_crc_t,
        0x43d23e48 as c_ulong as z_crc_t,
        0xfb6e592d as c_ulong as z_crc_t,
        0xe9dbf6c3 as c_ulong as z_crc_t,
        0x516791a6 as c_ulong as z_crc_t,
        0xccb0a91f as c_ulong as z_crc_t,
        0x740cce7a as c_ulong as z_crc_t,
        0x66b96194 as c_ulong as z_crc_t,
        0xde0506f1 as c_ulong as z_crc_t,
    ],
    [
        0 as c_ulong as z_crc_t,
        0x96300777 as c_ulong as z_crc_t,
        0x2c610eee as c_ulong as z_crc_t,
        0xba510999 as c_ulong as z_crc_t,
        0x19c46d07 as c_ulong as z_crc_t,
        0x8ff46a70 as c_ulong as z_crc_t,
        0x35a563e9 as c_ulong as z_crc_t,
        0xa395649e as c_ulong as z_crc_t,
        0x3288db0e as c_ulong as z_crc_t,
        0xa4b8dc79 as c_ulong as z_crc_t,
        0x1ee9d5e0 as c_ulong as z_crc_t,
        0x88d9d297 as c_ulong as z_crc_t,
        0x2b4cb609 as c_ulong as z_crc_t,
        0xbd7cb17e as c_ulong as z_crc_t,
        0x72db8e7 as c_ulong as z_crc_t,
        0x911dbf90 as c_ulong as z_crc_t,
        0x6410b71d as c_ulong as z_crc_t,
        0xf220b06a as c_ulong as z_crc_t,
        0x4871b9f3 as c_ulong as z_crc_t,
        0xde41be84 as c_ulong as z_crc_t,
        0x7dd4da1a as c_ulong as z_crc_t,
        0xebe4dd6d as c_ulong as z_crc_t,
        0x51b5d4f4 as c_ulong as z_crc_t,
        0xc785d383 as c_ulong as z_crc_t,
        0x56986c13 as c_ulong as z_crc_t,
        0xc0a86b64 as c_ulong as z_crc_t,
        0x7af962fd as c_ulong as z_crc_t,
        0xecc9658a as c_ulong as z_crc_t,
        0x4f5c0114 as c_ulong as z_crc_t,
        0xd96c0663 as c_ulong as z_crc_t,
        0x633d0ffa as c_ulong as z_crc_t,
        0xf50d088d as c_ulong as z_crc_t,
        0xc8206e3b as c_ulong as z_crc_t,
        0x5e10694c as c_ulong as z_crc_t,
        0xe44160d5 as c_ulong as z_crc_t,
        0x727167a2 as c_ulong as z_crc_t,
        0xd1e4033c as c_ulong as z_crc_t,
        0x47d4044b as c_ulong as z_crc_t,
        0xfd850dd2 as c_ulong as z_crc_t,
        0x6bb50aa5 as c_ulong as z_crc_t,
        0xfaa8b535 as c_ulong as z_crc_t,
        0x6c98b242 as c_ulong as z_crc_t,
        0xd6c9bbdb as c_ulong as z_crc_t,
        0x40f9bcac as c_ulong as z_crc_t,
        0xe36cd832 as c_ulong as z_crc_t,
        0x755cdf45 as c_ulong as z_crc_t,
        0xcf0dd6dc as c_ulong as z_crc_t,
        0x593dd1ab as c_ulong as z_crc_t,
        0xac30d926 as c_ulong as z_crc_t,
        0x3a00de51 as c_ulong as z_crc_t,
        0x8051d7c8 as c_ulong as z_crc_t,
        0x1661d0bf as c_ulong as z_crc_t,
        0xb5f4b421 as c_ulong as z_crc_t,
        0x23c4b356 as c_ulong as z_crc_t,
        0x9995bacf as c_ulong as z_crc_t,
        0xfa5bdb8 as c_ulong as z_crc_t,
        0x9eb80228 as c_ulong as z_crc_t,
        0x888055f as c_ulong as z_crc_t,
        0xb2d90cc6 as c_ulong as z_crc_t,
        0x24e90bb1 as c_ulong as z_crc_t,
        0x877c6f2f as c_ulong as z_crc_t,
        0x114c6858 as c_ulong as z_crc_t,
        0xab1d61c1 as c_ulong as z_crc_t,
        0x3d2d66b6 as c_ulong as z_crc_t,
        0x9041dc76 as c_ulong as z_crc_t,
        0x671db01 as c_ulong as z_crc_t,
        0xbc20d298 as c_ulong as z_crc_t,
        0x2a10d5ef as c_ulong as z_crc_t,
        0x8985b171 as c_ulong as z_crc_t,
        0x1fb5b606 as c_ulong as z_crc_t,
        0xa5e4bf9f as c_ulong as z_crc_t,
        0x33d4b8e8 as c_ulong as z_crc_t,
        0xa2c90778 as c_ulong as z_crc_t,
        0x34f9000f as c_ulong as z_crc_t,
        0x8ea80996 as c_ulong as z_crc_t,
        0x18980ee1 as c_ulong as z_crc_t,
        0xbb0d6a7f as c_ulong as z_crc_t,
        0x2d3d6d08 as c_ulong as z_crc_t,
        0x976c6491 as c_ulong as z_crc_t,
        0x15c63e6 as c_ulong as z_crc_t,
        0xf4516b6b as c_ulong as z_crc_t,
        0x62616c1c as c_ulong as z_crc_t,
        0xd8306585 as c_ulong as z_crc_t,
        0x4e0062f2 as c_ulong as z_crc_t,
        0xed95066c as c_ulong as z_crc_t,
        0x7ba5011b as c_ulong as z_crc_t,
        0xc1f40882 as c_ulong as z_crc_t,
        0x57c40ff5 as c_ulong as z_crc_t,
        0xc6d9b065 as c_ulong as z_crc_t,
        0x50e9b712 as c_ulong as z_crc_t,
        0xeab8be8b as c_ulong as z_crc_t,
        0x7c88b9fc as c_ulong as z_crc_t,
        0xdf1ddd62 as c_ulong as z_crc_t,
        0x492dda15 as c_ulong as z_crc_t,
        0xf37cd38c as c_ulong as z_crc_t,
        0x654cd4fb as c_ulong as z_crc_t,
        0x5861b24d as c_ulong as z_crc_t,
        0xce51b53a as c_ulong as z_crc_t,
        0x7400bca3 as c_ulong as z_crc_t,
        0xe230bbd4 as c_ulong as z_crc_t,
        0x41a5df4a as c_ulong as z_crc_t,
        0xd795d83d as c_ulong as z_crc_t,
        0x6dc4d1a4 as c_ulong as z_crc_t,
        0xfbf4d6d3 as c_ulong as z_crc_t,
        0x6ae96943 as c_ulong as z_crc_t,
        0xfcd96e34 as c_ulong as z_crc_t,
        0x468867ad as c_ulong as z_crc_t,
        0xd0b860da as c_ulong as z_crc_t,
        0x732d0444 as c_ulong as z_crc_t,
        0xe51d0333 as c_ulong as z_crc_t,
        0x5f4c0aaa as c_ulong as z_crc_t,
        0xc97c0ddd as c_ulong as z_crc_t,
        0x3c710550 as c_ulong as z_crc_t,
        0xaa410227 as c_ulong as z_crc_t,
        0x10100bbe as c_ulong as z_crc_t,
        0x86200cc9 as c_ulong as z_crc_t,
        0x25b56857 as c_ulong as z_crc_t,
        0xb3856f20 as c_ulong as z_crc_t,
        0x9d466b9 as c_ulong as z_crc_t,
        0x9fe461ce as c_ulong as z_crc_t,
        0xef9de5e as c_ulong as z_crc_t,
        0x98c9d929 as c_ulong as z_crc_t,
        0x2298d0b0 as c_ulong as z_crc_t,
        0xb4a8d7c7 as c_ulong as z_crc_t,
        0x173db359 as c_ulong as z_crc_t,
        0x810db42e as c_ulong as z_crc_t,
        0x3b5cbdb7 as c_ulong as z_crc_t,
        0xad6cbac0 as c_ulong as z_crc_t,
        0x2083b8ed as c_ulong as z_crc_t,
        0xb6b3bf9a as c_ulong as z_crc_t,
        0xce2b603 as c_ulong as z_crc_t,
        0x9ad2b174 as c_ulong as z_crc_t,
        0x3947d5ea as c_ulong as z_crc_t,
        0xaf77d29d as c_ulong as z_crc_t,
        0x1526db04 as c_ulong as z_crc_t,
        0x8316dc73 as c_ulong as z_crc_t,
        0x120b63e3 as c_ulong as z_crc_t,
        0x843b6494 as c_ulong as z_crc_t,
        0x3e6a6d0d as c_ulong as z_crc_t,
        0xa85a6a7a as c_ulong as z_crc_t,
        0xbcf0ee4 as c_ulong as z_crc_t,
        0x9dff0993 as c_ulong as z_crc_t,
        0x27ae000a as c_ulong as z_crc_t,
        0xb19e077d as c_ulong as z_crc_t,
        0x44930ff0 as c_ulong as z_crc_t,
        0xd2a30887 as c_ulong as z_crc_t,
        0x68f2011e as c_ulong as z_crc_t,
        0xfec20669 as c_ulong as z_crc_t,
        0x5d5762f7 as c_ulong as z_crc_t,
        0xcb676580 as c_ulong as z_crc_t,
        0x71366c19 as c_ulong as z_crc_t,
        0xe7066b6e as c_ulong as z_crc_t,
        0x761bd4fe as c_ulong as z_crc_t,
        0xe02bd389 as c_ulong as z_crc_t,
        0x5a7ada10 as c_ulong as z_crc_t,
        0xcc4add67 as c_ulong as z_crc_t,
        0x6fdfb9f9 as c_ulong as z_crc_t,
        0xf9efbe8e as c_ulong as z_crc_t,
        0x43beb717 as c_ulong as z_crc_t,
        0xd58eb060 as c_ulong as z_crc_t,
        0xe8a3d6d6 as c_ulong as z_crc_t,
        0x7e93d1a1 as c_ulong as z_crc_t,
        0xc4c2d838 as c_ulong as z_crc_t,
        0x52f2df4f as c_ulong as z_crc_t,
        0xf167bbd1 as c_ulong as z_crc_t,
        0x6757bca6 as c_ulong as z_crc_t,
        0xdd06b53f as c_ulong as z_crc_t,
        0x4b36b248 as c_ulong as z_crc_t,
        0xda2b0dd8 as c_ulong as z_crc_t,
        0x4c1b0aaf as c_ulong as z_crc_t,
        0xf64a0336 as c_ulong as z_crc_t,
        0x607a0441 as c_ulong as z_crc_t,
        0xc3ef60df as c_ulong as z_crc_t,
        0x55df67a8 as c_ulong as z_crc_t,
        0xef8e6e31 as c_ulong as z_crc_t,
        0x79be6946 as c_ulong as z_crc_t,
        0x8cb361cb as c_ulong as z_crc_t,
        0x1a8366bc as c_ulong as z_crc_t,
        0xa0d26f25 as c_ulong as z_crc_t,
        0x36e26852 as c_ulong as z_crc_t,
        0x95770ccc as c_ulong as z_crc_t,
        0x3470bbb as c_ulong as z_crc_t,
        0xb9160222 as c_ulong as z_crc_t,
        0x2f260555 as c_ulong as z_crc_t,
        0xbe3bbac5 as c_ulong as z_crc_t,
        0x280bbdb2 as c_ulong as z_crc_t,
        0x925ab42b as c_ulong as z_crc_t,
        0x46ab35c as c_ulong as z_crc_t,
        0xa7ffd7c2 as c_ulong as z_crc_t,
        0x31cfd0b5 as c_ulong as z_crc_t,
        0x8b9ed92c as c_ulong as z_crc_t,
        0x1daede5b as c_ulong as z_crc_t,
        0xb0c2649b as c_ulong as z_crc_t,
        0x26f263ec as c_ulong as z_crc_t,
        0x9ca36a75 as c_ulong as z_crc_t,
        0xa936d02 as c_ulong as z_crc_t,
        0xa906099c as c_ulong as z_crc_t,
        0x3f360eeb as c_ulong as z_crc_t,
        0x85670772 as c_ulong as z_crc_t,
        0x13570005 as c_ulong as z_crc_t,
        0x824abf95 as c_ulong as z_crc_t,
        0x147ab8e2 as c_ulong as z_crc_t,
        0xae2bb17b as c_ulong as z_crc_t,
        0x381bb60c as c_ulong as z_crc_t,
        0x9b8ed292 as c_ulong as z_crc_t,
        0xdbed5e5 as c_ulong as z_crc_t,
        0xb7efdc7c as c_ulong as z_crc_t,
        0x21dfdb0b as c_ulong as z_crc_t,
        0xd4d2d386 as c_ulong as z_crc_t,
        0x42e2d4f1 as c_ulong as z_crc_t,
        0xf8b3dd68 as c_ulong as z_crc_t,
        0x6e83da1f as c_ulong as z_crc_t,
        0xcd16be81 as c_ulong as z_crc_t,
        0x5b26b9f6 as c_ulong as z_crc_t,
        0xe177b06f as c_ulong as z_crc_t,
        0x7747b718 as c_ulong as z_crc_t,
        0xe65a0888 as c_ulong as z_crc_t,
        0x706a0fff as c_ulong as z_crc_t,
        0xca3b0666 as c_ulong as z_crc_t,
        0x5c0b0111 as c_ulong as z_crc_t,
        0xff9e658f as c_ulong as z_crc_t,
        0x69ae62f8 as c_ulong as z_crc_t,
        0xd3ff6b61 as c_ulong as z_crc_t,
        0x45cf6c16 as c_ulong as z_crc_t,
        0x78e20aa0 as c_ulong as z_crc_t,
        0xeed20dd7 as c_ulong as z_crc_t,
        0x5483044e as c_ulong as z_crc_t,
        0xc2b30339 as c_ulong as z_crc_t,
        0x612667a7 as c_ulong as z_crc_t,
        0xf71660d0 as c_ulong as z_crc_t,
        0x4d476949 as c_ulong as z_crc_t,
        0xdb776e3e as c_ulong as z_crc_t,
        0x4a6ad1ae as c_ulong as z_crc_t,
        0xdc5ad6d9 as c_ulong as z_crc_t,
        0x660bdf40 as c_ulong as z_crc_t,
        0xf03bd837 as c_ulong as z_crc_t,
        0x53aebca9 as c_ulong as z_crc_t,
        0xc59ebbde as c_ulong as z_crc_t,
        0x7fcfb247 as c_ulong as z_crc_t,
        0xe9ffb530 as c_ulong as z_crc_t,
        0x1cf2bdbd as c_ulong as z_crc_t,
        0x8ac2baca as c_ulong as z_crc_t,
        0x3093b353 as c_ulong as z_crc_t,
        0xa6a3b424 as c_ulong as z_crc_t,
        0x536d0ba as c_ulong as z_crc_t,
        0x9306d7cd as c_ulong as z_crc_t,
        0x2957de54 as c_ulong as z_crc_t,
        0xbf67d923 as c_ulong as z_crc_t,
        0x2e7a66b3 as c_ulong as z_crc_t,
        0xb84a61c4 as c_ulong as z_crc_t,
        0x21b685d as c_ulong as z_crc_t,
        0x942b6f2a as c_ulong as z_crc_t,
        0x37be0bb4 as c_ulong as z_crc_t,
        0xa18e0cc3 as c_ulong as z_crc_t,
        0x1bdf055a as c_ulong as z_crc_t,
        0x8def022d as c_ulong as z_crc_t,
    ],
    [
        0 as c_ulong as z_crc_t,
        0x41311b19 as c_ulong as z_crc_t,
        0x82623632 as c_ulong as z_crc_t,
        0xc3532d2b as c_ulong as z_crc_t,
        0x4c56c64 as c_ulong as z_crc_t,
        0x45f4777d as c_ulong as z_crc_t,
        0x86a75a56 as c_ulong as z_crc_t,
        0xc796414f as c_ulong as z_crc_t,
        0x88ad9c8 as c_ulong as z_crc_t,
        0x49bbc2d1 as c_ulong as z_crc_t,
        0x8ae8effa as c_ulong as z_crc_t,
        0xcbd9f4e3 as c_ulong as z_crc_t,
        0xc4fb5ac as c_ulong as z_crc_t,
        0x4d7eaeb5 as c_ulong as z_crc_t,
        0x8e2d839e as c_ulong as z_crc_t,
        0xcf1c9887 as c_ulong as z_crc_t,
        0x5112c24a as c_ulong as z_crc_t,
        0x1023d953 as c_ulong as z_crc_t,
        0xd370f478 as c_ulong as z_crc_t,
        0x9241ef61 as c_ulong as z_crc_t,
        0x55d7ae2e as c_ulong as z_crc_t,
        0x14e6b537 as c_ulong as z_crc_t,
        0xd7b5981c as c_ulong as z_crc_t,
        0x96848305 as c_ulong as z_crc_t,
        0x59981b82 as c_ulong as z_crc_t,
        0x18a9009b as c_ulong as z_crc_t,
        0xdbfa2db0 as c_ulong as z_crc_t,
        0x9acb36a9 as c_ulong as z_crc_t,
        0x5d5d77e6 as c_ulong as z_crc_t,
        0x1c6c6cff as c_ulong as z_crc_t,
        0xdf3f41d4 as c_ulong as z_crc_t,
        0x9e0e5acd as c_ulong as z_crc_t,
        0xa2248495 as c_ulong as z_crc_t,
        0xe3159f8c as c_ulong as z_crc_t,
        0x2046b2a7 as c_ulong as z_crc_t,
        0x6177a9be as c_ulong as z_crc_t,
        0xa6e1e8f1 as c_ulong as z_crc_t,
        0xe7d0f3e8 as c_ulong as z_crc_t,
        0x2483dec3 as c_ulong as z_crc_t,
        0x65b2c5da as c_ulong as z_crc_t,
        0xaaae5d5d as c_ulong as z_crc_t,
        0xeb9f4644 as c_ulong as z_crc_t,
        0x28cc6b6f as c_ulong as z_crc_t,
        0x69fd7076 as c_ulong as z_crc_t,
        0xae6b3139 as c_ulong as z_crc_t,
        0xef5a2a20 as c_ulong as z_crc_t,
        0x2c09070b as c_ulong as z_crc_t,
        0x6d381c12 as c_ulong as z_crc_t,
        0xf33646df as c_ulong as z_crc_t,
        0xb2075dc6 as c_ulong as z_crc_t,
        0x715470ed as c_ulong as z_crc_t,
        0x30656bf4 as c_ulong as z_crc_t,
        0xf7f32abb as c_ulong as z_crc_t,
        0xb6c231a2 as c_ulong as z_crc_t,
        0x75911c89 as c_ulong as z_crc_t,
        0x34a00790 as c_ulong as z_crc_t,
        0xfbbc9f17 as c_ulong as z_crc_t,
        0xba8d840e as c_ulong as z_crc_t,
        0x79dea925 as c_ulong as z_crc_t,
        0x38efb23c as c_ulong as z_crc_t,
        0xff79f373 as c_ulong as z_crc_t,
        0xbe48e86a as c_ulong as z_crc_t,
        0x7d1bc541 as c_ulong as z_crc_t,
        0x3c2ade58 as c_ulong as z_crc_t,
        0x54f79f0 as c_ulong as z_crc_t,
        0x447e62e9 as c_ulong as z_crc_t,
        0x872d4fc2 as c_ulong as z_crc_t,
        0xc61c54db as c_ulong as z_crc_t,
        0x18a1594 as c_ulong as z_crc_t,
        0x40bb0e8d as c_ulong as z_crc_t,
        0x83e823a6 as c_ulong as z_crc_t,
        0xc2d938bf as c_ulong as z_crc_t,
        0xdc5a038 as c_ulong as z_crc_t,
        0x4cf4bb21 as c_ulong as z_crc_t,
        0x8fa7960a as c_ulong as z_crc_t,
        0xce968d13 as c_ulong as z_crc_t,
        0x900cc5c as c_ulong as z_crc_t,
        0x4831d745 as c_ulong as z_crc_t,
        0x8b62fa6e as c_ulong as z_crc_t,
        0xca53e177 as c_ulong as z_crc_t,
        0x545dbbba as c_ulong as z_crc_t,
        0x156ca0a3 as c_ulong as z_crc_t,
        0xd63f8d88 as c_ulong as z_crc_t,
        0x970e9691 as c_ulong as z_crc_t,
        0x5098d7de as c_ulong as z_crc_t,
        0x11a9ccc7 as c_ulong as z_crc_t,
        0xd2fae1ec as c_ulong as z_crc_t,
        0x93cbfaf5 as c_ulong as z_crc_t,
        0x5cd76272 as c_ulong as z_crc_t,
        0x1de6796b as c_ulong as z_crc_t,
        0xdeb55440 as c_ulong as z_crc_t,
        0x9f844f59 as c_ulong as z_crc_t,
        0x58120e16 as c_ulong as z_crc_t,
        0x1923150f as c_ulong as z_crc_t,
        0xda703824 as c_ulong as z_crc_t,
        0x9b41233d as c_ulong as z_crc_t,
        0xa76bfd65 as c_ulong as z_crc_t,
        0xe65ae67c as c_ulong as z_crc_t,
        0x2509cb57 as c_ulong as z_crc_t,
        0x6438d04e as c_ulong as z_crc_t,
        0xa3ae9101 as c_ulong as z_crc_t,
        0xe29f8a18 as c_ulong as z_crc_t,
        0x21cca733 as c_ulong as z_crc_t,
        0x60fdbc2a as c_ulong as z_crc_t,
        0xafe124ad as c_ulong as z_crc_t,
        0xeed03fb4 as c_ulong as z_crc_t,
        0x2d83129f as c_ulong as z_crc_t,
        0x6cb20986 as c_ulong as z_crc_t,
        0xab2448c9 as c_ulong as z_crc_t,
        0xea1553d0 as c_ulong as z_crc_t,
        0x29467efb as c_ulong as z_crc_t,
        0x687765e2 as c_ulong as z_crc_t,
        0xf6793f2f as c_ulong as z_crc_t,
        0xb7482436 as c_ulong as z_crc_t,
        0x741b091d as c_ulong as z_crc_t,
        0x352a1204 as c_ulong as z_crc_t,
        0xf2bc534b as c_ulong as z_crc_t,
        0xb38d4852 as c_ulong as z_crc_t,
        0x70de6579 as c_ulong as z_crc_t,
        0x31ef7e60 as c_ulong as z_crc_t,
        0xfef3e6e7 as c_ulong as z_crc_t,
        0xbfc2fdfe as c_ulong as z_crc_t,
        0x7c91d0d5 as c_ulong as z_crc_t,
        0x3da0cbcc as c_ulong as z_crc_t,
        0xfa368a83 as c_ulong as z_crc_t,
        0xbb07919a as c_ulong as z_crc_t,
        0x7854bcb1 as c_ulong as z_crc_t,
        0x3965a7a8 as c_ulong as z_crc_t,
        0x4b98833b as c_ulong as z_crc_t,
        0xaa99822 as c_ulong as z_crc_t,
        0xc9fab509 as c_ulong as z_crc_t,
        0x88cbae10 as c_ulong as z_crc_t,
        0x4f5def5f as c_ulong as z_crc_t,
        0xe6cf446 as c_ulong as z_crc_t,
        0xcd3fd96d as c_ulong as z_crc_t,
        0x8c0ec274 as c_ulong as z_crc_t,
        0x43125af3 as c_ulong as z_crc_t,
        0x22341ea as c_ulong as z_crc_t,
        0xc1706cc1 as c_ulong as z_crc_t,
        0x804177d8 as c_ulong as z_crc_t,
        0x47d73697 as c_ulong as z_crc_t,
        0x6e62d8e as c_ulong as z_crc_t,
        0xc5b500a5 as c_ulong as z_crc_t,
        0x84841bbc as c_ulong as z_crc_t,
        0x1a8a4171 as c_ulong as z_crc_t,
        0x5bbb5a68 as c_ulong as z_crc_t,
        0x98e87743 as c_ulong as z_crc_t,
        0xd9d96c5a as c_ulong as z_crc_t,
        0x1e4f2d15 as c_ulong as z_crc_t,
        0x5f7e360c as c_ulong as z_crc_t,
        0x9c2d1b27 as c_ulong as z_crc_t,
        0xdd1c003e as c_ulong as z_crc_t,
        0x120098b9 as c_ulong as z_crc_t,
        0x533183a0 as c_ulong as z_crc_t,
        0x9062ae8b as c_ulong as z_crc_t,
        0xd153b592 as c_ulong as z_crc_t,
        0x16c5f4dd as c_ulong as z_crc_t,
        0x57f4efc4 as c_ulong as z_crc_t,
        0x94a7c2ef as c_ulong as z_crc_t,
        0xd596d9f6 as c_ulong as z_crc_t,
        0xe9bc07ae as c_ulong as z_crc_t,
        0xa88d1cb7 as c_ulong as z_crc_t,
        0x6bde319c as c_ulong as z_crc_t,
        0x2aef2a85 as c_ulong as z_crc_t,
        0xed796bca as c_ulong as z_crc_t,
        0xac4870d3 as c_ulong as z_crc_t,
        0x6f1b5df8 as c_ulong as z_crc_t,
        0x2e2a46e1 as c_ulong as z_crc_t,
        0xe136de66 as c_ulong as z_crc_t,
        0xa007c57f as c_ulong as z_crc_t,
        0x6354e854 as c_ulong as z_crc_t,
        0x2265f34d as c_ulong as z_crc_t,
        0xe5f3b202 as c_ulong as z_crc_t,
        0xa4c2a91b as c_ulong as z_crc_t,
        0x67918430 as c_ulong as z_crc_t,
        0x26a09f29 as c_ulong as z_crc_t,
        0xb8aec5e4 as c_ulong as z_crc_t,
        0xf99fdefd as c_ulong as z_crc_t,
        0x3accf3d6 as c_ulong as z_crc_t,
        0x7bfde8cf as c_ulong as z_crc_t,
        0xbc6ba980 as c_ulong as z_crc_t,
        0xfd5ab299 as c_ulong as z_crc_t,
        0x3e099fb2 as c_ulong as z_crc_t,
        0x7f3884ab as c_ulong as z_crc_t,
        0xb0241c2c as c_ulong as z_crc_t,
        0xf1150735 as c_ulong as z_crc_t,
        0x32462a1e as c_ulong as z_crc_t,
        0x73773107 as c_ulong as z_crc_t,
        0xb4e17048 as c_ulong as z_crc_t,
        0xf5d06b51 as c_ulong as z_crc_t,
        0x3683467a as c_ulong as z_crc_t,
        0x77b25d63 as c_ulong as z_crc_t,
        0x4ed7facb as c_ulong as z_crc_t,
        0xfe6e1d2 as c_ulong as z_crc_t,
        0xccb5ccf9 as c_ulong as z_crc_t,
        0x8d84d7e0 as c_ulong as z_crc_t,
        0x4a1296af as c_ulong as z_crc_t,
        0xb238db6 as c_ulong as z_crc_t,
        0xc870a09d as c_ulong as z_crc_t,
        0x8941bb84 as c_ulong as z_crc_t,
        0x465d2303 as c_ulong as z_crc_t,
        0x76c381a as c_ulong as z_crc_t,
        0xc43f1531 as c_ulong as z_crc_t,
        0x850e0e28 as c_ulong as z_crc_t,
        0x42984f67 as c_ulong as z_crc_t,
        0x3a9547e as c_ulong as z_crc_t,
        0xc0fa7955 as c_ulong as z_crc_t,
        0x81cb624c as c_ulong as z_crc_t,
        0x1fc53881 as c_ulong as z_crc_t,
        0x5ef42398 as c_ulong as z_crc_t,
        0x9da70eb3 as c_ulong as z_crc_t,
        0xdc9615aa as c_ulong as z_crc_t,
        0x1b0054e5 as c_ulong as z_crc_t,
        0x5a314ffc as c_ulong as z_crc_t,
        0x996262d7 as c_ulong as z_crc_t,
        0xd85379ce as c_ulong as z_crc_t,
        0x174fe149 as c_ulong as z_crc_t,
        0x567efa50 as c_ulong as z_crc_t,
        0x952dd77b as c_ulong as z_crc_t,
        0xd41ccc62 as c_ulong as z_crc_t,
        0x138a8d2d as c_ulong as z_crc_t,
        0x52bb9634 as c_ulong as z_crc_t,
        0x91e8bb1f as c_ulong as z_crc_t,
        0xd0d9a006 as c_ulong as z_crc_t,
        0xecf37e5e as c_ulong as z_crc_t,
        0xadc26547 as c_ulong as z_crc_t,
        0x6e91486c as c_ulong as z_crc_t,
        0x2fa05375 as c_ulong as z_crc_t,
        0xe836123a as c_ulong as z_crc_t,
        0xa9070923 as c_ulong as z_crc_t,
        0x6a542408 as c_ulong as z_crc_t,
        0x2b653f11 as c_ulong as z_crc_t,
        0xe479a796 as c_ulong as z_crc_t,
        0xa548bc8f as c_ulong as z_crc_t,
        0x661b91a4 as c_ulong as z_crc_t,
        0x272a8abd as c_ulong as z_crc_t,
        0xe0bccbf2 as c_ulong as z_crc_t,
        0xa18dd0eb as c_ulong as z_crc_t,
        0x62defdc0 as c_ulong as z_crc_t,
        0x23efe6d9 as c_ulong as z_crc_t,
        0xbde1bc14 as c_ulong as z_crc_t,
        0xfcd0a70d as c_ulong as z_crc_t,
        0x3f838a26 as c_ulong as z_crc_t,
        0x7eb2913f as c_ulong as z_crc_t,
        0xb924d070 as c_ulong as z_crc_t,
        0xf815cb69 as c_ulong as z_crc_t,
        0x3b46e642 as c_ulong as z_crc_t,
        0x7a77fd5b as c_ulong as z_crc_t,
        0xb56b65dc as c_ulong as z_crc_t,
        0xf45a7ec5 as c_ulong as z_crc_t,
        0x370953ee as c_ulong as z_crc_t,
        0x763848f7 as c_ulong as z_crc_t,
        0xb1ae09b8 as c_ulong as z_crc_t,
        0xf09f12a1 as c_ulong as z_crc_t,
        0x33cc3f8a as c_ulong as z_crc_t,
        0x72fd2493 as c_ulong as z_crc_t,
    ],
    [
        0 as c_ulong as z_crc_t,
        0x376ac201 as c_ulong as z_crc_t,
        0x6ed48403 as c_ulong as z_crc_t,
        0x59be4602 as c_ulong as z_crc_t,
        0xdca80907 as c_ulong as z_crc_t,
        0xebc2cb06 as c_ulong as z_crc_t,
        0xb27c8d04 as c_ulong as z_crc_t,
        0x85164f05 as c_ulong as z_crc_t,
        0xb851130e as c_ulong as z_crc_t,
        0x8f3bd10f as c_ulong as z_crc_t,
        0xd685970d as c_ulong as z_crc_t,
        0xe1ef550c as c_ulong as z_crc_t,
        0x64f91a09 as c_ulong as z_crc_t,
        0x5393d808 as c_ulong as z_crc_t,
        0xa2d9e0a as c_ulong as z_crc_t,
        0x3d475c0b as c_ulong as z_crc_t,
        0x70a3261c as c_ulong as z_crc_t,
        0x47c9e41d as c_ulong as z_crc_t,
        0x1e77a21f as c_ulong as z_crc_t,
        0x291d601e as c_ulong as z_crc_t,
        0xac0b2f1b as c_ulong as z_crc_t,
        0x9b61ed1a as c_ulong as z_crc_t,
        0xc2dfab18 as c_ulong as z_crc_t,
        0xf5b56919 as c_ulong as z_crc_t,
        0xc8f23512 as c_ulong as z_crc_t,
        0xff98f713 as c_ulong as z_crc_t,
        0xa626b111 as c_ulong as z_crc_t,
        0x914c7310 as c_ulong as z_crc_t,
        0x145a3c15 as c_ulong as z_crc_t,
        0x2330fe14 as c_ulong as z_crc_t,
        0x7a8eb816 as c_ulong as z_crc_t,
        0x4de47a17 as c_ulong as z_crc_t,
        0xe0464d38 as c_ulong as z_crc_t,
        0xd72c8f39 as c_ulong as z_crc_t,
        0x8e92c93b as c_ulong as z_crc_t,
        0xb9f80b3a as c_ulong as z_crc_t,
        0x3cee443f as c_ulong as z_crc_t,
        0xb84863e as c_ulong as z_crc_t,
        0x523ac03c as c_ulong as z_crc_t,
        0x6550023d as c_ulong as z_crc_t,
        0x58175e36 as c_ulong as z_crc_t,
        0x6f7d9c37 as c_ulong as z_crc_t,
        0x36c3da35 as c_ulong as z_crc_t,
        0x1a91834 as c_ulong as z_crc_t,
        0x84bf5731 as c_ulong as z_crc_t,
        0xb3d59530 as c_ulong as z_crc_t,
        0xea6bd332 as c_ulong as z_crc_t,
        0xdd011133 as c_ulong as z_crc_t,
        0x90e56b24 as c_ulong as z_crc_t,
        0xa78fa925 as c_ulong as z_crc_t,
        0xfe31ef27 as c_ulong as z_crc_t,
        0xc95b2d26 as c_ulong as z_crc_t,
        0x4c4d6223 as c_ulong as z_crc_t,
        0x7b27a022 as c_ulong as z_crc_t,
        0x2299e620 as c_ulong as z_crc_t,
        0x15f32421 as c_ulong as z_crc_t,
        0x28b4782a as c_ulong as z_crc_t,
        0x1fdeba2b as c_ulong as z_crc_t,
        0x4660fc29 as c_ulong as z_crc_t,
        0x710a3e28 as c_ulong as z_crc_t,
        0xf41c712d as c_ulong as z_crc_t,
        0xc376b32c as c_ulong as z_crc_t,
        0x9ac8f52e as c_ulong as z_crc_t,
        0xada2372f as c_ulong as z_crc_t,
        0xc08d9a70 as c_ulong as z_crc_t,
        0xf7e75871 as c_ulong as z_crc_t,
        0xae591e73 as c_ulong as z_crc_t,
        0x9933dc72 as c_ulong as z_crc_t,
        0x1c259377 as c_ulong as z_crc_t,
        0x2b4f5176 as c_ulong as z_crc_t,
        0x72f11774 as c_ulong as z_crc_t,
        0x459bd575 as c_ulong as z_crc_t,
        0x78dc897e as c_ulong as z_crc_t,
        0x4fb64b7f as c_ulong as z_crc_t,
        0x16080d7d as c_ulong as z_crc_t,
        0x2162cf7c as c_ulong as z_crc_t,
        0xa4748079 as c_ulong as z_crc_t,
        0x931e4278 as c_ulong as z_crc_t,
        0xcaa0047a as c_ulong as z_crc_t,
        0xfdcac67b as c_ulong as z_crc_t,
        0xb02ebc6c as c_ulong as z_crc_t,
        0x87447e6d as c_ulong as z_crc_t,
        0xdefa386f as c_ulong as z_crc_t,
        0xe990fa6e as c_ulong as z_crc_t,
        0x6c86b56b as c_ulong as z_crc_t,
        0x5bec776a as c_ulong as z_crc_t,
        0x2523168 as c_ulong as z_crc_t,
        0x3538f369 as c_ulong as z_crc_t,
        0x87faf62 as c_ulong as z_crc_t,
        0x3f156d63 as c_ulong as z_crc_t,
        0x66ab2b61 as c_ulong as z_crc_t,
        0x51c1e960 as c_ulong as z_crc_t,
        0xd4d7a665 as c_ulong as z_crc_t,
        0xe3bd6464 as c_ulong as z_crc_t,
        0xba032266 as c_ulong as z_crc_t,
        0x8d69e067 as c_ulong as z_crc_t,
        0x20cbd748 as c_ulong as z_crc_t,
        0x17a11549 as c_ulong as z_crc_t,
        0x4e1f534b as c_ulong as z_crc_t,
        0x7975914a as c_ulong as z_crc_t,
        0xfc63de4f as c_ulong as z_crc_t,
        0xcb091c4e as c_ulong as z_crc_t,
        0x92b75a4c as c_ulong as z_crc_t,
        0xa5dd984d as c_ulong as z_crc_t,
        0x989ac446 as c_ulong as z_crc_t,
        0xaff00647 as c_ulong as z_crc_t,
        0xf64e4045 as c_ulong as z_crc_t,
        0xc1248244 as c_ulong as z_crc_t,
        0x4432cd41 as c_ulong as z_crc_t,
        0x73580f40 as c_ulong as z_crc_t,
        0x2ae64942 as c_ulong as z_crc_t,
        0x1d8c8b43 as c_ulong as z_crc_t,
        0x5068f154 as c_ulong as z_crc_t,
        0x67023355 as c_ulong as z_crc_t,
        0x3ebc7557 as c_ulong as z_crc_t,
        0x9d6b756 as c_ulong as z_crc_t,
        0x8cc0f853 as c_ulong as z_crc_t,
        0xbbaa3a52 as c_ulong as z_crc_t,
        0xe2147c50 as c_ulong as z_crc_t,
        0xd57ebe51 as c_ulong as z_crc_t,
        0xe839e25a as c_ulong as z_crc_t,
        0xdf53205b as c_ulong as z_crc_t,
        0x86ed6659 as c_ulong as z_crc_t,
        0xb187a458 as c_ulong as z_crc_t,
        0x3491eb5d as c_ulong as z_crc_t,
        0x3fb295c as c_ulong as z_crc_t,
        0x5a456f5e as c_ulong as z_crc_t,
        0x6d2fad5f as c_ulong as z_crc_t,
        0x801b35e1 as c_ulong as z_crc_t,
        0xb771f7e0 as c_ulong as z_crc_t,
        0xeecfb1e2 as c_ulong as z_crc_t,
        0xd9a573e3 as c_ulong as z_crc_t,
        0x5cb33ce6 as c_ulong as z_crc_t,
        0x6bd9fee7 as c_ulong as z_crc_t,
        0x3267b8e5 as c_ulong as z_crc_t,
        0x50d7ae4 as c_ulong as z_crc_t,
        0x384a26ef as c_ulong as z_crc_t,
        0xf20e4ee as c_ulong as z_crc_t,
        0x569ea2ec as c_ulong as z_crc_t,
        0x61f460ed as c_ulong as z_crc_t,
        0xe4e22fe8 as c_ulong as z_crc_t,
        0xd388ede9 as c_ulong as z_crc_t,
        0x8a36abeb as c_ulong as z_crc_t,
        0xbd5c69ea as c_ulong as z_crc_t,
        0xf0b813fd as c_ulong as z_crc_t,
        0xc7d2d1fc as c_ulong as z_crc_t,
        0x9e6c97fe as c_ulong as z_crc_t,
        0xa90655ff as c_ulong as z_crc_t,
        0x2c101afa as c_ulong as z_crc_t,
        0x1b7ad8fb as c_ulong as z_crc_t,
        0x42c49ef9 as c_ulong as z_crc_t,
        0x75ae5cf8 as c_ulong as z_crc_t,
        0x48e900f3 as c_ulong as z_crc_t,
        0x7f83c2f2 as c_ulong as z_crc_t,
        0x263d84f0 as c_ulong as z_crc_t,
        0x115746f1 as c_ulong as z_crc_t,
        0x944109f4 as c_ulong as z_crc_t,
        0xa32bcbf5 as c_ulong as z_crc_t,
        0xfa958df7 as c_ulong as z_crc_t,
        0xcdff4ff6 as c_ulong as z_crc_t,
        0x605d78d9 as c_ulong as z_crc_t,
        0x5737bad8 as c_ulong as z_crc_t,
        0xe89fcda as c_ulong as z_crc_t,
        0x39e33edb as c_ulong as z_crc_t,
        0xbcf571de as c_ulong as z_crc_t,
        0x8b9fb3df as c_ulong as z_crc_t,
        0xd221f5dd as c_ulong as z_crc_t,
        0xe54b37dc as c_ulong as z_crc_t,
        0xd80c6bd7 as c_ulong as z_crc_t,
        0xef66a9d6 as c_ulong as z_crc_t,
        0xb6d8efd4 as c_ulong as z_crc_t,
        0x81b22dd5 as c_ulong as z_crc_t,
        0x4a462d0 as c_ulong as z_crc_t,
        0x33cea0d1 as c_ulong as z_crc_t,
        0x6a70e6d3 as c_ulong as z_crc_t,
        0x5d1a24d2 as c_ulong as z_crc_t,
        0x10fe5ec5 as c_ulong as z_crc_t,
        0x27949cc4 as c_ulong as z_crc_t,
        0x7e2adac6 as c_ulong as z_crc_t,
        0x494018c7 as c_ulong as z_crc_t,
        0xcc5657c2 as c_ulong as z_crc_t,
        0xfb3c95c3 as c_ulong as z_crc_t,
        0xa282d3c1 as c_ulong as z_crc_t,
        0x95e811c0 as c_ulong as z_crc_t,
        0xa8af4dcb as c_ulong as z_crc_t,
        0x9fc58fca as c_ulong as z_crc_t,
        0xc67bc9c8 as c_ulong as z_crc_t,
        0xf1110bc9 as c_ulong as z_crc_t,
        0x740744cc as c_ulong as z_crc_t,
        0x436d86cd as c_ulong as z_crc_t,
        0x1ad3c0cf as c_ulong as z_crc_t,
        0x2db902ce as c_ulong as z_crc_t,
        0x4096af91 as c_ulong as z_crc_t,
        0x77fc6d90 as c_ulong as z_crc_t,
        0x2e422b92 as c_ulong as z_crc_t,
        0x1928e993 as c_ulong as z_crc_t,
        0x9c3ea696 as c_ulong as z_crc_t,
        0xab546497 as c_ulong as z_crc_t,
        0xf2ea2295 as c_ulong as z_crc_t,
        0xc580e094 as c_ulong as z_crc_t,
        0xf8c7bc9f as c_ulong as z_crc_t,
        0xcfad7e9e as c_ulong as z_crc_t,
        0x9613389c as c_ulong as z_crc_t,
        0xa179fa9d as c_ulong as z_crc_t,
        0x246fb598 as c_ulong as z_crc_t,
        0x13057799 as c_ulong as z_crc_t,
        0x4abb319b as c_ulong as z_crc_t,
        0x7dd1f39a as c_ulong as z_crc_t,
        0x3035898d as c_ulong as z_crc_t,
        0x75f4b8c as c_ulong as z_crc_t,
        0x5ee10d8e as c_ulong as z_crc_t,
        0x698bcf8f as c_ulong as z_crc_t,
        0xec9d808a as c_ulong as z_crc_t,
        0xdbf7428b as c_ulong as z_crc_t,
        0x82490489 as c_ulong as z_crc_t,
        0xb523c688 as c_ulong as z_crc_t,
        0x88649a83 as c_ulong as z_crc_t,
        0xbf0e5882 as c_ulong as z_crc_t,
        0xe6b01e80 as c_ulong as z_crc_t,
        0xd1dadc81 as c_ulong as z_crc_t,
        0x54cc9384 as c_ulong as z_crc_t,
        0x63a65185 as c_ulong as z_crc_t,
        0x3a181787 as c_ulong as z_crc_t,
        0xd72d586 as c_ulong as z_crc_t,
        0xa0d0e2a9 as c_ulong as z_crc_t,
        0x97ba20a8 as c_ulong as z_crc_t,
        0xce0466aa as c_ulong as z_crc_t,
        0xf96ea4ab as c_ulong as z_crc_t,
        0x7c78ebae as c_ulong as z_crc_t,
        0x4b1229af as c_ulong as z_crc_t,
        0x12ac6fad as c_ulong as z_crc_t,
        0x25c6adac as c_ulong as z_crc_t,
        0x1881f1a7 as c_ulong as z_crc_t,
        0x2feb33a6 as c_ulong as z_crc_t,
        0x765575a4 as c_ulong as z_crc_t,
        0x413fb7a5 as c_ulong as z_crc_t,
        0xc429f8a0 as c_ulong as z_crc_t,
        0xf3433aa1 as c_ulong as z_crc_t,
        0xaafd7ca3 as c_ulong as z_crc_t,
        0x9d97bea2 as c_ulong as z_crc_t,
        0xd073c4b5 as c_ulong as z_crc_t,
        0xe71906b4 as c_ulong as z_crc_t,
        0xbea740b6 as c_ulong as z_crc_t,
        0x89cd82b7 as c_ulong as z_crc_t,
        0xcdbcdb2 as c_ulong as z_crc_t,
        0x3bb10fb3 as c_ulong as z_crc_t,
        0x620f49b1 as c_ulong as z_crc_t,
        0x55658bb0 as c_ulong as z_crc_t,
        0x6822d7bb as c_ulong as z_crc_t,
        0x5f4815ba as c_ulong as z_crc_t,
        0x6f653b8 as c_ulong as z_crc_t,
        0x319c91b9 as c_ulong as z_crc_t,
        0xb48adebc as c_ulong as z_crc_t,
        0x83e01cbd as c_ulong as z_crc_t,
        0xda5e5abf as c_ulong as z_crc_t,
        0xed3498be as c_ulong as z_crc_t,
    ],
    [
        0 as c_ulong as z_crc_t,
        0x6567bcb8 as c_ulong as z_crc_t,
        0x8bc809aa as c_ulong as z_crc_t,
        0xeeafb512 as c_ulong as z_crc_t,
        0x5797628f as c_ulong as z_crc_t,
        0x32f0de37 as c_ulong as z_crc_t,
        0xdc5f6b25 as c_ulong as z_crc_t,
        0xb938d79d as c_ulong as z_crc_t,
        0xef28b4c5 as c_ulong as z_crc_t,
        0x8a4f087d as c_ulong as z_crc_t,
        0x64e0bd6f as c_ulong as z_crc_t,
        0x18701d7 as c_ulong as z_crc_t,
        0xb8bfd64a as c_ulong as z_crc_t,
        0xddd86af2 as c_ulong as z_crc_t,
        0x3377dfe0 as c_ulong as z_crc_t,
        0x56106358 as c_ulong as z_crc_t,
        0x9f571950 as c_ulong as z_crc_t,
        0xfa30a5e8 as c_ulong as z_crc_t,
        0x149f10fa as c_ulong as z_crc_t,
        0x71f8ac42 as c_ulong as z_crc_t,
        0xc8c07bdf as c_ulong as z_crc_t,
        0xada7c767 as c_ulong as z_crc_t,
        0x43087275 as c_ulong as z_crc_t,
        0x266fcecd as c_ulong as z_crc_t,
        0x707fad95 as c_ulong as z_crc_t,
        0x1518112d as c_ulong as z_crc_t,
        0xfbb7a43f as c_ulong as z_crc_t,
        0x9ed01887 as c_ulong as z_crc_t,
        0x27e8cf1a as c_ulong as z_crc_t,
        0x428f73a2 as c_ulong as z_crc_t,
        0xac20c6b0 as c_ulong as z_crc_t,
        0xc9477a08 as c_ulong as z_crc_t,
        0x3eaf32a0 as c_ulong as z_crc_t,
        0x5bc88e18 as c_ulong as z_crc_t,
        0xb5673b0a as c_ulong as z_crc_t,
        0xd00087b2 as c_ulong as z_crc_t,
        0x6938502f as c_ulong as z_crc_t,
        0xc5fec97 as c_ulong as z_crc_t,
        0xe2f05985 as c_ulong as z_crc_t,
        0x8797e53d as c_ulong as z_crc_t,
        0xd1878665 as c_ulong as z_crc_t,
        0xb4e03add as c_ulong as z_crc_t,
        0x5a4f8fcf as c_ulong as z_crc_t,
        0x3f283377 as c_ulong as z_crc_t,
        0x8610e4ea as c_ulong as z_crc_t,
        0xe3775852 as c_ulong as z_crc_t,
        0xdd8ed40 as c_ulong as z_crc_t,
        0x68bf51f8 as c_ulong as z_crc_t,
        0xa1f82bf0 as c_ulong as z_crc_t,
        0xc49f9748 as c_ulong as z_crc_t,
        0x2a30225a as c_ulong as z_crc_t,
        0x4f579ee2 as c_ulong as z_crc_t,
        0xf66f497f as c_ulong as z_crc_t,
        0x9308f5c7 as c_ulong as z_crc_t,
        0x7da740d5 as c_ulong as z_crc_t,
        0x18c0fc6d as c_ulong as z_crc_t,
        0x4ed09f35 as c_ulong as z_crc_t,
        0x2bb7238d as c_ulong as z_crc_t,
        0xc518969f as c_ulong as z_crc_t,
        0xa07f2a27 as c_ulong as z_crc_t,
        0x1947fdba as c_ulong as z_crc_t,
        0x7c204102 as c_ulong as z_crc_t,
        0x928ff410 as c_ulong as z_crc_t,
        0xf7e848a8 as c_ulong as z_crc_t,
        0x3d58149b as c_ulong as z_crc_t,
        0x583fa823 as c_ulong as z_crc_t,
        0xb6901d31 as c_ulong as z_crc_t,
        0xd3f7a189 as c_ulong as z_crc_t,
        0x6acf7614 as c_ulong as z_crc_t,
        0xfa8caac as c_ulong as z_crc_t,
        0xe1077fbe as c_ulong as z_crc_t,
        0x8460c306 as c_ulong as z_crc_t,
        0xd270a05e as c_ulong as z_crc_t,
        0xb7171ce6 as c_ulong as z_crc_t,
        0x59b8a9f4 as c_ulong as z_crc_t,
        0x3cdf154c as c_ulong as z_crc_t,
        0x85e7c2d1 as c_ulong as z_crc_t,
        0xe0807e69 as c_ulong as z_crc_t,
        0xe2fcb7b as c_ulong as z_crc_t,
        0x6b4877c3 as c_ulong as z_crc_t,
        0xa20f0dcb as c_ulong as z_crc_t,
        0xc768b173 as c_ulong as z_crc_t,
        0x29c70461 as c_ulong as z_crc_t,
        0x4ca0b8d9 as c_ulong as z_crc_t,
        0xf5986f44 as c_ulong as z_crc_t,
        0x90ffd3fc as c_ulong as z_crc_t,
        0x7e5066ee as c_ulong as z_crc_t,
        0x1b37da56 as c_ulong as z_crc_t,
        0x4d27b90e as c_ulong as z_crc_t,
        0x284005b6 as c_ulong as z_crc_t,
        0xc6efb0a4 as c_ulong as z_crc_t,
        0xa3880c1c as c_ulong as z_crc_t,
        0x1ab0db81 as c_ulong as z_crc_t,
        0x7fd76739 as c_ulong as z_crc_t,
        0x9178d22b as c_ulong as z_crc_t,
        0xf41f6e93 as c_ulong as z_crc_t,
        0x3f7263b as c_ulong as z_crc_t,
        0x66909a83 as c_ulong as z_crc_t,
        0x883f2f91 as c_ulong as z_crc_t,
        0xed589329 as c_ulong as z_crc_t,
        0x546044b4 as c_ulong as z_crc_t,
        0x3107f80c as c_ulong as z_crc_t,
        0xdfa84d1e as c_ulong as z_crc_t,
        0xbacff1a6 as c_ulong as z_crc_t,
        0xecdf92fe as c_ulong as z_crc_t,
        0x89b82e46 as c_ulong as z_crc_t,
        0x67179b54 as c_ulong as z_crc_t,
        0x27027ec as c_ulong as z_crc_t,
        0xbb48f071 as c_ulong as z_crc_t,
        0xde2f4cc9 as c_ulong as z_crc_t,
        0x3080f9db as c_ulong as z_crc_t,
        0x55e74563 as c_ulong as z_crc_t,
        0x9ca03f6b as c_ulong as z_crc_t,
        0xf9c783d3 as c_ulong as z_crc_t,
        0x176836c1 as c_ulong as z_crc_t,
        0x720f8a79 as c_ulong as z_crc_t,
        0xcb375de4 as c_ulong as z_crc_t,
        0xae50e15c as c_ulong as z_crc_t,
        0x40ff544e as c_ulong as z_crc_t,
        0x2598e8f6 as c_ulong as z_crc_t,
        0x73888bae as c_ulong as z_crc_t,
        0x16ef3716 as c_ulong as z_crc_t,
        0xf8408204 as c_ulong as z_crc_t,
        0x9d273ebc as c_ulong as z_crc_t,
        0x241fe921 as c_ulong as z_crc_t,
        0x41785599 as c_ulong as z_crc_t,
        0xafd7e08b as c_ulong as z_crc_t,
        0xcab05c33 as c_ulong as z_crc_t,
        0x3bb659ed as c_ulong as z_crc_t,
        0x5ed1e555 as c_ulong as z_crc_t,
        0xb07e5047 as c_ulong as z_crc_t,
        0xd519ecff as c_ulong as z_crc_t,
        0x6c213b62 as c_ulong as z_crc_t,
        0x94687da as c_ulong as z_crc_t,
        0xe7e932c8 as c_ulong as z_crc_t,
        0x828e8e70 as c_ulong as z_crc_t,
        0xd49eed28 as c_ulong as z_crc_t,
        0xb1f95190 as c_ulong as z_crc_t,
        0x5f56e482 as c_ulong as z_crc_t,
        0x3a31583a as c_ulong as z_crc_t,
        0x83098fa7 as c_ulong as z_crc_t,
        0xe66e331f as c_ulong as z_crc_t,
        0x8c1860d as c_ulong as z_crc_t,
        0x6da63ab5 as c_ulong as z_crc_t,
        0xa4e140bd as c_ulong as z_crc_t,
        0xc186fc05 as c_ulong as z_crc_t,
        0x2f294917 as c_ulong as z_crc_t,
        0x4a4ef5af as c_ulong as z_crc_t,
        0xf3762232 as c_ulong as z_crc_t,
        0x96119e8a as c_ulong as z_crc_t,
        0x78be2b98 as c_ulong as z_crc_t,
        0x1dd99720 as c_ulong as z_crc_t,
        0x4bc9f478 as c_ulong as z_crc_t,
        0x2eae48c0 as c_ulong as z_crc_t,
        0xc001fdd2 as c_ulong as z_crc_t,
        0xa566416a as c_ulong as z_crc_t,
        0x1c5e96f7 as c_ulong as z_crc_t,
        0x79392a4f as c_ulong as z_crc_t,
        0x97969f5d as c_ulong as z_crc_t,
        0xf2f123e5 as c_ulong as z_crc_t,
        0x5196b4d as c_ulong as z_crc_t,
        0x607ed7f5 as c_ulong as z_crc_t,
        0x8ed162e7 as c_ulong as z_crc_t,
        0xebb6de5f as c_ulong as z_crc_t,
        0x528e09c2 as c_ulong as z_crc_t,
        0x37e9b57a as c_ulong as z_crc_t,
        0xd9460068 as c_ulong as z_crc_t,
        0xbc21bcd0 as c_ulong as z_crc_t,
        0xea31df88 as c_ulong as z_crc_t,
        0x8f566330 as c_ulong as z_crc_t,
        0x61f9d622 as c_ulong as z_crc_t,
        0x49e6a9a as c_ulong as z_crc_t,
        0xbda6bd07 as c_ulong as z_crc_t,
        0xd8c101bf as c_ulong as z_crc_t,
        0x366eb4ad as c_ulong as z_crc_t,
        0x53090815 as c_ulong as z_crc_t,
        0x9a4e721d as c_ulong as z_crc_t,
        0xff29cea5 as c_ulong as z_crc_t,
        0x11867bb7 as c_ulong as z_crc_t,
        0x74e1c70f as c_ulong as z_crc_t,
        0xcdd91092 as c_ulong as z_crc_t,
        0xa8beac2a as c_ulong as z_crc_t,
        0x46111938 as c_ulong as z_crc_t,
        0x2376a580 as c_ulong as z_crc_t,
        0x7566c6d8 as c_ulong as z_crc_t,
        0x10017a60 as c_ulong as z_crc_t,
        0xfeaecf72 as c_ulong as z_crc_t,
        0x9bc973ca as c_ulong as z_crc_t,
        0x22f1a457 as c_ulong as z_crc_t,
        0x479618ef as c_ulong as z_crc_t,
        0xa939adfd as c_ulong as z_crc_t,
        0xcc5e1145 as c_ulong as z_crc_t,
        0x6ee4d76 as c_ulong as z_crc_t,
        0x6389f1ce as c_ulong as z_crc_t,
        0x8d2644dc as c_ulong as z_crc_t,
        0xe841f864 as c_ulong as z_crc_t,
        0x51792ff9 as c_ulong as z_crc_t,
        0x341e9341 as c_ulong as z_crc_t,
        0xdab12653 as c_ulong as z_crc_t,
        0xbfd69aeb as c_ulong as z_crc_t,
        0xe9c6f9b3 as c_ulong as z_crc_t,
        0x8ca1450b as c_ulong as z_crc_t,
        0x620ef019 as c_ulong as z_crc_t,
        0x7694ca1 as c_ulong as z_crc_t,
        0xbe519b3c as c_ulong as z_crc_t,
        0xdb362784 as c_ulong as z_crc_t,
        0x35999296 as c_ulong as z_crc_t,
        0x50fe2e2e as c_ulong as z_crc_t,
        0x99b95426 as c_ulong as z_crc_t,
        0xfcdee89e as c_ulong as z_crc_t,
        0x12715d8c as c_ulong as z_crc_t,
        0x7716e134 as c_ulong as z_crc_t,
        0xce2e36a9 as c_ulong as z_crc_t,
        0xab498a11 as c_ulong as z_crc_t,
        0x45e63f03 as c_ulong as z_crc_t,
        0x208183bb as c_ulong as z_crc_t,
        0x7691e0e3 as c_ulong as z_crc_t,
        0x13f65c5b as c_ulong as z_crc_t,
        0xfd59e949 as c_ulong as z_crc_t,
        0x983e55f1 as c_ulong as z_crc_t,
        0x2106826c as c_ulong as z_crc_t,
        0x44613ed4 as c_ulong as z_crc_t,
        0xaace8bc6 as c_ulong as z_crc_t,
        0xcfa9377e as c_ulong as z_crc_t,
        0x38417fd6 as c_ulong as z_crc_t,
        0x5d26c36e as c_ulong as z_crc_t,
        0xb389767c as c_ulong as z_crc_t,
        0xd6eecac4 as c_ulong as z_crc_t,
        0x6fd61d59 as c_ulong as z_crc_t,
        0xab1a1e1 as c_ulong as z_crc_t,
        0xe41e14f3 as c_ulong as z_crc_t,
        0x8179a84b as c_ulong as z_crc_t,
        0xd769cb13 as c_ulong as z_crc_t,
        0xb20e77ab as c_ulong as z_crc_t,
        0x5ca1c2b9 as c_ulong as z_crc_t,
        0x39c67e01 as c_ulong as z_crc_t,
        0x80fea99c as c_ulong as z_crc_t,
        0xe5991524 as c_ulong as z_crc_t,
        0xb36a036 as c_ulong as z_crc_t,
        0x6e511c8e as c_ulong as z_crc_t,
        0xa7166686 as c_ulong as z_crc_t,
        0xc271da3e as c_ulong as z_crc_t,
        0x2cde6f2c as c_ulong as z_crc_t,
        0x49b9d394 as c_ulong as z_crc_t,
        0xf0810409 as c_ulong as z_crc_t,
        0x95e6b8b1 as c_ulong as z_crc_t,
        0x7b490da3 as c_ulong as z_crc_t,
        0x1e2eb11b as c_ulong as z_crc_t,
        0x483ed243 as c_ulong as z_crc_t,
        0x2d596efb as c_ulong as z_crc_t,
        0xc3f6dbe9 as c_ulong as z_crc_t,
        0xa6916751 as c_ulong as z_crc_t,
        0x1fa9b0cc as c_ulong as z_crc_t,
        0x7ace0c74 as c_ulong as z_crc_t,
        0x9461b966 as c_ulong as z_crc_t,
        0xf10605de as c_ulong as z_crc_t,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn get_crc_table() -> *const z_crc_t {
    return &raw const crc_table as *const [z_crc_t; 256] as *const z_crc_t;
}
#[no_mangle]
pub unsafe extern "C" fn crc32_z(
    mut crc: c_ulong,
    mut buf: *const c_uchar,
    mut len: z_size_t,
) -> uLong {
    if buf.is_null() {
        return 0 as uLong;
    }
    if ::core::mem::size_of::<*mut c_void>() as usize
        == ::core::mem::size_of::<ptrdiff_t>() as usize
    {
        let mut endian: z_crc_t = 0;
        endian = 1 as z_crc_t;
        if *(&raw mut endian as *mut c_uchar) != 0 {
            return crc32_little(crc, buf, len) as uLong;
        } else {
            return crc32_big(crc, buf, len) as uLong;
        }
    }
    crc = crc ^ 0xffffffff as c_ulong;
    while len >= 8 as z_size_t {
        let fresh0 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as c_int as usize][((crc as c_int
            ^ *fresh0 as c_int)
            & 0xff as c_int)
            as usize] as c_ulong
            ^ crc >> 8 as c_int;
        let fresh1 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as c_int as usize][((crc as c_int
            ^ *fresh1 as c_int)
            & 0xff as c_int)
            as usize] as c_ulong
            ^ crc >> 8 as c_int;
        let fresh2 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as c_int as usize][((crc as c_int
            ^ *fresh2 as c_int)
            & 0xff as c_int)
            as usize] as c_ulong
            ^ crc >> 8 as c_int;
        let fresh3 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as c_int as usize][((crc as c_int
            ^ *fresh3 as c_int)
            & 0xff as c_int)
            as usize] as c_ulong
            ^ crc >> 8 as c_int;
        let fresh4 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as c_int as usize][((crc as c_int
            ^ *fresh4 as c_int)
            & 0xff as c_int)
            as usize] as c_ulong
            ^ crc >> 8 as c_int;
        let fresh5 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as c_int as usize][((crc as c_int
            ^ *fresh5 as c_int)
            & 0xff as c_int)
            as usize] as c_ulong
            ^ crc >> 8 as c_int;
        let fresh6 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as c_int as usize][((crc as c_int
            ^ *fresh6 as c_int)
            & 0xff as c_int)
            as usize] as c_ulong
            ^ crc >> 8 as c_int;
        let fresh7 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as c_int as usize][((crc as c_int
            ^ *fresh7 as c_int)
            & 0xff as c_int)
            as usize] as c_ulong
            ^ crc >> 8 as c_int;
        len = (len as c_ulong).wrapping_sub(8 as c_ulong) as z_size_t
            as z_size_t;
    }
    if len != 0 {
        loop {
            let fresh8 = buf;
            buf = buf.offset(1);
            crc = crc_table[0 as c_int as usize][((crc as c_int
                ^ *fresh8 as c_int)
                & 0xff as c_int)
                as usize] as c_ulong
                ^ crc >> 8 as c_int;
            len = len.wrapping_sub(1);
            if !(len != 0) {
                break;
            }
        }
    }
    return crc as uLong ^ 0xffffffff as uLong;
}
#[no_mangle]
pub unsafe extern "C" fn crc32(
    mut crc: c_ulong,
    mut buf: *const c_uchar,
    mut len: uInt,
) -> uLong {
    return crc32_z(crc, buf, len as z_size_t);
}
unsafe extern "C" fn crc32_little(
    mut crc: c_ulong,
    mut buf: *const c_uchar,
    mut len: z_size_t,
) -> c_ulong {
    let mut c: z_crc_t = 0;
    let mut buf4: *const z_crc_t = ::core::ptr::null::<z_crc_t>();
    c = crc as z_crc_t;
    c = !c;
    while len != 0 && buf as ptrdiff_t & 3 as ptrdiff_t != 0 {
        let fresh20 = buf;
        buf = buf.offset(1);
        c = crc_table[0 as c_int as usize][((c as c_uint
            ^ *fresh20 as c_uint)
            & 0xff as c_uint)
            as usize]
            ^ c >> 8 as c_int;
        len = len.wrapping_sub(1);
    }
    buf4 = buf as *const c_void as *const z_crc_t;
    while len >= 32 as z_size_t {
        let fresh21 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh21 as c_uint;
        c = crc_table[3 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[2 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[1 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[0 as c_int as usize][(c >> 24 as c_int) as usize];
        let fresh22 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh22 as c_uint;
        c = crc_table[3 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[2 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[1 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[0 as c_int as usize][(c >> 24 as c_int) as usize];
        let fresh23 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh23 as c_uint;
        c = crc_table[3 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[2 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[1 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[0 as c_int as usize][(c >> 24 as c_int) as usize];
        let fresh24 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh24 as c_uint;
        c = crc_table[3 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[2 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[1 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[0 as c_int as usize][(c >> 24 as c_int) as usize];
        let fresh25 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh25 as c_uint;
        c = crc_table[3 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[2 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[1 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[0 as c_int as usize][(c >> 24 as c_int) as usize];
        let fresh26 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh26 as c_uint;
        c = crc_table[3 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[2 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[1 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[0 as c_int as usize][(c >> 24 as c_int) as usize];
        let fresh27 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh27 as c_uint;
        c = crc_table[3 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[2 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[1 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[0 as c_int as usize][(c >> 24 as c_int) as usize];
        let fresh28 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh28 as c_uint;
        c = crc_table[3 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[2 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[1 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[0 as c_int as usize][(c >> 24 as c_int) as usize];
        len = (len as c_ulong).wrapping_sub(32 as c_ulong) as z_size_t
            as z_size_t;
    }
    while len >= 4 as z_size_t {
        let fresh29 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh29 as c_uint;
        c = crc_table[3 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[2 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[1 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[0 as c_int as usize][(c >> 24 as c_int) as usize];
        len = (len as c_ulong).wrapping_sub(4 as c_ulong) as z_size_t
            as z_size_t;
    }
    buf = buf4 as *const c_uchar;
    if len != 0 {
        loop {
            let fresh30 = buf;
            buf = buf.offset(1);
            c = crc_table[0 as c_int as usize][((c as c_uint
                ^ *fresh30 as c_uint)
                & 0xff as c_uint)
                as usize]
                ^ c >> 8 as c_int;
            len = len.wrapping_sub(1);
            if !(len != 0) {
                break;
            }
        }
    }
    c = !c;
    return c as c_ulong;
}
unsafe extern "C" fn crc32_big(
    mut crc: c_ulong,
    mut buf: *const c_uchar,
    mut len: z_size_t,
) -> c_ulong {
    let mut c: z_crc_t = 0;
    let mut buf4: *const z_crc_t = ::core::ptr::null::<z_crc_t>();
    c = (crc as c_uint >> 24 as c_int & 0xff as c_uint)
        .wrapping_add(
            crc as c_uint >> 8 as c_int & 0xff00 as c_uint,
        )
        .wrapping_add(
            (crc as c_uint & 0xff00 as c_uint) << 8 as c_int,
        )
        .wrapping_add(
            (crc as c_uint & 0xff as c_uint) << 24 as c_int,
        ) as z_crc_t;
    c = !c;
    while len != 0 && buf as ptrdiff_t & 3 as ptrdiff_t != 0 {
        let fresh9 = buf;
        buf = buf.offset(1);
        c = crc_table[4 as c_int as usize][(c as c_uint
            >> 24 as c_int
            ^ *fresh9 as c_uint)
            as usize]
            ^ c << 8 as c_int;
        len = len.wrapping_sub(1);
    }
    buf4 = buf as *const c_void as *const z_crc_t;
    while len >= 32 as z_size_t {
        let fresh10 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh10 as c_uint;
        c = crc_table[4 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[5 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[6 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[7 as c_int as usize][(c >> 24 as c_int) as usize];
        let fresh11 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh11 as c_uint;
        c = crc_table[4 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[5 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[6 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[7 as c_int as usize][(c >> 24 as c_int) as usize];
        let fresh12 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh12 as c_uint;
        c = crc_table[4 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[5 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[6 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[7 as c_int as usize][(c >> 24 as c_int) as usize];
        let fresh13 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh13 as c_uint;
        c = crc_table[4 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[5 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[6 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[7 as c_int as usize][(c >> 24 as c_int) as usize];
        let fresh14 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh14 as c_uint;
        c = crc_table[4 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[5 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[6 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[7 as c_int as usize][(c >> 24 as c_int) as usize];
        let fresh15 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh15 as c_uint;
        c = crc_table[4 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[5 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[6 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[7 as c_int as usize][(c >> 24 as c_int) as usize];
        let fresh16 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh16 as c_uint;
        c = crc_table[4 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[5 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[6 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[7 as c_int as usize][(c >> 24 as c_int) as usize];
        let fresh17 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh17 as c_uint;
        c = crc_table[4 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[5 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[6 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[7 as c_int as usize][(c >> 24 as c_int) as usize];
        len = (len as c_ulong).wrapping_sub(32 as c_ulong) as z_size_t
            as z_size_t;
    }
    while len >= 4 as z_size_t {
        let fresh18 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh18 as c_uint;
        c = crc_table[4 as c_int as usize]
            [(c as c_uint & 0xff as c_uint) as usize]
            ^ crc_table[5 as c_int as usize][(c as c_uint
                >> 8 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[6 as c_int as usize][(c as c_uint
                >> 16 as c_int
                & 0xff as c_uint)
                as usize]
            ^ crc_table[7 as c_int as usize][(c >> 24 as c_int) as usize];
        len = (len as c_ulong).wrapping_sub(4 as c_ulong) as z_size_t
            as z_size_t;
    }
    buf = buf4 as *const c_uchar;
    if len != 0 {
        loop {
            let fresh19 = buf;
            buf = buf.offset(1);
            c = crc_table[4 as c_int as usize][(c as c_uint
                >> 24 as c_int
                ^ *fresh19 as c_uint)
                as usize]
                ^ c << 8 as c_int;
            len = len.wrapping_sub(1);
            if !(len != 0) {
                break;
            }
        }
    }
    c = !c;
    return (c as c_uint >> 24 as c_int & 0xff as c_uint)
        .wrapping_add(
            c as c_uint >> 8 as c_int & 0xff00 as c_uint,
        )
        .wrapping_add(
            (c as c_uint & 0xff00 as c_uint) << 8 as c_int,
        )
        .wrapping_add(
            (c as c_uint & 0xff as c_uint) << 24 as c_int,
        ) as c_ulong;
}
pub const GF2_DIM: c_int = 32 as c_int;
unsafe extern "C" fn gf2_matrix_times(
    mut mat: *mut c_ulong,
    mut vec: c_ulong,
) -> c_ulong {
    let mut sum: c_ulong = 0;
    sum = 0 as c_ulong;
    while vec != 0 {
        if vec & 1 as c_ulong != 0 {
            sum ^= *mat;
        }
        vec >>= 1 as c_int;
        mat = mat.offset(1);
    }
    return sum;
}
unsafe extern "C" fn gf2_matrix_square(
    mut square: *mut c_ulong,
    mut mat: *mut c_ulong,
) {
    let mut n: c_int = 0;
    n = 0 as c_int;
    while n < GF2_DIM {
        *square.offset(n as isize) = gf2_matrix_times(mat, *mat.offset(n as isize));
        n += 1;
    }
}
unsafe extern "C" fn crc32_combine_(mut crc1: uLong, mut crc2: uLong, mut len2: off_t) -> uLong {
    let mut n: c_int = 0;
    let mut row: c_ulong = 0;
    let mut even: [c_ulong; 32] = [0; 32];
    let mut odd: [c_ulong; 32] = [0; 32];
    if len2 <= 0 as c_long {
        return crc1;
    }
    odd[0 as c_int as usize] = 0xedb88320 as c_ulong;
    row = 1 as c_ulong;
    n = 1 as c_int;
    while n < GF2_DIM {
        odd[n as usize] = row;
        row <<= 1 as c_int;
        n += 1;
    }
    gf2_matrix_square(
        &raw mut even as *mut c_ulong,
        &raw mut odd as *mut c_ulong,
    );
    gf2_matrix_square(
        &raw mut odd as *mut c_ulong,
        &raw mut even as *mut c_ulong,
    );
    loop {
        gf2_matrix_square(
            &raw mut even as *mut c_ulong,
            &raw mut odd as *mut c_ulong,
        );
        if len2 as c_long & 1 as c_long != 0 {
            crc1 = gf2_matrix_times(
                &raw mut even as *mut c_ulong,
                crc1 as c_ulong,
            ) as uLong;
        }
        len2 >>= 1 as c_int;
        if len2 == 0 as c_long {
            break;
        }
        gf2_matrix_square(
            &raw mut odd as *mut c_ulong,
            &raw mut even as *mut c_ulong,
        );
        if len2 as c_long & 1 as c_long != 0 {
            crc1 = gf2_matrix_times(
                &raw mut odd as *mut c_ulong,
                crc1 as c_ulong,
            ) as uLong;
        }
        len2 >>= 1 as c_int;
        if !(len2 != 0 as c_long) {
            break;
        }
    }
    crc1 ^= crc2 as c_ulong;
    return crc1;
}
#[no_mangle]
pub unsafe extern "C" fn crc32_combine(mut crc1: uLong, mut crc2: uLong, mut len2: off_t) -> uLong {
    return crc32_combine_(crc1, crc2, len2);
}
#[no_mangle]
pub unsafe extern "C" fn crc32_combine64(
    mut crc1: uLong,
    mut crc2: uLong,
    mut len2: off_t,
) -> uLong {
    return crc32_combine_(crc1, crc2, len2);
}
