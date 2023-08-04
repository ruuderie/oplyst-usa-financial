<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	serviceName: 'Commercial Real Estate Loans',
	serviceDescription:
		"At Oplyst International, LLC, we specialize in transforming your commercial real estate aspirations into tangible realities. Our tailored Commercial Real Estate Loans are strategic instruments designed to fulfill your ambitions, whether it's acquiring, expanding, or unlocking capital from commercial properties. Backed by our vast lender network and streamlined underwriting process, we're equipped to facilitate your success in the commercial real estate landscape.",
	pageTitle: 'Commercial Real Estate Loans: Building Your Commercial Property Dreams',
	subTitle: 'Catalyzing Success in the Commercial Real Estate Space',
	heroImage:
		'[Image showcasing a variety of commercial properties, symbolizing the potential and diversity of commercial real estate investments]',
	why: {
		items: [
			{
				description: 'Tailored financing solutions for your specific commercial property needs.',
			},
			{
				description: 'Access to a vast network of lenders, increasing your chances of loan approval.',
			},
			{
				description: 'Dedicated and experienced financial professionals to guide you throughout the process.',
			},
			{
				description: 'Streamlined underwriting process for a smooth, efficient loan approval journey.',
			},
			{
				description:
					'Opportunities to expand your commercial property portfolio, irrespective of your business structure.',
			},
			{
				description: 'Unwavering commitment to fueling your growth in the commercial real estate sector.',
			},
		],
	},
	keyFeatures: {
		items: [
			{
				title: 'Offices',
			},
			{
				title: 'Hotels',
			},
			{
				title: 'Multi-Family',
			},
			{
				title: 'Warehouses',
			},
			{
				title: 'Care Homes',
			},
			{
				title: 'Pubs and Restaurants',
			},
			{
				title: 'Retail Units',
			},
			{
				title: 'Mixed-Use Properties',
			},
			{
				title: 'Factories',
			},
			{
				title: 'Agricultural Lands',
			},
		],
	},
	callToAction:
		'Ready to make your property dreams a reality with our strategic Real Estate Loans? Contact us today!',
	faq: 'Have questions about our Real Estate Loans? Visit our FAQ section below or reach out to us for more information.',
});
</script>

<template>
	<div>
		<LandingPage
			:page-title="content.pageTitle"
			:sub-title="content.subTitle"
			:industry-name="content.serviceName"
			:industry-description="content.serviceDescription"
			:why-items="content.why.items"
			:services-items="content.keyFeatures.items"
			:case-studies="content.caseStudies"
			:faq="content.faq"
			:call-to-action="content.callToAction"
		/>
	</div>
</template>
