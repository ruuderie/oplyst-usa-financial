<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	industryName: 'Real Estate',
	industryDescription:
		'At Oplyst International, LLC, we specialize in engineering financial solutions for the dynamic real estate industry. We recognize the potential that lies in real estate investments and are committed to empowering you to seize these opportunities through strategic financial partnerships.',
	pageTitle: 'Financial Solutions for the Real Estate Industry: Building Foundations for Success',
	subTitle: 'Providing Tailored Financial Solutions for the Real Estate Industry',
	heroImage:
		'[Image depicting the construction of a building with financial graphs overlay, symbolizing growth in real estate through financial solutions]',
	why: {
		items: [
			{
				description: 'Profound understanding of the complexities of the real estate market.',
			},
			{
				description: 'A diverse range of tailored financial solutions to suit varied real estate investments.',
			},
			{
				description: 'Expert advice and guidance from a team with rich experience in real estate financing.',
			},
			{
				description:
					'A partnership grounded in integrity, agility, and an unwavering commitment to your success.',
			},
		],
	},
	services: {
		items: [
			{
				title: 'Commercial Real Estate Loans',
				description:
					'Financing solutions for the purchase, expansion, or capital release from commercial properties.',
			},
			{
				title: 'Real Estate Lines of Credit',
				description: 'Gain flexible access to capital for acquiring, renovating, or expanding properties.',
			},
			{
				title: 'Construction Loans',
				description:
					'Funding designed specifically for builders and developers to finance new construction projects.',
			},
		],
	},
	caseStudies: {
		sectionTitle: 'Case Studies',
		subTitle:
			"Discover how we've laid foundations for success in the real estate industry with our expert financial solutions.",
		items: [],
	},
	callToAction:
		'Ready to lay the foundation for success in your real estate venture with our specialized financial solutions? Contact us today!',
	faq: 'Have questions about our financial services for the real estate industry? Visit our FAQ section below or reach out to us for more information.',
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
