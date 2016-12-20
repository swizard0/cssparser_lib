extern crate libc;
extern crate cssparser;

use libc::{c_int, c_char, size_t};
use cssparser::{Parser, RuleListParser, decode_stylesheet_bytes};

const C_RES_OK: c_int = 0;
const C_RES_ERR_INVALID_PARAMETER: c_int = 1;

#[no_mangle]
pub extern fn parse_document(css: *const c_char, css_size: size_t) -> c_int {
    if css.is_null() {
        return C_RES_ERR_INVALID_PARAMETER;
    }

    let css_slice = unsafe { std::slice::from_raw_parts(css as *const u8, css_size as usize) };
    let (css_unicode, encoding) =
        decode_stylesheet_bytes(css_slice, None, None);
    let input = &mut Parser::new(&css_unicode);

    //let rules = RuleListParser::new_for_stylesheet(input, ?).collect::<Vec<_>>();

    C_RES_OK
}
