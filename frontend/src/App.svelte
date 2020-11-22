<script lang="typescript">
  import Router from "svelte-spa-router";
  import { wrap } from "svelte-spa-router/wrap";
  import Home from "./routes/Home.svelte";

  const routes = {
    "/": Home,
    "/login": wrap({
      asyncComponent: () => import("./routes/Login.svelte")
    }),
    "*": Home
  };
</script>

<body>
  <Router {routes} />
</body>
