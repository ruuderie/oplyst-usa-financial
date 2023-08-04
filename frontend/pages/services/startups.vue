<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	serviceName: 'Startup Financing',
	serviceDescription:
		'Fuel your innovative startup venture with Oplyst International, LLC. Our customized financial solutions cater specifically to startups, helping to transform your ambitious ideas into a thriving business. Navigate the exciting, challenging world of startups with us as your financial partner.',
	pageTitle: 'Financial Solutions for Startups: Kickstarting Your Business Success',
	subTitle: 'Fueling Innovation with Tailored Startup Financing',
	heroImage: '[Image depicting a rocket launching, symbolizing the start and rapid growth of startup businesses]',
	why: {
		items: [
			{
				description: 'Tailored financing solutions designed for startups.',
			},
			{
				description:
					'Support for a wide array of startup needs, from initial seed funding to expansion capital.',
			},
			{
				description: 'Guidance from experienced professionals familiar with the unique challenges of startups.',
			},
			{
				description:
					'A partnership centered around integrity, agility, and a commitment to your business success.',
			},
		],
	},
	keyFeatures: {
		items: [
			{
				title: 'Seed Financing',
				description: 'Early-stage funding to help get your business off the ground.',
			},
			{
				title: 'Growth Capital',
				description: 'Financial support to aid in the expansion of your growing business.',
			},
			{
				title: 'Working Capital Loans',
				description: 'Essential financing to ensure the smooth operation of your day-to-day business.',
			},
			{
				title: 'Equipment Loans',
				description: 'Funding to acquire the vital equipment your startup needs to thrive.',
			},
			{
				title: 'Expert Guidance',
				description:
					'Valuable advice and support from our team of financial professionals experienced in startup financing.',
			},
		],
	},
	callToAction: "Ignite your startup's potential with our tailored financing solutions. Reach out to us today!",
	faq: 'Have questions about our Startup Financing services? Explore our FAQ section or get in touch for more information.',
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
			:services-items="content.keyFeatures.items"
			:case-studies="content.caseStudies"
			:faq="content.faq"
			:call-to-action="content.callToAction"
		/>
	</div>
</template>
