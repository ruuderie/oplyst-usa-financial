<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	serviceName: 'Equipment Loans',
	serviceDescription:
		"Revolutionize your operations with Oplyst International, LLC's tailored Equipment Loans. We understand the crucial role that modern, high-performance equipment plays in your business. Our loans are designed to enable you to acquire or upgrade necessary equipment across various industries, boosting your operational efficiency and competitiveness.",
	pageTitle: 'Equipment Loans: Powering Your Operational Success',
	subTitle: 'Empowering Businesses with Tailored Equipment Financing',
	heroImage: '[Image showcasing a variety of industrial equipment, symbolizing the scope of equipment financing]',
	why: {
		items: [
			{
				description: 'Customized financing solutions for a diverse range of equipment needs.',
			},
			{
				description: 'Easy application and quick approval process.',
			},
			{
				description: 'Competitive interest rates and flexible repayment terms.',
			},
			{
				description: 'Preserve your working capital and cash flow for other operational needs.',
			},
			{
				description: 'Expert guidance from our team of financial professionals.',
			},
			{
				description: 'Opportunity to upgrade outdated equipment and stay ahead in the market.',
			},
		],
	},
	services: {
		items: [
			{
				title: 'Manufacturing Equipment',
				description: 'Upgrade your production line with state-of-the-art machinery.',
			},
			{
				title: 'Medical Equipment',
				description: 'Equip your healthcare facility with cutting-edge medical technologies.',
			},
			{
				title: 'Construction Equipment',
				description: 'Invest in advanced tools to bolster your construction projects.',
			},
			{
				title: 'Office Equipment',
				description: 'Modernize your workspace with the latest office technologies.',
			},
			{
				title: 'Commercial Vehicle Financing',
				description: 'Expand your fleet with our commercial vehicle financing solutions.',
			},
		],
	},
	callToAction: 'Ready to modernize your operations with our Equipment Loans? Contact us today!',
	faq: 'Have questions about our Equipment Loans? Visit our FAQ section below or reach out to us for more information.',
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
