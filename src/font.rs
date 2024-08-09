const fn char(rows: [&str; 8]) -> [u8; 6] {
    let mut column_bytes: [u8; 6] = [0, 0, 0, 0, 0, 0];

    let mut col = 0;
    while col < 6 {
        let mut column_byte = 0;

        let mut row = 0;
        while row < 8{
            if rows[row].as_bytes()[col] != ' ' as u8 {
                column_byte |= 1 << row;
            }

            row += 1;
        }

        column_bytes[col] = column_byte;

        col += 1;
    }

    column_bytes
}

pub const FONT: [[u8; 6]; 4] = [
    char(["      ",
          "      ",
          "      ",
          "      ",
          "      ",
          "      ",
          "      ",
          "      "]),
    char(["  #   ",
          " # #  ",
          "#   # ",
          "#   # ",
          "##### ",
          "#   # ",
          "#   # ",
          "      "]),
    char(["####  ",
          "#   # ",
          "#   # ",
          "####  ",
          "#   # ",
          "#   # ",
          "####  ",
          "      "]),
    char([" ###  ",
          "#   # ",
          "#     ",
          "#     ",
          "#     ",
          "#   # ",
          " ###  ",
          "      "]),
];
