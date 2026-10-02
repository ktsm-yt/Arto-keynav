import { describe, test, expect, vi } from "vitest";
import {
  inputMethodHasKey,
  shouldSwallowReservedKey,
  setup,
  onKeydown,
} from "./keyboard-interceptor";

// ============================================================================
// shouldSwallowReservedKey
// ============================================================================

const NONE: ReadonlySet<string> = new Set();

/** Build a decision input with sensible defaults for the fields under test. */
function decision(overrides: {
  key: string;
  isMac?: boolean;
  ctrlKey?: boolean;
  metaKey?: boolean;
  altKey?: boolean;
  boundPrimaryKeys?: ReadonlySet<string>;
}) {
  return {
    key: overrides.key,
    isMac: overrides.isMac ?? false,
    ctrlKey: overrides.ctrlKey ?? false,
    metaKey: overrides.metaKey ?? false,
    altKey: overrides.altKey ?? false,
    boundPrimaryKeys: overrides.boundPrimaryKeys ?? NONE,
  };
}

describe("shouldSwallowReservedKey", () => {
  test("macOS Cmd+C is reserved (native clipboard)", () => {
    expect(shouldSwallowReservedKey(decision({ key: "c", isMac: true, metaKey: true }))).toBe(true);
  });

  test("non-mac Ctrl+C is reserved when unbound", () => {
    expect(shouldSwallowReservedKey(decision({ key: "c", isMac: false, ctrlKey: true }))).toBe(
      true,
    );
  });

  test("non-mac Ctrl+X is NOT swallowed when bound by config", () => {
    // Emacs binds the Ctrl+x prefix; the config override must let it through.
    expect(
      shouldSwallowReservedKey(
        decision({
          key: "x",
          isMac: false,
          ctrlKey: true,
          boundPrimaryKeys: new Set(["x", "v", "c"]),
        }),
      ),
    ).toBe(false);
  });

  test("non-mac Ctrl+V (bound, later chord) is NOT swallowed", () => {
    expect(
      shouldSwallowReservedKey(
        decision({
          key: "v",
          isMac: false,
          ctrlKey: true,
          boundPrimaryKeys: new Set(["v"]),
        }),
      ),
    ).toBe(false);
  });

  test("non-mac uppercase key is matched case-insensitively", () => {
    expect(shouldSwallowReservedKey(decision({ key: "C", isMac: false, ctrlKey: true }))).toBe(
      true,
    );
  });

  test("Super/Windows key alone is never swallowed", () => {
    // The Windows key reads as metaKey on non-mac (the secondary modifier).
    expect(shouldSwallowReservedKey(decision({ key: "c", isMac: false, metaKey: true }))).toBe(
      false,
    );
  });

  test("non-mac Ctrl+Meta (secondary also held) is not swallowed", () => {
    expect(
      shouldSwallowReservedKey(decision({ key: "c", isMac: false, ctrlKey: true, metaKey: true })),
    ).toBe(false);
  });

  test("primary+Alt is not swallowed (Alt escapes the reserved gate)", () => {
    expect(
      shouldSwallowReservedKey(decision({ key: "v", isMac: false, ctrlKey: true, altKey: true })),
    ).toBe(false);
  });

  test("non-reserved primary+letter is not swallowed", () => {
    expect(shouldSwallowReservedKey(decision({ key: "n", isMac: false, ctrlKey: true }))).toBe(
      false,
    );
  });

  test("reserved letter without the primary modifier is not swallowed", () => {
    expect(shouldSwallowReservedKey(decision({ key: "c", isMac: false }))).toBe(false);
  });

  test("macOS Ctrl+C (secondary modifier on mac) is not swallowed", () => {
    // On macOS the primary modifier is Cmd; a bare Ctrl press must pass through.
    expect(shouldSwallowReservedKey(decision({ key: "c", isMac: true, ctrlKey: true }))).toBe(
      false,
    );
  });
});

// ============================================================================
// inputMethodHasKey
// ============================================================================

/** A keystroke with nothing unusual about it. */
function keystroke(overrides: Partial<Parameters<typeof inputMethodHasKey>[0]> = {}) {
  return {
    composing: false,
    isComposing: false,
    keyCode: 74,
    editable: false,
    ...overrides,
  };
}

describe("inputMethodHasKey", () => {
  test("an ordinary keystroke is the keybindings'", () => {
    expect(inputMethodHasKey(keystroke())).toBe(false);
  });

  test("a composition in progress is the input method's", () => {
    expect(inputMethodHasKey(keystroke({ isComposing: true }))).toBe(true);
    expect(inputMethodHasKey(keystroke({ composing: true }))).toBe(true);
  });

  test("the keystroke that begins a composition, inside a field, is the input method's", () => {
    // `compositionstart` has not arrived yet; 229 is what says so.
    expect(inputMethodHasKey(keystroke({ keyCode: 229, editable: true }))).toBe(true);
  });

  test("229 over the document is still the keybindings'", () => {
    // Some input methods report it for every key while they are selected, and
    // there is nothing to compose into here. Reading it would take every
    // keystroke in the window away from the bindings.
    expect(inputMethodHasKey(keystroke({ keyCode: 229 }))).toBe(false);
  });
});

test("reading arrows cancel native scrolling while typing and IME keep their keys", () => {
  setup();
  const callback = vi.fn();
  onKeydown(callback);
  const page = document.createElement("div");
  const input = document.createElement("input");
  document.body.append(page, input);
  const press = (target: HTMLElement, key: string, options: KeyboardEventInit = {}) => {
    const event = new KeyboardEvent("keydown", {
      key,
      bubbles: true,
      cancelable: true,
      ...options,
    });
    target.dispatchEvent(event);
    return event;
  };
  try {
    for (const key of ["ArrowDown", "ArrowUp"]) {
      expect(press(page, key).defaultPrevented).toBe(true);
      expect(callback).toHaveBeenLastCalledWith({
        key,
        modifiers: 0,
        repeat: false,
        field: undefined,
      });
    }
    expect(press(page, "ArrowDown", { repeat: true }).defaultPrevented).toBe(true);
    expect(callback).toHaveBeenCalledTimes(3);

    callback.mockClear();
    expect(press(input, "ArrowUp").defaultPrevented).toBe(false);
    expect(callback).not.toHaveBeenCalled();
    input.className = "search-input";
    expect(press(input, "ArrowDown").defaultPrevented).toBe(false);
    expect(callback).toHaveBeenCalledTimes(1);
    input.className = "palette-input";
    expect(press(input, "ArrowUp").defaultPrevented).toBe(false);
    expect(callback).toHaveBeenCalledTimes(2);

    callback.mockClear();
    expect(press(input, "ArrowDown", { isComposing: true }).defaultPrevented).toBe(false);
    document.dispatchEvent(new CompositionEvent("compositionstart"));
    expect(press(page, "ArrowUp").defaultPrevented).toBe(false);
    document.dispatchEvent(new CompositionEvent("compositionend"));
    expect(callback).not.toHaveBeenCalled();
    expect(press(page, "ArrowDown", { altKey: true }).defaultPrevented).toBe(false);
    expect(press(page, "PageDown").defaultPrevented).toBe(false);
  } finally {
    document.dispatchEvent(new CompositionEvent("compositionend"));
    page.remove();
    input.remove();
  }
});
