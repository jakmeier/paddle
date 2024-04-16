import { mouseEventString, clickEventString, touchEventString, pointerEventString, keyboardEventString, keyEventEnum } from "./enums.js";

export class PaddleJsContext {
    // These are some methods written in Rust, inside Paddle. When a Rust
    // project depends on Paddle and paddle-js it will have these methods
    // exported in the final WASM module. We cannot depend on that module in a
    // proper way so instead we use dependency injection here.
    constructor(
        click_event_gate,
        mouse_event_gate,
        keyboard_event_gate,
        pointer_event_gate,
        touch_event_gate,
    ) {
        this.click_event_gate = click_event_gate;
        this.mouse_event_gate = mouse_event_gate;
        this.keyboard_event_gate = keyboard_event_gate;
        this.pointer_event_gate = pointer_event_gate;
        this.touch_event_gate = touch_event_gate;
    }

    // A request, incoming from Rust, to add a listener of the specified type to a frame.
    // @param eventType: MouseEventType
    //  An enum value is sent here to avoid copying a string between WASM linear memory and GC memory.
    // @param listener: HtmlElement
    // @param callbackId: usize 
    //  This value is provided again on each forwarded event
    registerMouseEventListener(eventType, listener, callbackId) {
        const event = mouseEventString(eventType);
        listener.addEventListener(event, (ev) => this.forwardMouseEvent(ev, eventType, callbackId));
    }
    registerClickEventListener(eventType, listener, callbackId) {
        const event = clickEventString(eventType);
        listener.addEventListener(event, (ev) => this.forwardClickEvent(ev, eventType, callbackId));
    }
    registerTouchEventListener(eventType, listener, callbackId) {
        const event = touchEventString(eventType);
        listener.addEventListener(event, (ev) => this.forwardTouchEvent(ev, eventType, callbackId));
    }
    registerPointerEventListener(eventType, listener, callbackId) {
        const event = pointerEventString(eventType);
        listener.addEventListener(event, (ev) => this.forwardPointerEvent(ev, eventType, callbackId));
    }
    registerKeyboardEventListener(eventType, callbackId) {
        const event = keyboardEventString(eventType);
        document.addEventListener(event, (ev) => this.forwardKeyboardEvent(ev, eventType, callbackId));
    }
    forwardClickEvent(event, eventType, callbackId) {
        const rect = event.target.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const y = event.clientY - rect.top;
        this.click_event_gate(callbackId, eventType, x, y);
    }
    forwardMouseEvent(event, eventType, callbackId) {
        const rect = event.target.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const y = event.clientY - rect.top;
        this.mouse_event_gate(callbackId, eventType, x, y);
    }
    forwardTouchEvent(event, eventType, callbackId) {
        // Do not call preventDefault(), we want the generate clicks events
        for (let i = 0; i < event.changedTouches.length; i++) {
            const touch = event.changedTouches.item(i);
            const rect = touch.target.getBoundingClientRect();
            const x = touch.clientX - rect.left;
            const y = touch.clientY - rect.top;
            this.touch_event_gate(callbackId, eventType, x, y);
        }
    }
    forwardPointerEvent(event, eventType, callbackId) {
        // Do not call preventDefault(), we want the generate clicks events
        const rect = event.target.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const y = event.clientY - rect.top;
        this.pointer_event_gate(callbackId, eventType, x, y);
    }
    forwardKeyboardEvent(event, eventType, callbackId) {
        let key = keyEventEnum(event);
        if (typeof (key) === "number") {
            this.keyboard_event_gate(callbackId, eventType, key);
        }
    }
}

export function supportsPointerEvents() {
    return window.PointerEvent !== undefined;
}