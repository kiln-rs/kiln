use super::case::{assert_case, Case};
use tetherscript::browser::{parse_html, query_selector, text_content};

const CASE: Case = Case {
    area: "html/syntax/parsing",
    wpt_shape: "authored table sections remain distinct after tree construction",
    unsupported: &["complete HTML5 table insertion-mode error recovery"],
};

pub fn run() {
    assert_case(&CASE);
    let doc = parse_html("<table><thead><tr><td>H</td></tr></thead><tbody id='body'><tr><td>A</td></tr></tbody></table>");
    let cells = query_selector(&doc, "td");
    assert_eq!(query_selector(&doc, "table > tbody[id='body']").len(), 1);
    assert_eq!(query_selector(&doc, "table > thead").len(), 1);
    assert_eq!(cells.len(), 2);
    assert_eq!(text_content(&cells[1]), "A");
}
