<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	industryName: 'Comprehensive Financial Services',
	industryDescription:
		'Oplyst International, LLC is proud to offer a comprehensive suite of financial solutions to fuel your business growth. Our experienced team understands the unique challenges of various industries, and we provide customized financial solutions, including Commercial Mortgages, Equipment Loans, Lines of Credit, Invoice Financing, Startup Loans, SBA Loans, and Merchant Cash Advances.',
	pageTitle: 'Our Services: Tailored Financial Solutions to Propel Your Business',
	subTitle: 'Explore Our Suite of Customized Loan Options',
	heroImage:
		'[Image showcasing various industry sectors with the corresponding financial services icons overlaid, symbolizing the broad range of our financial solutions.]',
	why: {
		items: [
			{
				description:
					'We offer a wide range of financial services tailored to meet your specific business needs.',
			},
			{
				description: 'Our team of experts is dedicated to helping you achieve your business goals.',
			},
			{
				description: 'We work closely with you to understand your business and provide the best solutions.',
			},
			{
				description: 'Our innovative solutions are designed to help you grow and succeed in your industry.',
			},
		],
	},
	services: {
		items: [
			{
				title: 'Commercial Mortgages',
				description: 'Flexible financing solutions for real estate purchases, expansions, and renovations.',
			},
			{
				title: 'Equipment Loans',
				description: 'Customized loans for purchasing or upgrading essential machinery and technology.',
			},
			{
				title: 'Lines of Credit',
				description: 'Accessible financial support for your daily business operations.',
			},
			{
				title: 'Invoice Financing',
				description: 'Optimize your cash flow by financing against unpaid invoices.',
			},
			{
				title: 'Startup Loans',
				description: 'Supporting new ventures with specialized financing solutions.',
			},
			{
				title: 'SBA Loans',
				description: 'Leverage government-guaranteed loans for your small business needs.',
			},
			{
				title: 'Merchant Cash Advance',
				description: 'Access quick cash advance against your future credit card sales.',
			},
		],
	},
	callToAction: 'Ready to accelerate your business growth with our specialized financial services? Contact us today!',
	faq: 'Have questions about our loan services? Visit our FAQ section below or reach out to us for more information.',
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
