//!
//! Maximum sequence length [Galois LFSRs](https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Galois_LFSRs), up to 32 bits wide.
//!
//! Taps taken from [https://web.archive.org/web/20161007061934/http://courses.cse.tamu.edu/csce680/walker/lfsr_table.pdf]
//!

use lfsr_base::LFSR;
use lfsr_macro_generate::galois_lfsr;

galois_lfsr!(Galois32, 32, 4294967295, 32, 30, 26, 25);
galois_lfsr!(Galois31, 31, 2147483647, 31, 28);
galois_lfsr!(Galois30, 30, 1073741823, 30, 29, 26, 24);
galois_lfsr!(Galois29, 29, 536870911, 29, 27);
galois_lfsr!(Galois28, 28, 268435455, 28, 25);
galois_lfsr!(Galois27, 27, 134217727, 27, 26, 25, 22);
galois_lfsr!(Galois26, 26, 67108863, 26, 25, 24, 20);
galois_lfsr!(Galois25, 25, 33554431, 25, 22);
galois_lfsr!(Galois24, 24, 16777215, 24, 23, 21, 20);
galois_lfsr!(Galois23, 23, 8388607, 23, 18);
galois_lfsr!(Galois22, 22, 4194303, 22, 21);
galois_lfsr!(Galois21, 21, 2097151, 21, 19);
galois_lfsr!(Galois20, 20, 1048575, 20, 17);
galois_lfsr!(Galois19, 19, 524287, 19, 18, 17, 14);
galois_lfsr!(Galois18, 18, 262143, 18, 11);
galois_lfsr!(Galois17, 17, 131071, 17, 14);
galois_lfsr!(Galois16, 16, 65535, 16, 14, 13, 11);
galois_lfsr!(Galois15, 15, 32767, 15, 14);
galois_lfsr!(Galois14, 14, 16383, 14, 13, 11, 9);
galois_lfsr!(Galois13, 13, 8191, 13, 12, 10, 9);
galois_lfsr!(Galois12, 12, 4095, 12, 11, 8, 6);
galois_lfsr!(Galois11, 11, 2047, 11, 9);
galois_lfsr!(Galois10, 10, 1023, 10, 7);
galois_lfsr!(Galois9, 9, 511, 9, 5);
galois_lfsr!(Galois8, 8, 255, 8, 6, 5, 4);
galois_lfsr!(Galois7, 7, 127, 7, 6);
galois_lfsr!(Galois6, 6, 63, 6, 5);
galois_lfsr!(Galois5, 5, 31, 5, 3);
galois_lfsr!(Galois4, 4, 15, 4, 3);
galois_lfsr!(Galois3, 3, 7, 3, 2);
galois_lfsr!(Galois2, 2, 3, 2, 1);
