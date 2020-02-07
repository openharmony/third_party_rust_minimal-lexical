//! Cached exponents for basen values with 80-bit extended floats.
//!
//! Exact versions of base**n as an extended-precision float, with both
//! large and small powers. Use the large powers to minimize the amount
//! of compounded error.
//!
//! These values were calculated using Python, using the arbitrary-precision
//! integer to calculate exact extended-representation of each value.
//! These values are all normalized.
//!
//! This files takes ~ 26KB of storage.
//!
//! This file is mostly automatically generated, do not change values
//! manually, unless you know what you are doing. The script to generate
//! the values is as follows:
//!
//! ```text
//! import math
//! from collections import deque
//!
//! STEP_STR = "const BASE{0}_STEP: i32 = {1};"
//! SMALL_MANTISSA_STR = "const BASE{0}_SMALL_MANTISSA: [u64; {1}] = ["
//! SMALL_EXPONENT_STR = "const BASE{0}_SMALL_EXPONENT: [i32; {1}] = ["
//! LARGE_MANTISSA_STR = "const BASE{0}_LARGE_MANTISSA: [u64; {1}] = ["
//! LARGE_EXPONENT_STR = "const BASE{0}_LARGE_EXPONENT: [i32; {1}] = ["
//! SMALL_INT_STR = "const BASE{0}_SMALL_INT_POWERS: [u64; {1}] = {2};"
//! BIAS_STR = "const BASE{0}_BIAS: i32 = {1};"
//! EXP_STR = "// {}^{}"
//! POWER_STR = """pub(crate) const BASE{0}_POWERS: ModeratePathPowers<u64> = ModeratePathPowers {{
//!     small: ExtendedFloatArray {{ mant: &BASE{0}_SMALL_MANTISSA, exp: &BASE{0}_SMALL_EXPONENT }},
//!     large: ExtendedFloatArray {{ mant: &BASE{0}_LARGE_MANTISSA, exp: &BASE{0}_LARGE_EXPONENT }},
//!     small_int: &BASE{0}_SMALL_INT_POWERS,
//!     step: BASE{0}_STEP,
//!     bias: BASE{0}_BIAS,
//! }};\n"""
//!
//! def calculate_bitshift(base, exponent):
//!     '''
//!     Calculate the bitshift required for a given base. The exponent
//!     is the absolute value of the max exponent (log distance from 1.)
//!     '''
//!
//!     return 63 + math.ceil(math.log2(base**exponent))
//!
//!
//! def next_fp(fp, base, step = 1):
//!     '''Generate the next extended-floating point value.'''
//!
//!     return (fp[0] * (base**step), fp[1])
//!
//!
//! def prev_fp(fp, base, step = 1):
//!     '''Generate the previous extended-floating point value.'''
//!
//!     return (fp[0] // (base**step), fp[1])
//!
//!
//! def normalize_fp(fp):
//!     '''Normalize a extended-float so the MSB is the 64th bit'''
//!
//!     while fp[0] >> 64 != 0:
//!         fp = (fp[0] >> 1, fp[1] + 1)
//!     return fp
//!
//!
//! def generate_small(base, count):
//!     '''Generate the small powers for a given base'''
//!
//!     bitshift = calculate_bitshift(base, count)
//!     fps = []
//!     fp = (1 << bitshift, -bitshift)
//!     for exp in range(count):
//!         fps.append((normalize_fp(fp), exp))
//!         fp = next_fp(fp, base)
//!
//!     # Print the small powers as integers.
//!     ints = [base**i for _, i in fps]
//!
//!     return fps, ints
//!
//!
//! def generate_large(base, step):
//!     '''Generate the large powers for a given base.'''
//!
//!     # Get our starting parameters
//!     min_exp = math.floor(math.log(5e-324, base) - math.log(0xFFFFFFFFFFFFFFFF, base))
//!     max_exp = math.ceil(math.log(1.7976931348623157e+308, base))
//!     bitshift = calculate_bitshift(base, abs(min_exp - step))
//!     fps = deque()
//!
//!     # Add negative exponents
//!     # We need to go below the minimum exponent, since we need
//!     # all resulting exponents to be positive.
//!     fp = (1 << bitshift, -bitshift)
//!     for exp in range(-step, min_exp-step, -step):
//!         fp = prev_fp(fp, base, step)
//!         fps.appendleft((normalize_fp(fp), exp))
//!
//!     # Add positive exponents
//!     fp = (1 << bitshift, -bitshift)
//!     fps.append((normalize_fp(fp), 0))
//!     for exp in range(step, max_exp, step):
//!         fp = next_fp(fp, base, step)
//!         fps.append((normalize_fp(fp), exp))
//!
//!     # Return the smallest exp, AKA, the bias
//!     return fps, -fps[0][1]
//!
//!
//! def print_array(base, string, fps, index):
//!     '''Print an entire array'''
//!
//!     print(string.format(base, len(fps)))
//!     for fp, exp in fps:
//!         value = "    {},".format(fp[index])
//!         exp = EXP_STR.format(base, exp)
//!         print(value.ljust(30, " ") + exp)
//!     print("];")
//!
//!
//! def generate_base(base):
//!     '''Generate all powers and variables.'''
//!
//!     step = math.floor(math.log(1e10, base))
//!     small, ints = generate_small(base, step)
//!     large, bias = generate_large(base, step)
//!
//!     print_array(base, SMALL_MANTISSA_STR, small, 0)
//!     print_array(base, SMALL_EXPONENT_STR, small, 1)
//!     print_array(base, LARGE_MANTISSA_STR, large, 0)
//!     print_array(base, LARGE_EXPONENT_STR, large, 1)
//!     print(SMALL_INT_STR.format(base, len(ints), ints))
//!     print(STEP_STR.format(base, step))
//!     print(BIAS_STR.format(base, bias))
//!
//!
//! def generate():
//!     '''Generate all bases.'''
//!
//!     bases = [
//!         3, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21,
//!         22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36
//!     ]
//!
//!     for base in bases:
//!         print("// BASE{}\n".format(base))
//!         generate_base(base)
//!         print("")
//!
//!     print("// HIGH LEVEL\n// ----------\n")
//!
//!     for base in bases:
//!         print(POWER_STR.format(base))
//!
//!
//! if __name__ == '__main__':
//!     generate()
//! ```

use super::cached::{ExtendedFloatArray, ModeratePathPowers};

// LOW-LEVEL
// ---------

// BASE10

const BASE10_SMALL_MANTISSA: [u64; 10] = [
    9223372036854775808,      // 10^0
    11529215046068469760,     // 10^1
    14411518807585587200,     // 10^2
    18014398509481984000,     // 10^3
    11258999068426240000,     // 10^4
    14073748835532800000,     // 10^5
    17592186044416000000,     // 10^6
    10995116277760000000,     // 10^7
    13743895347200000000,     // 10^8
    17179869184000000000,     // 10^9
];
const BASE10_SMALL_EXPONENT: [i32; 10] = [
    -63,                      // 10^0
    -60,                      // 10^1
    -57,                      // 10^2
    -54,                      // 10^3
    -50,                      // 10^4
    -47,                      // 10^5
    -44,                      // 10^6
    -40,                      // 10^7
    -37,                      // 10^8
    -34,                      // 10^9
];
const BASE10_LARGE_MANTISSA: [u64; 66] = [
    11555125961253852697,     // 10^-350
    13451937075301367670,     // 10^-340
    15660115838168849784,     // 10^-330
    18230774251475056848,     // 10^-320
    10611707258198326947,     // 10^-310
    12353653155963782858,     // 10^-300
    14381545078898527261,     // 10^-290
    16742321987285426889,     // 10^-280
    9745314011399999080,      // 10^-270
    11345038669416679861,     // 10^-260
    13207363278391631158,     // 10^-250
    15375394465392026070,     // 10^-240
    17899314949046850752,     // 10^-230
    10418772551374772303,     // 10^-220
    12129047596099288555,     // 10^-210
    14120069793541087484,     // 10^-200
    16437924692338667210,     // 10^-190
    9568131466127621947,      // 10^-180
    11138771039116687545,     // 10^-170
    12967236152753102995,     // 10^-160
    15095849699286165408,     // 10^-150
    17573882009934360870,     // 10^-140
    10229345649675443343,     // 10^-130
    11908525658859223294,     // 10^-120
    13863348470604074297,     // 10^-110
    16139061738043178685,     // 10^-100
    9394170331095332911,      // 10^-90
    10936253623915059621,     // 10^-80
    12731474852090538039,     // 10^-70
    14821387422376473014,     // 10^-60
    17254365866976409468,     // 10^-50
    10043362776618689222,     // 10^-40
    11692013098647223345,     // 10^-30
    13611294676837538538,     // 10^-20
    15845632502852867518,     // 10^-10
    9223372036854775808,      // 10^0
    10737418240000000000,     // 10^10
    12500000000000000000,     // 10^20
    14551915228366851806,     // 10^30
    16940658945086006781,     // 10^40
    9860761315262647567,      // 10^50
    11479437019748901445,     // 10^60
    13363823550460978230,     // 10^70
    15557538194652854267,     // 10^80
    18111358157653424735,     // 10^90
    10542197943230523224,     // 10^100
    12272733663244316382,     // 10^110
    14287342391028437277,     // 10^120
    16632655625031838749,     // 10^130
    9681479787123295682,      // 10^140
    11270725851789228247,     // 10^150
    13120851772591970218,     // 10^160
    15274681817498023410,     // 10^170
    17782069995880619867,     // 10^180
    10350527006597618960,     // 10^190
    12049599325514420588,     // 10^200
    14027579833653779454,     // 10^210
    16330252207878254650,     // 10^220
    9505457831475799117,      // 10^230
    11065809325636130661,     // 10^240
    12882297539194266616,     // 10^250
    14996968138956309548,     // 10^260
    17458768723248864463,     // 10^270
    10162340898095201970,     // 10^280
    11830521861667747109,     // 10^290
    13772540099066387756,     // 10^300
];
const BASE10_LARGE_EXPONENT: [i32; 66] = [
    -1226,                    // 10^-350
    -1193,                    // 10^-340
    -1160,                    // 10^-330
    -1127,                    // 10^-320
    -1093,                    // 10^-310
    -1060,                    // 10^-300
    -1027,                    // 10^-290
    -994,                     // 10^-280
    -960,                     // 10^-270
    -927,                     // 10^-260
    -894,                     // 10^-250
    -861,                     // 10^-240
    -828,                     // 10^-230
    -794,                     // 10^-220
    -761,                     // 10^-210
    -728,                     // 10^-200
    -695,                     // 10^-190
    -661,                     // 10^-180
    -628,                     // 10^-170
    -595,                     // 10^-160
    -562,                     // 10^-150
    -529,                     // 10^-140
    -495,                     // 10^-130
    -462,                     // 10^-120
    -429,                     // 10^-110
    -396,                     // 10^-100
    -362,                     // 10^-90
    -329,                     // 10^-80
    -296,                     // 10^-70
    -263,                     // 10^-60
    -230,                     // 10^-50
    -196,                     // 10^-40
    -163,                     // 10^-30
    -130,                     // 10^-20
    -97,                      // 10^-10
    -63,                      // 10^0
    -30,                      // 10^10
    3,                        // 10^20
    36,                       // 10^30
    69,                       // 10^40
    103,                      // 10^50
    136,                      // 10^60
    169,                      // 10^70
    202,                      // 10^80
    235,                      // 10^90
    269,                      // 10^100
    302,                      // 10^110
    335,                      // 10^120
    368,                      // 10^130
    402,                      // 10^140
    435,                      // 10^150
    468,                      // 10^160
    501,                      // 10^170
    534,                      // 10^180
    568,                      // 10^190
    601,                      // 10^200
    634,                      // 10^210
    667,                      // 10^220
    701,                      // 10^230
    734,                      // 10^240
    767,                      // 10^250
    800,                      // 10^260
    833,                      // 10^270
    867,                      // 10^280
    900,                      // 10^290
    933,                      // 10^300
];
const BASE10_SMALL_INT_POWERS: [u64; 10] = [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];
const BASE10_STEP: i32 = 10;
const BASE10_BIAS: i32 = 350;

// HIGH LEVEL
// ----------

pub(crate) const BASE10_POWERS: ModeratePathPowers = ModeratePathPowers {
    small: ExtendedFloatArray { mant: &BASE10_SMALL_MANTISSA, exp: &BASE10_SMALL_EXPONENT },
    large: ExtendedFloatArray { mant: &BASE10_LARGE_MANTISSA, exp: &BASE10_LARGE_EXPONENT },
    small_int: &BASE10_SMALL_INT_POWERS,
    step: BASE10_STEP,
    bias: BASE10_BIAS,
};

/// Get powers from base.
pub(crate) fn get_powers() -> &'static ModeratePathPowers {
    &BASE10_POWERS
}
