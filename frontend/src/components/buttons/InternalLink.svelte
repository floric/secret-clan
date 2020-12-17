<script lang="typescript">
  import { link } from "svelte-spa-router";

  export let href: string;
</script>

<a
  {href}
  use:link
  class="inline-flex text-xs sm:text-sm text-blue-500 hover:text-blue-700"><slot /></a>
