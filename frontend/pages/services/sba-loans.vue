<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	serviceName: 'SBA Loans',
	serviceDescription:
		'At Oplyst International, LLC, we navigate the complexities of Small Business Administration (SBA) loans to deliver customized financial solutions for your business. Leverage our expertise to secure low-interest, long-term loans designed to boost your business growth, whether you are starting out, expanding, or need working capital.',
	pageTitle: 'SBA Loans: Empowering Your Business Growth',
	subTitle: 'Unlocking the Potential of Small Business Administration Loans',
	heroImage:
		'[Image illustrating the potential of SBA Loans, highlighting key concepts such as growth, expansion, and success]',
	why: {
		items: [
			{
				description: 'Access to low-interest, long-term loans.',
			},
			{
				description: 'Customizable solutions for various business needs.',
			},
			{
				description: 'Expert guidance through the complex SBA loan process.',
			},
			{
				description: 'Flexible repayment terms to ease financial burden.',
			},
			{
				description: 'Commitment to your business growth and success.',
			},
		],
	},
	keyFeatures: {
		items: [
			{
				title: 'Competitive Rates',
				description: 'Benefit from some of the lowest interest rates available for small business loans.',
			},
			{
				title: 'Long Repayment Terms',
				description: 'Ease your financial management with extended repayment terms.',
			},
			{
				title: 'Loan Variety',
				description: 'Access different types of SBA loans to suit your specific business needs.',
			},
			{
				title: 'Flexible Use',
				description:
					'Use your loan for a variety of purposes such as starting a business, expansion, or covering operational costs.',
			},
			{
				title: 'Expert Support',
				description: 'Navigate the SBA loan process with guidance from our experienced professionals.',
			},
		],
	},
	callToAction: 'Take your business to new heights with our customized SBA Loans. Contact us today!',
	faq: 'Have questions about our SBA Loan services? Visit our FAQ section below or reach out to us for more information.',
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
