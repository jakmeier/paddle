// I don't know how to do this, I think it's not possible without some webpack plugin magic or the like.
// import { ClickEventType, MouseEventType, TouchEventType, BrowserPointerEventType, KeyEventType, KeyEnum } from "the-name-the-user-has-chose-for-their.wasm";

// Instead I copy-paste the generated types here once and then sync them manually. :(
/**
* Rust representation for key event types.
* Has a one-to-one correspondence to browser events.
*/
const KeyEventType = Object.freeze({ KeyDown: 0, "0": "KeyDown", KeyPress: 1, "1": "KeyPress", KeyUp: 2, "2": "KeyUp", });
/**
* Rust representation for click event types.
* Has a one-to-one correspondence to browser events.
*/
const ClickEventType = Object.freeze({ LeftClick: 0, "0": "LeftClick", RightClick: 1, "1": "RightClick", DoubleClick: 2, "2": "DoubleClick", });
/**
* Rust representation for mouse event types.
* Has a one-to-one correspondence to browser events.
*/
const MouseEventType = Object.freeze({ Up: 0, "0": "Up", Down: 1, "1": "Down", Move: 2, "2": "Move", Enter: 3, "3": "Enter", Leave: 4, "4": "Leave", });
/**
* Rust representation for touch event types.
* Has a one-to-one correspondence to browser events.
*/
const TouchEventType = Object.freeze({ Start: 0, "0": "Start", End: 1, "1": "End", Move: 2, "2": "Move", Cancel: 3, "3": "Cancel", });
/**
* Rust representation for pointer event types.
* Has a one-to-one correspondence to browser events.
*/
const BrowserPointerEventType = Object.freeze({ Down: 0, "0": "Down", Up: 1, "1": "Up", Move: 2, "2": "Move", Enter: 3, "3": "Enter", Leave: 4, "4": "Leave", Cancel: 5, "5": "Cancel", });
/**
* Rust representation of a set of common keys.
* The names match the [Key Code Values](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code/code_values).
* The keys listed should all have the same representation on all platforms.
*
* For older browsers that don't support the `code` value, a conversion from the `key` value is done with best effort. This may not consider keyboard layouts perfectly.
*/
const KeyEnum = Object.freeze({ Escape: 0, "0": "Escape", ArrowDown: 1, "1": "ArrowDown", ArrowLeft: 2, "2": "ArrowLeft", ArrowRight: 3, "3": "ArrowRight", ArrowUp: 4, "4": "ArrowUp", End: 5, "5": "End", Home: 6, "6": "Home", PageDown: 7, "7": "PageDown", PageUp: 8, "8": "PageUp", Enter: 9, "9": "Enter", Tab: 10, "10": "Tab", Backspace: 11, "11": "Backspace", Delete: 12, "12": "Delete", Space: 13, "13": "Space", AltLeft: 14, "14": "AltLeft", AltRight: 15, "15": "AltRight", ShiftLeft: 16, "16": "ShiftLeft", ShiftRight: 17, "17": "ShiftRight", Digit0: 18, "18": "Digit0", Digit1: 19, "19": "Digit1", Digit2: 20, "20": "Digit2", Digit3: 21, "21": "Digit3", Digit4: 22, "22": "Digit4", Digit5: 23, "23": "Digit5", Digit6: 24, "24": "Digit6", Digit7: 25, "25": "Digit7", Digit8: 26, "26": "Digit8", Digit9: 27, "27": "Digit9", Numpad0: 28, "28": "Numpad0", Numpad1: 29, "29": "Numpad1", Numpad2: 30, "30": "Numpad2", Numpad3: 31, "31": "Numpad3", Numpad4: 32, "32": "Numpad4", Numpad5: 33, "33": "Numpad5", Numpad6: 34, "34": "Numpad6", Numpad7: 35, "35": "Numpad7", Numpad8: 36, "36": "Numpad8", Numpad9: 37, "37": "Numpad9", KeyA: 38, "38": "KeyA", KeyB: 39, "39": "KeyB", KeyC: 40, "40": "KeyC", KeyD: 41, "41": "KeyD", KeyE: 42, "42": "KeyE", KeyF: 43, "43": "KeyF", KeyG: 44, "44": "KeyG", KeyH: 45, "45": "KeyH", KeyI: 46, "46": "KeyI", KeyJ: 47, "47": "KeyJ", KeyK: 48, "48": "KeyK", KeyL: 49, "49": "KeyL", KeyM: 50, "50": "KeyM", KeyN: 51, "51": "KeyN", KeyO: 52, "52": "KeyO", KeyP: 53, "53": "KeyP", KeyQ: 54, "54": "KeyQ", KeyR: 55, "55": "KeyR", KeyS: 56, "56": "KeyS", KeyT: 57, "57": "KeyT", KeyU: 58, "58": "KeyU", KeyV: 59, "59": "KeyV", KeyW: 60, "60": "KeyW", KeyX: 61, "61": "KeyX", KeyY: 62, "62": "KeyY", KeyZ: 63, "63": "KeyZ", });


export function clickEventString(paddleEventNum) {
    switch (paddleEventNum) {
        case ClickEventType.LeftClick:
            return "click";
        case ClickEventType.RightClick:
            return "contextmenu";
        case ClickEventType.DoubleClick:
            return "dblclick";
    }
    return `Click Event ${TouchEventType[paddleEventNum]}(${paddleEventNum}) not implemented`;
}
export function mouseEventString(paddleEventNum) {
    switch (paddleEventNum) {
        case MouseEventType.Up:
            return "mouseup";
        case MouseEventType.Down:
            return "mousedown";
        case MouseEventType.Move:
            return "mousemove";
        case MouseEventType.Enter:
            return "mouseenter";
        case MouseEventType.Leave:
            return "mouseleave";
    }
    return `Mouse Event ${MouseEventType[paddleEventNum]}(${paddleEventNum}) not implemented`;
}

export function touchEventString(paddleEventNum) {
    switch (paddleEventNum) {
        case TouchEventType.Start:
            return "touchstart";
        case TouchEventType.End:
            return "touchend";
        case TouchEventType.Move:
            return "touchmove";
        case TouchEventType.Cancel:
            return "touchcancel";
    }
    return `Touch Event ${TouchEventType[paddleEventNum]}(${paddleEventNum}) not implemented`;
}

export function pointerEventString(paddleEventNum) {
    switch (paddleEventNum) {
        case BrowserPointerEventType.Down:
            return "pointerdown";
        case BrowserPointerEventType.Up:
            return "pointerup";
        case BrowserPointerEventType.Move:
            return "pointermove";
        case BrowserPointerEventType.Enter:
            return "pointerenter";
        case BrowserPointerEventType.Leave:
            return "pointerleave";
        case BrowserPointerEventType.Cancel:
            return "pointercancel";
    }
    return `Pointer Event ${BrowserPointerEventType[paddleEventNum]}(${paddleEventNum}) not implemented`;
}

export function keyboardEventString(paddleEventNum) {
    switch (paddleEventNum) {
        case KeyEventType.KeyDown:
            return "keydown";
        case KeyEventType.KeyPress:
            return "keypress";
        case KeyEventType.KeyUp:
            return "keyup";
    }
    return `Keyboard Event ${KeyEventType[paddleEventNum]}(${paddleEventNum}) not implemented`;
}

function keyEventCode(event) {
    if (event.code) {
        return event.code;
    }
    switch (event.key) {
        case " ":
            return "Space";
        case "Alt":
            return "AltLeft";
        case "Shift":
            return "ShiftLeft";
        case "0":
            return "Digit0";
        case "1":
            return "Digit1";
        case "2":
            return "Digit2";
        case "3":
            return "Digit3";
        case "4":
            return "Digit4";
        case "5":
            return "Digit5";
        case "6":
            return "Digit6";
        case "7":
            return "Digit7";
        case "8":
            return "Digit8";
        case "9":
            return "Digit9";
        case "a":
            return "KeyA";
        case "b":
            return "KeyB";
        case "c":
            return "KeyC";
        case "d":
            return "KeyD";
        case "e":
            return "KeyE";
        case "f":
            return "KeyF";
        case "g":
            return "KeyG";
        case "h":
            return "KeyH";
        case "i":
            return "KeyI";
        case "j":
            return "KeyJ";
        case "k":
            return "KeyK";
        case "l":
            return "KeyL";
        case "m":
            return "KeyM";
        case "n":
            return "KeyN";
        case "o":
            return "KeyO";
        case "p":
            return "KeyP";
        case "q":
            return "KeyQ";
        case "r":
            return "KeyR";
        case "s":
            return "KeyS";
        case "t":
            return "KeyT";
        case "u":
            return "KeyU";
        case "v":
            return "KeyV";
        case "w":
            return "KeyW";
        case "x":
            return "KeyX";
        case "y":
            return "KeyY";
        case "z":
            return "KeyZ";
        case "A":
            return "KeyA";
        case "B":
            return "KeyB";
        case "C":
            return "KeyC";
        case "D":
            return "KeyD";
        case "E":
            return "KeyE";
        case "F":
            return "KeyF";
        case "G":
            return "KeyG";
        case "H":
            return "KeyH";
        case "I":
            return "KeyI";
        case "J":
            return "KeyJ";
        case "K":
            return "KeyK";
        case "L":
            return "KeyL";
        case "M":
            return "KeyM";
        case "N":
            return "KeyN";
        case "O":
            return "KeyO";
        case "P":
            return "KeyP";
        case "Q":
            return "KeyQ";
        case "R":
            return "KeyR";
        case "S":
            return "KeyS";
        case "T":
            return "KeyT";
        case "U":
            return "KeyU";
        case "V":
            return "KeyV";
        case "W":
            return "KeyW";
        case "X":
            return "KeyX";
        case "Y":
            return "KeyY";
        case "Z":
            return "KeyZ";
        default:
            return event.key;
    }
}

export function keyEventEnum(event) {
    return KeyEnum[keyEventCode(event)];
}