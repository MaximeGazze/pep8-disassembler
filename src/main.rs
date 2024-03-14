const INSTRUCTIONS: [Instruction; 39] = [
    Instruction {
        code: "00000000",
        instruction: "STOP",
        is_short: true,
    },
    Instruction {
        code: "00000001",
        instruction: "RETTR",
        is_short: true,
    },
    Instruction {
        code: "00000010",
        instruction: "MOVSPA",
        is_short: true,
    },
    Instruction {
        code: "00000011",
        instruction: "MOVFLGA",
        is_short: true,
    },
    Instruction {
        code: "0000010a",
        instruction: "BR",
        is_short: false,
    },
    Instruction {
        code: "0000011a",
        instruction: "BRLE",
        is_short: false,
    },
    Instruction {
        code: "0000100a",
        instruction: "BRLT",
        is_short: false,
    },
    Instruction {
        code: "0000101a",
        instruction: "BREQ",
        is_short: false,
    },
    Instruction {
        code: "0000110a",
        instruction: "BRNE",
        is_short: false,
    },
    Instruction {
        code: "0000111a",
        instruction: "BRGE",
        is_short: false,
    },
    Instruction {
        code: "0001000a",
        instruction: "BRGT",
        is_short: false,
    },
    Instruction {
        code: "0001001a",
        instruction: "BRV",
        is_short: false,
    },
    Instruction {
        code: "0001010a",
        instruction: "BRC",
        is_short: false,
    },
    Instruction {
        code: "0001011a",
        instruction: "CALL",
        is_short: false,
    },
    Instruction {
        code: "0001100r",
        instruction: "NOTr",
        is_short: true,
    },
    Instruction {
        code: "0001101r",
        instruction: "NEGr",
        is_short: true,
    },
    Instruction {
        code: "0001110r",
        instruction: "ASLr",
        is_short: true,
    },
    Instruction {
        code: "0001111r",
        instruction: "ASRr",
        is_short: true,
    },
    Instruction {
        code: "0010000r",
        instruction: "ROLr",
        is_short: true,
    },
    Instruction {
        code: "0010001r",
        instruction: "RORr",
        is_short: true,
    },
    Instruction {
        code: "001001nn",
        instruction: "NOPn",
        is_short: true,
    },
    Instruction {
        code: "00101aaa",
        instruction: "NOP",
        is_short: false,
    },
    Instruction {
        code: "00110aaa",
        instruction: "DECI",
        is_short: false,
    },
    Instruction {
        code: "00111aaa",
        instruction: "DECO",
        is_short: false,
    },
    Instruction {
        code: "01000aaa",
        instruction: "STRO",
        is_short: false,
    },
    Instruction {
        code: "01001aaa",
        instruction: "CHARI",
        is_short: false,
    },
    Instruction {
        code: "01010aaa",
        instruction: "CHARO",
        is_short: false,
    },
    Instruction {
        code: "01011nnn",
        instruction: "RETn",
        is_short: true,
    },
    Instruction {
        code: "01100aaa",
        instruction: "ADDSP",
        is_short: false,
    },
    Instruction {
        code: "01101aaa",
        instruction: "SUBSP",
        is_short: false,
    },
    Instruction {
        code: "0111raaa",
        instruction: "ADDr",
        is_short: false,
    },
    Instruction {
        code: "1000raaa",
        instruction: "SUBr",
        is_short: false,
    },
    Instruction {
        code: "1001raaa",
        instruction: "ANDr",
        is_short: false,
    },
    Instruction {
        code: "1010raaa",
        instruction: "ORr",
        is_short: false,
    },
    Instruction {
        code: "1011raaa",
        instruction: "CPr",
        is_short: false,
    },
    Instruction {
        code: "1100raaa",
        instruction: "LDr",
        is_short: false,
    },
    Instruction {
        code: "1101raaa",
        instruction: "LDBYTEr",
        is_short: false,
    },
    Instruction {
        code: "1110raaa",
        instruction: "STr",
        is_short: false,
    },
    Instruction {
        code: "1111raaa",
        instruction: "STBYTEr",
        is_short: false,
    },
];

struct Instruction {
    code: &'static str,
    instruction: &'static str,
    is_short: bool,
}

fn char_hex_to_bin(c: &char) -> String {
    let res = match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' | 'a' => "1010",
        'B' | 'b' => "1011",
        'C' | 'c' => "1100",
        'D' | 'd' => "1101",
        'E' | 'e' => "1110",
        'F' | 'f' => "1111",
        _ => "",
    };

    String::from(res)
}

fn hex_to_bin(s: String) -> String {
    s.chars().map(|c| char_hex_to_bin(&c)).collect()
}

fn code_matches(instruction_code: &str, code: &str) -> bool {
    let instruction_prefix: String = instruction_code
        .chars()
        .filter(|c| c == &'0' || c == &'1')
        .collect();

    code.starts_with(&instruction_prefix)
}

fn get_register_char(code: &str, instruction: &Instruction) -> char {
    if let Some(index) = instruction.code.chars().position(|c| c == 'r') {
        match code.chars().nth(index) {
            Some('0') => 'A',
            Some('1') => 'X',
            _ => panic!("Invalid register"),
        }
    } else {
        '_'
    }
}

fn get_n_param_string(code: &str, instruction: &Instruction) -> String {
    let mut res = String::new();

    for (i, c) in instruction.code.chars().enumerate() {
        if c == 'n' {
            res.push(code.chars().nth(i).unwrap());
        }
    }

    res
}

fn get_addressing_mode_str(code: &str, instruction: &Instruction) -> String {
    let mut addressing_mode_bin = String::new();

    for (i, c) in instruction.code.chars().enumerate() {
        if c == 'a' {
            addressing_mode_bin.push(code.chars().nth(i).unwrap());
        }
    }

    let res = match addressing_mode_bin.as_str() {
        "000" | "0" => "i",
        "001" => "d",
        "010" => "n",
        "011" => "s",
        "100" => "sf",
        "101" | "1" => "x",
        "110" => "sx",
        "111" => "sxf",
        _ => panic!("Invalid addressing mode"),
    };

    String::from(res)
}

fn get_next_instruction(instruction: &str) -> Option<&Instruction> {
    INSTRUCTIONS
        .iter()
        .find(|it| code_matches(it.code, instruction))
}

fn disassemble(code: &str) {
    let mut index = 0;

    while index < code.len() {
        let code_slice = &code[index..index + 8];
        let Some(instruction) = get_next_instruction(code_slice) else {
            panic!("Invalid instruction");
        };

        let assembly_instruction = String::from(instruction.instruction)
            .replace('r', &get_register_char(code_slice, instruction).to_string())
            .replace('n', &get_n_param_string(code_slice, instruction));

        let code_slice_dec = i32::from_str_radix(&code_slice, 2).unwrap();

        if instruction.is_short {
            println!(
                "         {assembly_instruction: <27};{index: <5} {0: <#5X} | {code_slice: <24} | {code_slice_dec:0<2X}",
                index / 8
            );

            index += 8;
        } else {
            let address = String::from(&code[index + 8..index + 24]);
            let digit_address = i32::from_str_radix(&address, 2).unwrap();
            let addressing_mode = get_addressing_mode_str(code_slice, instruction);

            let address_start = i32::from_str_radix(&address[0..8], 2).unwrap();
            let address_end = i32::from_str_radix(&address[8..16], 2).unwrap();

            println!(
                "         {assembly_instruction: <7} {digit_address:#06X},{addressing_mode: <11} ;{index: <5} {0: <#5X} | {code_slice}{address} | {code_slice_dec:0<2X} {address_start:0<2X} {address_end:0<2X}",
                index / 8
            );

            index += 24;
        }
    }
}

fn main() {
    if let Some(input_file_path) = std::env::args().nth(1) {
        let code = std::fs::read_to_string(input_file_path).expect("Invalid file path");
        let bin = hex_to_bin(code);
        disassemble(&bin);
    } else {
        panic!("No input file provided");
    }
}
