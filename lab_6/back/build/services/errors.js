"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.badCredentials = exports.userAlreadyExistsError = exports.passwordNotValidError = exports.passwordTooShortError = exports.usernameTooLongError = exports.emailTooLongError = exports.fullNameInvalidFormatError = exports.usernameInvalidFormatError = exports.emailInvalidFormatError = exports.fieldNotSetError = void 0;
const fieldNotSetError = (field) => ({
    status: 400,
    errorCode: 'MISSING_FIELD',
    errorMessage: `Field ${field} must be set`,
});
exports.fieldNotSetError = fieldNotSetError;
const emailInvalidFormatError = (_email) => ({
    status: 400,
    errorCode: 'EMAIL_INVALID',
    errorMessage: 'Invalid email',
});
exports.emailInvalidFormatError = emailInvalidFormatError;
const usernameInvalidFormatError = (_email) => ({
    status: 400,
    errorCode: 'USERNAME_INVALID',
    errorMessage: 'Invalid username',
});
exports.usernameInvalidFormatError = usernameInvalidFormatError;
const fullNameInvalidFormatError = (_fullName) => ({
    status: 400,
    errorCode: 'FULLNAME_INVALID',
    errorMessage: 'Invalid full name',
});
exports.fullNameInvalidFormatError = fullNameInvalidFormatError;
const emailTooLongError = (_email, maxEmailLength) => ({
    status: 400,
    errorCode: 'EMAIL_TOO_LONG',
    errorMessage: `Email is too long: should not be longer than ${maxEmailLength} characters`,
});
exports.emailTooLongError = emailTooLongError;
const usernameTooLongError = (_username, maxUsernameLength) => ({
    status: 400,
    errorCode: 'USERNAME_TOO_LONG',
    errorMessage: `Username is too long: should not be longer than ${maxUsernameLength} characters`,
});
exports.usernameTooLongError = usernameTooLongError;
const passwordTooShortError = (_password, minPasswordLength) => ({
    status: 400,
    errorCode: 'PASSWORD_TOO_SHORT',
    errorMessage: `Password is too short: should be at least ${minPasswordLength} characters long`,
});
exports.passwordTooShortError = passwordTooShortError;
const passwordNotValidError = (_password, message) => ({
    status: 400,
    errorCode: 'PASSWORD_NOT_VALID',
    errorMessage: `Password is not valid: ${message}`,
});
exports.passwordNotValidError = passwordNotValidError;
const userAlreadyExistsError = (email) => ({
    status: 400,
    errorCode: 'USER_ALREADY_EXISTS',
    errorMessage: `User "${email}" already exists`,
});
exports.userAlreadyExistsError = userAlreadyExistsError;
const badCredentials = () => ({
    status: 400,
    errorCode: 'BAD_CREDENTIALS',
    errorMessage: 'Wrong username or password',
});
exports.badCredentials = badCredentials;
//# sourceMappingURL=errors.js.map