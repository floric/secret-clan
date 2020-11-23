<script lang="typescript">
  import Router from "svelte-spa-router";
  import { wrap } from "svelte-spa-router/wrap";
  import Home from "./routes/Home.svelte";

  const routes = {
    "/": Home,
    "/games": wrap({
      asyncComponent: () => import("./routes/Games.svelte"),
    }),
    "/games/:id": wrap({
      asyncComponent: () => import("./routes/GameLobby.svelte"),
    }),
    "*": Home,
  };
</script>

<body>
  <Router {routes} />
</body>
