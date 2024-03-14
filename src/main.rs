// fn char_is_hex(c: &char) -> bool {
//     match c {
//         'a'..='f' | 'A'..='F' | '0'..='9' | 'z' => true,
//         _ => false,
//     }
// }

// fn clean_hex_str(s: &str) -> String {
//     s.chars().filter(char_is_hex).collect()
// }

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

fn hex_to_bin(s: &str) -> String {
    let mut res = String::new();

    for c in s.chars() {
        res.push_str(&char_hex_to_bin(&c));
    }

    res
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
    let mut res = String::new();

    for (i, c) in instruction.code.chars().enumerate() {
        if c == 'a' {
            res.push(code.chars().nth(i).unwrap());
        }
    }

    res = match res.as_str() {
        "000" | "0" => String::from("i"),
        "001" => String::from("d"),
        "010" => String::from("n"),
        "011" => String::from("s"),
        "100" => String::from("sf"),
        "101" | "1" => String::from("x"),
        "110" => String::from("sx"),
        "111" => String::from("sxf"),
        _ => panic!("Invalid addressing mode"),
    };

    res
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

        if instruction.is_short {
            println!("         {assembly_instruction}");
            index += 8;
        } else {
            let address = String::from(&code[index + 8..index + 24]);
            let digit_address = i32::from_str_radix(&address, 2).unwrap();
            let addressing_mode = get_addressing_mode_str(code_slice, instruction);
            println!("         {assembly_instruction: <7} {digit_address:#06X},{addressing_mode: <11} ;{index}");
            index += 24;
        }
    }
}

fn main() {
    let code = "C0 00 00 41 00 7F 31 00 A5 C9 00 A5 B8 04 D2 0C
00 15 70 00 01 41 00 85 31 00 A7 C9 00 A7 78 00
03 89 00 7B B9 00 7D 0C 00 2D 70 00 01 41 00 8B
31 00 A9 CA 00 A9 B8 05 F4 0C 00 3F 70 00 01 41
00 91 31 00 AB C9 00 AB 06 00 77 1B E9 00 52 C8
FF 33 28 00 E1 B8 21 21 0C 00 77 B0 00 03 0C 00
77 41 00 97 39 00 A5 39 00 A7 39 00 A9 39 00 AB
50 00 7D 50 00 0A 00 41 00 9D 00 02 64 05 F3 4E
49 50 31 3A 00 4E 49 50 32 3A 00 4E 49 50 33 3A
00 4E 49 50 34 3A 00 46 4C 41 47 7B 00 45 72 72
65 75 72 0A 00 00 00 00 00 00 00 00 00 zz
";

    let bin = hex_to_bin(code);
    disassemble(&bin);
}
