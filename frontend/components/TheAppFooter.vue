<script setup>
import { reactive, computed } from "vue";

const currentYear = computed(() => {
  const today = new Date();
  return today.getFullYear();
});
</script>

<template>
  <footer>
    <div class="px-5">
      <div class="grid grid-cols-12 gap-x-3 gap-y-12">
        <div class="col-span-12 md:col-span-6 lg:col-span-3">
          <div class="flex items-start justify-start lg:justify-end">
            <div>
              <h5 class="mb-3">About Us</h5>
              <ul>
                <li><NuxtLink to="/">Home</NuxtLink></li>
                <li><NuxtLink to="/services">Services</NuxtLink></li>
                <li><NuxtLink to="/team">Our Team</NuxtLink></li>
              </ul>
            </div>
          </div>
        </div>
        <div class="col-span-12 md:col-span-6 lg:col-span-3">
          <div class="flex items-start justify-start md:justify-start lg:justify-center">
            <div>
              <h5 class="mb-3">Quick Links</h5>
              <ul>
                <li><NuxtLink to="/terms">Terms of Service</NuxtLink></li>
                <li><NuxtLink to="/privacy">Privacy Policy</NuxtLink></li>
              </ul>
            </div>
          </div>
        </div>
        <div class="col-span-12 md:col-span-6 lg:col-span-3">
          <div class="flex items-start justify-start md:justify-start lg:justify-center">
            <div>
              <h5 class="mb-3">Contact Us</h5>
              <a target="_blank" href="tel:860-333-6524"> (860)-333-6524 </a><br />
              <a target="_blank" href="mailto:loans@oplystusa.com">
                loans@oplystusa.com
              </a>
            </div>
          </div>
        </div>
        <div class="col-span-12 md:col-span-6 lg:col-span-3">
          <div class="flex items-start justify-start md:justify-start lg:justify-start">
            <div>
              <h5 class="mb-3">Follow Us</h5>
              <div class="flex items-start justify-start md:justify-start gap-1">
                <!-- nuxt link to linkedin url -->
                <NuxtLink
                  href="https://www.linkedin.com/company/oplystusa"
                  target="_blank"
                >
                  <icon name="mdi:linkedin" />
                </NuxtLink>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="content has-text-centered mt-12">
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
