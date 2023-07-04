<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	industryName: 'Logistics',
	industryDescription:
		'Oplyst International, LLC provides specialized financial solutions crafted to empower the intricate and fast-paced world of logistics. We understand the unique challenges and opportunities of the logistics sector, delivering tailored financial services that drive your operational efficiency and growth.',
	pageTitle: 'Financial Solutions for the Logistics Industry: Catalyzing Efficient Operations',
	subTitle: 'Providing Tailored Financial Solutions for the Logistics Industry',
	heroImage:
		'[Image illustrating a bustling port with containers and cranes, overlaid with financial growth charts, symbolizing the synergy between logistics operations and financial backing]',
	why: {
		items: [
			{
				description: 'Expertise in the unique operational and financial demands of the logistics sector.',
			},
			{
				description: 'Tailored financial services to meet a diverse range of logistics needs.',
			},
			{
				description: 'Experienced team offering personalized guidance for your financial decision-making.',
			},
			{
				description:
					'A partnership rooted in integrity, agility, and an unwavering commitment to your success.',
			},
		],
	},
	services: {
		items: [
			{
				title: 'Equipment Loans',
				description:
					'Flexible financing options that enable you to acquire or upgrade crucial logistics equipment and vehicles.',
			},
			{
				title: 'Working Capital Loans',
				description:
					'Financial support for your daily operations, ensuring the smooth running of your logistics activities.',
			},
			{
				title: 'Expansion Loans',
				description: 'Capital support for your ambitions to expand operations or enter new markets.',
			},
		],
	},
	caseStudies: {
		sectionTitle: 'Case Studies',
		subTitle:
			'Discover how we’ve catalyzed efficiency and growth in the logistics industry with our specialized financial solutions.',
		items: [],
	},
	callToAction:
		'Ready to streamline your logistics operations with our customized financial solutions? Contact us today!',
	faq: 'Have questions about our financial services for the logistics industry? Visit our FAQ section below or reach out to us for more information.',
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
