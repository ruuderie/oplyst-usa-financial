<script setup>
import { ref, onMounted } from "vue";

const post = {
  summary: "This is a summary",
  title: "This is a title",
  featured_image: "https://example.com/image.png",
};
</script>

<template>
  <div class="is-responsive is-mobile finance-theme">
    <NuxtLayout>
      <section class="hero finance-hero-overview is-small is-primary is-fluid">
        <div class="hero-body is-fluid">
          <div class="my-3 has-text-centered">
            <p class="title">Overview of Oplyst International, LLC</p>
            <p class="subtitle">
              Explore a future shaped by innovation with Oplyst International, LLC. As
              disruptors and pioneers, we challenge conventional industry boundaries,
              focusing on simplicity, quality, and service. Our impact is demonstrated
              through over 100 innovative solutions, partnerships with over 50 global
              entities, and a growing network of more than 1000 satisfied clients.
              Experience the disruptive force of Oplyst, trusted by leading enterprises
              worldwide.
            </p>
          </div>
        </div>
      </section>
    </NuxtLayout>
  </div>
</template>

<style scoped>
.finance-theme {
  font-family: "Goldman Sachs", sans-serif; /* Assuming the font is named Goldman Sachs */
  background-color: #f5f5f5; /* Soft gray background */
}

.finance-hero-overview {
  background-color: #2173b3;
  color: #efefef; /* Soft white for text on dark background */
  padding: 2rem 1rem; /* Padding around content sections */
}

.title,
.subtitle {
  margin-bottom: 15px;
}
</style>
