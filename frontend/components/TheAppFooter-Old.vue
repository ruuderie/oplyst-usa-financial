<script setup>
import { reactive, computed } from "vue";

const currentYear = computed(() => {
  const today = new Date();
  return today.getFullYear();
});
</script>

<template>
  <footer class="footer py-6 finance-footer">
    <div class="container">
      <!-- Footer Sections -->
      <div class="columns is-multiline">
        <!-- About Section -->
        <div class="column is-3">
          <h5>About Us</h5>
          <ul>
            <li><NuxtLink to="/">Home</NuxtLink></li>
            <li><NuxtLink to="/services">Services</NuxtLink></li>
            <li><NuxtLink to="/team">Our Team</NuxtLink></li>
          </ul>
        </div>

        <!-- Quick Links -->
        <div class="column is-3">
          <h5>Quick Links</h5>
          <ul>
            <li><NuxtLink to="/terms">Terms of Service</NuxtLink></li>
            <li><NuxtLink to="/privacy">Privacy Policy</NuxtLink></li>
          </ul>
        </div>

        <!-- Contact Us -->
        <div class="column is-3">
          <h5>Contact Us</h5>
          <a target="_blank" href="tel:860-333-6524"> (860)-333-6524 </a><br />
          <a target="_blank" href="mailto:loans@oplystusa.com"> loans@oplystusa.com </a>
        </div>

        <!-- Follow Us -->
        <div class="column is-3">
          <h5>Follow Us</h5>
          <div class="flex items-center gap-3 -mx-1">
            <!-- nuxt link to linkedin url -->
            <NuxtLink href="https://www.linkedin.com/company/oplystusa" target="_blank">
              <icon name="mdi:linkedin" />
            </NuxtLink>
          </div>
        </div>
      </div>

      <!-- Copyright Information -->
      <div class="content has-text-centered">
        <p>
          © Oplyst International, LLC {{ currentYear }}. All rights reserved.
          <br />
          Made with
          <NuxtLink href="https://www.nuxtjs.org"
            ><Icon name="logos:nuxt" class="is-large"
          /></NuxtLink>
        </p>
      </div>
    </div>
  </footer>
</template>

<style scoped lang="scss">
.content {
  color: #ffffff;
}
.footer {
  background-color: #003366; // Matching with the navbar background color
  color: #ffffff; // Keeping the text color white for contrast
  border-top: 1px solid #e0e0e0; // Light gray border for separation from main content

  &.py-6 {
    padding: 3rem 1.5rem; // Increasing the padding to provide more breathing space
  }

  .content {
    h5 {
      font-weight: bold;
      margin-bottom: 15px;
      color: #d4a017; // Gold color for headings for emphasis
    }

    ul {
      list-style-type: none; // Remove bullet points
      padding: 0;

      li {
        margin-bottom: 10px; // Spacing between list items
        a {
          color: #ffffff; // White color for links

          &:hover {
            text-decoration: underline; // Underline links on hover for clarity
            color: #d4a017; // Gold color on hover to match the navbar's hover state
          }
        }
      }
    }
  }

  .icon {
    vertical-align: middle; // Vertically align the icon with the text
    margin-left: 5px; // A bit of space between the text and the icon
    color: #d4a017; // Gold color for icons
  }
}
</style>
