<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	industryName: 'Retail',
	industryDescription:
		'Oplyst International, LLC offers expert financial solutions aimed at empowering the dynamic and diverse world of retail. We understand the unique challenges and opportunities of the retail sector, providing tailored financial services designed to fuel your retail growth.',
	pageTitle: 'Financial Solutions for the Retail Industry: Boosting Commercial Success',
	subTitle: 'Providing Tailored Financial Solutions for the Retail Industry',
	heroImage:
		'[Image depicting a bustling retail store with financial graphs overlaid, symbolizing growth in retail through strategic financial solutions]',
	why: {
		items: [
			{
				description: 'Deep understanding of the retail sector and its unique challenges and opportunities.',
			},
			{
				description: 'Tailored financial services to meet a wide range of retail needs.',
			},
			{
				description: 'Expert guidance from a team with extensive experience in retail financing.',
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
					'Financing options that enable you to acquire or upgrade essential retail equipment or technology, keeping you competitive in a rapidly evolving market.',
			},
			{
				title: 'Inventory Loans',
				description:
					'Financial backing to manage seasonal inventory fluctuations and ensure your shelves remain stocked.',
			},
			{
				title: 'Expansion Loans',
				description: 'Capital support for opening new stores or expanding existing ones.',
			},
			{
				title: 'Working Capital Loans',
				description:
					'A safety net for managing the cyclical nature of retail cash flows and supporting your day-to-day operations.',
			},
		],
	},
	caseStudies: {
		sectionTitle: 'Case Studies',
		subTitle:
			"Discover how we've boosted commercial success in the retail industry with our expert financial solutions.",
		items: [],
	},
	callToAction: 'Ready to boost your retail operations with our specialized financial solutions? Contact us today!',
	faq: 'Have questions about our financial services for the retail industry? Visit our FAQ section below or reach out to us for more information.',
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
