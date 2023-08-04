<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	industryName: 'Construction',
	industryDescription:
		'Oplyst International, LLC offers uniquely tailored financial solutions designed to fuel the dynamic and intricate world of construction. We understand the unique complexities and opportunities within the construction sector, delivering specialized financial services that fuel your growth and success.',
	pageTitle: 'Financial Solutions for the Construction Industry: Paving the Path to Success',
	subTitle: 'Providing Tailored Financial Solutions for the Construction Industry',
	heroImage:
		'[Image showcasing a bustling construction site with cranes and buildings in progress, superimposed with financial graphs, indicating the synergy between construction operations and financial growth]',
	why: {
		items: [
			{
				description:
					'Profound understanding of the operational and financial demands unique to the construction sector.',
			},
			{
				description: 'Tailored financial services to meet diverse construction needs.',
			},
			{
				description:
					'A team of financial professionals offering personalized guidance for your financial decisions.',
			},
			{
				description: 'A partnership anchored in integrity, agility, and commitment to your success',
			},
		],
	},
	services: {
		items: [
			{
				title: 'Equipment Loans',
				description:
					'Customized financing options to help you purchase or upgrade essential construction machinery.',
			},
			{
				title: 'Working Capital Loans',
				description:
					'Reliable financial support for your daily operations, ensuring seamless project execution.',
			},
			{
				title: 'Project Loans',
				description: 'Capital support for launching new construction projects or expanding existing ones.',
			},
		],
	},
	caseStudies: {
		sectionTitle: 'Case Studies',
		subTitle:
			'Explore how we’ve empowered growth in the construction industry with our expert financial solutions.',
		items: [],
	},
	callToAction:
		'Ready to elevate your construction operations with our specialized financial solutions? Contact us today!',
	faq: 'Have questions about our financial services for the construction industry? Visit our FAQ section below or reach out to us for more information.',
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
