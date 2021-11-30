# Password storage

### General info

* `back` - backend застосунку. Написаний на ts + fastify. DB: postgresql. У нас є проект із минулого семестру на мікросервісній архітектурі. Ми взяли мікросервіс auth, видалили лишнє і просто поправили збереження паролів
* `front` - frontend застосунку. Так само взяли увесь фронт із минулого проекту, тільки вирізали усі сторінки, окрім login & register

### Demo

![](./assets/demo.gif)

### How we store passwords

Увесь код для хешування пароля та його перевірки можна знайти у файлі `./src/crypto/utils.ts`.
Спочатку пароль хешується за допомогою алгоритма `argon2`. На нашу думку, на даний момент він є найкращим. Сіль для хешування генерується автоматично в середині бібліотеки.
Потім цей хеш шифрується за домопомого `XSalsa20`. `nonce` для цього генерується випадковим чином. Функція для генерації nonce береться із стандартного модуля `crypto`. Вона є cryptographically secure. Ключ для неї береться із заданої змінної середовища.

### Framework fastify

Fastify не має під капотом готових механізмів для авторизації/реєстрації/аутентифікації. Усе писали руками, тому в цьому розділі не має що описувати.
