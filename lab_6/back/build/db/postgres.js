"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Postgres = void 0;
const pg_1 = require("pg");
class Postgres {
    constructor(opts) {
        try {
            this.connectionPool = new pg_1.Pool(opts);
        }
        catch (e) {
            console.error('Failed to connect to Postgres: ', e);
            throw e;
        }
    }
    async findUserByEmail(email) {
        const client = await this.connectionPool.connect();
        const rows = (await client.query('select * from users where email = $1', [email])).rows;
        client.release();
        return rows[0];
    }
    async findUserByUsername(username) {
        const client = await this.connectionPool.connect();
        const rows = (await client.query('select * from users where username = $1', [username])).rows.map((user) => ({ ...user, fullName: user['full_name'], full_name: undefined }));
        client.release();
        return rows[0];
    }
    async findUserById(id) {
        const client = await this.connectionPool.connect();
        const rows = (await client.query('select * from users where id = $1', [id])).rows;
        client.release();
        const rawUser = rows[0];
        return {
            id: rawUser.id,
            username: rawUser.username,
            email: rawUser.email,
            fullName: rawUser['full_name'],
        };
    }
    async saveUser(user) {
        const client = await this.connectionPool.connect();
        await client.query('insert into users values ($1, $2, $3, $4, $5)', [
            user.id,
            user.username,
            user.email,
            user.fullName,
            user.password,
        ]);
        client.release();
    }
    async findUsersByGroupId(groupId) {
        const client = await this.connectionPool.connect();
        const ids = (await client.query('select user_id from groups2users where group_id=$1', [groupId])).rows.map(row => row['user_id']);
        client.release();
        return ids;
    }
}
exports.Postgres = Postgres;
//# sourceMappingURL=postgres.js.map