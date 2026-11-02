import test from "node:test";
import assert from "node:assert/strict";
import { platesBack, plateWords, restoredAfterFailure, sameComposer, sameTrayOwner, sameWords, withoutPlates, wordsForPlates } from "../src/tray.js";

const plate = (id, name) => ({ id, file: { name, size: 3 }, kind: "doc" });

test("a successful send puts nothing back", () => {
  assert.deepEqual(restoredAfterFailure({ owner: "A", current: "A", ownerEpoch: 1, currentEpoch: 1, failed: [] }), []);
});

test("a failure in the same chat and visit comes back in its order", () => {
  assert.deepEqual(
    restoredAfterFailure({ owner: "A", current: "A", ownerEpoch: 1, currentEpoch: 1, failed: [2, 0] }),
    [2, 0],
  );
});

test("a late failure after switching to another chat is dropped, never offered to B", () => {
  assert.deepEqual(
    restoredAfterFailure({ owner: "A", current: "B", ownerEpoch: 1, currentEpoch: 1, failed: [0] }),
    [],
  );
});

test("a late failure after reopening the same chat is dropped too (new visit)", () => {
  assert.deepEqual(
    restoredAfterFailure({ owner: "A", current: "A", ownerEpoch: 1, currentEpoch: 2, failed: [0] }),
    [],
  );
});

test("ownership is both the chat and the visit", () => {
  assert.equal(sameTrayOwner("A", "A", 1, 1), true);
  assert.equal(sameTrayOwner("A", "B", 1, 1), false);
  assert.equal(sameTrayOwner("A", "A", 1, 2), false);
});

test("a switch during send does not change the new chat's tray", () => {
  let trayB = [];
  const failed = restoredAfterFailure({ owner: "A", current: "B", ownerEpoch: 1, currentEpoch: 1, failed: [0, 1] });
  if (failed.length) trayB = [...failed, ...trayB];
  assert.deepEqual(trayB, []);
});

test("a repeat send restores only its own failure once", () => {
  const first = restoredAfterFailure({ owner: "A", current: "A", ownerEpoch: 1, currentEpoch: 1, failed: [0] });
  const second = restoredAfterFailure({ owner: "A", current: "A", ownerEpoch: 1, currentEpoch: 1, failed: [] });
  assert.deepEqual(first, [0]);
  assert.deepEqual(second, []);
});

test("a failed plate comes back as a plate, so a retry sends its file", () => {
  const items = [plate("p1", "one.pdf"), plate("p2", "two.pdf")];
  const back = platesBack(items, [1]);
  assert.equal(back.length, 1);
  assert.equal(back[0].id, "p2");
  assert.equal(back[0].file.size, 3); // not an index: a retry can read .size and send it
});

test("plates staged while a send is finishing stay in the tray", () => {
  const tray = [plate("sent", "a.pdf"), plate("new", "b.pdf")];
  const left = withoutPlates(tray, ["sent"]);
  assert.deepEqual(left.map((p) => p.id), ["new"]);
});



test("a late preflight never clears another chat's composer", () => {
  assert.equal(sameComposer("A", "A", 1, 1), true);
  assert.equal(sameComposer("A", "B", 1, 1), false);
  assert.equal(sameComposer("A", "A", 1, 2), false);
});

test("a plate keeps the words it was staged with, another plate gets its own", () => {
  const first = plateWords({ caption: "A", reply: { mid: "M1" } }, "", null);
  assert.deepEqual(first, { caption: "A", reply: { mid: "M1" } });
  const second = plateWords(undefined, "B", null);
  assert.deepEqual(second, { caption: "B", reply: null });
});

test("reordering returned plates keeps each plate's own words", () => {
  const saved = new Map([["A", { caption: "caption A", reply: { mid: "M1" } }]]);
  const words = wordsForPlates([{ id: "B" }, { id: "A" }], saved, "", null);
  assert.deepEqual(words[0], { caption: "", reply: null });
  assert.deepEqual(words[1], { caption: "caption A", reply: { mid: "M1" } });
});

test("a draft typed during a retry is not the send's words", () => {
  assert.equal(sameWords("new draft", null, { caption: "old", reply: null }), false);
  assert.equal(sameWords("old", null, { caption: "old", reply: null }), true);
});

test("whitespace around an unchanged caption still clears the field", () => {
  assert.equal(sameWords("  caption  ", null, { caption: "caption", reply: null }), true);
});
