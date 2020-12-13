import { mouse_event_gate } from "#RUST#";
import { mouseEventString } from "./enums";

export class PaddleJsContext {
    constructor() {}

    // A request, incoming from Rust, to add a listener of the specified type to a frame.
    // @param eventType: MouseEventType as u32 
    //  An enum value is sent here to avoid copying a string between WASM linear memory and GC memory.
    // @param listener: HtmlElement
    // @param callbackId: usize 
    //  This value is provided again on each forwarded event
    registerMouseEventListener(eventType, listener, callbackId) {
        const event = mouseEventString(eventType);
        console.log("JS registers listener", event);
        listener.addEventListener(event, (ev) => this.forwardMouseEvent(ev, callbackId));
    }
    forwardMouseEvent(event, callbackId) {
        const rect = event.target.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const y = event.clientY - rect.top;
        console.log("Forwarding from JS");
        mouse_event_gate(callbackId, x, y);
    }
}