<script>
	//@ts-nocheck
	const bg_text_class = {
		normal: 'bg-slate-100 text-yellow-900',
		selected: 'bg-red-50 text-gray-500'
	};

	let page_num = 1;
	const set_page_num = (num) => {
		page_num = num;
	};
</script>

<section class="px-1">
	<nav class="w-full flex justify-end">
		<ul class="flex justify-end w-full gap-x-1">
			<li class="flex w-40">
				<a
					on:click={() => {
						set_page_num(1);
					}}
					href="/settings/paths"
					class={`pl-2 border text-left min-w-full rounded-sm ${
						page_num === 1 ? bg_text_class.selected : bg_text_class.normal
					}`}>Paths</a
				>
			</li>
			<li class="flex w-40">
				<a
					on:click={() => {
						set_page_num(2);
					}}
					href="/settings/photographers"
					class={`pl-2 border text-left min-w-full rounded-sm ${
						page_num === 2 ? bg_text_class.selected : bg_text_class.normal
					}`}>Photographer list</a
				>
			</li>
			<li class="flex w-40">
				<a
					on:click={() => {
						set_page_num(3);
					}}
					href="/settings/"
					class={`pl-2 border text-left min-w-full rounded-sm ${
						page_num === 3 ? bg_text_class.selected : bg_text_class.normal
					}`}>Other</a
				>
			</li>
		</ul>
	</nav>
	<div><slot /></div>
</section>
