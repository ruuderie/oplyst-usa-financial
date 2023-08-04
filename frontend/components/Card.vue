<script setup>
const props = defineProps({
  type: {
    type: String,
    required: true
  },
  quote: {
    type: String,
    required: false
  },
  quoteAuthor: {
    type: String,
    required: false
  },
  socialMediaLinks: {
    type: Array,
    required: false,
    default: []
  },
  imageLink: {
    type: String,
    required: false
  },
  imageDescription: {
    type: String,
    required: false,
    default: "Placeholder image"
  }
});
</script>

<template>
  <!-- IF -->
  <div 
    v-if="type === 'quote'" 
    class="card is-small is-mobile">
    <div class="card-content">
      <p class="title">
        {{ quote }}
      </p>
      <p class="subtitle">
        {{ quoteAuthor }}
      </p>
    </div>
    <footer class="card-footer">
      <ul>
        <li
          v-for="socialMediaLink in socialMediaLinks"
          :key="socialMediaLink.name"
        >
          <p class="card-footer-item">
            <span>
              <NuxtLink
                :href="socialMediaLink.url"
                class=" tag card-footer-item"
              >
                {{ socialMediaLink.name }}
              </NuxtLink>
            </span>
          </p>
        </li>
      </ul>
    </footer>
  </div>
  <!-- ELSE IF -->
  <div v-else-if="type === 'social'" 
       class="card is-small is-mobile">
    <div class="card-content">
      <p class="title">
        {{ quote }}
      </p>
      <p class="subtitle">
        {{ quoteAuthor }}
      </p>
    </div>
    <footer class="card-footer">
      <ul>
        <li
          v-for="socialMediaLink in socialMediaLinks"
          :key="socialMediaLink.name"
          class="card-footer-item"
        >
          <NuxtLink 
            :href="socialMediaLink.url" 
            class=" tag card-footer-item">
            {{ socialMediaLink.name }}
          </NuxtLink>
        </li>
      </ul>
    </footer>
  </div>
  <!-- ELSE -->
  <div v-else>
    <div class="card is-mobile is-small">
      <div class="card-image">
        <figure class="image is-4by3">
          <NuxtImg 
            :src="imageLink" 
            :alt="imageDescription" />
        </figure>
      </div>
    </div>
  </div>
</template>

<style></style>
