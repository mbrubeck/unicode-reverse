extern crate afl;
extern crate unicode_segmentation;
extern crate unicode_reverse;

use unicode_segmentation::UnicodeSegmentation;
use unicode_reverse::reverse_grapheme_clusters_in_place;

fn main() {
    afl::handle_string(|mut a| {
        let b: String = a.graphemes(true).rev().collect();
        reverse_grapheme_clusters_in_place(&mut a);
        assert_eq!(a, b);
    });
}
