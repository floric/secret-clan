<script lang="typescript">
  import Router from "svelte-spa-router";
  import { wrap } from "svelte-spa-router/wrap";
  import NotFound from "./routes/errors/NotFound.svelte";

  const routes = {
    "/": wrap({
      asyncComponent: () => import("./routes/Home.svelte"),
    }),
    "/games": wrap({
      asyncComponent: () => import("./routes/Games.svelte"),
    }),
    "/games/:token": wrap({
      asyncComponent: () => import("./routes/GameLobby.svelte"),
    }),
    "*": NotFound,
  };
</script>

<body>
  <Router {routes} />
</body>
