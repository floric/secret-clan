<script lang="typescript">
  import Router from "svelte-spa-router";
  import { wrap } from "svelte-spa-router/wrap";
  import NotFound from "./routes/errors/NotFound.svelte";
  import Unexpected from "./routes/errors/Unexpected.svelte";
  import css from "./main.css";

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
    "/errors/unexpected": Unexpected,
    "*": NotFound,
  };
</script>

<body>
  <Router {routes} />
</body>
