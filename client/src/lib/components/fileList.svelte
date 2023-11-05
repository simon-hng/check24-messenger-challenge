<script lang="ts">
	import Fa from 'svelte-fa';
	import { faFilePdf, faFileImage, faXmark } from '@fortawesome/free-solid-svg-icons';
	export let files: FileList;
</script>

{#if files?.length}
	<ul class="list">
		{#each [...Array(files.length).keys()].map((i) => files.item(i)) as file}
			<li>
				<Fa icon={file?.type === 'application/pdf' ? faFilePdf : faFileImage} />
				<span class="flex-auto">
					{file?.name}
				</span>
				<button class="btn-icon">
					<Fa icon={faXmark} />
				</button>
			</li>
		{/each}
	</ul>
{/if}
