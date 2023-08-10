<template>
  <div class="finance-error-container finance-theme">
    <h1 v-if="error.statusCode === 404" class="error-title">Page not found</h1>
    <h1 v-else class="error-title">An error occurred</h1>
    <NuxtLink to="/" class="finance-link">Return to Home page</NuxtLink>
  </div>
</template>

<script>
export default {
  props: ['error'],
  layout: 'error' // custom layout for the error page
}
</script>

<style scoped>
.finance-theme {
  font-family: 'Goldman Sachs', sans-serif; /* Assuming the font is named Goldman Sachs */
  background-color: #F5F5F5; /* Soft gray background */
  color: #2E3A46; /* Dark navy blue for text */
}

.finance-error-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh; /* Taking full viewport height to center content */
  padding: 2rem;
  text-align: center;
}

.error-title {
  font-size: 2rem;
  margin-bottom: 1rem;
}

.finance-link {
  color: #2E3A46; /* Dark navy blue for the link */
  font-size: 1rem;
  border-bottom: 1px solid transparent;

  &:hover {
      border-bottom: 1px solid #2E3A46;
  }
}
</style>
