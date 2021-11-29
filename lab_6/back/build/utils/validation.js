"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.validatePassword = exports.validateFullName = exports.validateUsername = exports.validateEmail = void 0;
const errors_1 = require("../services/errors");
const emailRegularExpression = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
const usernameRegularExpression = /^[a-zA-Z0-9]+([._]?[a-zA-Z0-9]+)*$/;
const fullNameRegularExpression = /^[a-zA-Z]{2,}(?: [a-zA-Z]+){0,2}$/;
const validateEmail = async (db, email, maxEmailLength = 250) => {
    if (email === undefined) {
        throw errors_1.fieldNotSetError('email');
    }
    if (email.length > maxEmailLength) {
        throw errors_1.emailTooLongError(email, maxEmailLength);
    }
    if (!emailRegularExpression.test(email)) {
        throw errors_1.emailInvalidFormatError(email);
    }
    const user = await db.findUserByEmail(email);
    if (user) {
        throw errors_1.userAlreadyExistsError(email);
    }
};
exports.validateEmail = validateEmail;
const validateUsername = async (db, username, maxUsernameLength = 250) => {
    if (username === undefined) {
        throw errors_1.fieldNotSetError('username');
    }
    if (username.length > maxUsernameLength) {
        throw errors_1.usernameTooLongError(username, maxUsernameLength);
    }
    if (!usernameRegularExpression.test(username)) {
        throw errors_1.usernameInvalidFormatError(username);
    }
    const user = await db.findUserByUsername(username);
    if (user) {
        throw errors_1.userAlreadyExistsError(username);
    }
};
exports.validateUsername = validateUsername;
const validateFullName = (fullName) => {
    if (fullName == undefined) {
        throw errors_1.fieldNotSetError('fullName');
    }
    if (!fullNameRegularExpression.test(fullName)) {
        throw errors_1.fullNameInvalidFormatError(fullName);
    }
};
exports.validateFullName = validateFullName;
const validatePassword = (password, minPasswordLength = 10) => {
    if (password === undefined) {
        throw errors_1.fieldNotSetError('password');
    }
    let lower = false;
    let upper = false;
    let num = false;
    let special = false;
    for (let i = 0; i < password.length; i++) {
        const c = password.charCodeAt(i);
        if (c >= 65 && c <= 90) {
            upper = true;
        }
        if (c >= 97 && c <= 122) {
            lower = true;
        }
        if (c >= 48 && c <= 57) {
            num = true;
        }
        if ((c >= 33 && c <= 47) || (c >= 58 && c <= 64) || (c >= 91 && c <= 96) || (c >= 123 && c <= 126)) {
            special = true;
        }
    }
    if (password.length < minPasswordLength) {
        throw errors_1.passwordTooShortError(password, minPasswordLength);
    }
    let errors = '';
    if (!upper) {
        errors += 'Need at least one uppercase.\n';
    }
    if (!lower) {
        errors += 'Need at least one lowercase.\n';
    }
    if (!num) {
        errors += 'Need at least one number.\n';
    }
    if (!special) {
        errors += 'Need at least one special character.\n';
    }
    if (errors !== '') {
        throw errors_1.passwordNotValidError(password, errors);
    }
};
exports.validatePassword = validatePassword;
//# sourceMappingURL=validation.js.map