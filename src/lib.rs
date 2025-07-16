use std::cmp::max;

use itertools::Itertools;

const CLIPPY: &str = "
⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣤⣤⣄⣀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢠⣶⠟⣉⣤⣢⣄⡪⢝⢦⡀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢰⡿⢁⣾⠟⠉⠉⠉⠹⣧⣃⢳⡀⠀⠀⠀
⠀⠀⠀⢀⣀⣼⡏⣼⠃⠀⠀⠀⠀⠀⢹⣏⣸⡅⠀⠀⠀
⠀⢀⣴⡿⠿⣿⠃⣿⠀⠀⠀⠀⠀⠀⣸⣷⣿⣶⣄⠀⠀
⠠⠞⠁⠀⢠⣿⠌⣿⠀⠀⠀⠀⠀⠀⣿⡇⣿⠛⠛⠿⣄
⠀⠀⢀⣠⠾⠿⠾⣷⡀⠀⠀⠀⡠⢶⠛⠹⠿⢶⣄⠀⠈
⠀⢠⠋⠀⢀⣁⡀⠘⠙⣦⡀⠘⠈⠀⣠⣤⡀⠀⠻⣦⠀
⠀⢀⠀⠀⢾⣿⣿⠀⠀⢘⣧⠇⡀⠘⢿⣿⠏⠀⠀⡿⠀
⠀⠈⢧⡀⠈⣉⡁⠀⣤⡞⠀⠘⢢⣀⡄⠀⢠⣠⠾⠃⠀
⠀⠀⠀⠉⣷⡖⣶⡛⠉⠀⠀⠀⠀⣿⡏⣿⠋⠁⠀⠀⠀
⠀⠀⠀⠀⢻⡇⣽⢺⣱⡄⠀⠀⠀⣿⢇⡏⠀⠀⣰⡖⣦
⠀⠀⠀⠀⣿⡇⣿⢻⠸⡇⠀⠀⠀⣿⢰⡏⢀⣾⢳⡾⠉
⠀⠀⠀⠀⣿⡄⡿⣿⠘⡁⠀⠀⠐⣿⢸⡇⣾⢇⡿⠀⠀
⠀⠀⠀⠀⣿⠐⣟⣧⢰⠀⠀⠀⢸⣿⢺⠆⣿⢸⡇⠀⠀
⠀⠀⠀⠀⣿⠡⣟⣿⢸⡇⠀⠀⢸⣇⢿⠆⣿⢸⡅⠀⠀
⠀⠀⠀⠀⣿⠡⣏⣿⡸⡅⠀⠀⣼⢏⣼⠆⣿⢸⠃⠀⠀
⠀⠀⠀⠀⣿⠰⣿⠹⣶⣭⣖⣪⣵⡾⠏⢠⣿⢸⡁⠀⠀
⠀⠀⠀⠀⣿⢂⡷⠀⠈⠉⠘⠉⠉⠀⠀⠸⣿⢼⡀⠀⠀
⠀⠀⠀⠀⣿⡍⢿⡀⠀⠀⠀⠀⠀⠀⠀⣸⠇⣼⠀⠀⠀
⠀⠀⠀⠀⠹⣯⡎⡻⢦⣀⣀⣀⣀⡤⠞⣉⣼⠃⠀⠀⠀
⠀⠀⠀⠀⠀⠈⠻⢷⣦⣢⣬⣤⣤⣶⠾⠋⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀
";

const TEXT_BUBBLE_LEFT_SIDE: &str = "   ⣿⡇  ";
const TEXT_BUBBLE_TOP_LEFT: &str = "   ⣴⡾⠿";
const TEXT_BUBBLE_TOP_SIDE: &str = "⠿";
const TEXT_BUBBLE_TOP_RIGHT: &str = "⠿⢷⣦\n";
const TEXT_BUBBLE_RIGHT_SIDE: &str = "  ⢸⣿\n";
const TEXT_BUBBLE_BOTTOM_RIGHT: &str = "⣾⠟\n";
const TEXT_BUBBLE_BOTTOM_SIDE: &str = "⣶";
const TEXT_BUBBLE_BOTTOM_LEFT: &str = "   ⠙⣿⡆⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣶";
const TEXT_BUBBLE_TAIL: &str = "⠀⠀⠀⢰⣿⠀⠀⠀⠀⠀⢀⣠⣾⠟⠋
⠀⠀⣠⣿⠃⠀⢀⣠⣤⣾⠟⠋
⠀⠀⢿⣷⡾⠿⠟⠛⠉
";

pub fn clippy() -> String {
    CLIPPY.to_string()
}

pub fn clippy_says(text: impl ToString) -> String {
    let mut clippy = CLIPPY.to_string();

    // Create the speech bubble
    let speech_bubble = says(text);

    // Get the heights of the two pieces of text to combine.
    let clippy_height = clippy.lines().count();
    let speech_bubble_height = speech_bubble.lines().count();

    // Add newlines to the top of clippy if the speech bubble is taller than him. We don't want the
    // speech bubble's tail to be below him.
    let difference = speech_bubble_height.saturating_sub(clippy_height);
    for _ in 0..difference {
        clippy.insert(0, '\n');
    }

    horizontal_stack(clippy, speech_bubble)
}

fn horizontal_stack(str1: impl ToString, str2: impl ToString) -> String {
    // Split both strings in to columns from the contained newlines.
    let stack1: Vec<String> = str1.to_string().lines().map_into().collect_vec();
    let stack2: Vec<String> = str2.to_string().lines().map_into().collect_vec();
    let tallest_stack_height = max(stack1.len(), stack2.len());

    // Get the longest line from the first stack. All lines from `str2` must start being added at
    // this index.
    let stack1_longest_line_length = longest_line_size(&str1.to_string());

    let mut output = String::new();
    for i in 0..tallest_stack_height {
        let stack1_str = stack1.get(i).map_or("", |v| v);
        let stack2_str = stack2.get(i).map_or("", |v| v);

        // Get the number of spaces to add to the line
        let spaces_to_add = stack1_longest_line_length - stack1_str.len();

        // Create the output for this line
        output.push_str(stack1_str);
        output.push_str(&" ".repeat(spaces_to_add));
        output.push_str(stack2_str);
        output.push('\n');
    }
    // Remove the last newline
    output.pop();

    output
}

pub fn says(text: impl ToString) -> String {
    let text = text.to_string();
    // Get the length of the longest line
    let full_width = longest_line_size(&text);

    let mut output = String::new();
    output.push_str(&top(full_width));
    output.push_str(&empty_line(full_width));

    for line in text.lines() {
        output.push_str(&speech_line(line, full_width));
    }
    output.push_str(&empty_line(full_width));
    output.push_str(&bottom(full_width));
    output
}

fn top(text_width: usize) -> String {
    let top_side = TEXT_BUBBLE_TOP_SIDE.repeat(text_width);
    format!("{TEXT_BUBBLE_TOP_LEFT}{top_side}{TEXT_BUBBLE_TOP_RIGHT}")
}

fn bottom(text_width: usize) -> String {
    let speech_bubble_width =
        TEXT_BUBBLE_LEFT_SIDE.len() + text_width + TEXT_BUBBLE_RIGHT_SIDE.len();
    let bottom_sides_needed = speech_bubble_width
        .saturating_sub(longest_line_size(TEXT_BUBBLE_BOTTOM_LEFT))
        .saturating_sub(TEXT_BUBBLE_BOTTOM_RIGHT.len());

    let bottom_side = TEXT_BUBBLE_BOTTOM_SIDE.repeat(bottom_sides_needed);
    format!("{TEXT_BUBBLE_BOTTOM_LEFT}{bottom_side}{TEXT_BUBBLE_BOTTOM_RIGHT}{TEXT_BUBBLE_TAIL}")
}

fn speech_line(text: &str, text_width: usize) -> String {
    let mut padded_text = text.to_string();
    padded_text.push_str(&" ".repeat(text_width.saturating_sub(padded_text.len() + 1)));
    format!("{TEXT_BUBBLE_LEFT_SIDE}{padded_text}{TEXT_BUBBLE_RIGHT_SIDE}",)
}

fn empty_line(text_width: usize) -> String {
    speech_line("", text_width)
}

fn longest_line_size(text: &str) -> usize {
    text.lines().map(|s| s.len()).max().map_or(0, |v| v)
}

#[cfg(test)]
mod test {
    use super::{horizontal_stack, longest_line_size, says};
    use test_case::test_case;

    #[test_case("hello", 5; "with 1 line")]
    #[test_case("hello\ngoodbye", 7; "with 2 lines")]
    #[test_case("hello\ngoodbye\n", 7; "with extra newline")]
    fn longest_line(text: &str, expected: usize) {
        let actual = longest_line_size(text);
        assert_eq!(expected, actual);
    }

    #[test_case("123\n123", "456\n456", "123456\n123456"; "with no formatting")]
    #[test_case("12345\n123", "678\n456", "12345678\n123  456"; "with formatting")]
    #[test_case("123\n123\n123", "456\n456", "123456\n123456\n123"; "with more lines in str1")]
    #[test_case("123\n123", "456\n456\n456", "123456\n123456\n   456"; "with more lines in str2")]
    fn stack(str1: &str, str2: &str, expected: &str) {
        let actual = horizontal_stack(str1, str2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn speech_bubble() {
        let text = "This is a test message";
        let expected = format!(
            "   ⣴⡾⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⢷⣦
   ⣿⡇                         ⢸⣿
   ⣿⡇  {text}  ⢸⣿
   ⣿⡇                         ⢸⣿
   ⠙⣿⡆⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣾⠟
⠀⠀⠀⢰⣿⠀⠀⠀⠀⠀⢀⣠⣾⠟⠋
⠀⠀⣠⣿⠃⠀⢀⣠⣤⣾⠟⠋
⠀⠀⢿⣷⡾⠿⠟⠛⠉⠀⠀⠀⠀⠀⠀⠀
"
        );
        let actual = says(text);

        println!("{expected}");
        println!("{actual}");

        assert_eq!(expected, actual)
    }
}
