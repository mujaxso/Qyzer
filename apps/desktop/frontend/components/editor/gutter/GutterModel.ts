//! Pure model for the line-number gutter.
//! Computes visible line range, gutter width, and line number info
//! based on viewport state.
//! No React dependencies – can be tested independently.

import { GUTTER_CONFIG } from './GutterConfig';
import { computeVisibleRange, computeGutterWidth, computeTotalHeight, ViewportState } from './GutterLayout';

/** Information about a single visible line in the gutter. */
export interface VisibleLineInfo {
  /** 0‑based line index in the document. */
  lineIndex: number;
  /** 1‑based line number displayed in the gutter. */
  lineNum: number;
  /** Whether this line is the current cursor line. */
  isCurrent: boolean;
}

/** Complete layout information for the gutter. */
export interface GutterLayout {
  /** Computed gutter width in pixels. */
  width: number;
  /** Visible line info for rendering (includes overscan). */
  visibleLines: VisibleLineInfo[];
  /** Total height of all lines in pixels. */
  totalHeight: number;
}

/**
 * Immutable model that computes gutter layout from viewport parameters.
 *
 * All computations are O(visible lines) – never O(document size).
 */
export class GutterModel {
  private readonly _viewport: ViewportState;
  private readonly _cursorLine: number;

  constructor(
    viewport: ViewportState,
    cursorLine: number = 1,
  ) {
    this._viewport = viewport;
    this._cursorLine = cursorLine;
  }

  // ── Accessors ──────────────────────────────────────────────────────

  get viewport(): ViewportState {
    return this._viewport;
  }

  get cursorLine(): number {
    return this._cursorLine;
  }

  // ── Computed properties ────────────────────────────────────────────

  /** Gutter width based on the number of digits in the total line count. */
  get width(): number {
    return computeGutterWidth(this._viewport.totalLines);
  }

  /** Total height of all lines in the document. */
  get totalHeight(): number {
    return computeTotalHeight(this._viewport.totalLines, this._viewport.lineHeight);
  }

  /**
   * Visible line range (clamped to document bounds, with overscan).
   * Returns `null` when the container hasn't been measured or when there are no lines.
   */
  get visibleRange(): { firstLine: number; lastLine: number } | null {
    return computeVisibleRange(this._viewport);
  }

  /** Array of `VisibleLineInfo` for the currently visible lines. */
  get visibleLines(): VisibleLineInfo[] {
    const range = this.visibleRange;
    if (!range) {
      return [];
    }

    const { firstLine, lastLine } = range;
    const lines: VisibleLineInfo[] = [];
    for (let lineIndex = firstLine; lineIndex <= lastLine; lineIndex++) {
      lines.push({
        lineIndex,
        lineNum: lineIndex + 1,
        isCurrent: lineIndex + 1 === this._cursorLine,
      });
    }
    return lines;
  }

  /** Complete layout information for rendering. */
  get layout(): GutterLayout {
    return {
      width: this.width,
      visibleLines: this.visibleLines,
      totalHeight: this.totalHeight,
    };
  }

  // ── Immutable update helpers ───────────────────────────────────────

  withViewport(viewport: ViewportState): GutterModel {
    return new GutterModel(viewport, this._cursorLine);
  }

  withCursorLine(cursorLine: number): GutterModel {
    return new GutterModel(this._viewport, cursorLine);
  }
}
