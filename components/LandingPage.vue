<script setup lang="ts">
const props = defineProps({
	pageTitle: {
		type: String,
		required: true,
	},
	subTitle: {
		type: String,
		required: true,
	},
	industryName: {
		type: String,
		required: false,
	},
	industryDescription: {
		type: String,
		required: true,
	},
	whyItems: {
		type: Array,
		required: true,
		default: () => [],
	},
	servicesItems: {
		type: Array,
		required: true,
		default: () => [],
	},
	caseStudies: {
		type: Object,
		required: true,
		default: () => [],
	},
	faq: {
		type: String,
		required: true,
		default: () => '',
	},
	callToAction: {
		type: String,
		required: false,
	},
});

const name = ref('');
const email = ref('');
const phone = ref('');

const submitForm = () => {
	console.log('name', name.value);
	console.log('email', email.value);
	console.log('phone', phone.value);
};
</script>

<template>
	<div class="is-responsive is-mobile">
		<NuxtLayout>
			<section class="hero is-small is-primary is-fluid">
				<div class="hero-body is-fluid">
					<div class="my-3 has-text-centered">
						<h1 class="title">{{ pageTitle }}</h1>
						<p class="subtitle">{{ subTitle }}</p>
					</div>
				</div>
			</section>
			<section class="container is-fluid">
				<div class="columns">
					<div class="column is-three-quarters">
						<div class="content is-medium">
							<p>{{ industryDescription }}</p>
							<div v-if="whyItems.length > 0">
								<h2>Why Choose Our Financial Solutions for {{ industryName }}</h2>
								<ul>
									<li v-for="item in whyItems">
										<p>{{ item.description }}</p>
									</li>
								</ul>
							</div>
							<ul>
								<li v-for="item in servicesItems">
									<h3>{{ item.title }}</h3>
									<p>{{ item.description }}</p>
								</li>
							</ul>
							<div v-if="caseStudies.length > 0">
								<h2>Case Studies/Testimonials</h2>
								<p>{{ caseStudies }}</p>
							</div>
							<h2>FAQ</h2>
							<p>{{ faq }}</p>
						</div>
					</div>
					<div class="column">
						<section class="is-fluid">
							<p>{{ callToAction }}</p>
							<div class="field">
								<label class="label">Name</label>
								<div class="control">
									<input v-model="name" class="input" type="text" />
								</div>
							</div>
							<div class="field">
								<label class="label">Email</label>
								<div class="control">
									<input v-model="email" class="input" type="email" />
								</div>
							</div>
							<div class="field">
								<label class="label">Phone</label>
								<div class="control">
									<input v-model="phone" class="input" type="tel" />
								</div>
							</div>
							<div class="control">
								<button class="button is-primary" @click="submitForm">Submit</button>
							</div>
						</section>
					</div>
				</div>
			</section>
			<NuxtPage />
		</NuxtLayout>
	</div>
</template>
