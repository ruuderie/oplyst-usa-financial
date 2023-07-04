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
						<p class="title">Mission of Oplyst International, LLC</p>
						<p class="subtitle">
							Our mission at Oplyst International is to empower enterprises. We're here to provide
							innovative technology solutions, efficient logistics services, and strategic financial
							products that boost business success. Our vision is a world where complex challenges are
							overcome by intuitive solutions, shaped by Oplyst innovation. We commit to excellence,
							paving the way for a future where businesses thrive, backed by our institutional approach
							and unwavering commitment to our mission.
						</p>
					</div>
				</div>
			</section>
		</NuxtLayout>
	</div>
</template>

<style></style>
