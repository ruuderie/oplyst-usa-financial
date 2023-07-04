<script setup>
import { ref, computed } from 'vue';
import LandingPage from '@/components/LandingPage.vue';

const content = ref({
	industryName: 'Startups',
	industryDescription:
		'Oplyst International, LLC provides strategic financial solutions designed to accelerate the vibrant and innovative world of startups. We comprehend the unique challenges and breakthrough opportunities that startups present, delivering tailored financial services that propel your entrepreneurial success.',
	pageTitle: 'Financial Solutions for Startups: Empowering Innovation and Growth',
	subTitle: 'Providing Tailored Financial Solutions for the Startup Industry',
	heroImage:
		'[Image showcasing a lively startup environment with financial graphs overlaid, symbolizing growth in startups through strategic financial backing]',
	why: {
		items: [
			{
				description:
					'Deep understanding of the startup ecosystem and its unique financial challenges and opportunities.',
			},
			{
				description: 'Tailored financial services to meet diverse startup needs.',
			},
			{
				description: 'Expert guidance from a team with extensive experience in startup financing.',
			},
			{
				description: 'A partnership grounded in integrity, agility, and unwavering commitment to your success.',
			},
		],
	},
	services: {
		items: [
			{
				title: 'Seed Funding',
				description: 'Financial backing to help get your startup off the ground and to the next level.',
			},
			{
				title: 'Working Capital Loans',
				description:
					'Ensure the smooth running of your day-to-day operations as you navigate the early stages of your business.',
			},
			{
				title: 'Equipment Loans',
				description:
					'Financing to help you purchase or upgrade essential equipment or technology for your startup.',
			},
		],
	},
	caseStudies: {
		sectionTitle: 'Case Studies',
		subTitle: "Explore how we've propelled growth in the startup industry with our expert financial solutions.",
		items: [],
	},
	callToAction: 'Ready to accelerate your startup with our tailored financial solutions? Contact us today!',
	faq: 'Have questions about our financial services for startups? Visit our FAQ section below or reach out to us for more information.',
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
