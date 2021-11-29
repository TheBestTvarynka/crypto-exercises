"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.checkAndGetEnv = void 0;
const checkAndGetEnv = (envName, errMsg) => {
    const envVar = process.env[envName];
    if (envVar === undefined) {
        const error = new Error(errMsg || `${envName} not found in environment variables`);
        console.error(error);
        throw error;
    }
    return envVar;
};
exports.checkAndGetEnv = checkAndGetEnv;
//# sourceMappingURL=utils.js.map