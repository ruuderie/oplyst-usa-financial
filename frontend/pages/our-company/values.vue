<script setup>
import { ref, onMounted } from 'vue';
</script>

<template>
    <div class="is-responsive is-mobile finance-theme">
        <NuxtLayout>
            <section class="hero finance-hero-values is-small is-primary is-fluid">
                <div class="hero-body is-fluid">
                    <div class="my-3 has-text-centered">
                        <p class="title">Values of Oplyst International, LLC</p>
                        <p class="subtitle">
                            Oplyst International isn't just a company – we're a mindset, a disruptive force. Our
                            strength lies in our unique blend of technology, logistics, and financial advisory. We hold
                            dear the values of quality, simplicity, and client satisfaction, ensuring that these
                            principles guide us in providing holistic solutions to complex challenges. With Oplyst,
                            you're not just getting a service – you're joining a forward-thinking community of
                            innovators and pioneers.
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
    /*background-color: #FAFAFA; /* Lighter gray background */
}

.finance-hero-values {
    background-color: #2173b3; 
    color: #EFEFEF; /* Darker navy blue for content */
    padding: 2rem 1rem; /* Padding around content sections */
}

.title, .subtitle {
    margin-bottom: 15px;
	color: #003366
}
</style>
