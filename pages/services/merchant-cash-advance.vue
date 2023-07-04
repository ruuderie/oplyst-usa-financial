<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	serviceName: 'Merchant Cash Advance',
	serviceDescription:
		'A Merchant Cash Advance from Oplyst International, LLC provides your business with a lump sum of cash in exchange for a portion of future sales. This flexible financing solution is perfect for businesses with fluctuating sales or those that are seasonally active, enabling them to access funds when they need them the most.',
	pageTitle: 'Merchant Cash Advance: Financing Tailored to Your Sales Cycle',
	subTitle: 'Flexibility and Speed for Your Financial Needs',
	heroImage:
		'[Image showcasing the concept of a merchant cash advance, with a positive growth curve indicating the potential of this financing solution]',
	why: {
		items: [
			{
				description: 'Quick access to cash for immediate business needs.',
			},
			{
				description: "Repayments aligned with your business's sales cycle.",
			},
			{
				description: 'No need for perfect credit score or collateral.',
			},
			{
				description: 'Use funds as per your business requirements.',
			},
			{
				description: 'Expert guidance throughout the process.',
			},
		],
	},
	keyFeatures: {
		items: [
			{
				title: 'Fast Funding',
				description: 'Gain access to funds swiftly without long approval delays.',
			},
			{
				title: 'Flexible Repayment',
				description: 'Payback is proportionate to your sales, helping manage cash flow effectively.',
			},
			{
				title: 'No Collateral',
				description: 'Unlike traditional loans, a Merchant Cash Advance doesn’t require any collateral.',
			},
			{
				title: 'Unrestricted Use',
				description: 'Use your funds as you see fit, from inventory purchase to business expansion.',
			},
			{
				title: 'Streamlined Process',
				description: 'Minimal paperwork and quick approval for your convenience.',
			},
		],
	},
	callToAction: 'Unlock the power of your future sales with our Merchant Cash Advance. Contact us today!',
	faq: 'Have questions about our Merchant Cash Advance service? Visit our FAQ section below or reach out to us for more information.',
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
