"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.registerPlugin = void 0;
const registerSchema = {
    schema: {
        body: {
            type: 'object',
            required: ['username', 'email', 'fullName', 'password'],
            properties: {
                username: { type: 'string' },
                email: { type: 'string' },
                fullName: { type: 'string' },
                password: { type: 'string' },
            },
        },
    },
};
const registerPlugin = (fastify, options, done) => {
    fastify.post('/auth/register', registerSchema, async (request, response) => {
        const userData = request.body;
        try {
            await options.authService.register(userData);
            response.code(204);
        }
        catch (err) {
            console.log('register error:');
            console.error(err);
            response.code(err.status || 500).send(err.errorMessage || err);
        }
    });
    done();
};
exports.registerPlugin = registerPlugin;
//# sourceMappingURL=register.js.map