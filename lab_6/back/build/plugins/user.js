"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.userPlugin = void 0;
const userPlugin = async (fastify, options, done) => {
    fastify.get('/auth/user/:id', async (request, response) => {
        const { id } = request.params;
        try {
            response.code(200).send(JSON.stringify(await options.userService.findUserById(id)));
        }
        catch (err) {
            console.error(err);
            response.code(err.status || 500).send(err.errorMessage || err);
        }
    });
    done();
};
exports.userPlugin = userPlugin;
//# sourceMappingURL=user.js.map