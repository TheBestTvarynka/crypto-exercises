"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.loginPlugin = void 0;
const loginSchema = {
    schema: {
        body: {
            type: 'object',
            required: ['username', 'password'],
            properties: {
                username: { type: 'string' },
                password: { type: 'string' },
            },
        },
    },
};
const loginPlugin = async (fastify, options, done) => {
    fastify.post('/auth/login', loginSchema, async (request, response) => {
        const credentials = request.body;
        try {
            const afterLoginDto = await options.authService.login(credentials);
            const expires = new Date();
            expires.setDate(expires.getTime() + 7200000);
            response.setCookie('Session', afterLoginDto.sessionId || '', {
                httpOnly: true,
                path: '/',
                secure: true,
                sameSite: 'none',
                domain: '127.0.0.1:8080',
            });
            response.code(200).send(JSON.stringify(afterLoginDto));
        }
        catch (err) {
            console.error(err);
            response.code(err.status || 500).send(err.errorMessage || err);
        }
    });
    done();
};
exports.loginPlugin = loginPlugin;
//# sourceMappingURL=login.js.map