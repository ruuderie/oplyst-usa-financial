<script setup>
import { defineProps } from 'vue';

const {
  id,
  name,
  description,
  imageSource
} = defineProps({
  id: {
    type: String,
    required: true
  },
  name: String,
  description: String,
  imageSource: String
});
</script>

<template>
<div class="card is-6">
  <div v-if="imageSource" class="card-image">
    <figure class="image is-4by3">
      <nuxt-img class="img" 
        :src="imageSource"
        alt="Oplyst International - Pioneering Commercial Lending Solutions" 
      />
    </figure>
  </div>
  <div class="card-content">
    <div class="media">
      <div class="media-content">
        <p class="card-title is-4">{{ name }}</p>
      </div>
    </div>

    <div class="content">
      {{ description }}
      <br>
    </div>
  </div>
</div>
</template>

<style scoped>
.card {
    border: 1px solid #e0e0e0; /* Light border to give definition */
    border-radius: 5px; /* Soften the corners */
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1); /* Subtle shadow for depth */
    transition: box-shadow 0.3s ease-in-out; /* For hover effect */
}

.card:hover {
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.15); /* Slightly more pronounced shadow on hover */
}

.card-image figure {
    border-radius: 5px 5px 0 0; /* Round the top corners to match the card */
    overflow: hidden; /* Ensures the image conforms to the border radius */
}

.card-content {
    padding: 2rem; /* A bit more padding for breathing room */
}

.card-title {
    color: #003366; /* Dark blue for the title */
    font-weight: bold; /* Bold for emphasis */
    margin-bottom: 1.5rem; /* Space between title and content */
}

.content {
    color: #333333; /* A slightly softer color for the text, good for reading */
    font-size: 1.2rem; /* Clear, readable font size */
}
</style>
