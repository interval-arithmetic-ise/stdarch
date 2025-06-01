#[cfg(test)]
use stdarch_test::assert_instr;

// FIXME: Using #[target_feature(enable = "xfintx")] gerenates
// a stub, and does not inline the intrinsic. Moreover, it sometimes manages to
// inline code, but the behavior not deterministic, and the behavior changes
// with every compiler build.

unsafe extern "unadjusted" {
    #[link_name = "llvm.riscv.intadd.s"]
    fn _intadd_s(rs1: f64, rs2: f64) -> f64;

    #[link_name = "llvm.riscv.intsub.s"]
    fn _intsub_s(rs1: f64, rs2: f64) -> f64;

    #[link_name = "llvm.riscv.intmul.s"]
    fn _intmul_s(rs1: f64, rs2: f64) -> f64;

    #[link_name = "llvm.riscv.intdiv.s"]
    fn _intdiv_s(rs1: f64, rs2: f64) -> f64;

    #[link_name = "llvm.riscv.intmid.s"]
    fn _intmid_s(rs1: f64) -> f32;

    #[link_name = "llvm.riscv.intwdt.s"]
    fn _intwdt_s(rs1: f64) -> f32;

    #[link_name = "llvm.riscv.intrad.s"]
    fn _intrad_s(rs1: f64) -> f32;

    #[link_name = "llvm.riscv.intmin.s"]
    fn _intmin_s(rs1: f64) -> f32;

    #[link_name = "llvm.riscv.intmax.s"]
    fn _intmax_s(rs1: f64) -> f32;

    #[link_name = "llvm.riscv.intbfr.s"]
    fn _intbfr_s(rs1: f64, rs2: f64) -> i32;

    #[link_name = "llvm.riscv.intmts.s"]
    fn _intmts_s(rs1: f64, rs2: f64) -> i32;

    #[link_name = "llvm.riscv.intovr.s"]
    fn _intovr_s(rs1: f64, rs2: f64) -> i32;

    #[link_name = "llvm.riscv.intdur.s"]
    fn _intdur_s(rs1: f64, rs2: f64) -> i32;

    #[link_name = "llvm.riscv.intstr.s"]
    fn _intstr_s(rs1: f64, rs2: f64) -> i32;

    #[link_name = "llvm.riscv.intfin.s"]
    fn _intfin_s(rs1: f64, rs2: f64) -> i32;

    #[link_name = "llvm.riscv.inteql.s"]
    fn _inteql_s(rs1: f64, rs2: f64) -> i32;
}

unsafe extern "unadjusted" {
    #[link_name = "llvm.riscv.intadd.h"]
    fn _intadd_h(rs1: f32, rs2: f32) -> f32;

    #[link_name = "llvm.riscv.intsub.h"]
    fn _intsub_h(rs1: f32, rs2: f32) -> f32;

    #[link_name = "llvm.riscv.intmul.h"]
    fn _intmul_h(rs1: f32, rs2: f32) -> f32;

    #[link_name = "llvm.riscv.intdiv.h"]
    fn _intdiv_h(rs1: f32, rs2: f32) -> f32;

    #[link_name = "llvm.riscv.intmid.h"]
    fn _intmid_h(rs1: f32) -> f16;

    #[link_name = "llvm.riscv.intwdt.h"]
    fn _intwdt_h(rs1: f32) -> f16;

    #[link_name = "llvm.riscv.intrad.h"]
    fn _intrad_h(rs1: f32) -> f16;

    #[link_name = "llvm.riscv.intmin.h"]
    fn _intmin_h(rs1: f32) -> f16;

    #[link_name = "llvm.riscv.intmax.h"]
    fn _intmax_h(rs1: f32) -> f16;

    #[link_name = "llvm.riscv.intbfr.h"]
    fn _intbfr_h(rs1: f32, rs2: f32) -> i32;

    #[link_name = "llvm.riscv.intmts.h"]
    fn _intmts_h(rs1: f32, rs2: f32) -> i32;

    #[link_name = "llvm.riscv.intovr.h"]
    fn _intovr_h(rs1: f32, rs2: f32) -> i32;

    #[link_name = "llvm.riscv.intdur.h"]
    fn _intdur_h(rs1: f32, rs2: f32) -> i32;

    #[link_name = "llvm.riscv.intstr.h"]
    fn _intstr_h(rs1: f32, rs2: f32) -> i32;

    #[link_name = "llvm.riscv.intfin.h"]
    fn _intfin_h(rs1: f32, rs2: f32) -> i32;

    #[link_name = "llvm.riscv.inteql.h"]
    fn _inteql_h(rs1: f32, rs2: f32) -> i32;
}

/// Performs interval addition.
///
/// Computes the sum of two intervals according to IEEE 1788.1.
///
/// Instruction: **INTADD.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(intadd.s))]
#[inline]
pub unsafe fn intadd_s(rs1: f64, rs2: f64) -> f64 {
    _intadd_s(rs1, rs2)
}

/// Performs interval subtraction.
///
/// Computes the difference of two intervals according to IEEE 1788.1.
///
/// Instruction: **INTSUB.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(intsub.s))]
#[inline]
pub unsafe fn intsub_s(rs1: f64, rs2: f64) -> f64 {
    _intsub_s(rs1, rs2)
}

/// Performs interval multiplication.
///
/// Computes the product of two intervals following IEEE 1788.1.
///
/// Instruction: **INTMUL.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(intmul.s))]
#[inline]
pub unsafe fn intmul_s(rs1: f64, rs2: f64) -> f64 {
    _intmul_s(rs1, rs2)
}

/// Performs interval division.
///
/// Computes the quotient of two intervals as per IEEE 1788.1. Division by an interval
/// containing zero will raise an exception flag.
///
/// Instruction: **INTDIV.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(intdiv.s))]
#[inline]
pub unsafe fn intdiv_s(rs1: f64, rs2: f64) -> f64 {
    _intdiv_s(rs1, rs2)
}

/// Computes the midpoint of an interval.
///
/// Calculates the average of the lower and upper bounds.
///
/// Instruction: **INTMID.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(intmid.s))]
#[inline]
pub unsafe fn intmid_s(rs1: f64) -> f32 {
    _intmid_s(rs1)
}

/// Computes the width of an interval.
///
/// Returns the difference between the upper and lower bounds.
///
/// Instruction: **INTWDT.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(intwdt.s))]
#[inline]
pub unsafe fn intwdt_s(rs1: f64) -> f32 {
    _intwdt_s(rs1)
}

/// Computes the radius (half-width) of an interval.
///
/// Returns half the difference between the upper and lower bounds.
///
/// Instruction: **INTRAD.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(intrad.s))]
#[inline]
pub unsafe fn intrad_s(rs1: f64) -> f32 {
    _intrad_s(rs1)
}

/// Extracts the lower bound of an interval.
///
/// Instruction: **INTMIN.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(intmin.s))]
#[inline]
pub unsafe fn intmin_s(rs1: f64) -> f32 {
    _intmin_s(rs1)
}

/// Extracts the upper bound of an interval.
///
/// Instruction: **INTMAX.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(intmax.s))]
#[inline]
pub unsafe fn intmax_s(rs1: f64) -> f32 {
    _intmax_s(rs1)
}

/// Determines if one interval occurs strictly before another.
///
/// Implements Allen’s "before" relation.
///
/// Instruction: **INTBFR.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(intbfr.s))]
#[inline]
pub unsafe fn intbfr_s(rs1: f64, rs2: f64) -> i32 {
    _intbfr_s(rs1, rs2)
}

/// Determines if one interval meets another.
///
/// Implements Allen’s "meets" relation.
///
/// Instruction: **INTMTS.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(intmts.s))]
#[inline]
pub unsafe fn intmts_s(rs1: f64, rs2: f64) -> i32 {
    _intmts_s(rs1, rs2)
}

/// Determines if two intervals overlap.
///
/// Implements Allen’s "overlaps" relation.
///
/// Instruction: **INTOVR.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(intovr.s))]
#[inline]
pub unsafe fn intovr_s(rs1: f64, rs2: f64) -> i32 {
    _intovr_s(rs1, rs2)
}

/// Determines if one interval is fully contained within another.
///
/// Implements Allen’s "during" relation.
///
/// Instruction: **INTDUR.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(intdur.s))]
#[inline]
pub unsafe fn intdur_s(rs1: f64, rs2: f64) -> i32 {
    _intdur_s(rs1, rs2)
}

/// Determines if one interval starts another.
///
/// Implements Allen’s "starts" relation.
///
/// Instruction: **INTSTR.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(intstr.s))]
#[inline]
pub unsafe fn intstr_s(rs1: f64, rs2: f64) -> i32 {
    _intstr_s(rs1, rs2)
}

/// Determines if one interval finishes another.
///
/// Implements Allen’s "finishes" relation.
///
/// Instruction: **INTFIN.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(intfin.s))]
#[inline]
pub unsafe fn intfin_s(rs1: f64, rs2: f64) -> i32 {
    _intfin_s(rs1, rs2)
}

/// Determines if two intervals are exactly equal.
///
/// Implements Allen’s "equals" relation.
///
/// Instruction: **INTEQL.S**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xfintx")]
#[cfg_attr(test, assert_instr(inteql.s))]
#[inline]
pub unsafe fn inteql_s(rs1: f64, rs2: f64) -> i32 {
    _inteql_s(rs1, rs2)
}

/// Performs interval addition.
///
/// Computes the sum of two intervals according to IEEE 1788.1.
///
/// Instruction: **INTADD.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(intadd.h))]
#[inline]
pub unsafe fn intadd_h(rs1: f32, rs2: f32) -> f32 {
    _intadd_h(rs1, rs2)
}

/// Performs interval subtraction.
///
/// Computes the difference of two intervals according to IEEE 1788.1.
///
/// Instruction: **INTSUB.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(intsub.h))]
#[inline]
pub unsafe fn intsub_h(rs1: f32, rs2: f32) -> f32 {
    _intsub_h(rs1, rs2)
}

/// Performs interval multiplication.
///
/// Computes the product of two intervals following IEEE 1788.1.
///
/// Instruction: **INTMUL.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(intmul.h))]
#[inline]
pub unsafe fn intmul_h(rs1: f32, rs2: f32) -> f32 {
    _intmul_h(rs1, rs2)
}

/// Performs interval division.
///
/// Computes the quotient of two intervals as per IEEE 1788.1. Division by an interval
/// containing zero will raise an exception flag.
///
/// Instruction: **INTDIV.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(intdiv.h))]
#[inline]
pub unsafe fn intdiv_h(rs1: f32, rs2: f32) -> f32 {
    _intdiv_h(rs1, rs2)
}

/// Computes the midpoint of an interval.
///
/// Calculates the average of the lower and upper bounds.
///
/// Instruction: **INTMID.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(intmid.h))]
#[inline]
pub unsafe fn intmid_h(rs1: f32) -> f16 {
    _intmid_h(rs1)
}

/// Computes the width of an interval.
///
/// Returns the difference between the upper and lower bounds.
///
/// Instruction: **INTWDT.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(intwdt.h))]
#[inline]
pub unsafe fn intwdt_h(rs1: f32) -> f16 {
    _intwdt_h(rs1)
}

/// Computes the radius (half-width) of an interval.
///
/// Returns half the difference between the upper and lower bounds.
///
/// Instruction: **INTRAD.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(intrad.h))]
#[inline]
pub unsafe fn intrad_h(rs1: f32) -> f16 {
    _intrad_h(rs1)
}

/// Extracts the lower bound of an interval.
///
/// Instruction: **INTMIN.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(intmin.h))]
#[inline]
pub unsafe fn intmin_h(rs1: f32) -> f16 {
    _intmin_h(rs1)
}

/// Extracts the upper bound of an interval.
///
/// Instruction: **INTMAX.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(intmax.h))]
#[inline]
pub unsafe fn intmax_h(rs1: f32) -> f16 {
    _intmax_h(rs1)
}

/// Determines if one interval occurs strictly before another.
///
/// Implements Allen’s "before" relation.
///
/// Instruction: **INTBFR.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(intbfr.h))]
#[inline]
pub unsafe fn intbfr_h(rs1: f32, rs2: f32) -> i32 {
    _intbfr_h(rs1, rs2)
}

/// Determines if one interval meets another.
///
/// Implements Allen’s "meets" relation.
///
/// Instruction: **INTMTS.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(intmts.h))]
#[inline]
pub unsafe fn intmts_h(rs1: f32, rs2: f32) -> i32 {
    _intmts_h(rs1, rs2)
}

/// Determines if two intervals overlap.
///
/// Implements Allen’s "overlaps" relation.
///
/// Instruction: **INTOVR.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(intovr.h))]
#[inline]
pub unsafe fn intovr_h(rs1: f32, rs2: f32) -> i32 {
    _intovr_h(rs1, rs2)
}

/// Determines if one interval is fully contained within another.
///
/// Implements Allen’s "during" relation.
///
/// Instruction: **INTDUR.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(intdur.h))]
#[inline]
pub unsafe fn intdur_h(rs1: f32, rs2: f32) -> i32 {
    _intdur_h(rs1, rs2)
}

/// Determines if one interval starts another.
///
/// Implements Allen’s "starts" relation.
///
/// Instruction: **INTSTR.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(intstr.h))]
#[inline]
pub unsafe fn intstr_h(rs1: f32, rs2: f32) -> i32 {
    _intstr_h(rs1, rs2)
}

/// Determines if one interval finishes another.
///
/// Implements Allen’s "finishes" relation.
///
/// Instruction: **INTFIN.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(intfin.h))]
#[inline]
pub unsafe fn intfin_h(rs1: f32, rs2: f32) -> i32 {
    _intfin_h(rs1, rs2)
}

/// Determines if two intervals are exactly equal.
///
/// Implements Allen’s "equals" relation.
///
/// Instruction: **INTEQL.H**
#[unstable(feature = "riscv_ext_intrinsics", issue = "114544")]
#[target_feature(enable = "xhintx")]
#[cfg_attr(test, assert_instr(inteql.h))]
#[inline]
pub unsafe fn inteql_h(rs1: f32, rs2: f32) -> i32 {
    _inteql_h(rs1, rs2)
}
