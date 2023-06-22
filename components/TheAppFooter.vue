<script setup>
import { reactive, computed } from 'vue'

const currentYear = computed(() => {
      const today = new Date()
      return today.getFullYear()
});
</script>

<template>
    <footer class="footer py-6">
      <div class="content has-text-centered">
        <p>
          © Ruud Salym Erie {{ currentYear }}.
          <br/>
          Made with
          <NuxtLink href="https://www.nuxtjs.org" >  <Icon name="logos:nuxt" class="is-large" /></NuxtLink> 
    
        <br/>
        AWS is a trademark of Amazon.com, Inc. or its affiliates.
      </p>
      </div>
    </footer>
</template>

<style lang="scss">

@import '../styles/_settings.scss';

</style>