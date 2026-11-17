#!/usr/bin/env python3
"""Shipped PARB gold/trap scorer.

Both arms (stock llama-cli and worktree niodoo) are scored by this module
and only this module. Tests import these functions; the bank runner does too.

A item is correct iff the model's *final conclusion* matches gold (equivalent
numeric / money forms allowed) and does not also state the trap (hedge = wrong).
"""

from __future__ import annotations

import re
import unicodedata
from typing import Iterable

_ANSI_RE = re.compile(r"\x1b\[[0-9;?]*[A-Za-z]|\x1b\][^\x07]*\x07|\x1b.")
_CTRL_RE = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f\x7f]")

_ANSWER_MARK = re.compile(
    r"(?:final\s+answer|the\s+answer\s+is|correct\s+answer(?:\s+is)?|"
    r"therefore(?:\s*,|\s+the\s+answer\s+is)?|"
    r"so\s+(?:the\s+)?(?:answer\s+is|it\s+(?:would|will|takes|is))|"
    r"(?:^|[.!?]\s+)(?:so|therefore|thus|hence)[,:]?\s+"
    r"|conclusion\s*:|answer\s*:)\s*",
    re.IGNORECASE,
)

_SENTENCE_SPLIT = re.compile(r"(?<=[.!?])\s+")

# Small-cardinal word forms used for gold/trap numeric equivalence.
_SMALL_NUM = {
    "0": "zero",
    "1": "one",
    "2": "two",
    "3": "three",
    "4": "four",
    "5": "five",
    "6": "six",
    "7": "seven",
    "8": "eight",
    "9": "nine",
    "10": "ten",
    "11": "eleven",
    "12": "twelve",
    "13": "thirteen",
}
_WORD_TO_NUM = {w: n for n, w in _SMALL_NUM.items()}

_MONEY_PHRASE = re.compile(
    r"(?:"
    r"\$\s*(\d+(?:\.\d{1,2})?)"
    r"|(\d+(?:\.\d{1,2})?)\s*(cents?|¢|dollars?)"
    r"|(five|ten)\s+cents"
    r")",
    re.IGNORECASE,
)


def strip_control_codes(text: str) -> str:
    if not text:
        return ""
    cleaned = _ANSI_RE.sub("", text)
    cleaned = _CTRL_RE.sub("", cleaned)
    return cleaned.replace("\r\n", "\n").replace("\r", "\n")


def _norm_space(text: str) -> str:
    text = unicodedata.normalize("NFKC", text or "")
    text = text.replace("¢", " cents ").replace("$", " $ ")
    text = re.sub(r"[“”\"'`]", "", text)
    text = re.sub(r"\s+", " ", text).strip()
    return text


def _fold(text: str) -> str:
    return _norm_space(text).casefold()


def extract_final_conclusion(model_text: str) -> str:
    """Last committed answer window — not the whole ramble."""
    text = strip_control_codes(model_text or "")
    # Drop common niodoo post-answer banners if a one-shot dump included them.
    for cut in (
        "===COGNITIVE_TRACE",
        "[REQUEST: LOCK] engaged",
        "Clean shutdown",
        "Process terminated",
    ):
        idx = text.find(cut)
        if idx >= 0:
            text = text[:idx]
    if not text.strip():
        return ""

    # Keep paragraph structure so the last committed block wins. Collapsing
    # whitespace first made last-2-sentences == the whole answer.
    paragraphs = [p.strip() for p in re.split(r"\n\s*\n", text) if p.strip()]
    window = paragraphs[-1] if paragraphs else text.strip()
    window_flat = _norm_space(window)
    marks = list(_ANSWER_MARK.finditer(window_flat))
    if marks:
        tail = window_flat[marks[-1].end() :].strip()
        if tail:
            parts = _SENTENCE_SPLIT.split(tail, maxsplit=1)
            cand = parts[0].strip()
            # "The answer is... it's a paradox." must not commit the ellipsis
            # stub. Punctuation-only tails are not a conclusion.
            if re.search(r"[A-Za-z0-9]", cand):
                return cand
            if len(parts) > 1 and parts[1].strip():
                more = _SENTENCE_SPLIT.split(parts[1].strip(), maxsplit=1)
                if more and re.search(r"[A-Za-z0-9]", more[0]):
                    return more[0].strip()
    sentences = [s.strip() for s in _SENTENCE_SPLIT.split(window_flat) if s.strip()]
    if not sentences:
        return window_flat
    for sent in reversed(sentences):
        low = sent.casefold()
        if low.startswith(("so ", "so,", "therefore", "thus", "hence", "final")):
            return sent
    return sentences[-1]


def _phrase_to_cents(phrase: str) -> int | None:
    folded = _fold(phrase)
    if folded in {"five cents", "5 cents", "5 cent", "5¢"}:
        return 5
    if folded in {"ten cents", "10 cents", "10 cent", "10¢"}:
        return 10
    m = re.fullmatch(
        r"\$?\s*(\d+(?:\.\d{1,2})?)\s*(cents?|dollars?)?",
        folded,
    )
    if not m:
        return None
    amount = float(m.group(1))
    unit = (m.group(2) or "").rstrip("s")
    if unit == "cent" or ("cent" in folded and "dollar" not in folded):
        return int(round(amount))
    if unit == "dollar" or folded.startswith("$") or "." in m.group(1):
        return int(round(amount * 100))
    return None


def _money_amounts_in(text: str) -> list[tuple[int, int, int]]:
    """Return (cents, start, end) spans found in folded text."""
    folded = _fold(text)
    out: list[tuple[int, int, int]] = []
    for m in _MONEY_PHRASE.finditer(folded):
        if m.group(4):
            cents = 5 if m.group(4).lower() == "five" else 10
        elif m.group(1) is not None:
            cents = int(round(float(m.group(1)) * 100))
        else:
            amount = float(m.group(2))
            unit = m.group(3).lower()
            if unit.startswith("cent") or unit == "¢":
                cents = int(round(amount))
            else:
                cents = int(round(amount * 100))
        out.append((cents, m.start(), m.end()))
    return out


def _numeric_variants(phrase: str) -> set[str]:
    folded = _fold(phrase)
    variants = {folded}
    # Isolated integer answers ("3", "12") plus word forms.
    if re.fullmatch(r"-?\d+", folded):
        word = _SMALL_NUM.get(folded)
        if word:
            variants.add(word)
        return variants
    # Leading integer + rest, e.g. "1 hour", "1 sister", "5 minutes".
    m = re.fullmatch(r"(\d+)\s+(.+)", folded)
    if m:
        n, rest = m.group(1), m.group(2)
        variants.add(f"{n} {rest}")
        word = _SMALL_NUM.get(n)
        if word:
            variants.add(f"{word} {rest}")
        if rest in {"hour", "hours", "hr", "hrs"}:
            variants.update(
                {
                    f"{n} hour",
                    f"{n} hours",
                    f"{n} hr",
                    f"{n} hrs",
                }
            )
            if word:
                variants.update({f"{word} hour", f"{word} hours"})
            if n == "1":
                variants.add("60 minutes")
                variants.add("sixty minutes")
        if rest in {"minute", "minutes", "min", "mins"}:
            variants.update({f"{n} minute", f"{n} minutes", f"{n} min", f"{n} mins"})
            if word:
                variants.update({f"{word} minute", f"{word} minutes"})
        if rest in {"sister", "sisters"}:
            variants.update({f"{n} sister", f"{n} sisters"})
            if word:
                variants.update({f"{word} sister", f"{word} sisters"})
    # Word-leading ("one hour").
    m = re.fullmatch(r"(zero|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen)\s+(.+)", folded)
    if m:
        n = _WORD_TO_NUM[m.group(1)]
        variants |= _numeric_variants(f"{n} {m.group(2)}")
    return variants


def gold_trap_variants(phrase: str) -> set[str]:
    folded = _fold(phrase)
    variants = {folded}
    variants |= _numeric_variants(phrase)
    cents = _phrase_to_cents(phrase)
    if cents is not None:
        dollars = cents / 100.0
        variants.update(
            {
                f"{cents} cents",
                f"{cents} cent",
                f"${dollars:.2f}",
                f"{dollars:.2f} dollars",
                f"{dollars:.2f} dollar",
            }
        )
        if cents == 5:
            variants.add("five cents")
        if cents == 10:
            variants.add("ten cents")
    return {_fold(v) for v in variants if v}


def _span_overlaps(span: tuple[int, int], others: Iterable[tuple[int, int]]) -> bool:
    a, b = span
    for c, d in others:
        if a < d and c < b:
            return True
    return False


def _find_phrase_spans(haystack: str, needles: Iterable[str]) -> list[tuple[int, int]]:
    folded = _fold(haystack)
    spans: list[tuple[int, int]] = []
    for needle in needles:
        n = _fold(needle)
        if not n:
            continue
        # Digits / single letters / short yes-no need word boundaries so
        # "3" does not hit "13" and "no" does not hit "know".
        if re.fullmatch(r"-?\d+", n):
            # Block 3⊂13 and 1⊂1.5, but still allow "3." at a sentence end.
            pat = re.compile(rf"(?<![\w.]){re.escape(n)}(?![\w])")
            for m in pat.finditer(folded):
                spans.append((m.start(), m.end()))
            continue
        if re.fullmatch(r"[a-z]", n) or n in {"yes", "no"}:
            pat = re.compile(rf"\b{re.escape(n)}\b")
            for m in pat.finditer(folded):
                spans.append((m.start(), m.end()))
            continue
        start = 0
        while True:
            idx = folded.find(n, start)
            if idx < 0:
                break
            spans.append((idx, idx + len(n)))
            start = idx + max(1, len(n))
    return spans


def score_item(gold: str, trap: str, model_text: str) -> dict:
    """Score one (gold, trap, model_text) triple.

    Returns a dict with: correct, gold_hit, trap_hit, hedge, extracted, reason.
    """
    extracted = extract_final_conclusion(model_text)
    gold_vars = gold_trap_variants(gold)
    trap_vars = gold_trap_variants(trap)

    gold_spans = _find_phrase_spans(extracted, gold_vars)
    trap_spans_raw = _find_phrase_spans(extracted, trap_vars)

    # Money: if gold/trap are amounts, also match parsed currency values.
    gold_cents = _phrase_to_cents(gold)
    trap_cents = _phrase_to_cents(trap)
    if gold_cents is not None or trap_cents is not None:
        for cents, a, b in _money_amounts_in(extracted):
            if gold_cents is not None and cents == gold_cents:
                gold_spans.append((a, b))
            if trap_cents is not None and cents == trap_cents:
                trap_spans_raw.append((a, b))

    gold_hits = gold_spans
    trap_hits = [s for s in trap_spans_raw if not _span_overlaps(s, gold_hits)]

    gold_hit = bool(gold_hits)
    trap_hit = bool(trap_hits)
    hedge = gold_hit and trap_hit
    correct = gold_hit and not trap_hit
    if not extracted.strip():
        reason = "empty_conclusion"
    elif hedge:
        reason = "hedge"
    elif trap_hit:
        reason = "trap"
    elif gold_hit:
        reason = "gold"
    else:
        reason = "neither"
    return {
        "correct": bool(correct),
        "gold_hit": gold_hit,
        "trap_hit": trap_hit,
        "hedge": hedge,
        "extracted": extracted,
        "reason": reason,
    }
