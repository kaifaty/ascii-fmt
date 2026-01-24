# Migration Plan: Table-Aware Formatting

This document describes how to migrate the current formatter toward table-aware algorithms for
box-drawing ASCII/Unicode tables.

Primary goals:
- Correct alignment with Unicode display widths (emoji, CJK, combining marks).
- Detect table structure (borders, columns, header rows) instead of relying on whitespace hacks.
- Support centering of spanning headers (single-cell headers across the whole table).
- Stretch tables/columns to fill the available container width.

Non-goals (initially):
- Perfect grapheme-cluster rendering parity across all terminals/fonts.
- Full markdown pipe-table support ("| a | b |").
- Complex table features like rowspan/colspan inside the table body.

## Background

Problem examples we want to handle:

1) Spanning header centering (emoji-aware):

```
┌──────────────────────────────────────────────────────────────┐
│                      🐾 КЛАССИФИКАЦИЯ КОТИКОВ 🐾             │
├──────────────────────────────────────────────────────────────┤
│  ┌─────────┬─────────────┬────────────┬──────────────────┐   │
```

2) Column stretching + per-cell alignment:

```
├─────────────────────────────────────────────────────────┤
│  Имя   │ Мурзик │ Пушок  │ Снежок │                     │
├────────┼────────┼────────┼────────┤                     │
│ Возраст│ 3 года │ 2 года │ 1 год  │                     │
├────────┼────────┼────────┼────────┤                     │
│ Любим  │ спать  │ играть │ кушать │                     │
├────────┼────────┼────────┼────────┤                     │
├─────────────────────────────────────────────────────────┤
```

## Design Principles

1) Display width is authoritative.
- Use `unicode-display-width` (via `crate::display_width::display_width`) for all alignment and padding decisions.
- Avoid `len()`, `chars().count()`, and byte offsets for layout decisions.

2) Prefer structure-aware formatting.
- Parse a table block and compute column boundaries.
- Render new border lines and rows based on computed widths.

3) Conservative edits.
- Only modify lines that we confidently identify as a table.
- Keep current heuristics as fallback for non-table boxes.

4) Deterministic output.
- Avoid dependence on terminal runtime features.
- Stable results across machines/CI.

## Proposed Architecture

### New Module: `src/tables.rs`

Responsibilities:
- Detect table blocks within a diagram.
- Parse borders/columns/rows into a lightweight table model.
- Compute layout (column widths) given a target container width.
- Render the table back to lines (box-drawing output).

### Display Width Primitives

Create a small set of reusable helpers (either in `src/utils.rs` or a new `src/display_width.rs`):
- `display_width(s: &str) -> usize`
- `center_to_width(s: &str, width: usize) -> String`
- `ljust_to_width(s: &str, width: usize) -> String`
- `rjust_to_width(s: &str, width: usize) -> String`

These must be used by the table renderer.

## Algorithms

### 1) Spanning Header Centering

Detection heuristic (initial):
- Line begins with a vertical border and ends with a vertical border (`│...│` or `║...║`).
- There are no additional vertical borders inside the cell region.
- The line is adjacent to a top border and/or a header separator.

Formatting:
- Let `inner_width` be the display width between the two outer borders.
- Trim content (but keep internal whitespace), then center via:
  - `pad = inner_width - display_width(content)`
  - `left = pad / 2`, `right = pad - left`
  - `" " * left + content + " " * right`

### 2) Column Width Calculation (Stretch to Fill)

Baseline algorithm (Phase 1):
- `min_width[col] = max(display_width(cell_content)) + 2 * cell_padding`
- `extra = target_inner_width - sum(min_widths) - separators_width`
- Distribute `extra` evenly:
  - `add = extra / ncols`, `rem = extra % ncols`
  - `width[col] = min_width[col] + add + (col < rem ? 1 : 0)`

Optional Phase 2 (closer to comfy-table Dynamic/DynamicFullWidth):
- Apply lower/upper bounds.
- Iteratively fix columns that need less than current average and re-distribute remaining width.
- For FullWidth mode, distribute leftover width across all columns.

Reference implementation ideas:
- comfy-table Dynamic/DynamicFullWidth arrangement:
  https://docs.rs/comfy-table/latest/src/comfy_table/utils/arrangement/dynamic.rs

### 3) Cell Alignment

Initial rules:
- Header rows: center alignment.
- Body rows: left alignment.

Implementation:
- For each cell, compute `content_width_target = col_width - 2 * padding`.
- Apply `center_to_width`/`ljust_to_width`/`rjust_to_width`.

## Integration Plan (Phases)

### Phase A: Tests + Primitives
- Add focused fixtures for:
  - spanning header centering with emoji
  - column stretching inside a container
  - nested table inside an outer box line
- Add display-width helper functions.

### Phase B: Table Detection + Minimal Renderer
- Detect table blocks by scanning for box-drawing border patterns.
- Parse vertical boundaries from border lines (positions of `┬┼┴` junctions / `│`).
- Render borders and content lines based on computed widths.

### Phase C: Container-Aware Stretching
- If a table is inside an outer box `│ ... │`, use the container inner width as target.
- Ensure right padding/"tail" is eliminated by expanding table to full width.

### Phase D: Header Detection Improvements
- Identify spanning headers between top border and first separator.
- Identify header row blocks (rows above first `┼` separator) and center them.

### Phase E: Cleanup / Fallback Behavior
- Keep existing box alignment heuristics for non-table boxes.
- Ensure table formatting does not fight with other passes (skip already-handled lines).

## Verification

Commands:
- `cargo fmt`
- `cargo clippy -- -D warnings`
- `cargo test`

Acceptance criteria:
- `kitties.md` and other markdown fixtures produce centered spanning headers (emoji-aware).
- Tables stretch to fill the container width, with aligned borders and stable output.
- No clippy warnings, tests pass.
