use std::str::FromStr;

// through the color_generator.py program, generate 146 RGB colors as evenly as possible.
const RGB_COLORS: [[u8; 3]; 146] = [
    [0, 7, 255],
    [0, 17, 255],
    [0, 28, 255],
    [0, 38, 255],
    [0, 49, 255],
    [0, 59, 255],
    [0, 70, 255],
    [0, 80, 255],
    [0, 91, 255],
    [0, 101, 255],
    [0, 112, 255],
    [0, 123, 255],
    [0, 133, 255],
    [0, 144, 255],
    [0, 154, 255],
    [0, 165, 255],
    [0, 175, 255],
    [0, 186, 255],
    [0, 196, 255],
    [0, 207, 255],
    [0, 218, 255],
    [0, 228, 255],
    [0, 239, 255],
    [0, 249, 255],
    [0, 255, 7],
    [0, 255, 17],
    [0, 255, 28],
    [0, 255, 38],
    [0, 255, 49],
    [0, 255, 59],
    [0, 255, 70],
    [0, 255, 80],
    [0, 255, 91],
    [0, 255, 102],
    [0, 255, 112],
    [0, 255, 123],
    [0, 255, 133],
    [0, 255, 144],
    [0, 255, 154],
    [0, 255, 165],
    [0, 255, 175],
    [0, 255, 186],
    [0, 255, 196],
    [0, 255, 207],
    [0, 255, 218],
    [0, 255, 228],
    [0, 255, 239],
    [0, 255, 249],
    [3, 0, 255],
    [3, 255, 0],
    [14, 0, 255],
    [14, 255, 0],
    [24, 0, 255],
    [24, 255, 0],
    [35, 0, 255],
    [35, 255, 0],
    [45, 0, 255],
    [45, 255, 0],
    [56, 0, 255],
    [56, 255, 0],
    [66, 0, 255],
    [66, 255, 0],
    [77, 0, 255],
    [77, 255, 0],
    [87, 0, 255],
    [87, 255, 0],
    [98, 0, 255],
    [98, 255, 0],
    [109, 0, 255],
    [109, 255, 0],
    [119, 0, 255],
    [119, 255, 0],
    [130, 0, 255],
    [130, 255, 0],
    [140, 0, 255],
    [140, 255, 0],
    [151, 0, 255],
    [151, 255, 0],
    [161, 0, 255],
    [161, 255, 0],
    [172, 0, 255],
    [172, 255, 0],
    [182, 0, 255],
    [182, 255, 0],
    [193, 0, 255],
    [193, 255, 0],
    [203, 0, 255],
    [203, 255, 0],
    [214, 0, 255],
    [214, 255, 0],
    [225, 0, 255],
    [225, 255, 0],
    [235, 0, 255],
    [235, 255, 0],
    [246, 0, 255],
    [246, 255, 0],
    [255, 0, 0],
    [255, 0, 10],
    [255, 0, 21],
    [255, 0, 31],
    [255, 0, 42],
    [255, 0, 52],
    [255, 0, 63],
    [255, 0, 73],
    [255, 0, 84],
    [255, 0, 94],
    [255, 0, 105],
    [255, 0, 116],
    [255, 0, 126],
    [255, 0, 137],
    [255, 0, 147],
    [255, 0, 158],
    [255, 0, 168],
    [255, 0, 179],
    [255, 0, 189],
    [255, 0, 200],
    [255, 0, 211],
    [255, 0, 221],
    [255, 0, 232],
    [255, 0, 242],
    [255, 0, 253],
    [255, 10, 0],
    [255, 21, 0],
    [255, 31, 0],
    [255, 42, 0],
    [255, 52, 0],
    [255, 63, 0],
    [255, 73, 0],
    [255, 84, 0],
    [255, 94, 0],
    [255, 105, 0],
    [255, 116, 0],
    [255, 126, 0],
    [255, 137, 0],
    [255, 147, 0],
    [255, 158, 0],
    [255, 168, 0],
    [255, 179, 0],
    [255, 189, 0],
    [255, 200, 0],
    [255, 211, 0],
    [255, 221, 0],
    [255, 232, 0],
    [255, 242, 0],
    [255, 253, 0],
    [255, 255, 255], // for non-opcode
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpCodeInfo {
    pub opcode_value: u8,
    pub rgb_color: [u8; 3],
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
    // 0x00 range
    STOP(OpCodeInfo),
    ADD(OpCodeInfo),
    MUL(OpCodeInfo),
    SUB(OpCodeInfo),
    DIV(OpCodeInfo),
    SDIV(OpCodeInfo),
    MOD(OpCodeInfo),
    SMOD(OpCodeInfo),
    ADDMOD(OpCodeInfo),
    MULMOD(OpCodeInfo),
    EXP(OpCodeInfo),
    SIGNEXTEND(OpCodeInfo),

    // 0x10 range
    LT(OpCodeInfo),
    GT(OpCodeInfo),
    SLT(OpCodeInfo),
    SGT(OpCodeInfo),
    EQ(OpCodeInfo),
    ISZERO(OpCodeInfo),
    AND(OpCodeInfo),
    OR(OpCodeInfo),
    XOR(OpCodeInfo),
    NOT(OpCodeInfo),
    BYTE(OpCodeInfo),
    SHL(OpCodeInfo),
    SHR(OpCodeInfo),
    SAR(OpCodeInfo),

    // 0x20 range
    SHA3(OpCodeInfo),

    // 0x30 range
    ADDRESS(OpCodeInfo),
    BALANCE(OpCodeInfo),
    ORIGIN(OpCodeInfo),
    CALLER(OpCodeInfo),
    CALLVALUE(OpCodeInfo),
    CALLDATALOAD(OpCodeInfo),
    CALLDATASIZE(OpCodeInfo),
    CALLDATACOPY(OpCodeInfo),
    CODESIZE(OpCodeInfo),
    CODECOPY(OpCodeInfo),
    GASPRICE(OpCodeInfo),
    EXTCODESIZE(OpCodeInfo),
    EXTCODECOPY(OpCodeInfo),
    RETURNDATASIZE(OpCodeInfo),
    RETURNDATACOPY(OpCodeInfo),
    EXTCODEHASH(OpCodeInfo),

    // 0x40 range
    BLOCKHASH(OpCodeInfo),
    COINBASE(OpCodeInfo),
    TIMESTAMP(OpCodeInfo),
    NUMBER(OpCodeInfo),
    DIFFICULTY(OpCodeInfo),
    GASLIMIT(OpCodeInfo),
    CHAINID(OpCodeInfo),
    SELFBALANCE(OpCodeInfo),
    BASEFEE(OpCodeInfo),
    BLOBHASH(OpCodeInfo),
    BLOBBASEFEE(OpCodeInfo),

    // 0x50 range
    POP(OpCodeInfo),
    MLOAD(OpCodeInfo),
    MSTORE(OpCodeInfo),
    MSTORE8(OpCodeInfo),
    SLOAD(OpCodeInfo),
    SSTORE(OpCodeInfo),
    JUMP(OpCodeInfo),
    JUMPI(OpCodeInfo),
    PC(OpCodeInfo),
    MSIZE(OpCodeInfo),
    GAS(OpCodeInfo),
    JUMPDEST(OpCodeInfo),
    TLOAD(OpCodeInfo),
    TSTORE(OpCodeInfo),
    MCOPY(OpCodeInfo),
    PUSH0(OpCodeInfo),

    // 0x60 range
    PUSH1(OpCodeInfo),
    PUSH2(OpCodeInfo),
    PUSH3(OpCodeInfo),
    PUSH4(OpCodeInfo),
    PUSH5(OpCodeInfo),
    PUSH6(OpCodeInfo),
    PUSH7(OpCodeInfo),
    PUSH8(OpCodeInfo),
    PUSH9(OpCodeInfo),
    PUSH10(OpCodeInfo),
    PUSH11(OpCodeInfo),
    PUSH12(OpCodeInfo),
    PUSH13(OpCodeInfo),
    PUSH14(OpCodeInfo),
    PUSH15(OpCodeInfo),
    PUSH16(OpCodeInfo),

    // 0x70 range
    PUSH17(OpCodeInfo),
    PUSH18(OpCodeInfo),
    PUSH19(OpCodeInfo),
    PUSH20(OpCodeInfo),
    PUSH21(OpCodeInfo),
    PUSH22(OpCodeInfo),
    PUSH23(OpCodeInfo),
    PUSH24(OpCodeInfo),
    PUSH25(OpCodeInfo),
    PUSH26(OpCodeInfo),
    PUSH27(OpCodeInfo),
    PUSH28(OpCodeInfo),
    PUSH29(OpCodeInfo),
    PUSH30(OpCodeInfo),
    PUSH31(OpCodeInfo),
    PUSH32(OpCodeInfo),

    // 0x80 range
    DUP1(OpCodeInfo),
    DUP2(OpCodeInfo),
    DUP3(OpCodeInfo),
    DUP4(OpCodeInfo),
    DUP5(OpCodeInfo),
    DUP6(OpCodeInfo),
    DUP7(OpCodeInfo),
    DUP8(OpCodeInfo),
    DUP9(OpCodeInfo),
    DUP10(OpCodeInfo),
    DUP11(OpCodeInfo),
    DUP12(OpCodeInfo),
    DUP13(OpCodeInfo),
    DUP14(OpCodeInfo),
    DUP15(OpCodeInfo),
    DUP16(OpCodeInfo),

    // 0x90 range
    SWAP1(OpCodeInfo),
    SWAP2(OpCodeInfo),
    SWAP3(OpCodeInfo),
    SWAP4(OpCodeInfo),
    SWAP5(OpCodeInfo),
    SWAP6(OpCodeInfo),
    SWAP7(OpCodeInfo),
    SWAP8(OpCodeInfo),
    SWAP9(OpCodeInfo),
    SWAP10(OpCodeInfo),
    SWAP11(OpCodeInfo),
    SWAP12(OpCodeInfo),
    SWAP13(OpCodeInfo),
    SWAP14(OpCodeInfo),
    SWAP15(OpCodeInfo),
    SWAP16(OpCodeInfo),

    // 0xA0 range
    LOG0(OpCodeInfo),
    LOG1(OpCodeInfo),
    LOG2(OpCodeInfo),
    LOG3(OpCodeInfo),
    LOG4(OpCodeInfo),

    // 0xB0 range
    PUSH(OpCodeInfo),
    DUP(OpCodeInfo),
    SWAP(OpCodeInfo),

    // 0xF0 range
    CREATE(OpCodeInfo),
    CALL(OpCodeInfo),
    CALLCODE(OpCodeInfo),
    RETURN(OpCodeInfo),
    DELEGATECALL(OpCodeInfo),
    CREATE2(OpCodeInfo),
    STATICCALL(OpCodeInfo),
    REVERT(OpCodeInfo),
    SELFDESTRUCT(OpCodeInfo),

    NOTOPCODE(OpCodeInfo),
}

impl FromStr for OpCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // 0x00 range
            "00" | "STOP" => Ok(OpCode::STOP(OpCodeInfo {
                opcode_value: 0x00,
                rgb_color: RGB_COLORS[0],
            })),
            "01" | "ADD" => Ok(OpCode::ADD(OpCodeInfo {
                opcode_value: 0x01,
                rgb_color: RGB_COLORS[1],
            })),
            "02" | "MUL" => Ok(OpCode::MUL(OpCodeInfo {
                opcode_value: 0x02,
                rgb_color: RGB_COLORS[2],
            })),
            "03" | "SUB" => Ok(OpCode::SUB(OpCodeInfo {
                opcode_value: 0x03,
                rgb_color: RGB_COLORS[3],
            })),
            "04" | "DIV" => Ok(OpCode::DIV(OpCodeInfo {
                opcode_value: 0x04,
                rgb_color: RGB_COLORS[4],
            })),
            "05" | "SDIV" => Ok(OpCode::SDIV(OpCodeInfo {
                opcode_value: 0x05,
                rgb_color: RGB_COLORS[5],
            })),
            "06" | "MOD" => Ok(OpCode::MOD(OpCodeInfo {
                opcode_value: 0x06,
                rgb_color: RGB_COLORS[6],
            })),
            "07" | "SMOD" => Ok(OpCode::SMOD(OpCodeInfo {
                opcode_value: 0x07,
                rgb_color: RGB_COLORS[7],
            })),
            "08" | "ADDMOD" => Ok(OpCode::ADDMOD(OpCodeInfo {
                opcode_value: 0x08,
                rgb_color: RGB_COLORS[8],
            })),
            "09" | "MULMOD" => Ok(OpCode::MULMOD(OpCodeInfo {
                opcode_value: 0x09,
                rgb_color: RGB_COLORS[9],
            })),
            "0A" | "EXP" => Ok(OpCode::EXP(OpCodeInfo {
                opcode_value: 0x0A,
                rgb_color: RGB_COLORS[10],
            })),
            "0B" | "SIGNEXTEND" => Ok(OpCode::SIGNEXTEND(OpCodeInfo {
                opcode_value: 0x0B,
                rgb_color: RGB_COLORS[11],
            })),

            // 0x10 range
            "10" | "LT" => Ok(OpCode::LT(OpCodeInfo {
                opcode_value: 0x10,
                rgb_color: RGB_COLORS[12],
            })),
            "11" | "GT" => Ok(OpCode::GT(OpCodeInfo {
                opcode_value: 0x11,
                rgb_color: RGB_COLORS[13],
            })),
            "12" | "SLT" => Ok(OpCode::SLT(OpCodeInfo {
                opcode_value: 0x12,
                rgb_color: RGB_COLORS[14],
            })),
            "13" | "SGT" => Ok(OpCode::SGT(OpCodeInfo {
                opcode_value: 0x13,
                rgb_color: RGB_COLORS[15],
            })),
            "14" | "EQ" => Ok(OpCode::EQ(OpCodeInfo {
                opcode_value: 0x14,
                rgb_color: RGB_COLORS[16],
            })),
            "15" | "ISZERO" => Ok(OpCode::ISZERO(OpCodeInfo {
                opcode_value: 0x15,
                rgb_color: RGB_COLORS[17],
            })),
            "16" | "AND" => Ok(OpCode::AND(OpCodeInfo {
                opcode_value: 0x16,
                rgb_color: RGB_COLORS[18],
            })),
            "17" | "OR" => Ok(OpCode::OR(OpCodeInfo {
                opcode_value: 0x17,
                rgb_color: RGB_COLORS[19],
            })),
            "18" | "XOR" => Ok(OpCode::XOR(OpCodeInfo {
                opcode_value: 0x18,
                rgb_color: RGB_COLORS[20],
            })),
            "19" | "NOT" => Ok(OpCode::NOT(OpCodeInfo {
                opcode_value: 0x19,
                rgb_color: RGB_COLORS[21],
            })),
            "1A" | "BYTE" => Ok(OpCode::BYTE(OpCodeInfo {
                opcode_value: 0x1A,
                rgb_color: RGB_COLORS[22],
            })),
            "1B" | "SHL" => Ok(OpCode::SHL(OpCodeInfo {
                opcode_value: 0x1B,
                rgb_color: RGB_COLORS[23],
            })),
            "1C" | "SHR" => Ok(OpCode::SHR(OpCodeInfo {
                opcode_value: 0x1C,
                rgb_color: RGB_COLORS[24],
            })),
            "1D" | "SAR" => Ok(OpCode::SAR(OpCodeInfo {
                opcode_value: 0x1D,
                rgb_color: RGB_COLORS[25],
            })),

            // 0x20 range
            "20" | "SHA3" => Ok(OpCode::SHA3(OpCodeInfo {
                opcode_value: 0x20,
                rgb_color: RGB_COLORS[25],
            })),

            // 0x30 range
            "30" | "ADDRESS" => Ok(OpCode::ADDRESS(OpCodeInfo {
                opcode_value: 0x30,
                rgb_color: RGB_COLORS[26],
            })),
            "31" | "BALANCE" => Ok(OpCode::BALANCE(OpCodeInfo {
                opcode_value: 0x31,
                rgb_color: RGB_COLORS[27],
            })),
            "32" | "ORIGIN" => Ok(OpCode::ORIGIN(OpCodeInfo {
                opcode_value: 0x32,
                rgb_color: RGB_COLORS[28],
            })),
            "33" | "CALLER" => Ok(OpCode::CALLER(OpCodeInfo {
                opcode_value: 0x33,
                rgb_color: RGB_COLORS[29],
            })),
            "34" | "CALLVALUE" => Ok(OpCode::CALLVALUE(OpCodeInfo {
                opcode_value: 0x34,
                rgb_color: RGB_COLORS[30],
            })),
            "35" | "CALLDATALOAD" => Ok(OpCode::CALLDATALOAD(OpCodeInfo {
                opcode_value: 0x35,
                rgb_color: RGB_COLORS[31],
            })),
            "36" | "CALLDATASIZE" => Ok(OpCode::CALLDATASIZE(OpCodeInfo {
                opcode_value: 0x36,
                rgb_color: RGB_COLORS[32],
            })),
            "37" | "CALLDATACOPY" => Ok(OpCode::CALLDATACOPY(OpCodeInfo {
                opcode_value: 0x37,
                rgb_color: RGB_COLORS[33],
            })),
            "38" | "CODESIZE" => Ok(OpCode::CODESIZE(OpCodeInfo {
                opcode_value: 0x38,
                rgb_color: RGB_COLORS[34],
            })),
            "39" | "CODECOPY" => Ok(OpCode::CODECOPY(OpCodeInfo {
                opcode_value: 0x39,
                rgb_color: RGB_COLORS[35],
            })),
            "3A" | "GASPRICE" => Ok(OpCode::GASPRICE(OpCodeInfo {
                opcode_value: 0x3A,
                rgb_color: RGB_COLORS[36],
            })),
            "3B" | "EXTCODESIZE" => Ok(OpCode::EXTCODESIZE(OpCodeInfo {
                opcode_value: 0x3B,
                rgb_color: RGB_COLORS[37],
            })),
            "3C" | "EXTCODECOPY" => Ok(OpCode::EXTCODECOPY(OpCodeInfo {
                opcode_value: 0x3C,
                rgb_color: RGB_COLORS[38],
            })),
            "3D" | "RETURNDATASIZE" => Ok(OpCode::RETURNDATASIZE(OpCodeInfo {
                opcode_value: 0x3D,
                rgb_color: RGB_COLORS[39],
            })),
            "3E" | "RETURNDATACOPY" => Ok(OpCode::RETURNDATACOPY(OpCodeInfo {
                opcode_value: 0x3E,
                rgb_color: RGB_COLORS[40],
            })),
            "3F" | "EXTCODEHASH" => Ok(OpCode::EXTCODEHASH(OpCodeInfo {
                opcode_value: 0x3F,
                rgb_color: RGB_COLORS[41],
            })),

            // 0x40 range
            "40" | "BLOCKHASH" => Ok(OpCode::BLOCKHASH(OpCodeInfo {
                opcode_value: 0x40,
                rgb_color: RGB_COLORS[42],
            })),
            "41" | "COINBASE" => Ok(OpCode::COINBASE(OpCodeInfo {
                opcode_value: 0x41,
                rgb_color: RGB_COLORS[43],
            })),
            "42" | "TIMESTAMP" => Ok(OpCode::TIMESTAMP(OpCodeInfo {
                opcode_value: 0x42,
                rgb_color: RGB_COLORS[44],
            })),
            "43" | "NUMBER" => Ok(OpCode::NUMBER(OpCodeInfo {
                opcode_value: 0x43,
                rgb_color: RGB_COLORS[45],
            })),
            "44" | "DIFFICULTY" => Ok(OpCode::DIFFICULTY(OpCodeInfo {
                opcode_value: 0x44,
                rgb_color: RGB_COLORS[46],
            })),
            "45" | "GASLIMIT" => Ok(OpCode::GASLIMIT(OpCodeInfo {
                opcode_value: 0x45,
                rgb_color: RGB_COLORS[47],
            })),
            "46" | "CHAINID" => Ok(OpCode::CHAINID(OpCodeInfo {
                opcode_value: 0x46,
                rgb_color: RGB_COLORS[48],
            })),
            "47" | "SELFBALANCE" => Ok(OpCode::SELFBALANCE(OpCodeInfo {
                opcode_value: 0x47,
                rgb_color: RGB_COLORS[49],
            })),
            "48" | "BASEFEE" => Ok(OpCode::BASEFEE(OpCodeInfo {
                opcode_value: 0x48,
                rgb_color: RGB_COLORS[50],
            })),
            "49" | "BLOBHASH" => Ok(OpCode::BLOBHASH(OpCodeInfo {
                opcode_value: 0x49,
                rgb_color: RGB_COLORS[51],
            })),
            "4A" | "BLOBBASEFEE" => Ok(OpCode::BLOBBASEFEE(OpCodeInfo {
                opcode_value: 0x4A,
                rgb_color: RGB_COLORS[52],
            })),

            // 0x50 range
            "50" | "POP" => Ok(OpCode::POP(OpCodeInfo {
                opcode_value: 0x50,
                rgb_color: RGB_COLORS[53],
            })),
            "51" | "MLOAD" => Ok(OpCode::MLOAD(OpCodeInfo {
                opcode_value: 0x51,
                rgb_color: RGB_COLORS[54],
            })),
            "52" | "MSTORE" => Ok(OpCode::MSTORE(OpCodeInfo {
                opcode_value: 0x52,
                rgb_color: RGB_COLORS[55],
            })),
            "53" | "MSTORE8" => Ok(OpCode::MSTORE8(OpCodeInfo {
                opcode_value: 0x53,
                rgb_color: RGB_COLORS[56],
            })),
            "54" | "SLOAD" => Ok(OpCode::SLOAD(OpCodeInfo {
                opcode_value: 0x54,
                rgb_color: RGB_COLORS[57],
            })),
            "55" | "SSTORE" => Ok(OpCode::SSTORE(OpCodeInfo {
                opcode_value: 0x55,
                rgb_color: RGB_COLORS[58],
            })),
            "56" | "JUMP" => Ok(OpCode::JUMP(OpCodeInfo {
                opcode_value: 0x56,
                rgb_color: RGB_COLORS[59],
            })),
            "57" | "JUMPI" => Ok(OpCode::JUMPI(OpCodeInfo {
                opcode_value: 0x57,
                rgb_color: RGB_COLORS[60],
            })),
            "58" | "PC" => Ok(OpCode::PC(OpCodeInfo {
                opcode_value: 0x58,
                rgb_color: RGB_COLORS[61],
            })),
            "59" | "MSIZE" => Ok(OpCode::MSIZE(OpCodeInfo {
                opcode_value: 0x59,
                rgb_color: RGB_COLORS[62],
            })),
            "5A" | "GAS" => Ok(OpCode::GAS(OpCodeInfo {
                opcode_value: 0x5A,
                rgb_color: RGB_COLORS[63],
            })),
            "5B" | "JUMPDEST" => Ok(OpCode::JUMPDEST(OpCodeInfo {
                opcode_value: 0x5B,
                rgb_color: RGB_COLORS[64],
            })),
            "5C" | "TLOAD" => Ok(OpCode::TLOAD(OpCodeInfo {
                opcode_value: 0x5C,
                rgb_color: RGB_COLORS[65],
            })),
            "5D" | "TSTORE" => Ok(OpCode::TSTORE(OpCodeInfo {
                opcode_value: 0x5D,
                rgb_color: RGB_COLORS[66],
            })),
            "5E" | "MCOPY" => Ok(OpCode::MCOPY(OpCodeInfo {
                opcode_value: 0x5E,
                rgb_color: RGB_COLORS[67],
            })),
            "5F" | "PUSH0" => Ok(OpCode::PUSH0(OpCodeInfo {
                opcode_value: 0x5F,
                rgb_color: RGB_COLORS[68],
            })),

            // 0x60 range
            "60" | "PUSH1" => Ok(OpCode::PUSH1(OpCodeInfo {
                opcode_value: 0x60,
                rgb_color: RGB_COLORS[69],
            })),
            "61" | "PUSH2" => Ok(OpCode::PUSH2(OpCodeInfo {
                opcode_value: 0x61,
                rgb_color: RGB_COLORS[70],
            })),
            "62" | "PUSH3" => Ok(OpCode::PUSH3(OpCodeInfo {
                opcode_value: 0x62,
                rgb_color: RGB_COLORS[71],
            })),
            "63" | "PUSH4" => Ok(OpCode::PUSH4(OpCodeInfo {
                opcode_value: 0x63,
                rgb_color: RGB_COLORS[72],
            })),
            "64" | "PUSH5" => Ok(OpCode::PUSH5(OpCodeInfo {
                opcode_value: 0x64,
                rgb_color: RGB_COLORS[73],
            })),
            "65" | "PUSH6" => Ok(OpCode::PUSH6(OpCodeInfo {
                opcode_value: 0x65,
                rgb_color: RGB_COLORS[74],
            })),
            "66" | "PUSH7" => Ok(OpCode::PUSH7(OpCodeInfo {
                opcode_value: 0x66,
                rgb_color: RGB_COLORS[75],
            })),
            "67" | "PUSH8" => Ok(OpCode::PUSH8(OpCodeInfo {
                opcode_value: 0x67,
                rgb_color: RGB_COLORS[76],
            })),
            "68" | "PUSH9" => Ok(OpCode::PUSH9(OpCodeInfo {
                opcode_value: 0x68,
                rgb_color: RGB_COLORS[77],
            })),
            "69" | "PUSH10" => Ok(OpCode::PUSH10(OpCodeInfo {
                opcode_value: 0x69,
                rgb_color: RGB_COLORS[78],
            })),
            "6A" | "PUSH11" => Ok(OpCode::PUSH11(OpCodeInfo {
                opcode_value: 0x6A,
                rgb_color: RGB_COLORS[79],
            })),
            "6B" | "PUSH12" => Ok(OpCode::PUSH12(OpCodeInfo {
                opcode_value: 0x6B,
                rgb_color: RGB_COLORS[80],
            })),
            "6C" | "PUSH13" => Ok(OpCode::PUSH13(OpCodeInfo {
                opcode_value: 0x6C,
                rgb_color: RGB_COLORS[81],
            })),
            "6D" | "PUSH14" => Ok(OpCode::PUSH14(OpCodeInfo {
                opcode_value: 0x6D,
                rgb_color: RGB_COLORS[82],
            })),
            "6E" | "PUSH15" => Ok(OpCode::PUSH15(OpCodeInfo {
                opcode_value: 0x6E,
                rgb_color: RGB_COLORS[83],
            })),
            "6F" | "PUSH16" => Ok(OpCode::PUSH16(OpCodeInfo {
                opcode_value: 0x6F,
                rgb_color: RGB_COLORS[84],
            })),

            // 0x70 range
            "70" | "PUSH17" => Ok(OpCode::PUSH17(OpCodeInfo {
                opcode_value: 0x70,
                rgb_color: RGB_COLORS[85],
            })),
            "71" | "PUSH18" => Ok(OpCode::PUSH18(OpCodeInfo {
                opcode_value: 0x71,
                rgb_color: RGB_COLORS[86],
            })),
            "72" | "PUSH19" => Ok(OpCode::PUSH19(OpCodeInfo {
                opcode_value: 0x72,
                rgb_color: RGB_COLORS[87],
            })),
            "73" | "PUSH20" => Ok(OpCode::PUSH20(OpCodeInfo {
                opcode_value: 0x73,
                rgb_color: RGB_COLORS[88],
            })),
            "74" | "PUSH21" => Ok(OpCode::PUSH21(OpCodeInfo {
                opcode_value: 0x74,
                rgb_color: RGB_COLORS[89],
            })),
            "75" | "PUSH22" => Ok(OpCode::PUSH22(OpCodeInfo {
                opcode_value: 0x75,
                rgb_color: RGB_COLORS[90],
            })),
            "76" | "PUSH23" => Ok(OpCode::PUSH23(OpCodeInfo {
                opcode_value: 0x76,
                rgb_color: RGB_COLORS[91],
            })),
            "77" | "PUSH24" => Ok(OpCode::PUSH24(OpCodeInfo {
                opcode_value: 0x77,
                rgb_color: RGB_COLORS[92],
            })),
            "78" | "PUSH25" => Ok(OpCode::PUSH25(OpCodeInfo {
                opcode_value: 0x78,
                rgb_color: RGB_COLORS[93],
            })),
            "79" | "PUSH26" => Ok(OpCode::PUSH26(OpCodeInfo {
                opcode_value: 0x79,
                rgb_color: RGB_COLORS[94],
            })),
            "7A" | "PUSH27" => Ok(OpCode::PUSH27(OpCodeInfo {
                opcode_value: 0x7A,
                rgb_color: RGB_COLORS[95],
            })),
            "7B" | "PUSH28" => Ok(OpCode::PUSH28(OpCodeInfo {
                opcode_value: 0x7B,
                rgb_color: RGB_COLORS[96],
            })),
            "7C" | "PUSH29" => Ok(OpCode::PUSH29(OpCodeInfo {
                opcode_value: 0x7C,
                rgb_color: RGB_COLORS[97],
            })),
            "7D" | "PUSH30" => Ok(OpCode::PUSH30(OpCodeInfo {
                opcode_value: 0x7D,
                rgb_color: RGB_COLORS[98],
            })),
            "7E" | "PUSH31" => Ok(OpCode::PUSH31(OpCodeInfo {
                opcode_value: 0x7E,
                rgb_color: RGB_COLORS[99],
            })),
            "7F" | "PUSH32" => Ok(OpCode::PUSH32(OpCodeInfo {
                opcode_value: 0x7F,
                rgb_color: RGB_COLORS[100],
            })),

            // 0x80 range
            "80" | "DUP1" => Ok(OpCode::DUP1(OpCodeInfo {
                opcode_value: 0x80,
                rgb_color: RGB_COLORS[101],
            })),
            "81" | "DUP2" => Ok(OpCode::DUP2(OpCodeInfo {
                opcode_value: 0x81,
                rgb_color: RGB_COLORS[101],
            })),
            "82" | "DUP3" => Ok(OpCode::DUP3(OpCodeInfo {
                opcode_value: 0x82,
                rgb_color: RGB_COLORS[103],
            })),
            "83" | "DUP4" => Ok(OpCode::DUP4(OpCodeInfo {
                opcode_value: 0x83,
                rgb_color: RGB_COLORS[104],
            })),
            "84" | "DUP5" => Ok(OpCode::DUP5(OpCodeInfo {
                opcode_value: 0x84,
                rgb_color: RGB_COLORS[105],
            })),
            "85" | "DUP6" => Ok(OpCode::DUP6(OpCodeInfo {
                opcode_value: 0x85,
                rgb_color: RGB_COLORS[106],
            })),
            "86" | "DUP7" => Ok(OpCode::DUP7(OpCodeInfo {
                opcode_value: 0x86,
                rgb_color: RGB_COLORS[107],
            })),
            "87" | "DUP8" => Ok(OpCode::DUP8(OpCodeInfo {
                opcode_value: 0x87,
                rgb_color: RGB_COLORS[108],
            })),
            "88" | "DUP9" => Ok(OpCode::DUP9(OpCodeInfo {
                opcode_value: 0x88,
                rgb_color: RGB_COLORS[109],
            })),
            "89" | "DUP10" => Ok(OpCode::DUP10(OpCodeInfo {
                opcode_value: 0x89,
                rgb_color: RGB_COLORS[110],
            })),
            "8A" | "DUP11" => Ok(OpCode::DUP11(OpCodeInfo {
                opcode_value: 0x8A,
                rgb_color: RGB_COLORS[111],
            })),
            "8B" | "DUP12" => Ok(OpCode::DUP12(OpCodeInfo {
                opcode_value: 0x8B,
                rgb_color: RGB_COLORS[112],
            })),
            "8C" | "DUP13" => Ok(OpCode::DUP13(OpCodeInfo {
                opcode_value: 0x8C,
                rgb_color: RGB_COLORS[113],
            })),
            "8D" | "DUP14" => Ok(OpCode::DUP14(OpCodeInfo {
                opcode_value: 0x8D,
                rgb_color: RGB_COLORS[114],
            })),
            "8E" | "DUP15" => Ok(OpCode::DUP15(OpCodeInfo {
                opcode_value: 0x8E,
                rgb_color: RGB_COLORS[115],
            })),
            "8F" | "DUP16" => Ok(OpCode::DUP16(OpCodeInfo {
                opcode_value: 0x8F,
                rgb_color: RGB_COLORS[116],
            })),

            // 0x90 range
            "90" | "SWAP1" => Ok(OpCode::SWAP1(OpCodeInfo {
                opcode_value: 0x90,
                rgb_color: RGB_COLORS[117],
            })),
            "91" | "SWAP2" => Ok(OpCode::SWAP2(OpCodeInfo {
                opcode_value: 0x91,
                rgb_color: RGB_COLORS[117],
            })),
            "92" | "SWAP3" => Ok(OpCode::SWAP3(OpCodeInfo {
                opcode_value: 0x92,
                rgb_color: RGB_COLORS[118],
            })),
            "93" | "SWAP4" => Ok(OpCode::SWAP4(OpCodeInfo {
                opcode_value: 0x93,
                rgb_color: RGB_COLORS[119],
            })),
            "94" | "SWAP5" => Ok(OpCode::SWAP5(OpCodeInfo {
                opcode_value: 0x94,
                rgb_color: RGB_COLORS[120],
            })),
            "95" | "SWAP6" => Ok(OpCode::SWAP6(OpCodeInfo {
                opcode_value: 0x95,
                rgb_color: RGB_COLORS[121],
            })),
            "96" | "SWAP7" => Ok(OpCode::SWAP7(OpCodeInfo {
                opcode_value: 0x96,
                rgb_color: RGB_COLORS[122],
            })),
            "97" | "SWAP8" => Ok(OpCode::SWAP8(OpCodeInfo {
                opcode_value: 0x97,
                rgb_color: RGB_COLORS[122],
            })),
            "98" | "SWAP9" => Ok(OpCode::SWAP9(OpCodeInfo {
                opcode_value: 0x98,
                rgb_color: RGB_COLORS[123],
            })),
            "99" | "SWAP10" => Ok(OpCode::SWAP10(OpCodeInfo {
                opcode_value: 0x99,
                rgb_color: RGB_COLORS[124],
            })),
            "9A" | "SWAP11" => Ok(OpCode::SWAP11(OpCodeInfo {
                opcode_value: 0x9A,
                rgb_color: RGB_COLORS[125],
            })),
            "9B" | "SWAP12" => Ok(OpCode::SWAP12(OpCodeInfo {
                opcode_value: 0x9B,
                rgb_color: RGB_COLORS[125],
            })),
            "9C" | "SWAP13" => Ok(OpCode::SWAP13(OpCodeInfo {
                opcode_value: 0x9C,
                rgb_color: RGB_COLORS[126],
            })),
            "9D" | "SWAP14" => Ok(OpCode::SWAP14(OpCodeInfo {
                opcode_value: 0x9D,
                rgb_color: RGB_COLORS[126],
            })),
            "9E" | "SWAP15" => Ok(OpCode::SWAP15(OpCodeInfo {
                opcode_value: 0x9E,
                rgb_color: RGB_COLORS[127],
            })),
            "9F" | "SWAP16" => Ok(OpCode::SWAP16(OpCodeInfo {
                opcode_value: 0x9F,
                rgb_color: RGB_COLORS[128],
            })),

            // 0xA0 range
            "A0" | "LOG0" => Ok(OpCode::LOG0(OpCodeInfo {
                opcode_value: 0xA0,
                rgb_color: RGB_COLORS[129],
            })),
            "A1" | "LOG1" => Ok(OpCode::LOG1(OpCodeInfo {
                opcode_value: 0xA1,
                rgb_color: RGB_COLORS[130],
            })),
            "A2" | "LOG2" => Ok(OpCode::LOG2(OpCodeInfo {
                opcode_value: 0xA2,
                rgb_color: RGB_COLORS[131],
            })),
            "A3" | "LOG3" => Ok(OpCode::LOG3(OpCodeInfo {
                opcode_value: 0xA3,
                rgb_color: RGB_COLORS[132],
            })),
            "A4" | "LOG4" => Ok(OpCode::LOG4(OpCodeInfo {
                opcode_value: 0xA4,
                rgb_color: RGB_COLORS[133],
            })),

            // 0xB0 range
            "B0" | "PUSH" => Ok(OpCode::PUSH(OpCodeInfo {
                opcode_value: 0xB0,
                rgb_color: RGB_COLORS[133],
            })),
            "B1" | "DUP" => Ok(OpCode::DUP(OpCodeInfo {
                opcode_value: 0xB1,
                rgb_color: RGB_COLORS[134],
            })),
            "B2" | "SWAP" => Ok(OpCode::SWAP(OpCodeInfo {
                opcode_value: 0xB2,
                rgb_color: RGB_COLORS[135],
            })),

            // 0xF0 range
            "F0" | "CREATE" => Ok(OpCode::CREATE(OpCodeInfo {
                opcode_value: 0xF0,
                rgb_color: RGB_COLORS[136],
            })),
            "F1" | "CALL" => Ok(OpCode::CALL(OpCodeInfo {
                opcode_value: 0xF1,
                rgb_color: RGB_COLORS[137],
            })),
            "F2" | "CALLCODE" => Ok(OpCode::CALLCODE(OpCodeInfo {
                opcode_value: 0xF2,
                rgb_color: RGB_COLORS[138],
            })),
            "F3" | "RETURN" => Ok(OpCode::RETURN(OpCodeInfo {
                opcode_value: 0xF3,
                rgb_color: RGB_COLORS[139],
            })),
            "F4" | "DELEGATECALL" => Ok(OpCode::DELEGATECALL(OpCodeInfo {
                opcode_value: 0xF4,
                rgb_color: RGB_COLORS[140],
            })),
            "F5" | "CREATE2" => Ok(OpCode::CREATE2(OpCodeInfo {
                opcode_value: 0xF5,
                rgb_color: RGB_COLORS[141],
            })),
            "FA" | "STATICCALL" => Ok(OpCode::STATICCALL(OpCodeInfo {
                opcode_value: 0xFA,
                rgb_color: RGB_COLORS[142],
            })),
            "FD" | "REVERT" => Ok(OpCode::REVERT(OpCodeInfo {
                opcode_value: 0xFD,
                rgb_color: RGB_COLORS[143],
            })),
            "FF" | "SELFDESTRUCT" => Ok(OpCode::SELFDESTRUCT(OpCodeInfo {
                opcode_value: 0xFF,
                rgb_color: RGB_COLORS[144],
            })),

            _ => {
                Ok(OpCode::NOTOPCODE(OpCodeInfo {
                    // attention: we use `2F` as invalid opcode.
                    // there are numerical values in bytecode, such as a value that follows `push1`, but this value is not an opcode
                    opcode_value: 0x2F,
                    rgb_color: RGB_COLORS[145],
                }))
            }
        }
    }
}

impl Into<OpCodeInfo> for OpCode {
    fn into(self) -> OpCodeInfo {
        match self {
            OpCode::STOP(info) => info,
            OpCode::ADD(info) => info,
            OpCode::MUL(info) => info,
            OpCode::SUB(info) => info,
            OpCode::DIV(info) => info,
            OpCode::SDIV(info) => info,
            OpCode::MOD(info) => info,
            OpCode::SMOD(info) => info,
            OpCode::ADDMOD(info) => info,
            OpCode::MULMOD(info) => info,
            OpCode::EXP(info) => info,
            OpCode::SIGNEXTEND(info) => info,

            OpCode::LT(info) => info,
            OpCode::GT(info) => info,
            OpCode::SLT(info) => info,
            OpCode::SGT(info) => info,
            OpCode::EQ(info) => info,
            OpCode::ISZERO(info) => info,
            OpCode::AND(info) => info,
            OpCode::OR(info) => info,
            OpCode::XOR(info) => info,
            OpCode::NOT(info) => info,
            OpCode::BYTE(info) => info,
            OpCode::SHL(info) => info,
            OpCode::SHR(info) => info,
            OpCode::SAR(info) => info,

            OpCode::SHA3(info) => info,

            OpCode::ADDRESS(info) => info,
            OpCode::BALANCE(info) => info,
            OpCode::ORIGIN(info) => info,
            OpCode::CALLER(info) => info,
            OpCode::CALLVALUE(info) => info,
            OpCode::CALLDATALOAD(info) => info,
            OpCode::CALLDATASIZE(info) => info,
            OpCode::CALLDATACOPY(info) => info,
            OpCode::CODESIZE(info) => info,
            OpCode::CODECOPY(info) => info,
            OpCode::GASPRICE(info) => info,
            OpCode::EXTCODESIZE(info) => info,
            OpCode::EXTCODECOPY(info) => info,
            OpCode::RETURNDATASIZE(info) => info,
            OpCode::RETURNDATACOPY(info) => info,
            OpCode::EXTCODEHASH(info) => info,

            OpCode::BLOCKHASH(info) => info,
            OpCode::COINBASE(info) => info,
            OpCode::TIMESTAMP(info) => info,
            OpCode::NUMBER(info) => info,
            OpCode::DIFFICULTY(info) => info,
            OpCode::GASLIMIT(info) => info,
            OpCode::CHAINID(info) => info,
            OpCode::SELFBALANCE(info) => info,
            OpCode::BASEFEE(info) => info,
            OpCode::BLOBHASH(info) => info,
            OpCode::BLOBBASEFEE(info) => info,

            OpCode::POP(info) => info,
            OpCode::MLOAD(info) => info,
            OpCode::MSTORE(info) => info,
            OpCode::MSTORE8(info) => info,
            OpCode::SLOAD(info) => info,
            OpCode::SSTORE(info) => info,
            OpCode::JUMP(info) => info,
            OpCode::JUMPI(info) => info,
            OpCode::PC(info) => info,
            OpCode::MSIZE(info) => info,
            OpCode::GAS(info) => info,
            OpCode::JUMPDEST(info) => info,
            OpCode::TLOAD(info) => info,
            OpCode::TSTORE(info) => info,
            OpCode::MCOPY(info) => info,
            OpCode::PUSH0(info) => info,

            OpCode::PUSH1(info) => info,
            OpCode::PUSH2(info) => info,
            OpCode::PUSH3(info) => info,
            OpCode::PUSH4(info) => info,
            OpCode::PUSH5(info) => info,
            OpCode::PUSH6(info) => info,
            OpCode::PUSH7(info) => info,
            OpCode::PUSH8(info) => info,
            OpCode::PUSH9(info) => info,
            OpCode::PUSH10(info) => info,
            OpCode::PUSH11(info) => info,
            OpCode::PUSH12(info) => info,
            OpCode::PUSH13(info) => info,
            OpCode::PUSH14(info) => info,
            OpCode::PUSH15(info) => info,
            OpCode::PUSH16(info) => info,

            OpCode::PUSH17(info) => info,
            OpCode::PUSH18(info) => info,
            OpCode::PUSH19(info) => info,
            OpCode::PUSH20(info) => info,
            OpCode::PUSH21(info) => info,
            OpCode::PUSH22(info) => info,
            OpCode::PUSH23(info) => info,
            OpCode::PUSH24(info) => info,
            OpCode::PUSH25(info) => info,
            OpCode::PUSH26(info) => info,
            OpCode::PUSH27(info) => info,
            OpCode::PUSH28(info) => info,
            OpCode::PUSH29(info) => info,
            OpCode::PUSH30(info) => info,
            OpCode::PUSH31(info) => info,
            OpCode::PUSH32(info) => info,

            OpCode::DUP1(info) => info,
            OpCode::DUP2(info) => info,
            OpCode::DUP3(info) => info,
            OpCode::DUP4(info) => info,
            OpCode::DUP5(info) => info,
            OpCode::DUP6(info) => info,
            OpCode::DUP7(info) => info,
            OpCode::DUP8(info) => info,
            OpCode::DUP9(info) => info,
            OpCode::DUP10(info) => info,
            OpCode::DUP11(info) => info,
            OpCode::DUP12(info) => info,
            OpCode::DUP13(info) => info,
            OpCode::DUP14(info) => info,
            OpCode::DUP15(info) => info,
            OpCode::DUP16(info) => info,

            OpCode::SWAP1(info) => info,
            OpCode::SWAP2(info) => info,
            OpCode::SWAP3(info) => info,
            OpCode::SWAP4(info) => info,
            OpCode::SWAP5(info) => info,
            OpCode::SWAP6(info) => info,
            OpCode::SWAP7(info) => info,
            OpCode::SWAP8(info) => info,
            OpCode::SWAP9(info) => info,
            OpCode::SWAP10(info) => info,
            OpCode::SWAP11(info) => info,
            OpCode::SWAP12(info) => info,
            OpCode::SWAP13(info) => info,
            OpCode::SWAP14(info) => info,
            OpCode::SWAP15(info) => info,
            OpCode::SWAP16(info) => info,

            OpCode::LOG0(info) => info,
            OpCode::LOG1(info) => info,
            OpCode::LOG2(info) => info,
            OpCode::LOG3(info) => info,
            OpCode::LOG4(info) => info,

            OpCode::PUSH(info) => info,
            OpCode::DUP(info) => info,
            OpCode::SWAP(info) => info,

            OpCode::CREATE(info) => info,
            OpCode::CALL(info) => info,
            OpCode::CALLCODE(info) => info,
            OpCode::RETURN(info) => info,
            OpCode::DELEGATECALL(info) => info,
            OpCode::CREATE2(info) => info,
            OpCode::STATICCALL(info) => info,
            OpCode::REVERT(info) => info,
            OpCode::SELFDESTRUCT(info) => info,
            OpCode::NOTOPCODE(info) => info,
        }
    }
}
