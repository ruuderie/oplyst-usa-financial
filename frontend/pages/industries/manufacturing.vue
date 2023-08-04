<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	industryName: 'Manufacturing',
	industryDescription:
		'Oplyst International, LLC provides expert financial solutions designed to turbocharge the vibrant sector of manufacturing. We understand the intricate dynamics of manufacturing operations and are committed to fostering industrial progress through robust financial partnerships.',
	pageTitle: 'Financial Solutions for the Manufacturing Industry: Fuelling Industrial Growth',
	subTitle: 'Providing Tailored Financial Solutions for the Manufacturing Industry',
	heroImage:
		'[Image depicting a manufacturing plant with financial graphs overlaid, symbolizing growth in manufacturing through strategic financial solutions]',
	why: {
		items: [
			{
				description: 'Deep understanding of the diverse aspects of the manufacturing sector.',
			},
			{
				description: 'Tailored financial solutions to meet a wide range of manufacturing needs.',
			},
			{
				description: 'Expert guidance from a team with vast experience in manufacturing financing.',
			},
			{
				description: 'A partnership based on integrity, agility, and an unwavering commitment to your success.',
			},
		],
	},
	services: {
		items: [
			{
				title: 'Equipment Loans',
				description:
					'Financing solutions to facilitate the acquisition or upgrade of crucial manufacturing equipment, thus boosting productivity.',
			},
			{
				title: 'Working Capital Loans',
				description:
					'Capital support to ensure the smooth running of your day-to-day manufacturing operations.',
			},
			{
				title: 'Expansion Loans',
				description: 'Financial backing for your plans to grow your operations or venture into new markets.',
			},
		],
	},
	caseStudies: {
		sectionTitle: 'Case Studies',
		subTitle: 'Explore how we’ve fueled growth in the manufacturing industry with our expert financial solutions.',
		items: [],
	},
	callToAction:
		'Ready to fuel your manufacturing operations with our customized financial solutions? Contact us today!',
	faq: 'Have questions about our financial services for the manufacturing industry? Visit our FAQ section below or reach out to us for more information.',
});
</script>

<template>
	<div>
		<LandingPage
			:page-title="content.pageTitle"
			:sub-title="content.subTitle"
			:industry-name="content.industryName"
			:industry-description="content.industryDescription"
			:why-items="content.why.items"
			:services-items="content.services.items"
			:case-studies="content.caseStudies"
			:faq="content.faq"
			:call-to-action="content.callToAction"
		/>
	</div>
</template>
