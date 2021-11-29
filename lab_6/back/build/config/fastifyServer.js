"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.createFastifyServer = void 0;
const fastify_1 = require("fastify");
const fastify_cors_1 = require("fastify-cors");
const healthCheck_1 = require("../plugins/healthCheck");
const fastify_cookie_1 = require("fastify-cookie");
const user_1 = require("../plugins/user");
const login_1 = require("../plugins/login");
const register_1 = require("../plugins/register");
const allowUnauthorized = new Set(['/', '/healthz', '/auth/health', '/auth/login', '/auth/register']);
const cookieHook = (request, reply, done) => {
    if (!allowUnauthorized.has(request.url) && !request.cookies['Session']) {
        reply.code(401).send('User is not authorized');
    }
    else {
        done();
    }
};
const createFastifyServer = (userService, authService) => {
    const server = fastify_1.default({});
    server.register(fastify_cors_1.default, {
        origin: (_origin, cb) => {
            cb(null, true);
        },
        methods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
        credentials: true,
    });
    server.register(fastify_cookie_1.default);
    server.register(healthCheck_1.healthCheckPlugin, {});
    server.register(user_1.userPlugin, { userService, authService });
    server.register(login_1.loginPlugin, { authService });
    server.register(register_1.registerPlugin, { authService });
    server.addHook('onRequest', cookieHook);
    return server;
};
exports.createFastifyServer = createFastifyServer;
//# sourceMappingURL=fastifyServer.js.map