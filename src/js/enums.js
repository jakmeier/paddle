export function mouseEventString(paddleEventNum) {
    switch (paddleEventNum) {
        case 1:
            return "click";
        case 2:
            return "contextmenu";
        case 3:
            return "dblclick";
        case 4:
            return "mousedown";
        case 7:
            return "mousemove";
        case 10:
            return "mouseup";
    }
    return `Event ${paddleEventNum} not implemented`;
}