const fn mk_font(rows: [&str; 8 * 8]) -> [[u8; 6]; 8] {
    let mut column_bytes: [[u8; 6]; 8] = [[0; 6]; 8];

    let mut char = 0;
    while char < 8 {
        let mut col = 0;
        while col < 6 {
            let mut column_byte = 0;

            let mut row = 0;
            while row < 8{
                let index = row * 8 + char;

                if rows[index].as_bytes()[col] != ' ' as u8 {
                    column_byte |= 1 << row;
                }

                row += 1;
            }

            column_bytes[char][col] = column_byte;

            col += 1;
        }

        char += 1;
    }

    column_bytes
}

pub const FONT: [[u8; 6]; 8] = mk_font([
    "      ", "  #   ", "####  ", " ###  ", "###   ", "##### ", "##### ", " ###  ",
    "      ", " # #  ", "#   # ", "#   # ", "#  #  ", "#     ", "#     ", "#   # ",
    "      ", "#   # ", "#   # ", "#     ", "#   # ", "#     ", "#     ", "#     ",
    "      ", "#   # ", "####  ", "#     ", "#   # ", "####  ", "####  ", "# ### ",
    "      ", "##### ", "#   # ", "#     ", "#   # ", "#     ", "#     ", "#   # ",
    "      ", "#   # ", "#   # ", "#   # ", "#  #  ", "#     ", "#     ", "#   # ",
    "      ", "#   # ", "####  ", " ###  ", "###   ", "##### ", "#     ", " ###  ",
    "      ", "      ", "      ", "      ", "      ", "      ", "      ", "      ",
]);
