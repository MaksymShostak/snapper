//! Type-6 void tags (`<hr>`, `<col>`, …) have no closer. Leftover prose
//! after the opener must stay Prose and still split, same class as
//! snapper-v85k closed `<div>…</div>`.

use snapper_fmt::format::Format;
use snapper_fmt::parser::markdown::MarkdownParser;
use snapper_fmt::parser::{FormatParser, Region};
use snapper_fmt::{FormatConfig, format_text};

fn md_cfg() -> FormatConfig {
    FormatConfig {
        format: Format::Markdown,
        max_width: 0,
        ..Default::default()
    }
    .without_safety_backstops()
}

fn ticket_fixture() -> &'static str {
    concat!(
        "Intro sentence here. Another intro sentence.\n",
        "<hr>\n",
        "After html. Next.\n",
    )
}

fn expected_ticket() -> &'static str {
    concat!(
        "Intro sentence here.\n",
        "Another intro sentence.\n",
        "<hr>\n",
        "After html.\n",
        "Next.\n",
    )
}

#[test]
fn void_hr_is_structure_through_opener() {
    let regions = MarkdownParser.parse(ticket_fixture());
    let hr = regions.iter().find_map(|r| match r {
        Region::Structure(s) if s.contains("<hr>") => Some(s.as_str()),
        _ => None,
    });
    let hr = hr.expect(&format!("hr block must be Structure, got {regions:?}"));
    assert!(hr.contains("<hr>"), "{hr}");
    assert!(
        !hr.contains("After html"),
        "void type-6 must end at <hr>, got {hr}"
    );
}

#[test]
fn after_html_stays_prose() {
    let regions = MarkdownParser.parse(ticket_fixture());
    assert!(
        regions.iter().any(|r| matches!(
            r,
            Region::Prose(p) if p.contains("After html.") && p.contains("Next.")
        )),
        "After html. / Next. must stay Prose, got {regions:?}"
    );
}

#[test]
fn ticket_fixture_keeps_hr_and_splits_next() {
    let input = ticket_fixture();
    let out = format_text(input, &md_cfg()).unwrap();
    assert_eq!(out, expected_ticket(), "got:\n{out}");
    assert_eq!(format_text(&out, &md_cfg()).unwrap(), out);
}

#[test]
fn closed_div_still_ends_at_close() {
    let input = concat!(
        "Intro sentence here. Another intro sentence.\n",
        "<div>\n",
        "First. Second.\n",
        "</div>\n",
        "After html. Next.\n",
    );
    let out = format_text(input, &md_cfg()).unwrap();
    assert!(
        out.contains("<div>\nFirst. Second.\n</div>\nAfter html.\nNext.\n"),
        "closed type-6 must stay raw through </div>, got:\n{out}"
    );
}
