"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.healthCheckPlugin = void 0;
const healthCheckPlugin = (fastify, options, done) => {
    fastify.get('/healthz', (request, response) => {
        response.code(200).send('ok');
    });
    fastify.get('/', (request, response) => {
        response.code(200).send('ok');
    });
    done();
};
exports.healthCheckPlugin = healthCheckPlugin;
//# sourceMappingURL=healthCheck.js.map