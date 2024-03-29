use ttf_parser::Face;

pub struct CharPixelDensityBuilder<'b, 'a> {
    chars: Option<&'a [char]>,
    font_bytes: Option<&'a Face<'b>>,
}