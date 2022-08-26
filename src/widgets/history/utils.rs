pub(crate) fn san_to_fan(san: String, white_player: bool) -> String {
    let san_as_vector: Vec<char> = san.chars().collect();
    let mut first_occurence_index = None;

    for i in 0..san_as_vector.len() {
        let current_char = san_as_vector[i];
        let match_expected_char = "NBRQK".contains(current_char);

        if match_expected_char {
            first_occurence_index = Some(i);
            break;
        }
    }

    if let Some(first_occurence_index) = first_occurence_index {
        let element = san_as_vector[first_occurence_index];
        let substitute = match element {
            'N' => knight_text(white_player),
            'B' => bishop_text(white_player),
            'R' => rook_text(white_player),
            'Q' => queen_text(white_player),
            'K' => king_text(white_player),
            _ => panic!("Should not happen"),
        };

        let first_part = san_as_vector.iter().take(first_occurence_index);
        let last_part = san_as_vector.iter().skip(first_occurence_index + 1);

        let first_part = String::from_iter(first_part);
        let last_part = String::from_iter(last_part);

        format!("{}{}{}", first_part, substitute, last_part)
    } else {
        san
    }
}

fn knight_text(white: bool) -> char {
    if white {
        '\u{2658}'
    } else {
        '\u{265e}'
    }
}

fn bishop_text(white: bool) -> char {
    if white {
        '\u{2657}'
    } else {
        '\u{265d}'
    }
}

fn rook_text(white: bool) -> char {
    if white {
        '\u{2656}'
    } else {
        '\u{265c}'
    }
}

fn queen_text(white: bool) -> char {
    if white {
        '\u{2655}'
    } else {
        '\u{265b}'
    }
}

fn king_text(white: bool) -> char {
    if white {
        '\u{2654}'
    } else {
        '\u{265a}'
    }
}
