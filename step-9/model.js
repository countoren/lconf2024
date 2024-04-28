import { Record, Union } from "./fable_modules/fable-library-js.4.13.0/Types.js";
import { record_type, string_type, union_type } from "./fable_modules/fable-library-js.4.13.0/Reflection.js";

export class EcoSystem extends Union {
    constructor(tag, fields) {
        super();
        this.tag = tag;
        this.fields = fields;
    }
    cases() {
        return ["FSharp", "JS", "Rust"];
    }
}

export function EcoSystem_$reflection() {
    return union_type("Main.EcoSystem", [], EcoSystem, () => [[], [], []]);
}

export class Message extends Record {
    constructor(ecoSystem, text) {
        super();
        this.ecoSystem = ecoSystem;
        this.text = text;
    }
}

export function Message_$reflection() {
    return record_type("Main.Message", [], Message, () => [["ecoSystem", EcoSystem_$reflection()], ["text", string_type]]);
}

