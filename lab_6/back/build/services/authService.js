"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.AuthService = void 0;
const utils_1 = require("../crypto/utils");
const errors_1 = require("./errors");
const uuid = require("uuid");
const validation_1 = require("../utils/validation");
class AuthService {
    constructor(database) {
        this.database = database;
    }
    async login(credentials) {
        const user = await this.database.findUserByUsername(credentials.username);
        if (!user) {
            throw errors_1.badCredentials();
        }
        if (user?.password && await utils_1.comparePasswords(credentials.password, user?.password)) {
            return {
                id: user.id,
                fullName: user.fullName,
                email: user.email,
                username: user.username,
                sessionId: uuid.v4(),
            };
        }
        throw errors_1.badCredentials();
    }
    async register(userData) {
        await validation_1.validateEmail(this.database, userData.email);
        await validation_1.validateUsername(this.database, userData.username);
        validation_1.validateFullName(userData.fullName);
        validation_1.validatePassword(userData.password);
        const userId = uuid.v4();
        userData.password = await utils_1.hashPassword(userData.password);
        const user = {
            ...userData,
            id: userId,
        };
        console.log(user);
        await this.database.saveUser(user);
    }
}
exports.AuthService = AuthService;
//# sourceMappingURL=authService.js.map