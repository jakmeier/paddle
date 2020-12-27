import { MouseEventType, KeyEventType, KeyEnum } from "#RUST#";

export function mouseEventString(paddleEventNum) {
    switch (paddleEventNum) {
        case MouseEventType.LeftClick:
            return "click";
        case MouseEventType.RightClick:
            return "contextmenu";
        case MouseEventType.DoubleClick:
            return "dblclick";
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
    return `Event ${MouseEventType[paddleEventNum]}(${paddleEventNum}) not implemented`;
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