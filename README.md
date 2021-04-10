# Pegasus Server

## About the project

Pegasus server provides a GraphQL API to store and retrieve passwords, it should be a zero knowledge system.

This project is part of a series of live streams by me, you can find them at: https://www.twitch.tv/simoneromano96

I stream at 16 pm CEST (UTC+01/+02)

Currently this is a toy project to learn more about cryptography, I wouldn't use it if I were in you.

## Libraries/Technologies

This list is non-exhaustive of course, see the `cargo.toml` file.

* Actix web (HTTP Server)

* Async Graphql (GraphQL Library)

* Anyhow (Easier idiomatic error handling)

* Redis (Redis client, duh)

* Log + log4rs (Well, logging)

* Djangohashers (Exposes some simple to use password hashing functions, to be removed)

* Config (Handle hierarical configuration)

* Wither (mongodb ODM)

* Rand + Rand chacha (Cryptographically secure random number generators)

* sha3 (For generic hashing)

## Security checklist

Securing the mongo db:

* https://docs.mongodb.com/manual/administration/security-checklist/

Securing redis:

* https://redis.io/topics/security

## References

Password hasher:

* https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html

Symmetric Encryption:

* https://blog.cloudflare.com/do-the-chacha-better-mobile-performance-with-cryptography/

* https://cr.yp.to/streamciphers/why.html
