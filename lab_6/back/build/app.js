"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Application = void 0;
const postgres_1 = require("./db/postgres");
const authService_1 = require("./services/authService");
const userService_1 = require("./services/userService");
const utils_1 = require("./utils/utils");
const fastifyServer_1 = require("./config/fastifyServer");
class Application {
    constructor() {
        utils_1.checkAndGetEnv('CIPHER_KEY');
        this.database = new postgres_1.Postgres({
            max: 20,
            idleTimeoutMillis: 10000,
            connectionTimeoutMillis: 2000,
            connectionString: utils_1.checkAndGetEnv('MAIN_DB_CONNECTION_STRING'),
        });
        this.authService = new authService_1.AuthService(this.database);
        this.userService = new userService_1.UserService(this.database);
        this.fastifyServer = fastifyServer_1.createFastifyServer(this.userService, this.authService);
    }
    startFastifyServer() {
        const port = utils_1.checkAndGetEnv('HTTP_SERVER_PORT');
        this.fastifyServer.listen({ port }).then(() => console.log(`App started! PORT: ${port}`));
    }
}
exports.Application = Application;
//# sourceMappingURL=app.js.map