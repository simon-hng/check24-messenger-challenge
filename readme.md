# CHECK24 GenDev Messenger Challenge

[![IMAGE ALT TEXT HERE](https://img.youtube.com/vi/56HFfVtnOIU/0.jpg)](https://www.youtube.com/watch?v=56HFfVtnOIU)

## Run locally

```bash
docker-compose up -d
```

Visit the client in `http://localhost:4173`.
You will need to create a new account.

## Approach

![System Design](assets/system_design.png)

We have a simple client/server architecture.
Due to the requirement to send and receive notifications in real time, I decided to use websockets.

Messages are sent to the application server with a rest API,
which persists them in the database and notifies recipients if they have an active session.

### API

- [Actix and Actix Web](https://actix.rs/) - Actor framework and web server
- [SeaORM](https://www.sea-ql.org/SeaORM/) - Migrations, ORM
- [PostreSQL](https://www.postgresql.org/) - Relational Database

### Web client

- [bun](https://bun.sh/) - JS Runtime, package manager, etc
- [Svelte Kit](https://kit.svelte.dev/) - UI Framework
- [TailwindCSS](https://tailwindcss.com/) - CSS Framework
- [SkeletonUI](https://www.skeleton.dev/) - UI Library

## Future work

- Authentication
- Bug fixes
  - File upload is not persisted
  - Reloading is necessary sometimes as state is not updated correctly
- Receiving files that are not images
- Sent / Received receipts (similiar to whatsapp)
- Scalability
- Cloud deployment
- CI/CD
- Add linting and testing

## Lessons learned

1. If you want to get a project done quickly, don't use a framework that has around 10000 active users (Actix-web), as support very extremley sparse when debugging.
1. If you want to get a project done quickly, use a language you already know by heart. (And maybe more high level than rust)
1. Making mistakes in the initial system design is fine, as long as you are able to rectify them later on.
