<script setup>
import { ref, onMounted } from 'vue';
</script>

<template>
    <div class="is-responsive is-mobile finance-theme">
        <NuxtLayout>
            <section class="hero finance-hero-mission is-small is-primary is-fluid">
                <div class="hero-body is-fluid">
                    <div class="my-3 has-text-centered">
                        <p class="title">Mission of Oplyst International, LLC</p>
                        <p class="subtitle">
                            Our mission at Oplyst International is to empower enterprises. We're here to provide
                            innovative technology solutions, efficient logistics services, and strategic financial
                            products that boost business success. Our vision is a world where complex challenges are
                            overcome by intuitive solutions, shaped by Oplyst innovation. We commit to excellence,
                            paving the way for a future where businesses thrive, backed by our institutional approach
                            and unwavering commitment to our mission.
                        </p>
                    </div>
                </div>
            </section>
        </NuxtLayout>
    </div>
</template>

<style scoped>
.finance-theme {
    font-family: 'Goldman Sachs', sans-serif; /* Assuming the font is named Goldman Sachs */
    background-color: #F5F5F5; /* Soft gray background */
}

.finance-hero-mission {
    background-color: #2173b3; 
    color: #EFEFEF; /* Soft white for text on dark background */
    padding: 2rem 1rem; /* Padding around content sections */
}

.title, .subtitle {
    margin-bottom: 15px;
}
</style>
