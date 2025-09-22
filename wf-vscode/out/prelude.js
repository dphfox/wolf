"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Ok = Ok;
exports.Err = Err;
function Ok(value) {
    return { ok: true, value: value };
}
function Err(reason, existing = { ok: false, reasons: [] }) {
    return { ok: false, reasons: [reason, ...existing.reasons] };
}
//# sourceMappingURL=prelude.js.map