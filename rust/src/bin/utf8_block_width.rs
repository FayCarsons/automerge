use automerge::{
    hydrate::Map, iter::Span, marks::UpdateSpansConfig, transaction::Transactable, AutoCommit,
    ObjType, ReadDoc, TextEncoding, ROOT,
};

fn main() {
    let mut doc = AutoCommit::new_with_encoding(TextEncoding::Utf8CodeUnit);
    let text = doc.put_object(ROOT, "text", ObjType::Text).unwrap();
    doc.split_block(&text, 0).unwrap();
    println!("block marker width: {}", doc.length(&text));
    doc.splice_text(&text, 3, 0, "xay").unwrap();
    println!("before: {:?}", doc.text(&text).unwrap());

    let spans = [
        Span::Block(Map::default()),
        Span::Text {
            text: "xy".into(),
            marks: None,
        },
    ];
    doc.update_spans(&text, UpdateSpansConfig::default(), spans)
        .unwrap();

    println!("after:  {:?}", doc.text(&text).unwrap());
}
