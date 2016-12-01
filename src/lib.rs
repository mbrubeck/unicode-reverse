#![no_std]

extern crate unicode_segmentation;

use core::slice;
use core::str;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse_grapheme_clusters_in_place(s: &mut str) {
    // Part 1: Reverse the bytes within each grapheme cluster.
    // This does not preserve UTF-8 validity. We must guarantee this `reverse` is
    // undone before the data is accessed as `str` again.
    {
        let mut tail = &mut s[..];
        loop {
            // Advance to the next grapheme cluster:
            let len = match tail.graphemes(true).next() {
                Some(grapheme) => grapheme.len(),
                None => break
            };
            let (head, new_tail) = {tail}.split_at_mut(len);
            tail = new_tail;

            // Reverse the bytes within this grapheme cluster.
            let bytes = unsafe {
                let head = head;
                // This is safe because `head` is &mut str so guaranteed not to be aliased.
                slice::from_raw_parts_mut(head.as_ptr() as *mut u8, head.len())
            };
            bytes.reverse();
        }
    }

    // Part 2: Reverse all the bytes.
    // This un-reverses all of the reversals from Part 1.
    let bytes = unsafe {
        let s = s;
        // This is safe because `s` is &mut str so guaranteed not to be aliased.
        slice::from_raw_parts_mut(s.as_ptr() as *mut u8, s.len())
    };
    bytes.reverse();

    // Each UTF-8 sequence is now in the right order.
    debug_assert!(str::from_utf8(bytes).is_ok());
}

#[cfg(test)]
mod tests {
    use super::reverse_grapheme_clusters_in_place;

    extern crate std;
    use self::std::string::ToString;

    fn test_rev(a: &str, b: &str) {
        let mut a = a.to_string();
        reverse_grapheme_clusters_in_place(&mut a);
        assert_eq!(a, b);
    }

    #[test]
    fn test_ascii() {
        test_rev("Hello", "olleH");
    }

    #[test]
    fn test_utf8() {
        test_rev("¡Hola!", "!aloH¡");
    }

    #[test]
    fn test_emoji() {
        test_rev("\u{1F36D}\u{1F36E}", "\u{1F36E}\u{1F36D}");
    }

    #[test]
    fn test_combining_mark() {
        test_rev("man\u{0303}ana", "anan\u{0303}am");
    }
}
