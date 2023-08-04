<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	serviceName: 'Lines of Credit',
	serviceDescription:
		"Unlock the financial flexibility you need to seize opportunities, manage daily expenses, and fuel your business's growth with Oplyst International, LLC's Lines of Credit solutions. Our streamlined process, flexible terms, and expert guidance make accessing extra funds as straightforward as possible.",
	pageTitle: 'Lines of Credit: Ensuring Financial Flexibility for your Business',
	subTitle: 'Access to Funds When You Need Them, With Our Lines of Credit Solutions',
	heroImage:
		'[Image depicting a credit card and a growing line graph, symbolizing the flexibility and growth potential offered by lines of credit]',
	why: {
		items: [
			{
				description: 'Flexibility to use funds as per your business needs.',
			},
			{
				description: 'Access to funds on demand, without applying for separate loans.',
			},
			{
				description: 'Only pay interest on the funds you actually use.',
			},
			{
				description: 'Potential to improve your business credit rating.',
			},
			{
				description: 'Serves as a financial safety net for unexpected business expenses.',
			},
			{
				description: 'Expert guidance from our experienced financial advisors.',
			},
		],
	},
	keyFeatures: {
		items: [
			{
				title: 'Flexible Access',
				description: 'Draw from your line of credit whenever you need to, up to your credit limit.',
			},
			{
				title: 'Revolve Credit',
				description: 'As you repay, your credit becomes available again to use in the future.',
			},
			{
				title: 'Competitive Rates',
				description: 'Attractive interest rates that make our lines of credit a cost-effective choice.',
			},
			{
				title: 'Quick Decisions',
				description: 'Fast, straightforward approval process for quick access to funds.',
			},
			{
				title: 'Tailored Solutions',
				description: "Lines of Credit tailored to your business's unique needs and financial situation.",
			},
		],
	},
	callToAction: 'Ready to experience financial flexibility with our Lines of Credit solutions? Contact us today!',
	faq: 'Have questions about our Lines of Credit service? Visit our FAQ section below or reach out to us for more information.',
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
