/**
 * TRAY OWNERSHIP (COMPOSE-039, PICK-013): a tray belongs to the conversation it was staged in.
 *
 * A send finishes asynchronously; by then the person may have opened another chat (or left and
 * returned to the same one). Only a completion that still stands in the tray's own conversation
 * and in the same visit of it may put failed files back. Anything else is dropped: a file staged
 * in one chat is never offered to another chat's recipient.
 */

/** Whether a completion still belongs to the tray it started in. */
export function sameTrayOwner(owner, current, ownerEpoch, currentEpoch) {
  return owner === current && ownerEpoch === currentEpoch;
}

/**
 * The files a finished send may put back into the tray: the failed ones, in their order, when the
 * tray is still the same conversation and the same visit; none otherwise.
 */
export function restoredAfterFailure({ owner, current, ownerEpoch, currentEpoch, failed }) {
  if (!failed || !failed.length) return [];
  if (!sameTrayOwner(owner, current, ownerEpoch, currentEpoch)) return [];
  return [...failed];
}

/** The failed plates themselves, in their order, from their indices (a tray keeps plates, never indices). */
export function platesBack(items, failed) {
  return items.filter((_, i) => failed.includes(i));
}

/** The tray without the plates a send took; plates staged meanwhile stay (they belong to the next send). */
export function withoutPlates(tray, sentIds) {
  return tray.filter((plate) => !sentIds.includes(plate.id));
}

/** Whether a returned first plate should bring its caption and quote back (the field is still empty). */
export function restoresCaption(back, firstPlateId, draftNow) {
  return back.some((plate) => plate.id === firstPlateId) && !String(draftNow || "").trim();
}

/** Whether a returned first plate may bring its quote back: only when no other reply was chosen meanwhile. */
export function restoresReply(back, firstPlateId, currentReply) {
  return back.some((plate) => plate.id === firstPlateId) && !currentReply;
}

/** Whether a completion still stands in the chat and visit that own the composer it would clear. */
export function sameComposer(owner, current, ownerEpoch, currentEpoch) {
  return owner === current && ownerEpoch === currentEpoch;
}

/** The words a plate carries: those it was staged with, or the composer's for a plate that has none yet. */
export function plateWords(saved, draft, reply) {
  return saved ?? { caption: draft, reply };
}

/** The words each plate carries, in order: its saved words, the composer's for the first, none for the rest. */
export function wordsForPlates(items, saved, draft, reply) {
  return items.map((it, i) => saved.get(it.id) ?? (i === 0 ? { caption: draft, reply } : { caption: "", reply: null }));
}

/** Whether the field still holds the words a send carried (a draft typed meanwhile is not deleted). */
export function sameWords(draft, reply, words) {
  return String(draft).trim() === words.caption && reply === words.reply;
}
