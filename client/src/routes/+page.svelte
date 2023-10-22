<script>
	import { userStore } from '$lib/userStore';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1 class="text-center mt-32 text-4xl font-serif">
		Aller Anfang ist schwer. Anfangen ist einfach, Beharrlichkeit eine Kunst
	</h1>

	{#if $userStore}
		<div class="m-8">
			Hello {$userStore.name}!
		</div>
	{/if}
</section>
