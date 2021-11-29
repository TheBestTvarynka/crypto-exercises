"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.UserService = void 0;
class UserService {
    constructor(userRepository) {
        this.userRepository = userRepository;
    }
    async findUserById(id) {
        const user = await this.userRepository.findUserById(id);
        if (user) {
            user.password = undefined;
            return user;
        }
        else {
            throw { status: 404, errorMessage: `User with id=${id} not found!` };
        }
    }
}
exports.UserService = UserService;
//# sourceMappingURL=userService.js.map