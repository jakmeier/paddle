import { mouse_event_gate, keyboard_event_gate } from "#RUST#";
import { mouseEventString, keyboardEventString, keyEventEnum } from "./enums";

export class PaddleJsContext {
    constructor() {}

    // A request, incoming from Rust, to add a listener of the specified type to a frame.
    // @param eventType: MouseEventType
    //  An enum value is sent here to avoid copying a string between WASM linear memory and GC memory.
    // @param listener: HtmlElement
    // @param callbackId: usize 
    //  This value is provided again on each forwarded event
    registerMouseEventListener(eventType, listener, callbackId) {
        const event = mouseEventString(eventType);
        listener.addEventListener(event, (ev) => this.forwardMouseEvent(ev, callbackId));
    }
    registerKeyboardEventListener(eventType, callbackId) {
        const event = keyboardEventString(eventType);
        document.addEventListener(event, (ev) => this.forwardKeyboardEvent(ev, eventType, callbackId));
    }
    forwardMouseEvent(event, callbackId) {
        const rect = event.target.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const y = event.clientY - rect.top;
        mouse_event_gate(callbackId, x, y);
    }
    forwardKeyboardEvent(event, eventType, callbackId) {
        let key = keyEventEnum(event);
        keyboard_event_gate(callbackId, eventType, key);
    }
}