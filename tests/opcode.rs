use paprika::opcode::{OpCode, OpCodeInfo};
use std::str::FromStr;

#[test]
fn test_opcode_from_str() {
    // EXTCODESIZE
    let extcodesize = OpCode::from_str("3B").unwrap();
    let extcodesize: OpCodeInfo = extcodesize.into();
    assert_eq!(
        extcodesize,
        OpCodeInfo {
            opcode_value: 0x3B,
            rgb_color: [0, 255, 144],
        }
    );

    // Invalid opcode
    let not_exist_opcode = OpCode::from_str("FE").unwrap();
    let not_exist_opcode: OpCodeInfo = not_exist_opcode.into();
    assert_eq!(
        not_exist_opcode,
        OpCodeInfo {
            opcode_value: 0x2F,
            rgb_color: [255, 255, 255],
        }
    );
}
