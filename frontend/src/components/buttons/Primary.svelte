<script lang="typescript">
  import { push } from "svelte-spa-router";

  export let onClick: () => Promise<any>;

  const handleOnClick = async () => {
    try {
      if (onClick) {
        await onClick();
      }
    } catch (err) {
      console.error(err);
      await push("/errors/unexpected");
    }
  };
</script>

<button
  type="submit"
  on:click={handleOnClick}
  class="flex items-center justify-center focus:outline-none text-white text-sm sm:text-base bg-blue-600 hover:bg-blue-700 rounded py-2 px-4 md:w-auto w-full transition duration-150 ease-in">
  <slot />
</button>
