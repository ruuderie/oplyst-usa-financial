<script setup>
import { ref, onMounted } from 'vue';

const post = {
	summary: 'This is a summary',
	title: 'This is a title',
	featured_image: 'https://example.com/image.png',
};
</script>

<template>
	<div class="is-responsive is-mobile">
		<NuxtLayout>
			<section class="hero is-small is-primary is-fluid">
				<div class="hero-body is-fluid">
					<div class="my-3 has-text-centered">
						<p class="title">Overview of Oplyst International, LLC</p>
						<p class="subtitle">
							Explore a future shaped by innovation with Oplyst International, LLC. As disruptors and
							pioneers, we challenge conventional industry boundaries, focusing on simplicity, quality,
							and service. Our impact is demonstrated through over 100 innovative solutions, partnerships
							with over 50 global entities, and a growing network of more than 1000 satisfied clients.
							Experience the disruptive force of Oplyst, trusted by leading enterprises worldwide.
						</p>
					</div>
				</div>
			</section>
		</NuxtLayout>
	</div>
</template>

<style></style>
