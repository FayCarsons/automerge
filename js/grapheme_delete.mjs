import * as A from "@automerge/automerge";

const show = (s) =>
  JSON.stringify(s).replace(/\p{M}/gu, (c) => `\\u{${c.codePointAt(0).toString(16)}}`);

for (const before of ["xéy", "x🇫🇷y", "x👍🏽y"]) {
  let doc = A.from({ text: before });
  doc = A.change(doc, (d) => {
    A.updateSpans(d, ["text"], [{ type: "text", value: "xy" }]);
  });
  console.log(show(before), "->", show(doc.text));
}
