<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	serviceName: 'Invoice Financing',
	serviceDescription:
		"Oplyst International, LLC's Invoice Financing solutions offer a lifeline to businesses in need of improved cash flow. By converting outstanding invoices into immediate funds, we empower businesses to maintain smooth operations, meet financial obligations, and seize growth opportunities without waiting for invoice payments.",
	pageTitle: 'Invoice Financing: Transforming Receivables into Working Capital',
	subTitle: 'Unlocking Cash Flow with Invoice Financing Solutions',
	heroImage:
		'[Image depicting an invoice transforming into cash, symbolizing the immediate liquidity offered by invoice financing]',
	why: {
		items: [
			{
				description: 'Immediate access to funds locked in unpaid invoices.',
			},
			{
				description: 'Enhanced cash flow management.',
			},
			{
				description: 'Greater flexibility to meet operational costs and invest in growth opportunities.',
			},
			{
				description: 'Reduced dependency on traditional loans and lines of credit.',
			},
			{
				description: 'Confidence to offer favorable payment terms to clients without impacting liquidity.',
			},
			{
				description: 'Professional guidance from our experienced financial advisors.',
			},
		],
	},
	services: {
		items: [
			{
				title: 'Fast Funding',
				description: 'Quick processing to convert your invoices into cash in a matter of days.',
			},
			{
				title: 'Flexible Terms',
				description: 'Options to finance selected invoices or your entire accounts receivable.',
			},
			{
				title: 'Competitive Rates',
				description: 'Attractive financing rates that make invoice financing a cost-effective choice.',
			},
			{
				title: 'Seamless Integration',
				description: 'Easy integration with existing accounting systems for a smooth experience.',
			},
			{
				title: 'Custom Solutions',
				description: "Tailored invoice financing solutions to suit your business's unique needs.",
			},
		],
	},
	callToAction: 'Ready to unlock your cash flow potential with our Invoice Financing solutions? Contact us today!',
	faq: 'Have questions about our Invoice Financing service? Visit our FAQ section below or reach out to us for more information.',
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
			:services-items="content.services.items"
			:case-studies="content.caseStudies"
			:faq="content.faq"
			:call-to-action="content.callToAction"
		/>
	</div>
</template>
