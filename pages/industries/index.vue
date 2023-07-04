<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	industryName: 'Tailored Commercial Lending for Multiple Industries',
	industryDescription:
		'At Oplyst International, LLC, we offer a wide range of tailored financial solutions that are specifically designed to meet the unique needs of various sectors. Our vast experience in the commercial lending brokerage industry allows us to deliver specialized financial services that help businesses in the Real Estate, Construction, Healthcare, Manufacturing, Retail, Financial Services, and Technology sectors.',
	pageTitle: 'Financial Solutions for Multiple Industries: Navigating the Path to Success',
	subTitle: 'Providing Tailored Financial Solutions for Various Industry Sectors',
	heroImage:
		'[Image showcasing different industry sectors, with commercial lending icons overlaid, indicating the synergy between our lending solutions and the success of these sectors]',
	why: {
		items: [
			{
				description: 'In-depth understanding of the operational and financial demands unique to each sector.',
			},
			{
				description: 'Tailored financial services to meet the diverse needs of different industries.',
			},
			{
				description:
					'A team of financial professionals offering personalized guidance for your financial decisions.',
			},
			{
				description: 'A partnership anchored in integrity, agility, and commitment to your success.',
			},
		],
	},
	services: {
		items: [
			{
				title: 'Real Estate Loans',
				description:
					'Customized financing options to assist in real estate purchases, expansions, and renovations.',
			},
			{
				title: 'Equipment Loans',
				description:
					'Flexible financing for the purchase or upgrade of essential machinery and technology across all sectors.',
			},
			{
				title: 'Working Capital Loans',
				description: 'Reliable financial support for daily operations, ensuring seamless business execution.',
			},
		],
	},
	caseStudies: {
		sectionTitle: 'Case Studies',
		subTitle: "Discover how we've empowered growth across various industries with our expert financial solutions.",
		items: [],
	},
	callToAction:
		'Ready to take your business to the next level with our specialized financial solutions? Contact us today!',
	faq: 'Have questions about our financial services for your specific industry? Visit our FAQ section below or reach out to us for more information.',
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
