use crate::dsl::{fmt, inst, r, rex, rw, sxl, sxq, sxw, w};
use crate::dsl::{Feature::*, Inst, Location::*};

pub fn list() -> Vec<Inst> {
    vec![
        // TODO
        inst("mulb", fmt("M", [rw(al).disas(false), r(rm8)]), rex(0xF6).digit(4), _64b | compat),
        inst(
            "mulw",
            fmt("M", [rw(ax).disas(false), w(dx).disas(false), r(rm16)]),
            rex([0x66, 0xF7]).digit(4),
            _64b | compat,
        ),
        inst(
            "mull",
            fmt("M", [rw(eax).disas(false), w(edx).disas(false), r(rm32)]),
            rex(0xF7).digit(4),
            _64b | compat,
        ),
        inst(
            "mulq",
            fmt("M", [rw(rax).disas(false), w(rdx).disas(false), r(rm64)]),
            rex(0xF7).w().digit(4),
            _64b,
        ),
        // TODO
        inst("imulb", fmt("M", [rw(al).disas(false), r(rm8)]), rex(0xF6).digit(5), _64b | compat),
        inst(
            "imulw",
            fmt("M", [rw(ax).disas(false), w(dx).disas(false), r(rm16)]),
            rex([0x66, 0xF7]).digit(5),
            _64b | compat,
        ),
        inst(
            "imull",
            fmt("M", [rw(eax).disas(false), w(edx).disas(false), r(rm32)]),
            rex(0xF7).digit(5),
            _64b | compat,
        ),
        inst(
            "imulq",
            fmt("M", [rw(rax).disas(false), w(rdx).disas(false), r(rm64)]),
            rex(0xF7).w().digit(5),
            _64b,
        ),
        inst("imulw", fmt("RM", [rw(r16), r(rm16)]), rex([0x66, 0x0F, 0xAF]), _64b | compat),
        inst("imull", fmt("RM", [rw(r32), r(rm32)]), rex([0x0F, 0xAF]), _64b | compat),
        inst("imulq", fmt("RM", [rw(r64), r(rm64)]), rex([0x0F, 0xAF]).w(), _64b),
        inst("imulw", fmt("RMI_SXW", [rw(r16), r(rm16), sxw(imm8)]), rex([0x66, 0x6B]).ib(), _64b | compat),
        inst("imull", fmt("RMI_SXL", [rw(r32), r(rm32), sxl(imm8)]), rex(0x6B).ib(), _64b | compat),
        inst("imulq", fmt("RMI_SXQ", [rw(r64), r(rm64), sxq(imm8)]), rex(0x6B).w().ib(), _64b),
        inst("imulw", fmt("RMI", [rw(r16), r(rm16), r(imm16)]), rex([0x66, 0x69]).iw(), _64b | compat),
        inst("imull", fmt("RMI", [rw(r32), r(rm32), r(imm32)]), rex(0x69).id(), _64b | compat),
        inst("imulq", fmt("RMI", [rw(r64), r(rm64), sxq(imm32)]), rex(0x69).w().id(), _64b),
    ]
}
