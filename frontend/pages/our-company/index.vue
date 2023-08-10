<script setup></script>

<template>
    <div class="is-responsive is-mobile finance-theme">
        <NuxtLayout>
            <section class="hero finance-hero is-small is-primary is-fluid">
                <div class="hero-body is-fluid">
                    <div class="my-3 has-text-centered">
                        <p class="title">Welcome to Oplyst International, LLC</p>
                        <p class="subtitle">
                            The future of financial solutions. We are the trusted choice for leading enterprises, with
                            100+ innovative solutions, 50+ global partners, and 1000+ satisfied clients. We stand as
                            innovators, disruptors, and pioneers in redefining industry boundaries.
                        </p>
                    </div>
                </div>
            </section>
            <section class="finance-content is-fluid">
                <div class="section-content my-6 py-4">
                    <h2 class="title">Overview</h2>
                    <p class="subtitle">
                        Explore a future shaped by innovation with Oplyst International, LLC. As disruptors and
                        pioneers, we challenge conventional industry boundaries, focusing on simplicity, quality,
                        and service. Our impact is demonstrated through over 100 innovative solutions, partnerships
                        with over 50 global entities, and a growing network of more than 1000 satisfied clients.
                        Experience the disruptive force of Oplyst, trusted by leading enterprises worldwide.
                    </p>
                </div>
                <div class="section-content my-6 py-4">
                    <h2 class="title">Values</h2>
                    <p class="subtitle">
                        Oplyst International isn't just a company – we're a mindset, a disruptive force. Our
                        strength lies in our unique blend of technology, logistics, and financial advisory. We hold
                        dear the values of quality, simplicity, and client satisfaction, ensuring that these
                        principles guide us in providing holistic solutions to complex challenges. With Oplyst,
                        you're not just getting a service – you're joining a forward-thinking community of
                        innovators and pioneers.
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
