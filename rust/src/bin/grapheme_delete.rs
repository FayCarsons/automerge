use automerge::{
    iter::Span, marks::UpdateSpansConfig, transaction::Transactable, AutoCommit, ObjType,
    ReadDoc, ROOT,
};

fn main() {
    for before in ["xe\u{301}y", "x🇫🇷y", "x👍🏽y"] {
        let mut doc = AutoCommit::new();
        let text = doc.put_object(ROOT, "text", ObjType::Text).unwrap();
        doc.splice_text(&text, 0, 0, before).unwrap();

        let span = Span::Text {
            text: "xy".into(),
            marks: None,
        };
        doc.update_spans(&text, UpdateSpansConfig::default(), [span])
            .unwrap();

        println!("{before:?} -> {:?}", doc.text(&text).unwrap());
    }
}
