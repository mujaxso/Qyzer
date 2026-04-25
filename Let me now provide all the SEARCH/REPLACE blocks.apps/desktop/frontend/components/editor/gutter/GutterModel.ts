//! Pure model for the line-number gutter.
//! Computes visible line range, gutter width, and line number info
//! based on scroll position, line height, and container dimensions.
//! No React dependencies – can be tested independently.

import { GUTTER_CONFIG } from './GutterConfig';

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
  private readonly _scrollTop: number;
  private readonly _lineHeight: number;
  private readonly _lineCount: number;
  private readonly _containerHeight: number;
  private readonly _cursorLine: number;
  private readonly _overscan: number;

  constructor(
    scrollTop: number = 0,
    lineHeight: number = GUTTER_CONFIG.LINE_HEIGHT,
    lineCount: number = 0,
    containerHeight: number = 0,
    cursorLine: number = 1,
    overscan: number = 3,
  ) {
    this._scrollTop = scrollTop;
    this._lineHeight = lineHeight;
    this._lineCount = lineCount;
    this._containerHeight = containerHeight;
    this._cursorLine = cursorLine;
    this._overscan = overscan;
  }

  // ── Accessors ──────────────────────────────────────────────────────

  get scrollTop(): number {
    return this._scrollTop;
  }
  get lineHeight(): number {
    return this._lineHeight;
  }
  get lineCount(): number {
    return this._lineCount;
  }
  get containerHeight(): number {
    return this._containerHeight;
  }
  get cursorLine(): number {
    return this._cursorLine;
  }
  get overscan(): number {
    return this._overscan;
  }

  // ── Computed properties ────────────────────────────────────────────

  /** Gutter width based on the number of digits in the total line count. */
  get width(): number {
    const digits = String(this._lineCount).length;
    return Math.max(
      GUTTER_CONFIG.MIN_WIDTH,
      digits * GUTTER_CONFIG.DIGIT_WIDTH +
        GUTTER_CONFIG.PADDING_LEFT +
        GUTTER_CONFIG.PADDING_RIGHT,
    );
  }

  /** Total height of all lines in the document. */
  get totalHeight(): number {
    return this._lineCount * this._lineHeight;
  }

  /**
   * Visible line range (clamped to document bounds, with overscan).
   * Returns `{ firstLine: -1, lastLine: -1 }` when the container hasn't
   * been measured yet or when there are no lines.
   */
  get visibleRange(): { firstLine: number; lastLine: number } {
    if (this._containerHeight <= 0 || this._lineHeight <= 0 || this._lineCount === 0) {
      return { firstLine: -1, lastLine: -1 };
    }

    const effectiveScrollTop = Math.max(0, this._scrollTop);
    const first = Math.max(
      0,
      Math.floor(effectiveScrollTop / this._lineHeight) - this._overscan,
    );
    const last = Math.min(
      this._lineCount - 1,
      Math.ceil((effectiveScrollTop + this._containerHeight) / this._lineHeight) +
        this._overscan -
        1,
    );

    if (!Number.isFinite(first) || !Number.isFinite(last)) {
      return { firstLine: -1, lastLine: -1 };
    }

    return { firstLine: first, lastLine: last };
  }

  /** Array of `VisibleLineInfo` for the currently visible lines. */
  get visibleLines(): VisibleLineInfo[] {
    const { firstLine, lastLine } = this.visibleRange;
    if (firstLine < 0 || lastLine < 0) {
      return [];
    }

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

  withScrollTop(scrollTop: number): GutterModel {
    return new GutterModel(
      scrollTop,
      this._lineHeight,
      this._lineCount,
      this._containerHeight,
      this._cursorLine,
      this._overscan,
    );
  }

  withLineHeight(lineHeight: number): GutterModel {
    return new GutterModel(
      this._scrollTop,
      lineHeight,
      this._lineCount,
      this._containerHeight,
      this._cursorLine,
      this._overscan,
    );
  }

  withLineCount(lineCount: number): GutterModel {
    return new GutterModel(
      this._scrollTop,
      this._lineHeight,
      lineCount,
      this._containerHeight,
      this._cursorLine,
      this._overscan,
    );
  }

  withContainerHeight(containerHeight: number): GutterModel {
    return new GutterModel(
      this._scrollTop,
      this._lineHeight,
      this._lineCount,
      containerHeight,
      this._cursorLine,
      this._overscan,
    );
  }

  withCursorLine(cursorLine: number): GutterModel {
    return new GutterModel(
      this._scrollTop,
      this._lineHeight,
      this._lineCount,
      this._containerHeight,
      cursorLine,
      this._overscan,
    );
  }
}
