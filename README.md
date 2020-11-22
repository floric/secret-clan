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

## Development

Run the following commands only the first time or if new dependencies were added:

```bash
cd frontend
yarn
yarn build
```

After that, start development with two terminals and the following commands:

`make watch-fe` and `make watch-be`

This will start watch modes for the server and frontend and will automatically update the code and restart the server. The update process is visible in the terminal output.

## Production Build

Run the following command to create the Docker image with the full app:

`make build`

Run the following command to start the image created previously. This will start the server and the application should be available on [localhost:3333](http://localhost:3333). Use the environment variables to change the host or other settings.

`make run`

## Necessary tools

Frontend: [Node 14, preferred installation with NVM](https://github.com/nvm-sh/nvm), [Yarn](https://yarnpkg.com/)

Backend: [Rust](https://www.rust-lang.org/learn/get-started), [Cargo Watch](https://crates.io/crates/cargo-watch)

General: [Docker Desktop](https://www.docker.com/get-started)

## Technologies

### Frontend

[Svelte](https://svelte.dev/)
[Tailwind CSS](https://tailwindcss.com/)

### Backend

[Warp](https://github.com/seanmonstar/warp)
