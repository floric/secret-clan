# Secret Clan

Based on another similiar, famous game we decided to create a small Webapp for fun.

## Structure

```
├── backend | Backend based on Warp (Rust)
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
├── frontend | Frontend based on Svelte and Tailwind (TS, HTML, JS)
|   ├── public
│   ├── src
│   ├── package.json
│   ├── postcss.config.js
│   ├── tailwind.config.js
│   ├── webpack.config.js
│   └── yarn.lock
├── docker-entrypoint.sh
├── Dockerfile
├── Makefile
└── README.md
```

## Commands

Run the following command to create the Docker image with the full app:

`make build`

Run the following command to start the image created previously. This will start the server and the application should be available on [localhost:3333](http://localhost:3333). Use the environment variables to change the host or other settings.

`make run`

## Links

### Frontend

[Svelte](https://svelte.dev/)

### Backend

[Warp](https://github.com/seanmonstar/warp)
