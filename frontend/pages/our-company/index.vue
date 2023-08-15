<script setup></script>

<template>
    <div class="is-responsive is-mobile finance-theme">
        <NuxtLayout>
            <section class="hero finance-hero is-small is-primary is-fluid">
                <div class="hero-body is-fluid">
                    <div class="my-3 has-text-centered">
                        <p class="title">Our Company</p>
                        <span/>
                        <p class="subtitle">
                        At Oplyst International, LLC, we understand the myriad challenges encountered by small to middle market enterprises across diverse economic and business cycles. Boasting a vast network of over 300 lenders, we confidently facilitate loans spanning from $100,000 to an impressive $200 million+. Our lending repertoire includes Term Loans, Asset-based Loans, Invoice Factoring, Trade Finance, Equipment Financing, and Commercial Mortgage Loans, catering to a spectrum of financial needs.
                        </p>
                    </div>
                </div>
            </section>
            <section class="finance-content is-fluid">
                <div class="section-content my-6 py-4">
                    <p class="subtitle">
                        Established in 2012 in Stamford, Connecticut, our primary goal is to support outstanding businesses led by dedicated management teams with innovative ideas, products, and services. We are driven by the potential for meaningful global progress and seek to fund initiatives aligned with this mission. We are here to facilitate the growth, stability, and success of businesses. Contact us to explore how we can assist in your journey to success.
                    </p>
                </div>
                <div class="section-content my-6 py-4">
                    <h2 class="title">Mission</h2>
                    <p class="subtitle">
                        Our mission at Oplyst International is to empower enterprises. We're here to provide
                        innovative technology solutions, efficient logistics services, and strategic financial
                        products that boost business success. Our vision is a world where complex challenges are
                        overcome by intuitive solutions, shaped by Oplyst innovation. We commit to excellence,
                        paving the way for a future where businesses thrive, backed by our institutional approach
                        and unwavering commitment to our mission.
                    </p>
                </div>
            </section>
            <NuxtPage />
        </NuxtLayout>
    </div>
</template>

<style scoped>
.finance-theme {
    font-family: 'Goldman Sachs', sans-serif; /* Assuming the font is named Goldman Sachs */
    background-color: #F5F5F5; /* Soft gray background */
}

.finance-hero {
    background-color: #2173b3; 
    color: #EFEFEF; /* Soft white for text on dark background */
}

.finance-content {
    color: #333; /* Dark gray color for the main content */
    padding: 2rem 1rem; /* Padding around content sections */
}

.title, .subtitle {
    margin-bottom: 15px;
}

.section-content:not(:last-child) {
    border-bottom: 1px solid #E0E0E0; /* Divider between sections */
}

.container a:hover, .hero a:hover {
    border-bottom: 2px solid #2E3A46; /* Subtle hover effect */
}

.container a:active, .hero a:active {
    color: #EFEFEF; /* Subtle active state */
}
</style>
