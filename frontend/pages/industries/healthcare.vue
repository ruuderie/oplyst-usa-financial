<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	industryName: 'Healthcare',
	industryDescription:
		'At Oplyst International, LLC, we specialize in creating tailored financial solutions for the dynamic and vital healthcare industry. Our extensive knowledge and commitment to your growth position us as the preferred financial partner for healthcare professionals and institutions alike.',
	pageTitle: 'Financial Solutions for the Healthcare Industry: Powering Growth, Ensuring Success',
	subTitle: 'Providing Tailored Financial Solutions for the Healthcare Industry',
	heroImage: '[Image depicting financial vitality infused into the healthcare sector]',
	why: {
		items: [
			{
				description: "In-depth understanding of the healthcare sector's unique challenges and opportunities.",
			},
			{
				description:
					'A broad range of customized financial solutions to suit the varied needs of healthcare professionals and institutions.',
			},
			{
				description:
					'Expert guidance and support from a team with extensive experience in healthcare financing.',
			},
			{
				description: 'A partnership built on integrity, agility, and an unwavering commitment to your success.',
			},
		],
	},
	services: {
		items: [
			{
				title: 'Equipment Loans',
				description: 'Financing to acquire or upgrade crucial medical equipment, ensuring top-tier care.',
			},
			{
				title: 'Practice Loans',
				description: 'Capital for starting, expanding, or enhancing your medical practice.',
			},
			{
				title: 'Working Capital Loans',
				description: 'Financing to help manage day-to-day operations smoothly.',
			},
		],
	},
	caseStudies: {
		sectionTitle: 'Case Studies',
		subTitle: 'Explore how we’ve empowered growth in the healthcare industry with our expert financial solutions.',
		items: [],
	},
	callToAction:
		'Ready to elevate your healthcare services with our specialized financial solutions? Contact us today!',
	faq: 'Have questions about our financial services for the healthcare industry? Visit our FAQ section below or reach out to us for more information.',
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
