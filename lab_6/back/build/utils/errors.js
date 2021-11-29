"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.GrpcCallError = void 0;
class GrpcCallError extends Error {
    constructor(error) {
        super();
        this.status = 501;
        this.errorMessage = error.details || '';
    }
}
exports.GrpcCallError = GrpcCallError;
//# sourceMappingURL=errors.js.map